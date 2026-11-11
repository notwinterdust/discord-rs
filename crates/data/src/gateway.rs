//! Gateway payloads and protocol constants.
//!
//! The gateway exchanges JSON envelopes with an opcode, data, sequence and
//! optional event name. This module models that envelope together with the
//! payloads a client sends while connecting, resuming and keeping a session
//! alive.

use std::error::Error;
use std::fmt;

use abc::Snowflake;
use enums::Intents;

use crate::json::{self, ParseError};
use crate::value::{Number, Object, Value};

/// A gateway opcode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum GatewayOpcode {
    /// A dispatched event.
    Dispatch = 0,
    /// A heartbeat, sent by both sides.
    Heartbeat = 1,
    /// Starts a new session.
    Identify = 2,
    /// Updates the client's presence.
    PresenceUpdate = 3,
    /// Joins, moves or leaves a voice channel.
    VoiceStateUpdate = 4,
    /// Checks voice latency.
    VoicePing = 5,
    /// Resumes an existing session.
    Resume = 6,
    /// Asks the client to reconnect.
    Reconnect = 7,
    /// Requests guild members.
    RequestGuildMembers = 8,
    /// Reports that a session is no longer valid.
    InvalidSession = 9,
    /// Reports the heartbeat interval.
    Hello = 10,
    /// Acknowledges a heartbeat.
    HeartbeatAck = 11,
    /// Requests a guild sync.
    GuildSync = 12,
}

impl GatewayOpcode {
    /// Returns the raw opcode value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw opcode, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Dispatch),
            1 => Some(Self::Heartbeat),
            2 => Some(Self::Identify),
            3 => Some(Self::PresenceUpdate),
            4 => Some(Self::VoiceStateUpdate),
            5 => Some(Self::VoicePing),
            6 => Some(Self::Resume),
            7 => Some(Self::Reconnect),
            8 => Some(Self::RequestGuildMembers),
            9 => Some(Self::InvalidSession),
            10 => Some(Self::Hello),
            11 => Some(Self::HeartbeatAck),
            12 => Some(Self::GuildSync),
            _ => None,
        }
    }

    /// Returns the lowercase name used in documentation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dispatch => "dispatch",
            Self::Heartbeat => "heartbeat",
            Self::Identify => "identify",
            Self::PresenceUpdate => "presence_update",
            Self::VoiceStateUpdate => "voice_state_update",
            Self::VoicePing => "voice_ping",
            Self::Resume => "resume",
            Self::Reconnect => "reconnect",
            Self::RequestGuildMembers => "request_guild_members",
            Self::InvalidSession => "invalid_session",
            Self::Hello => "hello",
            Self::HeartbeatAck => "heartbeat_ack",
            Self::GuildSync => "guild_sync",
        }
    }

    /// Returns `true` if a client may send this opcode.
    #[must_use]
    pub const fn is_client_send(self) -> bool {
        matches!(
            self,
            Self::Heartbeat
                | Self::Identify
                | Self::PresenceUpdate
                | Self::VoiceStateUpdate
                | Self::VoicePing
                | Self::Resume
                | Self::RequestGuildMembers
                | Self::GuildSync
        )
    }

    /// Returns `true` if the gateway may send this opcode.
    #[must_use]
    pub const fn is_server_send(self) -> bool {
        matches!(
            self,
            Self::Dispatch
                | Self::Heartbeat
                | Self::Reconnect
                | Self::InvalidSession
                | Self::Hello
                | Self::HeartbeatAck
        )
    }
}

impl fmt::Display for GatewayOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A gateway close event code.
///
/// The newtype keeps unknown codes intact so they can be logged or reported.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GatewayCloseCode(u16);

impl GatewayCloseCode {
    /// The connection closed normally.
    pub const NORMAL: Self = Self(1000);
    /// The peer is going away.
    pub const GOING_AWAY: Self = Self(1001);
    /// An unknown error occurred.
    pub const UNKNOWN_ERROR: Self = Self(4000);
    /// An unknown opcode was received.
    pub const UNKNOWN_OPCODE: Self = Self(4001);
    /// A payload could not be decoded.
    pub const DECODE_ERROR: Self = Self(4002);
    /// A payload was sent before identifying.
    pub const NOT_AUTHENTICATED: Self = Self(4003);
    /// Authentication failed.
    pub const AUTHENTICATION_FAILED: Self = Self(4004);
    /// The client identified more than once.
    pub const ALREADY_AUTHENTICATED: Self = Self(4005);
    /// An invalid sequence number was sent.
    pub const INVALID_SEQUENCE: Self = Self(4007);
    /// The client is being rate limited.
    pub const RATE_LIMITED: Self = Self(4008);
    /// The session timed out.
    pub const SESSION_TIMED_OUT: Self = Self(4009);
    /// The shard ID is invalid.
    pub const INVALID_SHARD: Self = Self(4010);
    /// Sharding is required.
    pub const SHARDING_REQUIRED: Self = Self(4011);
    /// The API version is invalid.
    pub const INVALID_API_VERSION: Self = Self(4012);
    /// The requested intents are invalid.
    pub const INVALID_INTENTS: Self = Self(4013);
    /// The requested intents are not allowed for this client.
    pub const DISALLOWED_INTENTS: Self = Self(4014);

