//! Typed gateway event dispatch.
//!
//! [`EventDispatcher`] routes decoded [`Event`] values to registered handlers.
//! [`GatewayRuntime`] combines it with the gateway state machine: one
//! [`GatewayRuntime::handle_payload`] call updates the session and invokes the
//! handlers whose filters match the event.

use std::error::Error;
use std::fmt;

use data::{GatewayCloseCode, GatewayOpcode, GatewayPayload};
use models::{CastError, Event};

use crate::gateway::{Gateway, GatewayAction, GatewayConfig, GatewayError, GatewayState};

type Handler = Box<dyn FnMut(&Event)>;

/// Selects which events a handler receives.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EventFilter {
    /// Every event, including names this library does not model yet.
    Any,
    /// Only events with this gateway name.
    Name(String),
}

impl EventFilter {
    /// Creates a filter that matches every event.
    #[must_use]
    pub const fn any() -> Self {
        Self::Any
    }

    /// Creates a filter that matches one gateway event name.
    #[must_use]
    pub fn name(name: impl Into<String>) -> Self {
        Self::Name(name.into())
    }

    /// Returns `true` if `event` should be delivered to the handler.
    #[must_use]
    pub fn matches(&self, event: &Event) -> bool {
        match self {
            Self::Any => true,
            Self::Name(name) => event.name() == name,
        }
    }
}

impl fmt::Display for EventFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Any => f.write_str("all events"),
            Self::Name(name) => f.write_str(name),
        }
    }
}

impl From<&str> for EventFilter {
    fn from(name: &str) -> Self {
        Self::Name(name.to_owned())
    }
}

impl From<String> for EventFilter {
    fn from(name: String) -> Self {
        Self::Name(name)
    }
}

struct HandlerEntry {
    filter: EventFilter,
    handler: Handler,
}

/// Routes decoded gateway events to registered handlers.
///
/// Handlers are called in registration order. A handler registered for
/// [`EventFilter::Any`] receives unknown events too, which makes it useful for
/// logging and for events added to Discord after this crate was built.
pub struct EventDispatcher {
    handlers: Vec<HandlerEntry>,
}

impl EventDispatcher {
    /// Creates an empty dispatcher.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    /// Returns the number of registered handlers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.handlers.len()
    }

    /// Returns `true` if no handlers are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.handlers.is_empty()
    }

    /// Registers a handler for the given filter.
    pub fn on(
        &mut self,
        filter: impl Into<EventFilter>,
        handler: impl FnMut(&Event) + 'static,
    ) -> &mut Self {
        self.handlers.push(HandlerEntry {
            filter: filter.into(),
            handler: Box::new(handler),
        });
        self
    }

    /// Registers a handler for one gateway event name.
    pub fn on_name(
        &mut self,
        name: impl Into<String>,
        handler: impl FnMut(&Event) + 'static,
    ) -> &mut Self {
        self.on(EventFilter::name(name), handler)
    }

    /// Registers a handler for every event.
    pub fn on_any(&mut self, handler: impl FnMut(&Event) + 'static) -> &mut Self {
        self.on(EventFilter::Any, handler)
    }

    /// Removes every registered handler.
    pub fn clear(&mut self) {
        self.handlers.clear();
    }

    /// Delivers an event to every matching handler.
    pub fn dispatch(&mut self, event: &Event) {
        for entry in &mut self.handlers {
            if entry.filter.matches(event) {
                (entry.handler)(event);
            }
        }
    }

    /// Decodes a dispatch payload, delivers its event, and returns it.
    ///
    /// Non-dispatch payloads are ignored.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a dispatch event's data is malformed.
    pub fn dispatch_payload(
        &mut self,
        payload: &GatewayPayload,
    ) -> Result<Option<Event>, CastError> {
        if payload.op != GatewayOpcode::Dispatch {
            return Ok(None);
        }
        let event = Event::from_payload(payload)?;
        self.dispatch(&event);
        Ok(Some(event))
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for EventDispatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EventDispatcher")
            .field("handlers", &self.handlers.len())
            .finish()
    }
}

/// The result of feeding one payload to a [`GatewayRuntime`].
#[derive(Clone, Debug, PartialEq)]
pub struct GatewayUpdate {
    /// An action the transport should perform, if the payload required one.
    pub action: Option<GatewayAction>,
    /// The decoded event, for dispatch payloads.
    pub event: Option<Event>,
}

impl GatewayUpdate {
    /// Creates an update from an action and an optional event.
    #[must_use]
    pub const fn new(action: Option<GatewayAction>, event: Option<Event>) -> Self {
        Self { action, event }
    }
}

