//! Integration tests covering a full gateway + REST round-trip.
//!
//! The mock gateway transport replays real gateway frames so that
//! [`GatewayConnection`], [`ThreadedCoordinator`], [`ThreadedEventDispatcher`],
//! and [`EventWaiter`] all exercise the full happy-path together.
//!
//! The REST section validates that [`HttpRequest`] is constructed with correct
//! paths and methods for the most common Discord endpoints.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use clients::{
    EventWaiter, GatewayConfig, GatewayConnection, GatewayFrame, GatewayTransport,
    GatewayTransportError, HttpMethod, HttpRequest, ThreadedCoordinator,
    ThreadedEventDispatcher,
};
use data::{
    gateway::{GatewayOpcode, GatewayPayload, Hello},
    Object, Value,
};
use enums::Intents;
use models::Event;

// ---------------------------------------------------------------------------
// Mock transport
// ---------------------------------------------------------------------------

struct MockTransport {
    frames: VecDeque<GatewayFrame>,
    sent: Arc<Mutex<Vec<String>>>,
}

impl GatewayTransport for MockTransport {
    fn connect(&mut self, _url: &str) -> Result<(), GatewayTransportError> {
        Ok(())
    }

    fn recv(&mut self, _timeout: Option<Duration>) -> Result<GatewayFrame, GatewayTransportError> {
        if let Some(frame) = self.frames.pop_front() {
            Ok(frame)
        } else {
            thread::sleep(Duration::from_millis(10));
            Ok(GatewayFrame::Timeout)
        }
    }

    fn send(&mut self, text: &str) -> Result<(), GatewayTransportError> {
        self.sent.lock().unwrap().push(text.to_owned());
        Ok(())
    }

