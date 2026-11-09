//! Abstract base concepts shared by Discord models.
//!
//! Rust has no multiple inheritance, so the behaviour that 
//! models used to inherit is expressed through small traits and 
//! shared value types instead.
//!
//! The first building block is [`Snowflake`], Discord's unique identifier
//! format. Later model crates (`models`, `event`, ...) are expected to store a
//! [`Snowflake`] for their `id` field and implement [`HasId`] so generic code
//! can work with any Discord object that has an ID.

#![forbid(unsafe_code)]

mod snowflake;

pub use snowflake::{DISCORD_EPOCH_MS, HasId, Snowflake};
