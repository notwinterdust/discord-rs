//! Transport-driven gateway connection loop.
//!
//! [`GatewayConnection`] feeds received frames into a [`GatewayRuntime`], sends
//! the actions the runtime requests, and drives heartbeats and reconnects. The
//! transport performs the actual I/O: an implementation owns the socket, the
//! WebSocket framing and any TLS layer, and exchanges JSON text frames and
//! close codes with this loop.

use std::error::Error;
use std::fmt;
use std::thread;
use std::time::{Duration, Instant};

use data::{GatewayCloseCode, GatewayOpcode, GatewayPayload, GatewayPayloadError};
use models::CastError;

use crate::events::{EventDispatcher, GatewayRuntime, GatewayUpdate, RuntimeError};
use crate::gateway::{GatewayAction, GatewayConfig, GatewayError, GatewayState};

/// The longest the loop waits for a frame when no heartbeat is scheduled.
const IDLE_TIMEOUT: Duration = Duration::from_secs(60);

/// An error reported by a gateway transport.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GatewayTransportError {
    /// A human-readable description of the failure.
    pub message: String,
}

impl GatewayTransportError {
    /// Creates a transport error with the given description.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for GatewayTransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for GatewayTransportError {}

impl From<String> for GatewayTransportError {
    fn from(message: String) -> Self {
        Self::new(message)
    }
}

impl From<&str> for GatewayTransportError {
    fn from(message: &str) -> Self {
        Self::new(message)
    }
}

/// A frame received from a gateway transport.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GatewayFrame {
    /// A JSON text frame.
    Text(String),
    /// The peer closed the connection with this code.
    Close(GatewayCloseCode),
    /// No frame arrived before the timeout.
    Timeout,
}

/// WebSocket-level frames required by a gateway transport.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WebSocketFrame {
    /// UTF-8 application data.
    Text(String),
    /// Binary application data.
    Binary(Vec<u8>),
    /// A ping payload.
    Ping(Vec<u8>),
    /// A pong payload.
    Pong(Vec<u8>),
    /// A close frame.
    Close(GatewayCloseCode),
}

/// A WebSocket implementation supplied by the host application.
pub trait WebSocketTransport {
    /// Opens a WebSocket connection.
    fn open(&mut self, url: &str) -> Result<(), GatewayTransportError>;
    /// Receives one frame.
    fn receive(&mut self) -> Result<WebSocketFrame, GatewayTransportError>;
    /// Sends one frame.
    fn send_frame(&mut self, frame: WebSocketFrame) -> Result<(), GatewayTransportError>;
    /// Closes the connection.
    fn close(&mut self);
}

/// Performs gateway I/O for [`GatewayConnection`].
///
/// Implementations own the socket and handle WebSocket framing and TLS. The
/// connection loop only deals in JSON text frames and Discord close codes.
pub trait GatewayTransport {
    /// Opens a connection to the given gateway URL.
    ///
    /// # Errors
    ///
    /// Returns [`GatewayTransportError`] when the connection cannot be opened.
    fn connect(&mut self, url: &str) -> Result<(), GatewayTransportError>;

    /// Receives the next frame, waiting at most `timeout`.
    ///
    /// Returning [`GatewayFrame::Timeout`] is not an error.
    ///
    /// # Errors
    ///
    /// Returns [`GatewayTransportError`] when the connection fails.
    fn recv(&mut self, timeout: Option<Duration>) -> Result<GatewayFrame, GatewayTransportError>;

    /// Sends one JSON text frame.
    ///
    /// # Errors
    ///
    /// Returns [`GatewayTransportError`] when the frame cannot be sent.
    fn send(&mut self, text: &str) -> Result<(), GatewayTransportError>;

    /// Closes the connection, if it is open.
    fn close(&mut self);
}

impl<T: GatewayTransport + ?Sized> GatewayTransport for Box<T> {
    fn connect(&mut self, url: &str) -> Result<(), GatewayTransportError> {
        (**self).connect(url)
    }

