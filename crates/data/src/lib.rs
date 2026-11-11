//! Raw gateway event payloads and data containers.
//!
//! The crate provides a dependency-free JSON value tree, parser and serializer
//! used by the REST and gateway layers, plus the gateway opcodes, close codes
//! and client payloads needed to connect and maintain a session.

#![forbid(unsafe_code)]

pub mod gateway;
pub mod json;
pub mod value;

pub use gateway::{
    GatewayCloseCode, GatewayOpcode, GatewayPayload, GatewayPayloadError, Heartbeat, Hello,
    Identify, IdentifyProperties, InvalidSession, RequestGuildMembers, Resume, ShardId,
    UpdateVoiceState,
};
pub use json::{MAX_DEPTH, ParseError, from_str, to_string, to_string_pretty};
pub use value::{Number, Object, Value};
