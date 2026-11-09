//! Discord API models.
//!
//! This crate will grow to contain `User`, `Guild`, `Channel`, `Message` and
//! the rest of Discord's API models. It currently covers users, client users,
//! guild roles and guild members.

#![forbid(unsafe_code)]

mod client_user;
mod member;
mod role;
mod user;

pub use abc::{Asset, Color, Colour, HasId, Snowflake};
pub use client_user::ClientUser;
pub use enums::{MemberFlags, Permissions, PublicUserFlags, RoleFlags};
pub use member::Member;
pub use role::{Role, RoleIcon, RoleTags};
pub use user::User;