    /// Creates a close code from its raw value.
    #[must_use]
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    /// Returns the raw close code.
    #[must_use]
    pub const fn value(self) -> u16 {
        self.0
    }

    /// Returns a human-readable description of the close code.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self.0 {
            1000 => "normal closure",
            1001 => "going away",
            4000 => "unknown error",
            4001 => "unknown opcode",
            4002 => "decode error",
            4003 => "not authenticated",
            4004 => "authentication failed",
            4005 => "already authenticated",
            4007 => "invalid sequence",
            4008 => "rate limited",
            4009 => "session timed out",
            4010 => "invalid shard",
            4011 => "sharding required",
            4012 => "invalid API version",
            4013 => "invalid intents",
            4014 => "disallowed intents",
            _ => "unknown close code",
        }
    }

    /// Returns `true` if the connection closed normally.
    #[must_use]
    pub const fn is_normal(self) -> bool {
        self.0 == 1000
    }

    /// Returns `true` if a client should reconnect after this close code.
    ///
    /// Authentication, sharding and intent errors are not recoverable by
    /// reconnecting with the same settings.
    #[must_use]
    pub const fn is_reconnectable(self) -> bool {
        !matches!(self.0, 1000 | 4004 | 4010..=4014)
    }
}

impl fmt::Display for GatewayCloseCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.0, self.description())
    }
}

/// An error produced while reading or writing a gateway payload.
#[derive(Clone, Debug, PartialEq)]
pub enum GatewayPayloadError {
    /// A required field was missing.
    MissingField(&'static str),
    /// A field had the wrong JSON type.
    InvalidField {
        /// The field name.
        field: &'static str,
        /// The expected JSON type.
        expected: &'static str,
    },
    /// The opcode is not known to this library.
    UnknownOpcode(u64),
    /// The payload was not valid JSON.
    Json(ParseError),
}

impl fmt::Display for GatewayPayloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingField(field) => write!(f, "missing field `{field}`"),
            Self::InvalidField { field, expected } => {
                write!(f, "field `{field}` must be {expected}")
            }
            Self::UnknownOpcode(value) => write!(f, "unknown gateway opcode {value}"),
            Self::Json(error) => write!(f, "invalid gateway JSON: {error}"),
        }
    }
}

impl Error for GatewayPayloadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ParseError> for GatewayPayloadError {
    fn from(error: ParseError) -> Self {
        Self::Json(error)
    }
}

/// A gateway envelope.
#[derive(Clone, Debug, PartialEq)]
pub struct GatewayPayload {
    /// The opcode.
    pub op: GatewayOpcode,
    /// The payload data.
    pub data: Value,
    /// The sequence number, for dispatched events.
    pub sequence: Option<u64>,
    /// The event name, for dispatched events.
    pub event: Option<String>,
}

impl GatewayPayload {
    /// Creates a payload with no sequence or event name.
    #[must_use]
    pub fn new(op: GatewayOpcode, data: Value) -> Self {
        Self {
            op,
            data,
            sequence: None,
            event: None,
        }
    }

    /// Sets the sequence number.
    #[must_use]
    pub fn with_sequence(mut self, sequence: u64) -> Self {
        self.sequence = Some(sequence);
        self
    }

    /// Sets the event name.
    #[must_use]
    pub fn with_event(mut self, event: impl Into<String>) -> Self {
        self.event = Some(event.into());
        self
    }

    /// Reads an envelope from a JSON value.
    ///
    /// # Errors
    ///
    /// Returns [`GatewayPayloadError`] when the envelope is not an object, the
    /// opcode is missing or unknown, or a field has the wrong type.
    pub fn from_value(value: &Value) -> Result<Self, GatewayPayloadError> {
        let object = value.as_object().ok_or(GatewayPayloadError::InvalidField {
            field: "payload",
            expected: "an object",
        })?;
        let op_value =
            required(object, "op")?
                .as_u64()
                .ok_or(GatewayPayloadError::InvalidField {
                    field: "op",
                    expected: "a non-negative integer",
                })?;
        let op = u8::try_from(op_value)
            .ok()
            .and_then(GatewayOpcode::from_value)
            .ok_or(GatewayPayloadError::UnknownOpcode(op_value))?;

        Ok(Self {
            op,
            data: object.get("d").cloned().unwrap_or(Value::Null),
            sequence: optional_u64(object, "s")?,
            event: optional_string(object, "t")?,
        })
    }

