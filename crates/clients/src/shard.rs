//! Shard assignment and identify concurrency helpers.

use std::time::Duration;

/// A shard identifier and total shard count.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Shard {
    /// Zero-based shard identifier.
    pub id: u32,
    /// Total number of shards.
    pub count: u32,
}

impl Shard {
    /// Creates a shard assignment.
    pub const fn new(id: u32, count: u32) -> Result<Self, &'static str> {
        if count == 0 || id >= count {
            return Err("invalid shard assignment");
        }
        Ok(Self { id, count })
    }

    /// Returns the gateway session start delay for a shard bucket.
    #[must_use]
    pub const fn identify_delay(self, max_concurrency: u32) -> Duration {
        let bucket = if max_concurrency == 0 {
            0
        } else {
            self.id % max_concurrency
        };
        Duration::from_secs(bucket as u64 * 5)
    }
}
