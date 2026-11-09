//! Discord enumerations and bitfields.
//! The first inhabitants of this crate are the two flag types used everywhere
//! in a Discord client: [`Intents`] for the gateway and [`Permissions`] for
//! guild and channel authorization.

#![forbid(unsafe_code)]

mod intents;
mod member_flags;
mod permissions;
mod public_user_flags;
mod role_flags;

pub use intents::Intents;
pub use member_flags::MemberFlags;
pub use permissions::Permissions;
pub use public_user_flags::PublicUserFlags;
pub use role_flags::RoleFlags;
