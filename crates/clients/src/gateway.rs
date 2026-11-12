//! Gateway connection state machine.
//!
//! [`Gateway`] turns gateway payloads into [`GatewayAction`] values. It does not
//! own a socket or a timer: a transport delivers payloads, sends the requested
//! actions and calls [`Gateway::heartbeat`] when the heartbeat interval elapses.
//! Keeping the lifecycle pure makes reconnects, resumes and heartbeats testable.

use std::error::Error;
use std::fmt;
use std::time::Duration;

use data::{
    GatewayCloseCode, GatewayOpcode, GatewayPayload, GatewayPayloadError, Heartbeat, Hello,
    Identify, IdentifyProperties, InvalidSession, Resume, ShardId,
};
use enums::Intents;

/// The phase of a gateway connection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GatewayState {
    /// No connection is open.
    Disconnected,
    /// The socket is open and waiting for `HELLO`.
    Connecting,
    /// An identify payload has been sent.
    Identifying,
    /// A resume payload has been sent.
    Resuming,
    /// A session is active.
    Connected,
    /// The connection was lost and should be re-established.
    Reconnecting,
    /// The gateway session has ended permanently.
    Closed,
}

impl GatewayState {
    /// Returns the lowercase state name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disconnected => "disconnected",
            Self::Connecting => "connecting",
            Self::Identifying => "identifying",
            Self::Resuming => "resuming",
            Self::Connected => "connected",
            Self::Reconnecting => "reconnecting",
            Self::Closed => "closed",
        }
    }
}

impl fmt::Display for GatewayState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// An action requested by the gateway state machine.
#[derive(Clone, Debug, PartialEq)]
pub enum GatewayAction {
    /// Send a payload over the gateway socket.
    Send(GatewayPayload),
    /// Reconnect, resuming the current session when possible.
    Reconnect,
    /// Close the socket permanently.
    Close(GatewayCloseCode),
}

impl GatewayAction {
    /// Creates a send action.
    #[must_use]
    pub fn send(payload: GatewayPayload) -> Self {
        Self::Send(payload)
    }

    /// Creates an identify action.
    #[must_use]
    pub fn identify(identify: Identify) -> Self {
        Self::Send(GatewayPayload::new(
            GatewayOpcode::Identify,
            identify.to_value(),
        ))
    }

    /// Creates a resume action.
    #[must_use]
    pub fn resume(resume: Resume) -> Self {
        Self::Send(GatewayPayload::new(
            GatewayOpcode::Resume,
            resume.to_value(),
        ))
    }

    /// Creates a heartbeat action.
    #[must_use]
    pub fn heartbeat(heartbeat: Heartbeat) -> Self {
        Self::Send(GatewayPayload::new(
            GatewayOpcode::Heartbeat,
            heartbeat.to_value(),
        ))
    }

    /// Returns the payload to send, if this is a send action.
    #[must_use]
    pub const fn payload(&self) -> Option<&GatewayPayload> {
        match self {
            Self::Send(payload) => Some(payload),
            Self::Reconnect | Self::Close(_) => None,
        }
    }
}

/// An error produced by the gateway state machine.
#[derive(Clone, Debug, PartialEq)]
pub enum GatewayError {
    /// A payload could not be decoded.
    Payload(GatewayPayloadError),
    /// The requested transition is not valid for the current state.
    InvalidState {
        /// The state the gateway was in.
        state: GatewayState,
    },
    /// A required field was missing from a dispatch payload.
    MissingField(&'static str),
    /// A dispatch payload had no event name.
    MissingEvent,
}

impl fmt::Display for GatewayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Payload(error) => write!(f, "gateway payload error: {error}"),
            Self::InvalidState { state } => {
                write!(f, "operation is not valid while {state}")
            }
            Self::MissingField(field) => write!(f, "missing gateway field `{field}`"),
            Self::MissingEvent => f.write_str("dispatch payload has no event name"),
        }
    }
}

impl Error for GatewayError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Payload(error) => Some(error),
            _ => None,
        }
    }
}

impl From<GatewayPayloadError> for GatewayError {
    fn from(error: GatewayPayloadError) -> Self {
        Self::Payload(error)
    }
}

/// Reconnect backoff policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReconnectPolicy {
    /// The delay before the first reconnect attempt.
    pub base: Duration,
    /// The maximum delay between attempts.
    pub max: Duration,
}

impl ReconnectPolicy {
    /// Creates a policy with the given base and maximum delays.
    #[must_use]
    pub const fn new(base: Duration, max: Duration) -> Self {
        Self { base, max }
    }