    fn recv(&mut self, timeout: Option<Duration>) -> Result<GatewayFrame, GatewayTransportError> {
        (**self).recv(timeout)
    }

    fn send(&mut self, text: &str) -> Result<(), GatewayTransportError> {
        (**self).send(text)
    }

    fn close(&mut self) {
        (**self).close();
    }
}

/// An error produced by a [`GatewayConnection`].
#[derive(Clone, Debug, PartialEq)]
pub enum GatewayConnectionError {
    /// The transport failed.
    Transport(GatewayTransportError),
    /// A received frame was not a valid gateway payload.
    Payload(GatewayPayloadError),
    /// The gateway lifecycle rejected a payload.
    Gateway(GatewayError),
    /// A dispatch event's data was malformed.
    Event(CastError),
}

impl fmt::Display for GatewayConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(error) => write!(f, "gateway transport error: {error}"),
            Self::Payload(error) => write!(f, "gateway payload error: {error}"),
            Self::Gateway(error) => write!(f, "gateway error: {error}"),
            Self::Event(error) => write!(f, "gateway event error: {error}"),
        }
    }
}

impl Error for GatewayConnectionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Transport(error) => Some(error),
            Self::Payload(error) => Some(error),
            Self::Gateway(error) => Some(error),
            Self::Event(error) => Some(error),
        }
    }
}

impl From<GatewayTransportError> for GatewayConnectionError {
    fn from(error: GatewayTransportError) -> Self {
        Self::Transport(error)
    }
}

impl From<GatewayPayloadError> for GatewayConnectionError {
    fn from(error: GatewayPayloadError) -> Self {
        Self::Payload(error)
    }
}

impl From<GatewayError> for GatewayConnectionError {
    fn from(error: GatewayError) -> Self {
        Self::Gateway(error)
    }
}

impl From<CastError> for GatewayConnectionError {
    fn from(error: CastError) -> Self {
        Self::Event(error)
    }
}

impl From<RuntimeError> for GatewayConnectionError {
    fn from(error: RuntimeError) -> Self {
        match error {
            RuntimeError::Gateway(error) => Self::Gateway(error),
            RuntimeError::Event(error) => Self::Event(error),
        }
    }
}

/// A gateway runtime driven by a [`GatewayTransport`].
///
/// The connection loop is synchronous and blocking. [`GatewayConnection::poll`]
/// performs one receive/action cycle, while [`GatewayConnection::run`] keeps
/// polling, reconnects when requested and returns when the gateway closes
/// permanently.
pub struct GatewayConnection<T> {
    runtime: GatewayRuntime,
    transport: T,
    next_heartbeat: Option<Instant>,
    needs_reconnect: bool,
    closed: bool,
    close_code: Option<GatewayCloseCode>,
}

impl<T: GatewayTransport> GatewayConnection<T> {
    /// Creates a connection with an empty dispatcher.
    #[must_use]
    pub fn new(config: GatewayConfig, transport: T) -> Self {
        Self::with_dispatcher(config, transport, EventDispatcher::new())
    }

    /// Creates a connection with the given dispatcher.
    #[must_use]
    pub fn with_dispatcher(config: GatewayConfig, transport: T, events: EventDispatcher) -> Self {
        Self {
            runtime: GatewayRuntime::with_dispatcher(config, events),
            transport,
            next_heartbeat: None,
            needs_reconnect: false,
            closed: false,
            close_code: None,
        }
    }

    /// Returns the underlying runtime.
    #[must_use]
    pub const fn runtime(&self) -> &GatewayRuntime {
        &self.runtime
    }

    /// Returns the underlying runtime mutably.
    pub fn runtime_mut(&mut self) -> &mut GatewayRuntime {
        &mut self.runtime
    }

    /// Returns the transport.
    #[must_use]
    pub const fn transport(&self) -> &T {
        &self.transport
    }

    /// Returns the transport mutably.
    pub fn transport_mut(&mut self) -> &mut T {
        &mut self.transport
    }

    /// Consumes the connection and returns its runtime and transport.
    #[must_use]
    pub fn into_parts(self) -> (GatewayRuntime, T) {
        (self.runtime, self.transport)
    }

