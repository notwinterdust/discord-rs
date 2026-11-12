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

/// Coordinates a fixed shard set.
#[derive(Clone, Debug)]
pub struct ShardManager {
    shards: Vec<Shard>,
    max_concurrency: u32,
}

impl ShardManager {
    /// Creates a manager automatically sizing from Discord gateway recommendations.
    pub fn from_gateway_recommendation(
        recommended_count: u32,
        max_concurrency: u32,
    ) -> Result<Self, &'static str> {
        Self::new(recommended_count.max(1), max_concurrency.max(1))
    }

    /// Creates a manager for `count` shards.
    pub fn new(count: u32, max_concurrency: u32) -> Result<Self, &'static str> {
        let mut shards = Vec::new();
        for id in 0..count {
            shards.push(Shard::new(id, count)?);
        }
        Ok(Self {
            shards,
            max_concurrency,
        })
    }

    /// Returns all shard assignments.
    #[must_use]
    pub fn shards(&self) -> &[Shard] {
        &self.shards
    }

    /// Returns the identify delay for one shard.
    #[must_use]
    pub fn identify_delay(&self, shard: Shard) -> Duration {
        shard.identify_delay(self.max_concurrency)
    }
}