    /// Converts the envelope to a JSON value.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert("op", Value::from(self.op.value()));
        object.insert("d", self.data.clone());
        if let Some(sequence) = self.sequence {
            object.insert("s", Value::from(sequence));
        }
        if let Some(event) = &self.event {
            object.insert("t", Value::from(event.as_str()));
        }
        Value::Object(object)
    }

    /// Parses an envelope from JSON text.
    ///
    /// # Errors
    ///
    /// Returns [`GatewayPayloadError`] when the JSON is invalid or the envelope
    /// is malformed.
    pub fn from_json(input: &str) -> Result<Self, GatewayPayloadError> {
        let value = json::from_str(input)?;
        Self::from_value(&value)
    }

    /// Serializes the envelope to compact JSON.
    #[must_use]
    pub fn to_json(&self) -> String {
        json::to_string(&self.to_value())
    }
}

/// The payload of a [`GatewayOpcode::Hello`] event.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Hello {
    /// The heartbeat interval in milliseconds.
    pub heartbeat_interval: u64,
}

impl Hello {
    /// Reads a hello payload.
    ///
    /// # Errors
    ///
    /// Returns [`GatewayPayloadError`] when the payload is malformed.
    pub fn from_value(data: &Value) -> Result<Self, GatewayPayloadError> {
        let object = expect_object(data)?;
        Ok(Self {
            heartbeat_interval: required_u64(object, "heartbeat_interval")?,
        })
    }

    /// Converts the payload to a JSON value.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert("heartbeat_interval", Value::from(self.heartbeat_interval));
        Value::Object(object)
    }
}

/// A client shard identifier, sent as `[id, count]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShardId {
    /// The shard's zero-based index.
    pub id: u32,
    /// The total number of shards.
    pub count: u32,
}

impl ShardId {
    /// Creates a shard identifier.
    #[must_use]
    pub const fn new(id: u32, count: u32) -> Self {
        Self { id, count }
    }

    /// Reads a shard identifier from its two-element array.
    ///
    /// # Errors
    ///
    /// Returns [`GatewayPayloadError`] when the value is not a two-element
    /// array of non-negative integers.
    pub fn from_value(value: &Value) -> Result<Self, GatewayPayloadError> {
        let array = value.as_array().ok_or(GatewayPayloadError::InvalidField {
            field: "shard",
            expected: "a two-element array",
        })?;
        if array.len() != 2 {
            return Err(GatewayPayloadError::InvalidField {
                field: "shard",
                expected: "a two-element array",
            });
        }
        let id = array[0]
            .as_u64()
            .and_then(|value| u32::try_from(value).ok())
            .ok_or(GatewayPayloadError::InvalidField {
                field: "shard",
                expected: "two non-negative integers",
            })?;
        let count = array[1]
            .as_u64()
            .and_then(|value| u32::try_from(value).ok())
            .ok_or(GatewayPayloadError::InvalidField {
                field: "shard",
                expected: "two non-negative integers",
            })?;
        Ok(Self { id, count })
    }

    /// Converts the shard identifier to a JSON array.
    #[must_use]
    pub fn to_value(self) -> Value {
        Value::Array(vec![
            Value::from(u64::from(self.id)),
            Value::from(u64::from(self.count)),
        ])
    }
}

/// The client properties sent with an identify payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdentifyProperties {
    /// The operating system.
    pub os: String,
    /// The browser name.
    pub browser: String,
    /// The device name.
    pub device: String,
}

impl IdentifyProperties {
    /// Creates identify properties.
    #[must_use]
    pub fn new(
        os: impl Into<String>,
        browser: impl Into<String>,
        device: impl Into<String>,
    ) -> Self {
        Self {
            os: os.into(),
            browser: browser.into(),
            device: device.into(),
        }
    }

    /// Reads identify properties.
    ///
    /// # Errors
    ///
    /// Returns [`GatewayPayloadError`] when a property is missing or not a
    /// string.
    pub fn from_value(value: &Value) -> Result<Self, GatewayPayloadError> {
        let object = expect_object(value)?;
        Ok(Self {
            os: required_string(object, "os")?,
            browser: required_string(object, "browser")?,
            device: required_string(object, "device")?,
        })
    }

    /// Converts the properties to a JSON value.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert("os", Value::from(self.os.as_str()));
        object.insert("browser", Value::from(self.browser.as_str()));
        object.insert("device", Value::from(self.device.as_str()));
        Value::Object(object)
    }
}

impl Default for IdentifyProperties {
    fn default() -> Self {
        Self {
            os: std::env::consts::OS.to_owned(),
            browser: String::from("discord-rs"),
            device: String::from("discord-rs"),
        }
    }
}

/// The data sent to start a new gateway session.
#[derive(Clone, Debug, PartialEq)]
pub struct Identify {
    /// The authentication token.
    pub token: String,
    /// The gateway intents.
    pub intents: Intents,
    /// The client properties.
    pub properties: IdentifyProperties,
    /// Whether payloads should be compressed.
    pub compress: bool,
    /// The member list threshold for large guilds.
    pub large_threshold: u64,
    /// The shard this connection handles, if any.
    pub shard: Option<ShardId>,
    /// The initial presence, if any.
    pub presence: Option<Value>,
}