    /// Returns the delay before the given zero-based attempt.
    ///
    /// The delay doubles per attempt and is capped at [`ReconnectPolicy::max`].
    #[must_use]
    pub fn delay(&self, attempt: u32) -> Duration {
        let factor = 1_u32.checked_shl(attempt).unwrap_or(u32::MAX);
        self.base.saturating_mul(factor).min(self.max)
    }
}

impl Default for ReconnectPolicy {
    fn default() -> Self {
        Self {
            base: Duration::from_secs(1),
            max: Duration::from_secs(60),
        }
    }
}

/// Static gateway configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct GatewayConfig {
    /// The authentication token.
    pub token: String,
    /// The gateway intents.
    pub intents: Intents,
    /// The client properties sent while identifying.
    pub properties: IdentifyProperties,
    /// The shard this connection handles, if any.
    pub shard: Option<ShardId>,
    /// The initial presence, if any.
    pub presence: Option<data::Value>,
    /// Whether payloads should be compressed.
    pub compress: bool,
    /// The large guild member threshold.
    pub large_threshold: u64,
    /// The URL used for new connections.
    pub gateway_url: String,
    /// The reconnect backoff policy.
    pub reconnect: ReconnectPolicy,
}

impl GatewayConfig {
    /// Creates a configuration for the given token and intents.
    #[must_use]
    pub fn new(token: impl Into<String>, intents: Intents) -> Self {
        Self {
            token: token.into(),
            intents,
            properties: IdentifyProperties::default(),
            shard: None,
            presence: None,
            compress: false,
            large_threshold: 50,
            gateway_url: default_gateway_url(),
            reconnect: ReconnectPolicy::default(),
        }
    }

    /// Returns the default gateway URL for the supported API and encoding.
    #[must_use]
    pub fn default_url() -> String {
        default_gateway_url()
    }

    /// Builds an identify payload from the configuration.
    #[must_use]
    pub fn identify(&self) -> Identify {
        let mut identify = Identify::new(self.token.clone(), self.intents)
            .with_properties(self.properties.clone())
            .with_compress(self.compress)
            .with_large_threshold(self.large_threshold);
        if let Some(shard) = self.shard {
            identify = identify.with_shard(shard);
        }
        if let Some(presence) = &self.presence {
            identify = identify.with_presence(presence.clone());
        }
        identify
    }
}

fn default_gateway_url() -> String {
    format!(
        "{}/?v={}&encoding=json",
        version::GATEWAY_URL,
        version::DISCORD_API_VERSION
    )
}

/// The pure gateway connection lifecycle.
#[derive(Clone, Debug)]
pub struct Gateway {
    config: GatewayConfig,
    state: GatewayState,
    session_id: Option<String>,
    resume_gateway_url: Option<String>,
    sequence: Option<u64>,
    heartbeat_interval: Option<Duration>,
    awaiting_heartbeat_ack: bool,
    reconnect_attempts: u32,
}

impl Gateway {
    /// Creates a gateway in the [`GatewayState::Disconnected`] state.
    #[must_use]
    pub fn new(config: GatewayConfig) -> Self {
        Self {
            config,
            state: GatewayState::Disconnected,
            session_id: None,
            resume_gateway_url: None,
            sequence: None,
            heartbeat_interval: None,
            awaiting_heartbeat_ack: false,
            reconnect_attempts: 0,
        }
    }

    /// Returns the configuration.
    #[must_use]
    pub const fn config(&self) -> &GatewayConfig {
        &self.config
    }

    /// Returns the current state.
    #[must_use]
    pub const fn state(&self) -> GatewayState {
        self.state
    }

    /// Returns the session ID after a successful ready event.
    #[must_use]
    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    /// Returns the resume gateway URL after a successful ready event.
    #[must_use]
    pub fn resume_gateway_url(&self) -> Option<&str> {
        self.resume_gateway_url.as_deref()
    }

    /// Returns the last sequence number received.
    #[must_use]
    pub const fn sequence(&self) -> Option<u64> {
        self.sequence
    }

    /// Returns the heartbeat interval reported by `HELLO`.
    #[must_use]
    pub const fn heartbeat_interval(&self) -> Option<Duration> {
        self.heartbeat_interval
    }

    /// Returns the number of consecutive reconnect attempts.
    #[must_use]
    pub const fn reconnect_attempts(&self) -> u32 {
        self.reconnect_attempts
    }

