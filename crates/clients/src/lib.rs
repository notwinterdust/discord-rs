//! Discord clients and gateway lifecycle.
//!
//! The crate provides the HTTP request, response and rate-limit primitives that
//! a REST client builds on, plus a transport-independent gateway state machine
//! covering identify, resume, heartbeat and reconnect decisions. Typed gateway
//! events are decoded and routed to handlers by [`GatewayRuntime`].
//! Socket implementations, sharding and the async runtime will follow in later
//! milestones.

#![forbid(unsafe_code)]

pub mod api;
pub mod cache;
pub mod client;
pub mod connection;
pub mod events;
pub mod gateway;
pub mod health;
pub mod http;
pub mod shard;

pub use api::DiscordApi;
pub use cache::Cache;
pub use client::{
    Client, ClientState, HttpClient, RetryPolicy, SecureTransport, TcpTransport, Transport,
};
pub use connection::{
    GatewayConnection, GatewayConnectionError, GatewayFrame, GatewayTransport,
    GatewayTransportError, WebSocketFrame,
};
pub use events::{EventDispatcher, EventFilter, GatewayRuntime, GatewayUpdate, RuntimeError};
pub use gateway::{
    Gateway, GatewayAction, GatewayConfig, GatewayError, GatewayState, ReconnectPolicy,
};
pub use health::HealthReport;
pub use http::{HttpMethod, HttpRequest, HttpResponse, RateLimit, RateLimitScope};
pub use shard::Shard;