impl Identify {
    /// Creates an identify payload with default properties and thresholds.
    #[must_use]
    pub fn new(token: impl Into<String>, intents: Intents) -> Self {
        Self {
            token: token.into(),
            intents,
            properties: IdentifyProperties::default(),
            compress: false,
            large_threshold: 50,
            shard: None,
            presence: None,
        }
    }

    /// Sets the client properties.
    #[must_use]
    pub fn with_properties(mut self, properties: IdentifyProperties) -> Self {
        self.properties = properties;
        self
    }

    /// Sets whether payloads should be compressed.
    #[must_use]
    pub const fn with_compress(mut self, compress: bool) -> Self {
        self.compress = compress;
        self
    }

    /// Sets the large guild member threshold.
    #[must_use]
    pub const fn with_large_threshold(mut self, large_threshold: u64) -> Self {
        self.large_threshold = large_threshold;
        self
    }

    /// Sets the shard this connection handles.
    #[must_use]
    pub const fn with_shard(mut self, shard: ShardId) -> Self {
        self.shard = Some(shard);
        self
    }

    /// Sets the initial presence.
    #[must_use]
    pub fn with_presence(mut self, presence: Value) -> Self {
        self.presence = Some(presence);
        self
    }

    /// Reads an identify payload.
    ///
    /// # Errors
    ///
    /// Returns [`GatewayPayloadError`] when the payload is malformed.
    pub fn from_value(data: &Value) -> Result<Self, GatewayPayloadError> {
        let object = expect_object(data)?;
        Ok(Self {
            token: required_string(object, "token")?,
            intents: Intents::from_bits(required_u64(object, "intents")?),
            properties: IdentifyProperties::from_value(required(object, "properties")?)?,
            compress: optional_bool(object, "compress", false)?,
            large_threshold: optional_u64(object, "large_threshold")?.unwrap_or(50),
            shard: optional(object, "shard")
                .map(ShardId::from_value)
                .transpose()?,
            presence: optional(object, "presence").cloned(),
        })
    }

    /// Converts the payload to a JSON value.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert("token", Value::from(self.token.as_str()));
        object.insert("intents", Value::from(self.intents.bits()));
        object.insert("properties", self.properties.to_value());
        object.insert("compress", Value::from(self.compress));
        object.insert("large_threshold", Value::from(self.large_threshold));
        if let Some(shard) = self.shard {
            object.insert("shard", shard.to_value());
        }
        if let Some(presence) = &self.presence {
            object.insert("presence", presence.clone());
        }
        Value::Object(object)
    }
}

/// The data sent to resume an existing gateway session.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Resume {
    /// The authentication token.
    pub token: String,
    /// The session ID to resume.
    pub session_id: String,
    /// The last sequence number received.
    pub sequence: u64,
}

impl Resume {
    /// Creates a resume payload.
    #[must_use]
    pub fn new(token: impl Into<String>, session_id: impl Into<String>, sequence: u64) -> Self {
        Self {
            token: token.into(),
            session_id: session_id.into(),
            sequence,
        }
    }

    /// Reads a resume payload.
    ///
    /// # Errors
    ///
    /// Returns [`GatewayPayloadError`] when the payload is malformed.
    pub fn from_value(data: &Value) -> Result<Self, GatewayPayloadError> {
        let object = expect_object(data)?;
        Ok(Self {
            token: required_string(object, "token")?,
            session_id: required_string(object, "session_id")?,
            sequence: required_u64(object, "seq")?,
        })
    }

    /// Converts the payload to a JSON value.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert("token", Value::from(self.token.as_str()));
        object.insert("session_id", Value::from(self.session_id.as_str()));
        object.insert("seq", Value::from(self.sequence));
        Value::Object(object)
    }
}

/// The data sent with a heartbeat.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Heartbeat {
    /// The last sequence number received, if any.
    pub sequence: Option<u64>,
}

impl Heartbeat {
    /// Creates a heartbeat payload.
    #[must_use]
    pub const fn new(sequence: Option<u64>) -> Self {
        Self { sequence }
    }

    /// Reads a heartbeat payload.
    ///
    /// A `null` data value is a valid heartbeat before the first dispatch.
    ///
    /// # Errors
    ///
    /// Returns [`GatewayPayloadError`] when the payload is malformed.
    pub fn from_value(data: &Value) -> Result<Self, GatewayPayloadError> {
        match data {
            Value::Null => Ok(Self { sequence: None }),
            Value::Number(Number::Integer(sequence)) if *sequence >= 0 => Ok(Self {
                sequence: u64::try_from(*sequence).ok(),
            }),
            Value::Number(Number::Unsigned(sequence)) => Ok(Self {
                sequence: Some(*sequence),
            }),
            _ => Err(GatewayPayloadError::InvalidField {
                field: "heartbeat",
                expected: "null or a non-negative integer",
            }),
        }
    }

