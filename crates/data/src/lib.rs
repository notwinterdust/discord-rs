//! Raw gateway event payloads and data containers.
//!
//! The crate currently provides a dependency-free JSON value tree, parser and
//! serializer used by the REST and gateway layers. Typed gateway payloads and
//! event containers will follow in later milestones.

#![forbid(unsafe_code)]

pub mod json;
pub mod value;

pub use json::{MAX_DEPTH, ParseError, from_str, to_string, to_string_pretty};
pub use value::{Number, Object, Value};
