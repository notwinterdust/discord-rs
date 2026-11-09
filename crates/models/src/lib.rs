//! Discord API models.
//!
//! This crate will grow to contain `User`, `Guild`, `Channel`, `Message` and
//! the rest of the models documented under "Discord Models" in `discord.py`.
//! The first milestone covers users and the value types their fields need.

#![forbid(unsafe_code)]

mod user;

pub use abc::{Asset, Color, Colour, HasId, Snowflake};
pub use enums::PublicUserFlags;
pub use user::User;