    /// Returns the delay before the next reconnect attempt.
    #[must_use]
    pub fn reconnect_delay(&self) -> Duration {
        self.config
            .reconnect
            .delay(self.reconnect_attempts.saturating_sub(1))
    }

    /// Returns `true` if the current session can be resumed.
    #[must_use]
    pub fn can_resume(&self) -> bool {
        self.session_id.is_some() && self.sequence.is_some()
    }

    /// Returns the URL to use for the next connection.
    ///
    /// A reconnect prefers the resume URL supplied by Discord.
    #[must_use]
    pub fn url(&self) -> &str {
        if self.state == GatewayState::Reconnecting {
            if let Some(url) = &self.resume_gateway_url {
                return url;
            }
        }
        &self.config.gateway_url
    }

    /// Marks the socket as open, before `HELLO` arrives.
    ///
    /// # Errors
    ///
    /// Returns [`GatewayError::InvalidState`] unless the gateway is disconnected
    /// or reconnecting.
    pub fn on_connected(&mut self) -> Result<(), GatewayError> {
        if !matches!(
            self.state,
            GatewayState::Disconnected | GatewayState::Reconnecting
        ) {
            return Err(GatewayError::InvalidState { state: self.state });
        }
        self.state = GatewayState::Connecting;
        self.heartbeat_interval = None;
        self.awaiting_heartbeat_ack = false;
        Ok(())
    }

    /// Handles one payload from the gateway.
    ///
    /// # Errors
    ///
    /// Returns [`GatewayError`] when a payload cannot be decoded or a dispatch
    /// event is missing required fields.
    pub fn handle_payload(
        &mut self,
        payload: &GatewayPayload,
    ) -> Result<Option<GatewayAction>, GatewayError> {
        if let Some(sequence) = payload.sequence {
            self.sequence = Some(sequence);
        }

        match payload.op {
            GatewayOpcode::Hello => {
                let hello = Hello::from_value(&payload.data)?;
                self.heartbeat_interval = Some(Duration::from_millis(hello.heartbeat_interval));
                self.awaiting_heartbeat_ack = false;
                if let Some(resume) = self.resume_payload() {
                    self.state = GatewayState::Resuming;
                    Ok(Some(GatewayAction::resume(resume)))
                } else {
                    self.state = GatewayState::Identifying;
                    Ok(Some(GatewayAction::identify(self.config.identify())))
                }
            }
            GatewayOpcode::Dispatch => match payload.event.as_deref() {
                Some("READY") => {
                    self.handle_ready(&payload.data)?;
                    Ok(None)
                }
                Some("RESUMED") => {
                    self.state = GatewayState::Connected;
                    self.reconnect_attempts = 0;
                    Ok(None)
                }
                Some(_) => Ok(None),
                None => Err(GatewayError::MissingEvent),
            },
            GatewayOpcode::HeartbeatAck => {
                self.awaiting_heartbeat_ack = false;
                Ok(None)
            }
            GatewayOpcode::Heartbeat => Ok(Some(GatewayAction::heartbeat(Heartbeat::new(
                self.sequence,
            )))),
            GatewayOpcode::Reconnect => {
                self.state = GatewayState::Reconnecting;
                self.reconnect_attempts = self.reconnect_attempts.saturating_add(1);
                Ok(Some(GatewayAction::Reconnect))
            }
            GatewayOpcode::InvalidSession => {
                let invalid = InvalidSession::from_value(&payload.data)?;
                if invalid.resumable {
                    if let Some(resume) = self.resume_payload() {
                        self.state = GatewayState::Resuming;
                        return Ok(Some(GatewayAction::resume(resume)));
                    }
                }
                self.clear_session();
                self.state = GatewayState::Identifying;
                Ok(Some(GatewayAction::identify(self.config.identify())))
            }
            _ => Ok(None),
        }
    }

    /// Produces the heartbeat that the timer should send.
    ///
    /// If the previous heartbeat has not been acknowledged, the connection is
    /// treated as a zombie and a reconnect is requested instead.
    #[must_use]
    pub fn heartbeat(&mut self) -> Option<GatewayAction> {
        if self.heartbeat_interval.is_none()
            || matches!(
                self.state,
                GatewayState::Disconnected | GatewayState::Closed
            )
        {
            return None;
        }

        if self.awaiting_heartbeat_ack {
            self.state = GatewayState::Reconnecting;
            self.reconnect_attempts = self.reconnect_attempts.saturating_add(1);
            return Some(GatewayAction::Reconnect);
        }

        self.awaiting_heartbeat_ack = true;
        Some(GatewayAction::heartbeat(Heartbeat::new(self.sequence)))
    }