    /// Registers a handler for the given filter.
    pub fn on(
        &mut self,
        filter: impl Into<crate::events::EventFilter>,
        handler: impl FnMut(&models::Event) + 'static,
    ) -> &mut Self {
        self.runtime.on(filter, handler);
        self
    }

    /// Registers a handler for one gateway event name.
    pub fn on_name(
        &mut self,
        name: impl Into<String>,
        handler: impl FnMut(&models::Event) + 'static,
    ) -> &mut Self {
        self.runtime.on_name(name, handler);
        self
    }

    /// Registers a handler for every event.
    pub fn on_any(&mut self, handler: impl FnMut(&models::Event) + 'static) -> &mut Self {
        self.runtime.on_any(handler);
        self
    }

    /// Returns the time of the next scheduled heartbeat.
    #[must_use]
    pub const fn next_heartbeat(&self) -> Option<Instant> {
        self.next_heartbeat
    }

    /// Returns `true` if the loop should reconnect before polling again.
    #[must_use]
    pub const fn needs_reconnect(&self) -> bool {
        self.needs_reconnect
    }

    /// Returns `true` if the gateway closed permanently.
    #[must_use]
    pub const fn is_closed(&self) -> bool {
        self.closed
    }

    /// Returns the close code that ended the connection, if any.
    #[must_use]
    pub const fn close_code(&self) -> Option<GatewayCloseCode> {
        self.close_code
    }

    /// Opens the transport and marks the gateway socket as open.
    ///
    /// # Errors
    ///
    /// Returns an error when the transport cannot connect or the gateway is not
    /// in a state that accepts a new connection.
    pub fn connect(&mut self) -> Result<(), GatewayConnectionError> {
        let state = self.runtime.state();
        if !matches!(
            state,
            GatewayState::Disconnected | GatewayState::Reconnecting
        ) {
            return Err(GatewayError::InvalidState { state }.into());
        }

        let url = self.runtime.gateway().url().to_owned();
        self.transport.connect(&url)?;
        if let Err(error) = self.runtime.on_connected() {
            self.transport.close();
            return Err(error.into());
        }
        self.next_heartbeat = None;
        self.needs_reconnect = false;
        self.closed = false;
        self.close_code = None;
        Ok(())
    }

    /// Handles a JSON text frame and sends any action it produced.
    ///
    /// # Errors
    ///
    /// Returns an error when the frame is not a gateway payload, the lifecycle
    /// rejects it, an event is malformed, or a requested action cannot be sent.
    pub fn handle_text(&mut self, text: &str) -> Result<GatewayUpdate, GatewayConnectionError> {
        let payload = GatewayPayload::from_json(text)?;
        self.handle_payload(&payload)
    }

    /// Handles a decoded gateway payload and sends any action it produced.
    ///
    /// # Errors
    ///
    /// Returns an error when the lifecycle rejects the payload, an event is
    /// malformed, or a requested action cannot be sent.
    pub fn handle_payload(
        &mut self,
        payload: &GatewayPayload,
    ) -> Result<GatewayUpdate, GatewayConnectionError> {
        let update = self.runtime.handle_payload(payload)?;
        if let Some(action) = update.action.clone() {
            self.perform_action(action)?;
        }
        if payload.op == GatewayOpcode::Hello {
            self.schedule_heartbeat(Instant::now());
        }
        Ok(update)
    }

    /// Performs one receive, handle and heartbeat cycle.
    ///
    /// # Errors
    ///
    /// Returns an error when the transport fails or a received frame cannot be
    /// handled.
    pub fn poll(&mut self) -> Result<Option<GatewayUpdate>, GatewayConnectionError> {
        if self.closed {
            return Ok(None);
        }

        let timeout = self.poll_timeout(Instant::now());
        let frame = self.transport.recv(Some(timeout))?;
        let update = match frame {
            GatewayFrame::Text(text) => Some(self.handle_text(&text)?),
            GatewayFrame::Close(code) => {
                let action = self.runtime.gateway_mut().on_close(code);
                self.perform_action(action)?;
                None
            }
            GatewayFrame::Timeout => None,
        };

        self.tick(Instant::now())?;
        Ok(update)
    }

