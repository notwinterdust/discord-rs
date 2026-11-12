//! Typed Discord events dispatched to client callbacks.

#![forbid(unsafe_code)]

pub use models::Event;

/// Handles one decoded gateway event.
pub trait EventHandler {
    /// Delivers an event to the handler.
    fn handle(&mut self, event: &Event);
}

impl<F> EventHandler for F
where
    F: FnMut(&Event),
{
    fn handle(&mut self, event: &Event) {
        self(event);
    }
}

/// A small ordered collection of event handlers.
#[derive(Default)]
pub struct EventBus {
    handlers: Vec<Box<dyn EventHandler>>,
}

impl EventBus {
    /// Creates an empty event bus.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    /// Registers a handler.
    pub fn add(&mut self, handler: impl EventHandler + 'static) {
        self.handlers.push(Box::new(handler));
    }

    /// Delivers an event to every registered handler.
    pub fn dispatch(&mut self, event: &Event) {
        for handler in &mut self.handlers {
            handler.handle(event);
        }
    }

    /// Returns the number of registered handlers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.handlers.len()
    }
}
