//! Discord clients and gateway lifecycle.
//!
//! The crate currently provides the HTTP request, response and rate-limit
//! primitives that a REST client builds on. Gateway lifecycle, sharding and
//! reconnection will follow in later milestones.

#![forbid(unsafe_code)]

pub mod client;
pub mod http;

pub use client::{HttpClient, Transport};
pub use http::{HttpMethod, HttpRequest, HttpResponse, RateLimit, RateLimitScope};
