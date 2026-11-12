//! Bounded identifier-keyed caches for client models.

use std::collections::HashMap;

/// A simple bounded cache keyed by Discord IDs.
#[derive(Clone, Debug)]
pub struct Cache<T> {
    capacity: usize,
    values: HashMap<u64, T>,
}

impl<T> Cache<T> {
    /// Creates a cache with a maximum entry count.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            values: HashMap::new(),
        }
    }

    /// Inserts a value, evicting an arbitrary entry when full.
    pub fn insert(&mut self, id: u64, value: T) {
        if self.capacity == 0 {
            return;
        }
        if self.values.len() >= self.capacity && !self.values.contains_key(&id) {
            if let Some(key) = self.values.keys().next().copied() {
                self.values.remove(&key);
            }
        }
        self.values.insert(id, value);
    }

    /// Returns a cached value.
    #[must_use]
    pub fn get(&self, id: u64) -> Option<&T> {
        self.values.get(&id)
    }

    /// Removes a cached value.
    pub fn remove(&mut self, id: u64) -> Option<T> {
        self.values.remove(&id)
    }

    /// Clears the cache.
    pub fn clear(&mut self) {
        self.values.clear();
    }

    /// Removes all identifiers supplied by an invalidation event.
    pub fn invalidate<I>(&mut self, ids: I)
    where
        I: IntoIterator<Item = u64>,
    {
        for id in ids {
            self.values.remove(&id);
        }
    }

    /// Returns the current number of entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }
}
