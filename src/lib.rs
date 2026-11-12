#![forbid(unsafe_code)]
//! # discord-rust
//!
//! A feature-rich, standard-library-only Discord API and Gateway library for Rust.
//!
//! Designed with zero external runtime dependencies (`async`/`tokio`-free, pure `std`),
//! strict `#![forbid(unsafe_code)]`, and modular architecture.
//!
//! ## Modules
//!
//! - [`abc`]: Abstract base components (Snowflake, Colour, CDN Asset)
//! - [`app`]: Application commands, context menus, and UI components
//! - [`clients`]: HTTP REST client, WebSocket gateway, sharding, cache, and bot runner
//! - [`commands`]: Command framework, arguments, and cog extensions
//! - [`data`]: JSON primitives and gateway payload encoding/decoding
//! - [`enums`]: Bitflags, permissions, intents, and opcodes
//! - [`event`]: Gateway event types and dispatching
//! - [`except`]: Error types and error hierarchy
//! - [`log`]: Zero-dependency structured logging framework
//! - [`models`]: Strongly-typed Discord domain models
//! - [`util`]: Percent-encoding and query utilities
//! - [`version`]: Discord API version constants and endpoint builders
//! - [`voice`]: Voice gateway connection, RTP packets, and encryption traits
//! - [`webhook`]: Webhook client and multipart execution

pub use abc;
pub use app;
pub use clients;
pub use commands;
pub use data;
pub use enums;
pub use event;
pub use except;
pub use log;
pub use models;
pub use util;
pub use version;
pub use voice;
pub use webhook;

// Re-export key high-level types at the crate root for ergonomics
pub use clients::bot::{Bot, BotBuilder, EventWaiter, Hooks};
pub use clients::client::HttpClient;
pub use clients::connection::GatewayConnection;
pub use clients::shard::{ConcurrentShardRuntime, ShardManager};
pub use enums::Intents;
pub use models::Event;