    /// Sends a heartbeat when `now` has reached the next scheduled time.
    ///
    /// Returns the action performed, if any. A missed heartbeat acknowledgement
    /// produces [`GatewayAction::Reconnect`].
    ///
    /// # Errors
    ///
    /// Returns an error when the heartbeat cannot be sent.
    pub fn tick(&mut self, now: Instant) -> Result<Option<GatewayAction>, GatewayConnectionError> {
        let Some(deadline) = self.next_heartbeat else {
            return Ok(None);
        };
        if now < deadline {
            return Ok(None);
        }

        let action = self.runtime.heartbeat();
        match &action {
            Some(GatewayAction::Reconnect) => {
                self.perform_action(GatewayAction::Reconnect)?;
            }
            Some(action) => {
                self.perform_action(action.clone())?;
                self.schedule_heartbeat(now);
            }
            None => {
                self.next_heartbeat = None;
            }
        }
        Ok(action)
    }

    /// Blocks until the gateway closes permanently.
    ///
    /// Transport failures and reconnect requests are recovered by reconnecting
    /// with the configured backoff. A reconnect only closes the socket and
    /// opens a new one; the gateway lifecycle decides whether to identify or
    /// resume after the new `HELLO`.
    ///
    /// # Errors
    ///
    /// Returns an error when connecting or reconnecting fails, or when a
    /// received frame cannot be handled.
    pub fn run(&mut self) -> Result<(), GatewayConnectionError> {
        if self.closed || self.runtime.state() == GatewayState::Closed {
            self.closed = true;
            return Ok(());
        }
        if matches!(
            self.runtime.state(),
            GatewayState::Disconnected | GatewayState::Reconnecting
        ) {
            self.connect()?;
        }

        loop {
            if self.closed {
                return Ok(());
            }
            if self.needs_reconnect {
                self.reconnect()?;
                continue;
            }

            match self.poll() {
                Ok(_) => {}
                Err(GatewayConnectionError::Transport(_)) => {
                    let _ = self.runtime.gateway_mut().on_transport_error();
                    self.needs_reconnect = true;
                    self.next_heartbeat = None;
                }
                Err(error) => return Err(error),
            }
        }
    }

    /// Closes the transport and reconnects with the configured backoff.
    ///
    /// # Errors
    ///
    /// Returns an error when the new connection cannot be opened.
    pub fn reconnect(&mut self) -> Result<(), GatewayConnectionError> {
        self.transport.close();
        self.next_heartbeat = None;
        self.needs_reconnect = false;

        let delay = self.runtime.gateway().reconnect_delay();
        if !delay.is_zero() {
            thread::sleep(delay);
        }
        self.connect()
    }

    fn perform_action(&mut self, action: GatewayAction) -> Result<(), GatewayConnectionError> {
        match action {
            GatewayAction::Send(payload) => {
                self.transport.send(&payload.to_json())?;
            }
            GatewayAction::Reconnect => {
                self.needs_reconnect = true;
                self.next_heartbeat = None;
            }
            GatewayAction::Close(code) => {
                self.transport.close();
                self.close_code = Some(code);
                self.closed = true;
                self.needs_reconnect = false;
                self.next_heartbeat = None;
            }
        }
        Ok(())
    }

    fn schedule_heartbeat(&mut self, now: Instant) {
        self.next_heartbeat = self
            .runtime
            .gateway()
            .heartbeat_interval()
            .map(|interval| now + interval);
    }

    fn poll_timeout(&self, now: Instant) -> Duration {
        match self.next_heartbeat {
            Some(deadline) => deadline.saturating_duration_since(now).min(IDLE_TIMEOUT),
            None => IDLE_TIMEOUT,
        }
    }
}