/// An error produced while updating a [`GatewayRuntime`].
#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeError {
    /// The gateway lifecycle rejected the payload.
    Gateway(GatewayError),
    /// The payload's event data could not be decoded.
    Event(CastError),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gateway(error) => write!(f, "gateway error: {error}"),
            Self::Event(error) => write!(f, "gateway event error: {error}"),
        }
    }
}

impl Error for RuntimeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Gateway(error) => Some(error),
            Self::Event(error) => Some(error),
        }
    }
}

impl From<GatewayError> for RuntimeError {
    fn from(error: GatewayError) -> Self {
        Self::Gateway(error)
    }
}

impl From<CastError> for RuntimeError {
    fn from(error: CastError) -> Self {
        Self::Event(error)
    }
}

/// A gateway state machine paired with an event dispatcher.
///
/// The lifecycle is updated before the event is decoded. If decoding fails, the
/// error is returned but the lifecycle update is kept; a malformed event does
/// not roll back the session state, sequence number or reconnect bookkeeping.
pub struct GatewayRuntime {
    gateway: Gateway,
    events: EventDispatcher,
}

impl GatewayRuntime {
    /// Creates a runtime with an empty dispatcher.
    #[must_use]
    pub fn new(config: GatewayConfig) -> Self {
        Self::with_dispatcher(config, EventDispatcher::new())
    }

    /// Creates a runtime with the given dispatcher.
    #[must_use]
    pub fn with_dispatcher(config: GatewayConfig, events: EventDispatcher) -> Self {
        Self {
            gateway: Gateway::new(config),
            events,
        }
    }

    /// Returns the underlying gateway.
    #[must_use]
    pub const fn gateway(&self) -> &Gateway {
        &self.gateway
    }

    /// Returns the underlying gateway mutably.
    pub fn gateway_mut(&mut self) -> &mut Gateway {
        &mut self.gateway
    }

    /// Returns the event dispatcher.
    #[must_use]
    pub const fn events(&self) -> &EventDispatcher {
        &self.events
    }

    /// Returns the event dispatcher mutably.
    pub fn events_mut(&mut self) -> &mut EventDispatcher {
        &mut self.events
    }

    /// Consumes the runtime and returns its parts.
    #[must_use]
    pub fn into_parts(self) -> (Gateway, EventDispatcher) {
        (self.gateway, self.events)
    }

    /// Registers a handler for the given filter.
    pub fn on(
        &mut self,
        filter: impl Into<EventFilter>,
        handler: impl FnMut(&Event) + 'static,
    ) -> &mut Self {
        self.events.on(filter, handler);
        self
    }

    /// Registers a handler for one gateway event name.
    pub fn on_name(
        &mut self,
        name: impl Into<String>,
        handler: impl FnMut(&Event) + 'static,
    ) -> &mut Self {
        self.events.on_name(name, handler);
        self
    }

    /// Registers a handler for every event.
    pub fn on_any(&mut self, handler: impl FnMut(&Event) + 'static) -> &mut Self {
        self.events.on_any(handler);
        self
    }

    /// Returns the current gateway state.
    #[must_use]
    pub const fn state(&self) -> GatewayState {
        self.gateway.state()
    }

    /// Returns the session ID after a successful ready event.
    #[must_use]
    pub fn session_id(&self) -> Option<&str> {
        self.gateway.session_id()
    }

    /// Returns the last sequence number received.
    #[must_use]
    pub const fn sequence(&self) -> Option<u64> {
        self.gateway.sequence()
    }

    /// Returns `true` if the current session can be resumed.
    #[must_use]
    pub fn can_resume(&self) -> bool {
        self.gateway.can_resume()
    }

    /// Marks the socket as open, before `HELLO` arrives.
    ///
    /// # Errors
    ///
    /// Returns [`GatewayError::InvalidState`] unless the gateway is disconnected
    /// or reconnecting.
    pub fn on_connected(&mut self) -> Result<(), GatewayError> {
        self.gateway.on_connected()
    }

    /// Handles one payload, updating the lifecycle and dispatching its event.
    ///
    /// # Errors
    ///
    /// Returns [`RuntimeError::Gateway`] when the lifecycle rejects the payload
    /// and [`RuntimeError::Event`] when a dispatch event is malformed.
    pub fn handle_payload(
        &mut self,
        payload: &GatewayPayload,
    ) -> Result<GatewayUpdate, RuntimeError> {
        let action = self.gateway.handle_payload(payload)?;
        let event = if payload.op == GatewayOpcode::Dispatch {
            let event = Event::from_payload(payload)?;
            self.events.dispatch(&event);
            Some(event)
        } else {
            None
        };
        Ok(GatewayUpdate::new(action, event))
    }

