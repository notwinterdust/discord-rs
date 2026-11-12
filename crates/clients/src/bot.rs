//! Simplified bot builder and event waiter utilities.

use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use models::Event;

/// Hooks called around command dispatch.
#[derive(Default)]
pub struct Hooks {
    /// Called before every event dispatch.
    pub before_invoke: Option<Box<dyn Fn(&Event) + Send + Sync>>,
    /// Called after every successful event dispatch.
    pub after_invoke: Option<Box<dyn Fn(&Event) + Send + Sync>>,
    /// Called when an event handler returns an error (passed as a string).
    pub on_error: Option<Box<dyn Fn(&Event, &str) + Send + Sync>>,
}

impl std::fmt::Debug for Hooks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hooks").finish_non_exhaustive()
    }
}

/// An ergonomic bot configuration built with the builder pattern.
///
/// # Example
///
/// ```ignore
/// let bot = Bot::builder()
///     .token("Bot TOKEN")
///     .intents(0x8000)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug)]
pub struct Bot {
    /// Bot token (including the `Bot ` prefix).
    pub token: String,
    /// Gateway intents bitfield.
    pub intents: u64,
    /// Event-dispatch hooks.
    pub hooks: Hooks,
}

/// Builder for [`Bot`].
#[derive(Debug, Default)]
pub struct BotBuilder {
    token: Option<String>,
    intents: u64,
    hooks: Hooks,
}

impl BotBuilder {
    /// Sets the bot token.
    #[must_use]
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Sets the gateway intents bitfield.
    #[must_use]
    pub const fn intents(mut self, intents: u64) -> Self {
        self.intents = intents;
        self
    }

    /// Registers a `before_invoke` hook.
    #[must_use]
    pub fn before_invoke<F>(mut self, f: F) -> Self
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        self.hooks.before_invoke = Some(Box::new(f));
        self
    }

    /// Registers an `after_invoke` hook.
    #[must_use]
    pub fn after_invoke<F>(mut self, f: F) -> Self
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        self.hooks.after_invoke = Some(Box::new(f));
        self
    }

    /// Registers an `on_error` hook.
    #[must_use]
    pub fn on_error<F>(mut self, f: F) -> Self
    where
        F: Fn(&Event, &str) + Send + Sync + 'static,
    {
        self.hooks.on_error = Some(Box::new(f));
        self
    }

    /// Builds the [`Bot`].
    ///
    /// # Errors
    ///
    /// Returns an error when no token was provided.
    pub fn build(self) -> Result<Bot, &'static str> {
        let token = self.token.ok_or("token is required")?;
        Ok(Bot {
            token,
            intents: self.intents,
            hooks: self.hooks,
        })
    }
}

impl Bot {
    /// Returns a new builder.
    #[must_use]
    pub fn builder() -> BotBuilder {
        BotBuilder::default()
    }
}

/// Subscribes to events matching a predicate, blocking until one arrives or the
/// timeout elapses.
///
/// Each call to [`EventWaiter::wait_for`] registers a one-shot listener on the
/// shared sender that feeds the waiter's internal channel.
pub struct EventWaiter {
    listeners: Arc<Mutex<Vec<Sender<Event>>>>,
}

impl Default for EventWaiter {
    fn default() -> Self {
        Self::new()
    }
}

impl EventWaiter {
    /// Creates a new waiter.
    #[must_use]
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Call this from your event-dispatch loop to fan out to any waiting
    /// listeners.
    pub fn dispatch(&self, event: &Event) {
        let mut listeners = self.listeners.lock().unwrap_or_else(|e| e.into_inner());
        listeners.retain(|tx| tx.send(event.clone()).is_ok());
    }

    /// Returns the number of active listeners.
    #[must_use]
    pub fn listener_count(&self) -> usize {
        let listeners = self.listeners.lock().unwrap_or_else(|e| e.into_inner());
        listeners.len()
    }

    /// Blocks the calling thread until an event matching `predicate` arrives or
    /// `timeout` elapses.
    ///
    /// # Errors
    ///
    /// Returns [`RecvTimeoutError::Timeout`] when the deadline is reached before
    /// a matching event arrives, or [`RecvTimeoutError::Disconnected`] when the
    /// waiter is dropped.
    pub fn wait_for<F>(
        &self,
        predicate: F,
        timeout: Duration,
    ) -> Result<Event, RecvTimeoutError>
    where
        F: Fn(&Event) -> bool,
    {
        let (tx, rx): (Sender<Event>, Receiver<Event>) = mpsc::channel();
        {
            let mut listeners = self.listeners.lock().unwrap_or_else(|e| e.into_inner());
            listeners.push(tx);
        }
        let deadline = std::time::Instant::now() + timeout;
        loop {
            let remaining = deadline.saturating_duration_since(std::time::Instant::now());
            if remaining.is_zero() {
                return Err(RecvTimeoutError::Timeout);
            }
            match rx.recv_timeout(remaining) {
                Ok(event) if predicate(&event) => return Ok(event),
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bot_builder_requires_token() {
        assert!(Bot::builder().intents(0x8000).build().is_err());
    }

    #[test]
    fn bot_builder_produces_bot() {
        let bot = Bot::builder()
            .token("Bot TOKEN")
            .intents(0x8000)
            .build()
            .unwrap();
        assert_eq!(bot.token, "Bot TOKEN");
        assert_eq!(bot.intents, 0x8000);
    }

    #[test]
    fn event_waiter_dispatches_to_listener() {
        use std::sync::Arc;
        use std::thread;

        let waiter = Arc::new(EventWaiter::new());
        let waiter2 = Arc::clone(&waiter);

        let handle = thread::spawn(move || {
            waiter2.wait_for(
                |e| matches!(e, Event::Resumed),
                Duration::from_secs(2),
            )
        });

        // Wait until the listener is registered.
        while waiter.listener_count() == 0 {
            thread::sleep(Duration::from_millis(5));
        }

        // Dispatch a non-matching event first (e.g. unknown), then the matching one.
        waiter.dispatch(&Event::Unknown {
            name: "OTHER".into(),
            data: data::Value::Null,
        });
        waiter.dispatch(&Event::Resumed);

        let result = handle.join().unwrap();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Event::Resumed);
    }

    #[test]
    fn event_waiter_times_out() {
        let waiter = EventWaiter::new();
        let result = waiter.wait_for(
            |_| false,
            Duration::from_millis(10),
        );
        assert_eq!(result, Err(RecvTimeoutError::Timeout));
    }

    #[test]
    fn bot_builder_hooks_are_stored() {
        let bot = Bot::builder()
            .token("Bot T")
            .before_invoke(|_| {})
            .after_invoke(|_| {})
            .on_error(|_, _| {})
            .build()
            .unwrap();
        assert!(bot.hooks.before_invoke.is_some());
        assert!(bot.hooks.after_invoke.is_some());
        assert!(bot.hooks.on_error.is_some());
    }
}