    /// Handles a transport failure.
    #[must_use]
    pub fn on_transport_error(&mut self) -> GatewayAction {
        self.heartbeat_interval = None;
        self.awaiting_heartbeat_ack = false;
        self.state = GatewayState::Reconnecting;
        self.reconnect_attempts = self.reconnect_attempts.saturating_add(1);
        GatewayAction::Reconnect
    }

    /// Handles a socket close code.
    #[must_use]
    pub fn on_close(&mut self, code: GatewayCloseCode) -> GatewayAction {
        self.heartbeat_interval = None;
        self.awaiting_heartbeat_ack = false;

        if code.is_reconnectable() {
            self.state = GatewayState::Reconnecting;
            self.reconnect_attempts = self.reconnect_attempts.saturating_add(1);
            GatewayAction::Reconnect
        } else {
            self.state = GatewayState::Closed;
            self.clear_session();
            GatewayAction::Close(code)
        }
    }

    /// Closes the gateway and clears the session.
    pub fn disconnect(&mut self) {
        self.state = GatewayState::Closed;
        self.heartbeat_interval = None;
        self.awaiting_heartbeat_ack = false;
        self.clear_session();
    }

    fn handle_ready(&mut self, data: &data::Value) -> Result<(), GatewayError> {
        let session_id = data
            .get("session_id")
            .and_then(data::Value::as_str)
            .ok_or(GatewayError::MissingField("session_id"))?;
        self.session_id = Some(session_id.to_owned());
        if let Some(url) = data.get("resume_gateway_url").and_then(data::Value::as_str) {
            self.resume_gateway_url = Some(url.to_owned());
        }
        self.state = GatewayState::Connected;
        self.reconnect_attempts = 0;
        Ok(())
    }

    fn resume_payload(&self) -> Option<Resume> {
        let session_id = self.session_id.as_deref()?;
        let sequence = self.sequence?;
        Some(Resume::new(self.config.token.clone(), session_id, sequence))
    }