    fn close(&mut self) {}
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Validates that [`HttpRequest`] is constructed with the correct HTTP method
/// and path for the most-used Discord REST endpoints.
#[test]
fn rest_request_construction() {
    // GET /channels/{id}
    let req = HttpRequest::new(HttpMethod::Get, "channels/123456789");
    assert_eq!(req.method, HttpMethod::Get);
    assert!(req.path.contains("123456789"), "path: {}", req.path);

    // POST /channels/{id}/messages
    let msg_req = HttpRequest::new(HttpMethod::Post, "channels/123456789/messages");
    assert_eq!(msg_req.method, HttpMethod::Post);
    assert!(msg_req.path.contains("messages"), "path: {}", msg_req.path);

    // GET /guilds/{id}/members
    let members_req = HttpRequest::new(HttpMethod::Get, "guilds/987654321/members");
    assert_eq!(members_req.method, HttpMethod::Get);
    assert!(
        members_req.path.contains("987654321"),
        "path: {}",
        members_req.path
    );
    assert!(
        members_req.path.contains("members"),
        "path: {}",
        members_req.path
    );
}

/// Runs a scripted gateway lifecycle — Hello → Identify → Ready → TypingStart →
/// HeartbeatAck — through [`GatewayConnection`] running on a background thread
/// via [`ThreadedCoordinator`], fanning events to both a
/// [`ThreadedEventDispatcher`] and an [`EventWaiter`].
#[test]
fn gateway_full_lifecycle_round_trip() {
    // -----------------------------------------------------------------------
    // Build scripted gateway frames
    // -----------------------------------------------------------------------
    let mut frames = VecDeque::new();

    // 1. Hello
    let hello = GatewayPayload::new(
        GatewayOpcode::Hello,
        Hello { heartbeat_interval: 10 }.to_value(),
    )
    .to_json();
    frames.push_back(GatewayFrame::Text(hello));

    // 2. Dispatch: READY
    let mut ready_data = Object::new();
    ready_data.insert("v", Value::from(10_u64));

    let mut user_obj = Object::new();
    user_obj.insert("id", Value::String("1".into()));
    user_obj.insert("username", Value::String("testbot".into()));
    user_obj.insert("discriminator", Value::String("0000".into()));
    user_obj.insert("bot", Value::Bool(true));
    user_obj.insert("mfa_enabled", Value::Bool(false));
    user_obj.insert("verified", Value::Bool(false));
    user_obj.insert("flags", Value::from(0_u64));
    user_obj.insert("premium_type", Value::from(0_u64));
    user_obj.insert("public_flags", Value::from(0_u64));
    ready_data.insert("user", Value::Object(user_obj));
    ready_data.insert("guilds", Value::array());
    ready_data.insert("session_id", Value::String("session_abc".into()));
    ready_data.insert(
        "resume_gateway_url",
        Value::String("wss://gateway.discord.gg".into()),
    );

    let ready_frame = GatewayPayload::new(GatewayOpcode::Dispatch, Value::Object(ready_data))
        .with_sequence(1)
        .with_event("READY")
        .to_json();
    frames.push_back(GatewayFrame::Text(ready_frame));

    // 3. Dispatch: TYPING_START
    let mut typing_data = Object::new();
    typing_data.insert("channel_id", Value::String("111".into()));
    typing_data.insert("user_id", Value::String("222".into()));
    typing_data.insert("timestamp", Value::from(1_700_000_000_u64));
    typing_data.insert("guild_id", Value::String("333".into()));

    let typing_frame =
        GatewayPayload::new(GatewayOpcode::Dispatch, Value::Object(typing_data))
            .with_sequence(2)
            .with_event("TYPING_START")
            .to_json();
    frames.push_back(GatewayFrame::Text(typing_frame));

    // 4. Heartbeat ACK
    let ack = GatewayPayload::new(GatewayOpcode::HeartbeatAck, Value::Null).to_json();
    frames.push_back(GatewayFrame::Text(ack));

    // -----------------------------------------------------------------------
    // Wire transport + dispatcher + waiter
    // -----------------------------------------------------------------------
    let sent = Arc::new(Mutex::new(Vec::<String>::new()));
    let sent_clone = Arc::clone(&sent);

    let transport = MockTransport {
        frames,
        sent: sent_clone,
    };

    let dispatch_count = Arc::new(AtomicUsize::new(0));
    let dc = Arc::clone(&dispatch_count);

    let dispatcher = ThreadedEventDispatcher::builder()
        .on_name("TYPING_START", move |_: &Event| {
            dc.fetch_add(1, Ordering::SeqCst);
        })
        .build();

    let waiter = Arc::new(EventWaiter::new());
    let waiter_clone = Arc::clone(&waiter);

    let config = GatewayConfig::new("Bot test-token", Intents::none());

    let mut coordinator = ThreadedCoordinator::spawn(move |flag: Arc<AtomicBool>| {
        let mut conn = GatewayConnection::new(config, transport);
        conn.connect()?;
        while !flag.load(Ordering::SeqCst) {
            match conn.poll() {
                Ok(Some(update)) => {
                    if let Some(event) = update.event.clone() {
                        let _ = dispatcher.dispatch(event.clone());
                        waiter_clone.dispatch(&event);
                    }
                }
                Ok(None) => {
                    thread::sleep(Duration::from_millis(5));
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    });

    // Wait for the TYPING_START event via EventWaiter.
    let waited = waiter.wait_for(
        |e| matches!(e, Event::TypingStart(_)),
        Duration::from_secs(2),
    );
    assert!(waited.is_ok(), "wait_for timed out or disconnected");

    thread::sleep(Duration::from_millis(20));
    assert!(coordinator.shutdown().is_ok());

    // The dispatcher should have received the TYPING_START.
    assert_eq!(dispatch_count.load(Ordering::SeqCst), 1);

    // The gateway should have sent at least one Identify payload (op 2).
    let sends = sent.lock().unwrap();
    let identified = sends
        .iter()
        .any(|s| s.contains("\"op\":2") || s.contains("\"op\": 2"));
    assert!(identified, "No Identify was sent. Sends: {sends:?}");
}

/// Demonstrates that REST request construction and gateway lifecycle are
/// independent — a REST [`HttpRequest`] can be built while a gateway
/// coordinator runs on a background thread without either blocking the other.
#[test]
fn gateway_and_rest_concurrent_non_blocking() {
    // Start a minimal gateway background thread.
    let frames: VecDeque<GatewayFrame> = VecDeque::new();
    let transport = MockTransport {
        frames,
        sent: Arc::new(Mutex::new(Vec::new())),
    };

    let config = GatewayConfig::new("Bot test-token", Intents::none());

    let mut coordinator = ThreadedCoordinator::spawn(move |flag: Arc<AtomicBool>| {
        let mut conn = GatewayConnection::new(config, transport);
        conn.connect()?;
        while !flag.load(Ordering::SeqCst) {
            match conn.poll() {
                Ok(_) => thread::sleep(Duration::from_millis(5)),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    });

    // Build REST requests from the main thread concurrently.
    let req = HttpRequest::new(HttpMethod::Get, "channels/123");
    assert_eq!(req.method, HttpMethod::Get);
    assert!(req.path.contains("123"));

    // Both can be torn down cleanly.
    assert!(coordinator.shutdown().is_ok());
}
