//! Application commands and interaction response builders.

#![forbid(unsafe_code)]

use data::{Object, Value};

pub mod ui;

pub use ui::{Component, ComponentRouter, Modal, View};

/// The kind of application command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandKind {
    /// A slash command.
    ChatInput,
    /// A user context-menu command.
    User,
    /// A message context-menu command.
    Message,
}

/// A command definition ready for registration.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandDefinition {
    /// The command name.
    pub name: String,
    /// The command description.
    pub description: String,
    /// The command kind.
    pub kind: CommandKind,
    /// The command options as raw JSON values.
    pub options: Vec<Value>,
}

impl CommandDefinition {
    /// Creates a slash command.
    #[must_use]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            kind: CommandKind::ChatInput,
            options: Vec::new(),
        }
    }

    /// Sets the command kind.
    #[must_use]
    pub const fn with_kind(mut self, kind: CommandKind) -> Self {
        self.kind = kind;
        self
    }

    /// Adds an option.
    #[must_use]
    pub fn with_option(mut self, option: Value) -> Self {
        self.options.push(option);
        self
    }

    /// Encodes the definition as a Discord-style JSON object.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert("name", Value::String(self.name.clone()));
        object.insert("description", Value::String(self.description.clone()));
        object.insert("type", Value::Number((self.kind as u64 + 1).into()));
        object.insert("options", Value::Array(self.options.clone()));
        Value::Object(object)
    }
}

/// An ordered collection of application commands.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommandTree {
    commands: Vec<CommandDefinition>,
}

/// Computes command registration changes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommandSync {
    /// Commands that must be created or replaced.
    pub upsert: Vec<CommandDefinition>,
    /// Names that should be removed remotely.
    pub remove: Vec<String>,
}

impl CommandSync {
    /// Computes a name-based synchronization plan.
    #[must_use]
    pub fn between(current: &CommandTree, desired: &CommandTree) -> Self {
        let mut sync = Self::default();
        for command in &desired.commands {
            if !current.commands.iter().any(|item| item.name == command.name) {
                sync.upsert.push(command.clone());
            }
        }
        for command in &current.commands {
            if !desired.commands.iter().any(|item| item.name == command.name) {
                sync.remove.push(command.name.clone());
            }
        }
        sync
    }
}

impl CommandTree {
    /// Creates an empty command tree.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    /// Adds a command, rejecting duplicate names.
    pub fn add(&mut self, command: CommandDefinition) -> Result<(), &'static str> {
        if self.commands.iter().any(|item| item.name == command.name) {
            return Err("command name already exists");
        }
        self.commands.push(command);
        Ok(())
    }

    /// Returns registered commands.
    #[must_use]
    pub fn commands(&self) -> &[CommandDefinition] {
        &self.commands
    }

    /// Encodes all commands as a JSON array.
    #[must_use]
    pub fn to_value(&self) -> Value {
        Value::Array(
            self.commands
                .iter()
                .map(CommandDefinition::to_value)
                .collect(),
        )
    }
}

/// Builds an interaction response payload.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InteractionResponse {
    /// Discord interaction response type.
    pub response_type: u8,
    /// Optional response data.
    pub data: Option<Value>,
}

/// Tracks the acknowledgement state of an interaction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InteractionState {
    /// No response has been sent.
    #[default]
    Pending,
    /// The interaction was acknowledged.
    Acknowledged,
    /// The interaction token expired.
    Expired,
}

/// A token-bound interaction response controller.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InteractionToken {
    token: String,
    state: InteractionState,
}

impl InteractionToken {
    /// Creates a pending interaction token.
    #[must_use]
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            state: InteractionState::Pending,
        }
    }

    /// Returns the token.
    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Returns the acknowledgement state.
    #[must_use]
    pub const fn state(&self) -> InteractionState {
        self.state
    }

    /// Acknowledges the interaction.
    pub fn acknowledge(&mut self) -> Result<(), &'static str> {
        if self.state != InteractionState::Pending {
            return Err("interaction is no longer pending");
        }
        self.state = InteractionState::Acknowledged;
        Ok(())
    }

    /// Marks the interaction expired.
    pub fn expire(&mut self) {
        self.state = InteractionState::Expired;
    }
}

impl InteractionResponse {
    /// Creates a response with a type.
    #[must_use]
    pub const fn new(response_type: u8) -> Self {
        Self {
            response_type,
            data: None,
        }
    }

    /// Adds response data.
    #[must_use]
    pub fn with_data(mut self, data: Value) -> Self {
        self.data = Some(data);
        self
    }

    /// Encodes the response as a JSON object.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert(
            "type",
            Value::Number(data::Number::from_u64(self.response_type as u64)),
        );
        if let Some(data) = &self.data {
            object.insert("data", data.clone());
        }
        Value::Object(object)
    }
}
