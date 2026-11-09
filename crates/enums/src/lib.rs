//! Discord enumerations and bitfields.
//! The first inhabitants of this crate are the two flag types used everywhere
//! in a Discord client: [`Intents`] for the gateway and [`Permissions`] for
//! guild and channel authorization.

#![forbid(unsafe_code)]

mod intents;
mod permissions;

pub use intents::Intents;
pub use permissions::Permissions;