impl<T: GatewayTransport> fmt::Debug for GatewayConnection<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GatewayConnection")
            .field("runtime", &self.runtime)
            .field("next_heartbeat", &self.next_heartbeat)
            .field("needs_reconnect", &self.needs_reconnect)
            .field("closed", &self.closed)
            .field("close_code", &self.close_code)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::time::Duration;

    use data::gateway::{GatewayOpcode, GatewayPayload, Hello};
    use data::{GatewayCloseCode, Value};
    use enums::Intents;

    use super::{
        GatewayConnection, GatewayConnectionError, GatewayFrame, GatewayTransport,
        GatewayTransportError,
    };
    use crate::gateway::{GatewayAction, GatewayConfig, GatewayError, GatewayState};
    use models::Event;

    #[derive(Debug, Default)]
    struct MockTransport {
        connected: Vec<String>,
        sent: Vec<GatewayPayload>,
        frames: VecDeque<GatewayFrame>,
        closes: u32,
        fail_connect: bool,
    }

    impl GatewayTransport for MockTransport {
        fn connect(&mut self, url: &str) -> Result<(), GatewayTransportError> {
            if self.fail_connect {
                return Err(GatewayTransportError::new("connect failed"));
            }
            self.connected.push(url.to_owned());
            Ok(())
        }

        fn recv(
            &mut self,
            _timeout: Option<Duration>,
        ) -> Result<GatewayFrame, GatewayTransportError> {
            Ok(self.frames.pop_front().unwrap_or(GatewayFrame::Timeout))
        }

        fn send(&mut self, text: &str) -> Result<(), GatewayTransportError> {
            self.sent.push(GatewayPayload::from_json(text).unwrap());
            Ok(())
        }

        fn close(&mut self) {
            self.closes += 1;
        }
    }

    fn config() -> GatewayConfig {
        let mut config = GatewayConfig::new("token", Intents::none());
        config.reconnect = crate::ReconnectPolicy::new(Duration::ZERO, Duration::ZERO);
        config
    }

    fn hello(interval: u64) -> String {
        GatewayPayload::new(
            GatewayOpcode::Hello,
            Hello {
                heartbeat_interval: interval,
            }
            .to_value(),
        )
        .to_json()
    }

    fn ready() -> String {
        GatewayPayload::from_json(
            r#"{
                "op": 0,
                "s": 1,
                "t": "READY",
                "d": {
                    "user": {"id": "1", "username": "winter"},
                    "session_id": "session",
                    "resume_gateway_url": "wss://resume.example",
                    "guilds": []
                }
            }"#,
        )
        .unwrap()
        .to_json()
    }

    fn resumed() -> String {
        GatewayPayload::new(GatewayOpcode::Dispatch, Value::Null)
            .with_sequence(2)
            .with_event("RESUMED")
            .to_json()
    }

    fn message() -> String {
        GatewayPayload::from_json(
            r#"{
                "op": 0,
                "s": 2,
                "t": "MESSAGE_CREATE",
                "d": {
                    "id": "10",
                    "channel_id": "4",
                    "author": {"id": "6", "username": "winter"},
                    "content": "hello",
                    "timestamp": "2021-08-01T12:34:56Z"
                }
            }"#,
        )
        .unwrap()
        .to_json()
    }

    #[test]
    fn run_connects_handles_frames_and_stops_on_close() {
        let mut transport = MockTransport::default();
        transport.frames.push_back(GatewayFrame::Text(hello(1_000)));
        transport.frames.push_back(GatewayFrame::Text(ready()));
        transport.frames.push_back(GatewayFrame::Text(message()));
        transport
            .frames
            .push_back(GatewayFrame::Close(GatewayCloseCode::AUTHENTICATION_FAILED));

        let mut connection = GatewayConnection::new(config(), transport);
        let received = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = std::rc::Rc::clone(&received);
        connection.on_name("MESSAGE_CREATE", move |event| {
            if let Event::MessageCreate(message) = event {
                seen.borrow_mut().push(message.content.clone());
            }
        });

        connection.run().unwrap();

        assert_eq!(connection.runtime().state(), GatewayState::Closed);
        assert!(connection.is_closed());
        assert_eq!(
            connection.close_code(),
            Some(GatewayCloseCode::AUTHENTICATION_FAILED)
        );
        assert_eq!(*received.borrow(), vec!["hello"]);

        let transport = connection.transport();
        assert_eq!(transport.connected.len(), 1);
        assert_eq!(transport.closes, 1);
        assert_eq!(transport.sent[0].op, GatewayOpcode::Identify);
    }

    #[test]
    fn reconnect_resumes_the_existing_session() {
        let transport = MockTransport::default();
        let mut connection = GatewayConnection::new(config(), transport);
        connection.connect().unwrap();
        connection.handle_text(&hello(1_000)).unwrap();
        connection.handle_text(&ready()).unwrap();

        connection
            .transport_mut()
            .frames
            .push_back(GatewayFrame::Close(GatewayCloseCode::SESSION_TIMED_OUT));
        assert!(connection.poll().unwrap().is_none());
        assert!(connection.needs_reconnect());
        assert_eq!(connection.runtime().state(), GatewayState::Reconnecting);

        connection.reconnect().unwrap();
        assert_eq!(connection.runtime().state(), GatewayState::Connecting);
        assert_eq!(
            connection.transport().connected.last().map(String::as_str),
            Some("wss://resume.example")
        );

        connection.handle_text(&hello(1_000)).unwrap();
        assert_eq!(
            connection.transport().sent.last().unwrap().op,
            GatewayOpcode::Resume
        );
        connection.handle_text(&resumed()).unwrap();
        assert_eq!(connection.runtime().state(), GatewayState::Connected);
    }

    #[test]
    fn heartbeats_are_scheduled_and_a_missed_ack_reconnects() {
        let mut connection = GatewayConnection::new(config(), MockTransport::default());
        connection.connect().unwrap();
        connection.handle_text(&hello(1_000)).unwrap();

        let deadline = connection.next_heartbeat().unwrap();
        assert!(
            connection
                .tick(deadline - Duration::from_millis(1))
                .unwrap()
                .is_none()
        );

        let action = connection.tick(deadline).unwrap().unwrap();
        assert!(matches!(action, GatewayAction::Send(_)));
        assert_eq!(
            connection.transport().sent.last().unwrap().op,
            GatewayOpcode::Heartbeat
        );

        let ack = GatewayPayload::new(GatewayOpcode::HeartbeatAck, Value::Null).to_json();
        connection.handle_text(&ack).unwrap();

        let deadline = connection.next_heartbeat().unwrap();
        let action = connection.tick(deadline).unwrap().unwrap();
        assert!(matches!(action, GatewayAction::Send(_)));

        let deadline = connection.next_heartbeat().unwrap();
        let action = connection.tick(deadline).unwrap().unwrap();
        assert_eq!(action, GatewayAction::Reconnect);
        assert!(connection.needs_reconnect());
    }

    #[test]
    fn connect_rejects_duplicate_connections() {
        let mut connection = GatewayConnection::new(config(), MockTransport::default());
        connection.connect().unwrap();

        assert_eq!(
            connection.connect(),
            Err(GatewayConnectionError::Gateway(
                GatewayError::InvalidState {
                    state: GatewayState::Connecting
                }
            ))
        );
        assert_eq!(connection.transport().connected.len(), 1);
    }

    #[test]
    fn transport_connect_errors_are_reported() {
        let transport = MockTransport {
            fail_connect: true,
            ..MockTransport::default()
        };
        let mut connection = GatewayConnection::new(config(), transport);
        let error = connection.connect().unwrap_err();
        assert_eq!(error.to_string(), "gateway transport error: connect failed");
        assert_eq!(connection.runtime().state(), GatewayState::Disconnected);
    }

    #[test]
    fn ticks_before_hello_do_nothing() {
        let mut connection = GatewayConnection::new(config(), MockTransport::default());
        connection.connect().unwrap();
        assert!(
            connection
                .tick(std::time::Instant::now())
                .unwrap()
                .is_none()
        );
        assert!(connection.poll().unwrap().is_none());
    }
}
