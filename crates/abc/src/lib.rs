//! Abstract base concepts and shared value types used by Discord models.
//!
//! Rust has no multiple inheritance, so behaviour that models used to inherit is
//! expressed through traits instead.
//!
//! Later model crates (`models`, `event`, ...) store a [`Snowflake`] for their
//! `id` field and implement [`HasId`] so generic code can work with any Discord
//! object that has an ID.

#![forbid(unsafe_code)]

mod asset;
mod colour;
mod snowflake;
mod user;

pub use asset::{Asset, AssetError, AssetFormat, CDN_BASE, valid_icon_size};
pub use colour::{Color, Colour, ColourError};
pub use snowflake::{DISCORD_EPOCH_MS, HasId, Snowflake};
pub use user::User;