    /// Converts the payload to a JSON value.
    #[must_use]
    pub fn to_value(self) -> Value {
        self.sequence.map_or(Value::Null, Value::from)
    }
}

/// The data sent to request guild members.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestGuildMembers {
    /// The guild to request members from.
    pub guild_id: Snowflake,
    /// The member name prefix to match, or an empty string for all members.
    pub query: String,
    /// The maximum number of members to return.
    pub limit: u64,
    /// Whether to include presence data.
    pub presences: bool,
    /// Specific user IDs to request, if any.
    pub user_ids: Option<Vec<Snowflake>>,
    /// An optional nonce echoed in the response.
    pub nonce: Option<String>,
}

impl RequestGuildMembers {
    /// Creates a request for the given guild and query.
    #[must_use]
    pub fn new(guild_id: impl Into<Snowflake>, query: impl Into<String>) -> Self {
        Self {
            guild_id: guild_id.into(),
            query: query.into(),
            limit: 0,
            presences: false,
            user_ids: None,
            nonce: None,
        }
    }

    /// Sets the maximum number of members to return.
    #[must_use]
    pub const fn with_limit(mut self, limit: u64) -> Self {
        self.limit = limit;
        self
    }

    /// Sets whether presence data is included.
    #[must_use]
    pub const fn with_presences(mut self, presences: bool) -> Self {
        self.presences = presences;
        self
    }

    /// Sets specific user IDs to request.
    #[must_use]
    pub fn with_user_ids(mut self, user_ids: Vec<Snowflake>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    /// Sets the nonce echoed in the response.
    #[must_use]
    pub fn with_nonce(mut self, nonce: impl Into<String>) -> Self {
        self.nonce = Some(nonce.into());
        self
    }

    /// Reads a guild member request.
    ///
    /// # Errors
    ///
    /// Returns [`GatewayPayloadError`] when the payload is malformed.
    pub fn from_value(data: &Value) -> Result<Self, GatewayPayloadError> {
        let object = expect_object(data)?;
        let user_ids = optional(object, "user_ids")
            .map(|value| {
                value
                    .as_array()
                    .ok_or(GatewayPayloadError::InvalidField {
                        field: "user_ids",
                        expected: "an array of snowflakes",
                    })?
                    .iter()
                    .map(|value| snowflake(value, "user_ids"))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        Ok(Self {
            guild_id: required_snowflake(object, "guild_id")?,
            query: optional_string(object, "query")?.unwrap_or_default(),
            limit: required_u64(object, "limit")?,
            presences: optional_bool(object, "presences", false)?,
            user_ids,
            nonce: optional_string(object, "nonce")?,
        })
    }

    /// Converts the payload to a JSON value.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert("guild_id", snowflake_value(self.guild_id));
        object.insert("query", Value::from(self.query.as_str()));
        object.insert("limit", Value::from(self.limit));
        object.insert("presences", Value::from(self.presences));
        if let Some(user_ids) = &self.user_ids {
            object.insert(
                "user_ids",
                Value::Array(user_ids.iter().copied().map(snowflake_value).collect()),
            );
        }
        if let Some(nonce) = &self.nonce {
            object.insert("nonce", Value::from(nonce.as_str()));
        }
        Value::Object(object)
    }
}

/// The data sent to join, move or leave a voice channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UpdateVoiceState {
    /// The guild whose voice state is changing.
    pub guild_id: Snowflake,
    /// The channel to join, or `None` to leave voice.
    pub channel_id: Option<Snowflake>,
    /// Whether the client is self-muted.
    pub self_mute: bool,
    /// Whether the client is self-deafened.
    pub self_deaf: bool,
}

impl UpdateVoiceState {
    /// Creates a voice state update for the given guild.
    #[must_use]
    pub fn new(guild_id: impl Into<Snowflake>, self_mute: bool, self_deaf: bool) -> Self {
        Self {
            guild_id: guild_id.into(),
            channel_id: None,
            self_mute,
            self_deaf,
        }
    }

    /// Sets the channel to join.
    #[must_use]
    pub fn with_channel_id(mut self, channel_id: impl Into<Snowflake>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    /// Reads a voice state update.
    ///
    /// # Errors
    ///
    /// Returns [`GatewayPayloadError`] when the payload is malformed.
    pub fn from_value(data: &Value) -> Result<Self, GatewayPayloadError> {
        let object = expect_object(data)?;
        Ok(Self {
            guild_id: required_snowflake(object, "guild_id")?,
            channel_id: optional_snowflake(object, "channel_id")?,
            self_mute: required_bool(object, "self_mute")?,
            self_deaf: required_bool(object, "self_deaf")?,
        })
    }

    /// Converts the payload to a JSON value.
    #[must_use]
    pub fn to_value(self) -> Value {
        let mut object = Object::new();
        object.insert("guild_id", snowflake_value(self.guild_id));
        object.insert(
            "channel_id",
            self.channel_id.map_or(Value::Null, snowflake_value),
        );
        object.insert("self_mute", Value::from(self.self_mute));
        object.insert("self_deaf", Value::from(self.self_deaf));
        Value::Object(object)
    }
}

/// The data sent when a session becomes invalid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidSession {
    /// Whether the session can be resumed.
    pub resumable: bool,
}

impl InvalidSession {
    /// Creates an invalid session payload.
    #[must_use]
    pub const fn new(resumable: bool) -> Self {
        Self { resumable }
    }

