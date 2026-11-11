//! Discord clients and gateway lifecycle.
//!
//! The crate provides the HTTP request, response and rate-limit primitives that
//! a REST client builds on, plus a transport-independent gateway state machine
//! covering identify, resume, heartbeat and reconnect decisions. Socket
//! implementations, sharding and the async runtime will follow in later
//! milestones.

#![forbid(unsafe_code)]

pub mod client;
pub mod gateway;
pub mod http;

pub use client::{HttpClient, Transport};
pub use gateway::{
    Gateway, GatewayAction, GatewayConfig, GatewayError, GatewayState, ReconnectPolicy,
};
pub use http::{HttpMethod, HttpRequest, HttpResponse, RateLimit, RateLimitScope};
