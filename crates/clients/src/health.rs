//! Runtime health and shutdown diagnostics.

/// A point-in-time runtime health report.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HealthReport {
    /// Whether the REST transport is reachable.
    pub http_ready: bool,
    /// Whether the gateway session is connected.
    pub gateway_ready: bool,
    /// Number of cached objects.
    pub cached_objects: usize,
    /// Human-readable diagnostic details.
    pub details: String,
}

impl HealthReport {
    /// Creates an empty report.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            http_ready: false,
            gateway_ready: false,
            cached_objects: 0,
            details: String::new(),
        }
    }

    /// Returns whether all required runtime layers are ready.
    #[must_use]
    pub const fn is_healthy(&self) -> bool {
        self.http_ready && self.gateway_ready
    }
}