    /// Reads an invalid session payload.
    ///
    /// # Errors
    ///
    /// Returns [`GatewayPayloadError`] when the payload is not a boolean.
    pub fn from_value(data: &Value) -> Result<Self, GatewayPayloadError> {
        data.as_bool()
            .map(Self::new)
            .ok_or(GatewayPayloadError::InvalidField {
                field: "invalid_session",
                expected: "a boolean",
            })
    }

    /// Converts the payload to a JSON value.
    #[must_use]
    pub fn to_value(self) -> Value {
        Value::Bool(self.resumable)
    }
}

fn expect_object(value: &Value) -> Result<&Object, GatewayPayloadError> {
    value.as_object().ok_or(GatewayPayloadError::InvalidField {
        field: "data",
        expected: "an object",
    })
}

fn required<'a>(object: &'a Object, field: &'static str) -> Result<&'a Value, GatewayPayloadError> {
    object
        .get(field)
        .ok_or(GatewayPayloadError::MissingField(field))
}

fn required_string(object: &Object, field: &'static str) -> Result<String, GatewayPayloadError> {
    required(object, field)?
        .as_str()
        .map(str::to_owned)
        .ok_or(GatewayPayloadError::InvalidField {
            field,
            expected: "a string",
        })
}

fn optional_string(
    object: &Object,
    field: &'static str,
) -> Result<Option<String>, GatewayPayloadError> {
    optional(object, field)
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .ok_or(GatewayPayloadError::InvalidField {
                    field,
                    expected: "a string",
                })
        })
        .transpose()
}

fn required_u64(object: &Object, field: &'static str) -> Result<u64, GatewayPayloadError> {
    required(object, field)?
        .as_u64()
        .ok_or(GatewayPayloadError::InvalidField {
            field,
            expected: "a non-negative integer",
        })
}

fn optional_u64(object: &Object, field: &'static str) -> Result<Option<u64>, GatewayPayloadError> {
    optional(object, field)
        .map(|value| {
            value.as_u64().ok_or(GatewayPayloadError::InvalidField {
                field,
                expected: "a non-negative integer",
            })
        })
        .transpose()
}

fn required_bool(object: &Object, field: &'static str) -> Result<bool, GatewayPayloadError> {
    required(object, field)?
        .as_bool()
        .ok_or(GatewayPayloadError::InvalidField {
            field,
            expected: "a boolean",
        })
}

fn optional_bool(
    object: &Object,
    field: &'static str,
    default: bool,
) -> Result<bool, GatewayPayloadError> {
    optional(object, field)
        .map(|value| {
            value.as_bool().ok_or(GatewayPayloadError::InvalidField {
                field,
                expected: "a boolean",
            })
        })
        .transpose()
        .map(|value| value.unwrap_or(default))
}

fn required_snowflake(
    object: &Object,
    field: &'static str,
) -> Result<Snowflake, GatewayPayloadError> {
    snowflake(required(object, field)?, field)
}

fn optional_snowflake(
    object: &Object,
    field: &'static str,
) -> Result<Option<Snowflake>, GatewayPayloadError> {
    optional(object, field)
        .map(|value| snowflake(value, field))
        .transpose()
}

fn snowflake(value: &Value, field: &'static str) -> Result<Snowflake, GatewayPayloadError> {
    match value {
        Value::String(string) => string.parse::<u64>().map(Snowflake::new).map_err(|_| {
            GatewayPayloadError::InvalidField {
                field,
                expected: "a snowflake",
            }
        }),
        _ => value
            .as_u64()
            .map(Snowflake::new)
            .ok_or(GatewayPayloadError::InvalidField {
                field,
                expected: "a snowflake",
            }),
    }
}

fn snowflake_value(snowflake: Snowflake) -> Value {
    Value::String(snowflake.to_string())
}

fn optional<'a>(object: &'a Object, field: &str) -> Option<&'a Value> {
    match object.get(field) {
        Some(Value::Null) | None => None,
        Some(value) => Some(value),
    }
}

#[cfg(test)]
mod tests {
    use abc::Snowflake;
    use enums::Intents;

    use super::{
        GatewayCloseCode, GatewayOpcode, GatewayPayload, GatewayPayloadError, Heartbeat, Hello,
        Identify, IdentifyProperties, InvalidSession, RequestGuildMembers, Resume, ShardId,
        UpdateVoiceState,
    };
    use crate::json;
    use crate::value::Value;