    /// Produces the heartbeat that the timer should send.
    #[must_use]
    pub fn heartbeat(&mut self) -> Option<GatewayAction> {
        self.gateway.heartbeat()
    }

    /// Handles a transport failure.
    #[must_use]
    pub fn on_transport_error(&mut self) -> GatewayAction {
        self.gateway.on_transport_error()
    }

    /// Handles a socket close code.
    #[must_use]
    pub fn on_close(&mut self, code: GatewayCloseCode) -> GatewayAction {
        self.gateway.on_close(code)
    }

    /// Closes the gateway and clears the session.
    pub fn disconnect(&mut self) {
        self.gateway.disconnect();
    }
}

impl fmt::Debug for GatewayRuntime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GatewayRuntime")
            .field("gateway", &self.gateway)
            .field("events", &self.events)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::error::Error;
    use std::rc::Rc;

    use data::Value;
    use data::gateway::{GatewayOpcode, GatewayPayload, Hello};
    use enums::Intents;
    use models::Event;

    use super::{EventDispatcher, EventFilter, GatewayRuntime, RuntimeError};
    use crate::gateway::{GatewayAction, GatewayConfig, GatewayState};

    fn config() -> GatewayConfig {
        GatewayConfig::new("token", Intents::none())
    }

    fn hello() -> GatewayPayload {
        GatewayPayload::new(
            GatewayOpcode::Hello,
            Hello {
                heartbeat_interval: 1_000,
            }
            .to_value(),
        )
    }

    fn ready() -> GatewayPayload {
        data::gateway::GatewayPayload::from_json(
            r#"{
                "op": 0,
                "s": 1,
                "t": "READY",
                "d": {
                    "user": {"id": "1", "username": "winter"},
                    "session_id": "session",
                    "guilds": []
                }
            }"#,
        )
        .unwrap()
    }

    fn message_create() -> GatewayPayload {
        GatewayPayload::from_json(
            r#"{
                "op": 0,
                "s": 2,
                "t": "MESSAGE_CREATE",
                "d": {
                    "id": "10",
                    "channel_id": "4",
                    "guild_id": "2",
                    "author": {"id": "6", "username": "winter"},
                    "content": "hello",
                    "timestamp": "2021-08-01T12:34:56Z"
                }
            }"#,
        )
        .unwrap()
    }

    fn unknown() -> GatewayPayload {
        GatewayPayload::from_json(
            r#"{"op":0,"s":3,"t":"FUTURE_EVENT","d":{"id":"1","future":true}}"#,
        )
        .unwrap()
    }

    #[test]
    fn filters_and_dispatch_order() {
        let seen = Rc::new(RefCell::new(Vec::new()));

        let mut dispatcher = EventDispatcher::new();
        assert!(dispatcher.is_empty());

        let message_seen = Rc::clone(&seen);
        dispatcher.on("MESSAGE_CREATE", move |event| {
            message_seen
                .borrow_mut()
                .push(format!("message:{}", event.name()));
        });
        let any_seen = Rc::clone(&seen);
        dispatcher.on_any(move |event| {
            any_seen.borrow_mut().push(format!("any:{}", event.name()));
        });

        assert_eq!(dispatcher.len(), 2);
        assert_eq!(
            EventFilter::name("MESSAGE_CREATE"),
            EventFilter::from("MESSAGE_CREATE")
        );
        assert!(EventFilter::Any.matches(&Event::Resumed));

        let message = Event::from_value(
            "MESSAGE_CREATE",
            &data::json::from_str(
                r#"{
                    "id": "10",
                    "channel_id": "4",
                    "author": {"id": "6", "username": "winter"},
                    "timestamp": "2021-08-01T12:34:56Z"
                }"#,
            )
            .unwrap(),
        )
        .unwrap();
        dispatcher.dispatch(&message);
        dispatcher.dispatch(&Event::Resumed);

        assert_eq!(
            *seen.borrow(),
            vec![
                "message:MESSAGE_CREATE",
                "any:MESSAGE_CREATE",
                "any:RESUMED"
            ]
        );

        dispatcher.clear();
        assert!(dispatcher.is_empty());
        dispatcher.dispatch(&message);
        assert_eq!(seen.borrow().len(), 3);
    }

    #[test]
    fn dispatcher_decodes_only_dispatch_payloads() {
        let mut dispatcher = EventDispatcher::new();
        let seen = Rc::new(RefCell::new(0_u32));
        let counter = Rc::clone(&seen);
        dispatcher.on_any(move |_| *counter.borrow_mut() += 1);

        let hello = GatewayPayload::new(GatewayOpcode::Hello, Value::Null);
        assert_eq!(dispatcher.dispatch_payload(&hello).unwrap(), None);
        assert_eq!(*seen.borrow(), 0);

        let event = dispatcher.dispatch_payload(&message_create()).unwrap();
        assert!(matches!(event, Some(Event::MessageCreate(_))));
        assert_eq!(*seen.borrow(), 1);

        let malformed = GatewayPayload::new(GatewayOpcode::Dispatch, Value::object())
            .with_event("MESSAGE_DELETE");
        assert!(dispatcher.dispatch_payload(&malformed).is_err());
        assert_eq!(*seen.borrow(), 1);
    }

    #[test]
    fn runtime_updates_lifecycle_and_dispatches_events() {
        let received = Rc::new(RefCell::new(Vec::new()));
        let mut runtime = GatewayRuntime::new(config());

        let message_received = Rc::clone(&received);
        runtime.on("MESSAGE_CREATE", move |event| {
            if let Event::MessageCreate(message) = event {
                message_received.borrow_mut().push(message.content.clone());
            }
        });
        let unknown_received = Rc::clone(&received);
        runtime.on("FUTURE_EVENT", move |event| {
            unknown_received.borrow_mut().push(event.name().to_owned());
        });

        runtime.on_connected().unwrap();
        let update = runtime.handle_payload(&hello()).unwrap();
        assert_eq!(
            update.action.unwrap().payload().unwrap().op,
            GatewayOpcode::Identify
        );
        assert!(update.event.is_none());
        assert_eq!(runtime.state(), GatewayState::Identifying);

        let update = runtime.handle_payload(&ready()).unwrap();
        assert!(matches!(update.event, Some(Event::Ready(_))));
        assert_eq!(runtime.state(), GatewayState::Connected);
        assert_eq!(runtime.session_id(), Some("session"));
        assert_eq!(runtime.sequence(), Some(1));

        let update = runtime.handle_payload(&message_create()).unwrap();
        assert!(matches!(update.event, Some(Event::MessageCreate(_))));
        assert_eq!(runtime.sequence(), Some(2));

        let update = runtime.handle_payload(&unknown()).unwrap();
        assert!(matches!(update.event, Some(Event::Unknown { .. })));
        assert_eq!(runtime.sequence(), Some(3));

        assert_eq!(*received.borrow(), vec!["hello", "FUTURE_EVENT"]);
    }

    #[test]
    fn runtime_reports_event_errors_after_updating_the_sequence() {
        let mut runtime = GatewayRuntime::new(config());
        runtime.on_connected().unwrap();
        runtime.handle_payload(&hello()).unwrap();

        let malformed =
            GatewayPayload::from_json(r#"{"op":0,"s":4,"t":"MESSAGE_DELETE","d":{"id":"10"}}"#)
                .unwrap();
        let error = runtime.handle_payload(&malformed).unwrap_err();
        assert!(matches!(error, RuntimeError::Event(_)));
        assert_eq!(
            error.to_string(),
            "gateway event error: missing field `message_delete.channel_id`"
        );
        assert_eq!(runtime.sequence(), Some(4));
        assert!(error.source().is_some());

        let missing_name =
            GatewayPayload::new(GatewayOpcode::Dispatch, Value::object()).with_sequence(5);
        let error = runtime.handle_payload(&missing_name).unwrap_err();
        assert!(matches!(error, RuntimeError::Gateway(_)));
        assert_eq!(
            error.to_string(),
            "gateway error: dispatch payload has no event name"
        );

        let heartbeat_ack = GatewayPayload::new(GatewayOpcode::HeartbeatAck, Value::Null);
        let update = runtime.handle_payload(&heartbeat_ack).unwrap();
        assert!(update.action.is_none());
        assert!(update.event.is_none());
    }

    #[test]
    fn runtime_delegates_connection_controls() {
        let mut runtime = GatewayRuntime::new(config());
        runtime.on_connected().unwrap();
        runtime.handle_payload(&hello()).unwrap();

        assert_eq!(
            runtime.on_close(data::GatewayCloseCode::SESSION_TIMED_OUT),
            GatewayAction::Reconnect
        );
        assert_eq!(runtime.state(), GatewayState::Reconnecting);

        runtime.disconnect();
        assert_eq!(runtime.state(), GatewayState::Closed);
        assert!(!runtime.can_resume());
        assert_eq!(
            runtime.on_connected(),
            Err(crate::GatewayError::InvalidState {
                state: GatewayState::Closed
            })
        );
    }
}