    fn clear_session(&mut self) {
        self.session_id = None;
        self.resume_gateway_url = None;
        self.sequence = None;
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::time::Duration;

    use data::{
        GatewayCloseCode, GatewayOpcode, GatewayPayload, Heartbeat, Hello, Identify, Resume,
        ShardId,
    };
    use enums::Intents;

    use super::{
        Gateway, GatewayAction, GatewayConfig, GatewayError, GatewayState, ReconnectPolicy,
    };

    fn config() -> GatewayConfig {
        let mut intents = Intents::none();
        intents.insert(Intents::GUILDS);
        GatewayConfig::new("token", intents)
    }

    fn hello(interval: u64) -> GatewayPayload {
        GatewayPayload::new(
            GatewayOpcode::Hello,
            Hello {
                heartbeat_interval: interval,
            }
            .to_value(),
        )
    }

    fn ready(session: &str) -> GatewayPayload {
        let mut object = data::Object::new();
        object.insert("session_id", data::Value::from(session));
        object.insert("resume_gateway_url", data::Value::from("wss://resume"));
        GatewayPayload::new(GatewayOpcode::Dispatch, data::Value::Object(object))
            .with_sequence(1)
            .with_event("READY")
    }

    fn resumed() -> GatewayPayload {
        GatewayPayload::new(GatewayOpcode::Dispatch, data::Value::Null)
            .with_sequence(2)
            .with_event("RESUMED")
    }

    #[test]
    fn config_identifies_with_defaults_and_overrides() {
        let mut config = config();
        config.shard = Some(ShardId::new(1, 2));
        config.presence = Some(data::Value::from("online"));

        let identify = config.identify();
        assert_eq!(identify.token, "token");
        assert_eq!(identify.intents, config.intents);
        assert_eq!(identify.large_threshold, 50);
        assert!(!identify.compress);
        assert_eq!(identify.shard, Some(ShardId::new(1, 2)));
        assert_eq!(identify.presence, Some(data::Value::from("online")));
        assert!(GatewayConfig::default_url().contains("v=10"));
        assert!(GatewayConfig::default_url().contains("encoding=json"));
        assert_eq!(GatewayConfig::default_url(), config.gateway_url);
    }

    #[test]
    fn hello_sends_identify_and_ready_connects() {
        let mut gateway = Gateway::new(config());
        assert_eq!(gateway.state(), GatewayState::Disconnected);
        assert_eq!(gateway.url(), config().gateway_url);

        gateway.on_connected().unwrap();
        assert_eq!(gateway.state(), GatewayState::Connecting);

        let action = gateway.handle_payload(&hello(41_250)).unwrap().unwrap();
        let payload = action.payload().unwrap();
        assert_eq!(payload.op, GatewayOpcode::Identify);
        assert_eq!(gateway.state(), GatewayState::Identifying);
        assert_eq!(
            gateway.heartbeat_interval(),
            Some(Duration::from_millis(41_250))
        );
        assert_eq!(Identify::from_value(&payload.data).unwrap().token, "token");

        assert_eq!(gateway.handle_payload(&ready("session")).unwrap(), None);
        assert_eq!(gateway.state(), GatewayState::Connected);
        assert_eq!(gateway.session_id(), Some("session"));
        assert_eq!(gateway.resume_gateway_url(), Some("wss://resume"));
        assert_eq!(gateway.sequence(), Some(1));
        assert!(gateway.can_resume());
    }

    #[test]
    fn reconnect_uses_resume_url_and_resumes_after_hello() {
        let mut gateway = Gateway::new(config());
        gateway.on_connected().unwrap();
        gateway.handle_payload(&hello(1_000)).unwrap();
        gateway.handle_payload(&ready("session")).unwrap();

        assert!(matches!(
            gateway.on_close(GatewayCloseCode::SESSION_TIMED_OUT),
            GatewayAction::Reconnect
        ));
        assert_eq!(gateway.state(), GatewayState::Reconnecting);
        assert_eq!(gateway.reconnect_attempts(), 1);
        assert_eq!(gateway.url(), "wss://resume");
        assert_eq!(gateway.reconnect_delay(), Duration::from_secs(1));

        gateway.on_connected().unwrap();
        let action = gateway.handle_payload(&hello(1_000)).unwrap().unwrap();
        let payload = action.payload().unwrap();
        assert_eq!(payload.op, GatewayOpcode::Resume);
        assert_eq!(gateway.state(), GatewayState::Resuming);
        let resume = Resume::from_value(&payload.data).unwrap();
        assert_eq!(resume.session_id, "session");
        assert_eq!(resume.sequence, 1);

        assert_eq!(gateway.handle_payload(&resumed()).unwrap(), None);
        assert_eq!(gateway.state(), GatewayState::Connected);
        assert_eq!(gateway.reconnect_attempts(), 0);
    }

    #[test]
    fn heartbeats_are_acknowledged_or_reconnect() {
        let mut gateway = Gateway::new(config());
        gateway.on_connected().unwrap();
        assert!(gateway.heartbeat().is_none());

        gateway.handle_payload(&hello(1_000)).unwrap();
        let action = gateway.heartbeat().unwrap();
        let payload = action.payload().unwrap();
        assert_eq!(payload.op, GatewayOpcode::Heartbeat);
        assert_eq!(
            Heartbeat::from_value(&payload.data).unwrap(),
            Heartbeat::new(None)
        );

        let ack = GatewayPayload::new(GatewayOpcode::HeartbeatAck, data::Value::Null);
        assert_eq!(gateway.handle_payload(&ack).unwrap(), None);

        let action = gateway.heartbeat().unwrap();
        assert_eq!(action.payload().unwrap().op, GatewayOpcode::Heartbeat);

        let reconnect = gateway.heartbeat().unwrap();
        assert_eq!(reconnect, GatewayAction::Reconnect);
        assert_eq!(gateway.state(), GatewayState::Reconnecting);
        assert_eq!(gateway.reconnect_attempts(), 1);
    }

    #[test]
    fn server_heartbeats_echo_the_sequence() {
        let mut gateway = Gateway::new(config());
        gateway.on_connected().unwrap();
        gateway.handle_payload(&hello(1_000)).unwrap();
        gateway.handle_payload(&ready("session")).unwrap();

        let heartbeat = GatewayPayload::new(GatewayOpcode::Heartbeat, data::Value::Null);
        let action = gateway.handle_payload(&heartbeat).unwrap().unwrap();
        let payload = action.payload().unwrap();
        assert_eq!(
            Heartbeat::from_value(&payload.data).unwrap(),
            Heartbeat::new(Some(1))
        );
    }

    #[test]
    fn dispatch_sequence_updates_and_reconnect_opcode_requests_reconnect() {
        let mut gateway = Gateway::new(config());
        gateway.on_connected().unwrap();
        gateway.handle_payload(&hello(1_000)).unwrap();

        let dispatch = GatewayPayload::new(GatewayOpcode::Dispatch, data::Value::object())
            .with_sequence(7)
            .with_event("MESSAGE_CREATE");
        assert_eq!(gateway.handle_payload(&dispatch).unwrap(), None);
        assert_eq!(gateway.sequence(), Some(7));

        let reconnect = GatewayPayload::new(GatewayOpcode::Reconnect, data::Value::Null);
        assert_eq!(
            gateway.handle_payload(&reconnect).unwrap(),
            Some(GatewayAction::Reconnect)
        );
        assert_eq!(gateway.state(), GatewayState::Reconnecting);
        assert_eq!(gateway.reconnect_attempts(), 1);
    }

    #[test]
    fn invalid_sessions_resume_or_identify() {
        let mut gateway = Gateway::new(config());
        gateway.on_connected().unwrap();
        gateway.handle_payload(&hello(1_000)).unwrap();
        gateway.handle_payload(&ready("session")).unwrap();

        let resumable = GatewayPayload::new(GatewayOpcode::InvalidSession, data::Value::Bool(true));
        let action = gateway.handle_payload(&resumable).unwrap().unwrap();
        assert_eq!(action.payload().unwrap().op, GatewayOpcode::Resume);
        assert_eq!(gateway.state(), GatewayState::Resuming);

        let non_resumable =
            GatewayPayload::new(GatewayOpcode::InvalidSession, data::Value::Bool(false));
        let action = gateway.handle_payload(&non_resumable).unwrap().unwrap();
        assert_eq!(action.payload().unwrap().op, GatewayOpcode::Identify);
        assert_eq!(gateway.state(), GatewayState::Identifying);
        assert!(!gateway.can_resume());
        assert_eq!(gateway.sequence(), None);
    }

    #[test]
    fn close_codes_decide_reconnect_or_shutdown() {
        let mut gateway = Gateway::new(config());
        gateway.on_connected().unwrap();
        gateway.handle_payload(&hello(1_000)).unwrap();

        let action = gateway.on_close(GatewayCloseCode::AUTHENTICATION_FAILED);
        assert_eq!(
            action,
            GatewayAction::Close(GatewayCloseCode::AUTHENTICATION_FAILED)
        );
        assert_eq!(gateway.state(), GatewayState::Closed);
        assert!(gateway.heartbeat().is_none());
        assert_eq!(
            gateway.on_connected(),
            Err(GatewayError::InvalidState {
                state: GatewayState::Closed
            })
        );

        let mut transport = Gateway::new(config());
        transport.on_connected().unwrap();
        assert_eq!(transport.on_transport_error(), GatewayAction::Reconnect);
        assert_eq!(transport.state(), GatewayState::Reconnecting);
        assert_eq!(transport.reconnect_attempts(), 1);

        transport.disconnect();
        assert_eq!(transport.state(), GatewayState::Closed);
    }

    #[test]
    fn malformed_payloads_are_rejected() {
        let mut gateway = Gateway::new(config());
        gateway.on_connected().unwrap();

        let bad_hello = GatewayPayload::new(GatewayOpcode::Hello, data::Value::object());
        assert!(matches!(
            gateway.handle_payload(&bad_hello),
            Err(GatewayError::Payload(_))
        ));

        let missing_event = GatewayPayload::new(GatewayOpcode::Dispatch, data::Value::object());
        assert_eq!(
            gateway.handle_payload(&missing_event),
            Err(GatewayError::MissingEvent)
        );

        let missing_session =
            GatewayPayload::new(GatewayOpcode::Dispatch, data::Value::object()).with_event("READY");
        assert_eq!(
            gateway.handle_payload(&missing_session),
            Err(GatewayError::MissingField("session_id"))
        );
        assert!(
            gateway
                .handle_payload(&missing_session)
                .unwrap_err()
                .source()
                .is_none()
        );
    }

    #[test]
    fn reconnect_policy_backs_off_and_caps() {
        let policy = ReconnectPolicy::new(Duration::from_secs(2), Duration::from_secs(10));
        assert_eq!(policy.delay(0), Duration::from_secs(2));
        assert_eq!(policy.delay(1), Duration::from_secs(4));
        assert_eq!(policy.delay(2), Duration::from_secs(8));
        assert_eq!(policy.delay(3), Duration::from_secs(10));
        assert_eq!(policy.delay(99), Duration::from_secs(10));
    }
}