    #[test]
    fn opcodes_round_trip_and_report_direction() {
        for value in 0..=12 {
            let opcode = GatewayOpcode::from_value(value).unwrap();
            assert_eq!(opcode.value(), value);
        }
        assert_eq!(GatewayOpcode::from_value(13), None);
        assert_eq!(GatewayOpcode::Dispatch.to_string(), "dispatch");
        assert!(GatewayOpcode::Identify.is_client_send());
        assert!(GatewayOpcode::Heartbeat.is_client_send());
        assert!(GatewayOpcode::Heartbeat.is_server_send());
        assert!(!GatewayOpcode::Dispatch.is_client_send());
        assert!(GatewayOpcode::Hello.is_server_send());
        assert!(!GatewayOpcode::Resume.is_server_send());
    }

    #[test]
    fn close_codes_describe_and_report_reconnectability() {
        assert_eq!(GatewayCloseCode::NORMAL.value(), 1000);
        assert!(GatewayCloseCode::NORMAL.is_normal());
        assert!(!GatewayCloseCode::NORMAL.is_reconnectable());
        assert_eq!(
            GatewayCloseCode::AUTHENTICATION_FAILED.description(),
            "authentication failed"
        );
        assert!(!GatewayCloseCode::AUTHENTICATION_FAILED.is_reconnectable());
        assert!(!GatewayCloseCode::INVALID_SHARD.is_reconnectable());
        assert!(!GatewayCloseCode::DISALLOWED_INTENTS.is_reconnectable());
        assert!(GatewayCloseCode::GOING_AWAY.is_reconnectable());
        assert!(GatewayCloseCode::SESSION_TIMED_OUT.is_reconnectable());
        assert!(GatewayCloseCode::UNKNOWN_ERROR.is_reconnectable());
        assert_eq!(
            GatewayCloseCode::new(4999).description(),
            "unknown close code"
        );
        assert_eq!(
            GatewayCloseCode::new(4000).to_string(),
            "4000 (unknown error)"
        );
    }

