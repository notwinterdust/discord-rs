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

/// A per-shard thread handle from the concurrent shard runtime.
pub struct ShardHandle {
    /// The shard assignment.
    pub shard: Shard,
    thread: std::thread::JoinHandle<()>,
}

impl ShardHandle {
    /// Waits for the shard thread to finish.
    pub fn join(self) -> std::thread::Result<()> {
        self.thread.join()
    }
}

impl std::fmt::Debug for ShardHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShardHandle")
            .field("shard", &self.shard)
            .finish_non_exhaustive()
    }
}

/// Concurrent shard runtime: spawns one thread per shard, merges events into a
/// shared `mpsc` channel, and respects max-concurrency bucket delays on startup.
pub struct ConcurrentShardRuntime {
    manager: ShardManager,
    event_tx: std::sync::mpsc::SyncSender<(Shard, String)>,
    /// Receives `(shard, raw_event_payload)` from all shard threads.
    pub event_rx: std::sync::mpsc::Receiver<(Shard, String)>,
}

impl ConcurrentShardRuntime {
    /// Creates a runtime from an existing [`ShardManager`].
    #[must_use]
    pub fn new(manager: ShardManager) -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(256);
        Self {
            manager,
            event_tx: tx,
            event_rx: rx,
        }
    }

    /// Spawns one thread per shard, applying identify delays per max-concurrency
    /// buckets.  Each thread calls `run` with its [`Shard`] assignment and a
    /// sender for raw event payloads.
    ///
    /// Returns one [`ShardHandle`] per shard.
    pub fn spawn<F>(&self, run: F) -> Vec<ShardHandle>
    where
        F: Fn(Shard, std::sync::mpsc::SyncSender<(Shard, String)>) + Send + Sync + 'static,
    {
        use std::sync::Arc;
        let run = Arc::new(run);
        let mut handles = Vec::new();
        for &shard in self.manager.shards() {
            let delay = self.manager.identify_delay(shard);
            let tx = self.event_tx.clone();
            let run = Arc::clone(&run);
            let thread = std::thread::spawn(move || {
                std::thread::sleep(delay);
                run(shard, tx);
            });
            handles.push(ShardHandle { shard, thread });
        }
        handles
    }
}
