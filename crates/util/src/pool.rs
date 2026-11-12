//! Bounded worker thread pool implementation.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{Builder, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct SharedState {
    queue: VecDeque<Job>,
    shutdown: bool,
    active_workers: usize,
}

/// A builder for configuring a [`ThreadPool`].
#[derive(Clone, Debug)]
pub struct ThreadPoolBuilder {
    num_threads: usize,
    name_prefix: Option<String>,
    stack_size: Option<usize>,
}

impl Default for ThreadPoolBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ThreadPoolBuilder {
    /// Creates a builder initialized with default settings.
    #[must_use]
    pub fn new() -> Self {
        let default_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        Self {
            num_threads: default_threads.max(1),
            name_prefix: Some("discord-pool-worker".to_owned()),
            stack_size: None,
        }
    }

    /// Sets the number of worker threads in the pool.
    #[must_use]
    pub const fn num_threads(mut self, count: usize) -> Self {
        self.num_threads = if count == 0 { 1 } else { count };
        self
    }

    /// Sets the prefix for worker thread names.
    #[must_use]
    pub fn name_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.name_prefix = Some(prefix.into());
        self
    }

    /// Sets the stack size in bytes for worker threads.
    #[must_use]
    pub const fn stack_size(mut self, bytes: usize) -> Self {
        self.stack_size = Some(bytes);
        self
    }

    /// Constructs the [`ThreadPool`] with the configured settings.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if any OS thread fails to spawn.
    pub fn build(self) -> Result<ThreadPool, std::io::Error> {
        let state = Arc::new((
            Mutex::new(SharedState {
                queue: VecDeque::new(),
                shutdown: false,
                active_workers: 0,
            }),
            Condvar::new(),
        ));

        let is_shutdown = Arc::new(AtomicBool::new(false));
        let mut workers = Vec::with_capacity(self.num_threads);

        for index in 0..self.num_threads {
            let shared = Arc::clone(&state);
            let mut builder = Builder::new();
            if let Some(ref prefix) = self.name_prefix {
                builder = builder.name(format!("{prefix}-{index}"));
            }
            if let Some(stack) = self.stack_size {
                builder = builder.stack_size(stack);
            }

            let handle = builder.spawn(move || {
                let (lock, cvar) = &*shared;
                loop {
                    let mut guard = match lock.lock() {
                        Ok(g) => g,
                        Err(poisoned) => poisoned.into_inner(),
                    };

                    while guard.queue.is_empty() && !guard.shutdown {
                        guard = match cvar.wait(guard) {
                            Ok(g) => g,
                            Err(poisoned) => poisoned.into_inner(),
                        };
                    }

                    if guard.shutdown && guard.queue.is_empty() {
                        break;
                    }

                    if let Some(job) = guard.queue.pop_front() {
                        guard.active_workers += 1;
                        drop(guard);

                        // Execute job outside the lock
                        job();

                        let mut post_guard = match lock.lock() {
                            Ok(g) => g,
                            Err(poisoned) => poisoned.into_inner(),
                        };
                        post_guard.active_workers = post_guard.active_workers.saturating_sub(1);
                    }
                }
            })?;

            workers.push(handle);
        }

        Ok(ThreadPool {
            state,
            workers: Mutex::new(workers),
            is_shutdown,
            num_threads: self.num_threads,
        })
    }
}

/// A fixed-size bounded worker thread pool for parallel job execution.
pub struct ThreadPool {
    state: Arc<(Mutex<SharedState>, Condvar)>,
    workers: Mutex<Vec<JoinHandle<()>>>,
    is_shutdown: Arc<AtomicBool>,
    num_threads: usize,
}

impl ThreadPool {
    /// Creates a new [`ThreadPool`] with the specified number of threads.
    ///
    /// # Panics
    ///
    /// Panics if the OS fails to spawn the requested threads.
    #[must_use]
    pub fn new(num_threads: usize) -> Self {
        ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .expect("failed to spawn worker thread")
    }

    /// Creates a builder for fine-grained configuration.
    #[must_use]
    pub fn builder() -> ThreadPoolBuilder {
        ThreadPoolBuilder::new()
    }

    /// Submits a task for execution in the thread pool.
    ///
    /// If the pool has already been shut down, the job is not executed.
    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if self.is_shutdown.load(Ordering::Acquire) {
            return;
        }

        let (lock, cvar) = &*self.state;
        let mut guard = match lock.lock() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner(),
        };

        if guard.shutdown {
            return;
        }

        guard.queue.push_back(Box::new(job));
        cvar.notify_one();
    }

    /// Returns the total configured number of worker threads.
    #[must_use]
    pub const fn thread_count(&self) -> usize {
        self.num_threads
    }

    /// Returns the number of currently queued jobs pending execution.
    #[must_use]
    pub fn queued_jobs(&self) -> usize {
        let (lock, _) = &*self.state;
        let guard = match lock.lock() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner(),
        };
        guard.queue.len()
    }

    /// Returns the number of workers currently executing a task.
    #[must_use]
    pub fn active_workers(&self) -> usize {
        let (lock, _) = &*self.state;
        let guard = match lock.lock() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner(),
        };
        guard.active_workers
    }

    /// Returns whether the thread pool has been shut down.
    #[must_use]
    pub fn is_shutdown(&self) -> bool {
        self.is_shutdown.load(Ordering::Acquire)
    }

    /// Shuts down the thread pool and waits for all worker threads to complete.
    pub fn shutdown(&self) {
        if self.is_shutdown.swap(true, Ordering::AcqRel) {
            return;
        }

        let (lock, cvar) = &*self.state;
        {
            let mut guard = match lock.lock() {
                Ok(g) => g,
                Err(poisoned) => poisoned.into_inner(),
            };
            guard.shutdown = true;
            cvar.notify_all();
        }

        let mut workers = match self.workers.lock() {
            Ok(w) => w,
            Err(poisoned) => poisoned.into_inner(),
        };
        for worker in workers.drain(..) {
            let _ = worker.join();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    #[test]
    fn execute_multiple_tasks() {
        let pool = ThreadPool::new(4);
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..20 {
            let c = Arc::clone(&counter);
            pool.execute(move || {
                c.fetch_add(1, Ordering::SeqCst);
            });
        }

        // Wait for tasks to complete
        let start = std::time::Instant::now();
        while counter.load(Ordering::SeqCst) < 20 && start.elapsed() < Duration::from_secs(2) {
            std::thread::sleep(Duration::from_millis(5));
        }

        assert_eq!(counter.load(Ordering::SeqCst), 20);
    }

    #[test]
    fn pool_builder_configuration() {
        let pool = ThreadPool::builder()
            .num_threads(2)
            .name_prefix("test-worker")
            .stack_size(1024 * 1024)
            .build()
            .unwrap();

        assert_eq!(pool.thread_count(), 2);
        assert!(!pool.is_shutdown());
    }

    #[test]
    fn shutdown_joins_threads() {
        let pool = ThreadPool::new(2);
        let flag = Arc::new(AtomicBool::new(false));
        let f = Arc::clone(&flag);

        pool.execute(move || {
            std::thread::sleep(Duration::from_millis(20));
            f.store(true, Ordering::SeqCst);
        });

        pool.shutdown();
        assert!(pool.is_shutdown());
        assert!(flag.load(Ordering::SeqCst));
    }
}