    #[test]
    fn envelopes_parse_and_serialize() {
        let payload =
            GatewayPayload::from_json(r#"{"op":0,"d":{"name":"ready"},"s":42,"t":"READY"}"#)
                .unwrap();
        assert_eq!(payload.op, GatewayOpcode::Dispatch);
        assert_eq!(payload.sequence, Some(42));
        assert_eq!(payload.event.as_deref(), Some("READY"));
        assert_eq!(
            payload.to_json(),
            r#"{"op":0,"d":{"name":"ready"},"s":42,"t":"READY"}"#
        );

        let minimal = GatewayPayload::from_value(&json::from_str(r#"{"op":1}"#).unwrap()).unwrap();
        assert_eq!(minimal.op, GatewayOpcode::Heartbeat);
        assert!(minimal.data.is_null());
        assert_eq!(minimal.sequence, None);
        assert_eq!(minimal.event, None);
        assert_eq!(minimal.to_json(), r#"{"op":1,"d":null}"#);

        let built = GatewayPayload::new(GatewayOpcode::Hello, Value::object())
            .with_sequence(1)
            .with_event("HELLO");
        assert_eq!(
            built.to_value().get("t").and_then(Value::as_str),
            Some("HELLO")
        );
    }

    #[test]
    fn envelope_errors_are_reported() {
        let not_object = GatewayPayload::from_value(&Value::Null).unwrap_err();
        assert_eq!(
            not_object,
            GatewayPayloadError::InvalidField {
                field: "payload",
                expected: "an object"
            }
        );

        let missing = GatewayPayload::from_value(&Value::object()).unwrap_err();
        assert_eq!(missing, GatewayPayloadError::MissingField("op"));

        let unknown = GatewayPayload::from_json(r#"{"op":99}"#).unwrap_err();
        assert_eq!(unknown, GatewayPayloadError::UnknownOpcode(99));

        let bad_type = GatewayPayload::from_json(r#"{"op":"hello"}"#).unwrap_err();
        assert!(matches!(
            bad_type,
            GatewayPayloadError::InvalidField { field: "op", .. }
        ));

        let invalid_json = GatewayPayload::from_json("{").unwrap_err();
        assert!(matches!(invalid_json, GatewayPayloadError::Json(_)));
        assert!(
            invalid_json
                .to_string()
                .starts_with("invalid gateway JSON:")
        );
        assert!(std::error::Error::source(&invalid_json).is_some());
        assert!(std::error::Error::source(&missing).is_none());
    }

    #[test]
    fn hello_round_trips() {
        let hello = Hello {
            heartbeat_interval: 41_250,
        };
        let value = hello.to_value();
        assert_eq!(Hello::from_value(&value), Ok(hello));
        assert!(Hello::from_value(&Value::object()).is_err());
    }

    #[test]
    fn shard_ids_round_trip() {
        let shard = ShardId::new(1, 4);
        assert_eq!(
            shard.to_value(),
            Value::from(vec![Value::from(1_u64), Value::from(4_u64)])
        );
        assert_eq!(ShardId::from_value(&shard.to_value()), Ok(shard));
        assert!(ShardId::from_value(&Value::from("bad")).is_err());
        assert!(ShardId::from_value(&Value::from(vec![Value::from(1_u64)])).is_err());
        assert!(
            ShardId::from_value(&Value::from(vec![Value::from(-1_i64), Value::from(4_u64)]))
                .is_err()
        );
    }

    #[test]
    fn identify_round_trips() {
        let mut intents = Intents::none();
        intents.insert(Intents::GUILDS);
        intents.insert(Intents::GUILD_MESSAGES);
        let identify = Identify::new("token", intents)
            .with_properties(IdentifyProperties::new("linux", "browser", "device"))
            .with_compress(true)
            .with_large_threshold(100)
            .with_shard(ShardId::new(0, 2))
            .with_presence(Value::from("online"));

        let value = identify.to_value();
        assert_eq!(
            value.get("intents").and_then(Value::as_u64),
            Some(intents.bits())
        );
        assert_eq!(
            value
                .get("properties")
                .and_then(|properties| properties.get("os"))
                .and_then(Value::as_str),
            Some("linux")
        );
        assert_eq!(value.get("compress").and_then(Value::as_bool), Some(true));
        assert_eq!(Identify::from_value(&value), Ok(identify));

        let minimal = Identify::from_value(
            &json::from_str(
                r#"{"token":"t","intents":0,"properties":{"os":"o","browser":"b","device":"d"}}"#,
            )
            .unwrap(),
        )
        .unwrap();
        assert!(!minimal.compress);
        assert_eq!(minimal.large_threshold, 50);
        assert_eq!(minimal.shard, None);
        assert_eq!(minimal.presence, None);
        assert!(!IdentifyProperties::default().browser.is_empty());
    }

    #[test]
    fn resume_and_heartbeat_round_trip() {
        let resume = Resume::new("token", "session", 10);
        assert_eq!(Resume::from_value(&resume.to_value()), Ok(resume));

        assert_eq!(Heartbeat::new(None).to_value(), Value::Null);
        assert_eq!(
            Heartbeat::from_value(&Value::Null),
            Ok(Heartbeat::new(None))
        );
        assert_eq!(
            Heartbeat::from_value(&Value::from(7_u64)),
            Ok(Heartbeat::new(Some(7)))
        );
        assert_eq!(
            Heartbeat::from_value(&Value::from(7_i64)),
            Ok(Heartbeat::new(Some(7)))
        );
        assert!(Heartbeat::from_value(&Value::from(-1_i64)).is_err());
        assert!(Heartbeat::from_value(&Value::from("7")).is_err());
    }

    #[test]
    fn guild_member_requests_round_trip() {
        let request = RequestGuildMembers::new(100_u64, "prefix")
            .with_limit(100)
            .with_presences(true)
            .with_user_ids(vec![Snowflake::new(1), Snowflake::new(2)])
            .with_nonce("nonce");

        let value = request.to_value();
        assert_eq!(value.get("guild_id").and_then(Value::as_str), Some("100"));
        assert_eq!(
            value
                .get("user_ids")
                .and_then(Value::as_array)
                .map(<[Value]>::len),
            Some(2)
        );
        assert_eq!(RequestGuildMembers::from_value(&value), Ok(request));

        let minimal = RequestGuildMembers::from_value(
            &json::from_str(r#"{"guild_id":1,"limit":0}"#).unwrap(),
        )
        .unwrap();
        assert_eq!(minimal.query, "");
        assert!(!minimal.presences);
        assert_eq!(minimal.user_ids, None);

        assert!(
            RequestGuildMembers::from_value(
                &json::from_str(r#"{"guild_id":1,"limit":0,"user_ids":"bad"}"#).unwrap()
            )
            .is_err()
        );
    }

    #[test]
    fn voice_state_and_invalid_session_round_trip() {
        let update = UpdateVoiceState::new(1_u64, true, false).with_channel_id(2_u64);
        let value = update.to_value();
        assert_eq!(value.get("guild_id").and_then(Value::as_str), Some("1"));
        assert_eq!(value.get("channel_id").and_then(Value::as_str), Some("2"));
        assert_eq!(UpdateVoiceState::from_value(&value), Ok(update));

        let leave = UpdateVoiceState::new(1_u64, false, false);
        assert_eq!(leave.to_value().get("channel_id"), Some(&Value::Null));
        assert_eq!(
            UpdateVoiceState::from_value(
                &json::from_str(
                    r#"{"guild_id":"1","channel_id":null,"self_mute":false,"self_deaf":false}"#
                )
                .unwrap()
            ),
            Ok(leave)
        );

        let invalid = InvalidSession::new(true);
        assert_eq!(InvalidSession::from_value(&invalid.to_value()), Ok(invalid));
        assert!(InvalidSession::from_value(&Value::from("yes")).is_err());
    }
}
