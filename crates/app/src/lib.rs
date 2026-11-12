//! Application commands and interaction response builders.

#![forbid(unsafe_code)]

use data::{Object, Value};
use std::collections::HashMap;

pub mod ui;

pub use ui::{
    ActionRow, Button, ButtonStyle, Component, ComponentRouter, Container, Modal, PersistentViews,
    SelectOption, StringSelect, TextInput, TextInputStyle, View,
};

/// A choice for an application command option.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandOptionChoice {
    /// The choice display name (1-100 characters).
    pub name: String,
    /// The choice value.
    pub value: Value,
    /// Name localizations.
    pub name_localizations: HashMap<String, String>,
}

impl CommandOptionChoice {
    /// Creates a choice with a string value.
    #[must_use]
    pub fn string(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: Value::String(value.into()),
            name_localizations: HashMap::new(),
        }
    }

    /// Creates a choice with an integer value.
    #[must_use]
    pub fn integer(name: impl Into<String>, value: i64) -> Self {
        Self {
            name: name.into(),
            value: Value::Number(data::Number::from_i64(value)),
            name_localizations: HashMap::new(),
        }
    }

    /// Creates a choice with a floating point value.
    #[must_use]
    pub fn number(name: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            value: data::Number::from_f64(value)
                .map(Value::Number)
                .unwrap_or(Value::Null),
            name_localizations: HashMap::new(),
        }
    }

    /// Adds a name localization.
    #[must_use]
    pub fn with_localization(mut self, locale: impl Into<String>, name: impl Into<String>) -> Self {
        self.name_localizations.insert(locale.into(), name.into());
        self
    }

    /// Encodes the choice to JSON.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert("name", Value::String(self.name.clone()));
        object.insert("value", self.value.clone());
        if !self.name_localizations.is_empty() {
            let mut locs = Object::new();
            for (k, v) in &self.name_localizations {
                locs.insert(k, Value::String(v.clone()));
            }
            object.insert("name_localizations", Value::Object(locs));
        }
        Value::Object(object)
    }
}

/// A typed application command option.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandOption {
    /// The option name (1-32 lowercase characters).
    pub name: String,
    /// The option description (1-100 characters).
    pub description: String,
    /// The option type.
    pub kind: enums::ApplicationCommandOptionType,
    /// Whether the option is required.
    pub required: bool,
    /// Choices for string/integer/number options (max 25).
    pub choices: Vec<CommandOptionChoice>,
    /// Nested options for Subcommand or SubcommandGroup.
    pub options: Vec<CommandOption>,
    /// Minimum value (for integer or number).
    pub min_value: Option<f64>,
    /// Maximum value (for integer or number).
    pub max_value: Option<f64>,
    /// Minimum length (for string).
    pub min_length: Option<u16>,
    /// Maximum length (for string).
    pub max_length: Option<u16>,
    /// Whether autocomplete is enabled for this option.
    pub autocomplete: bool,
    /// Channel types permitted (for Channel option).
    pub channel_types: Vec<u8>,
    /// Name localizations.
    pub name_localizations: HashMap<String, String>,
    /// Description localizations.
    pub description_localizations: HashMap<String, String>,
}

impl CommandOption {
    /// Creates a new option.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        kind: enums::ApplicationCommandOptionType,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            kind,
            required: false,
            choices: Vec::new(),
            options: Vec::new(),
            min_value: None,
            max_value: None,
            min_length: None,
            max_length: None,
            autocomplete: false,
            channel_types: Vec::new(),
            name_localizations: HashMap::new(),
            description_localizations: HashMap::new(),
        }
    }

    /// Creates a string option.
    #[must_use]
    pub fn string(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, enums::ApplicationCommandOptionType::String)
    }

    /// Creates an integer option.
    #[must_use]
    pub fn integer(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, enums::ApplicationCommandOptionType::Integer)
    }

    /// Creates a boolean option.
    #[must_use]
    pub fn boolean(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, enums::ApplicationCommandOptionType::Boolean)
    }

    /// Creates a user option.
    #[must_use]
    pub fn user(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, enums::ApplicationCommandOptionType::User)
    }

    /// Creates a channel option.
    #[must_use]
    pub fn channel(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, enums::ApplicationCommandOptionType::Channel)
    }

    /// Creates a role option.
    #[must_use]
    pub fn role(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, enums::ApplicationCommandOptionType::Role)
    }

    /// Creates a mentionable option.
    #[must_use]
    pub fn mentionable(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, enums::ApplicationCommandOptionType::Mentionable)
    }

    /// Creates a number (floating point) option.
    #[must_use]
    pub fn number(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, enums::ApplicationCommandOptionType::Number)
    }

    /// Creates an attachment option.
    #[must_use]
    pub fn attachment(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, enums::ApplicationCommandOptionType::Attachment)
    }

    /// Creates a subcommand option.
    #[must_use]
    pub fn subcommand(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, enums::ApplicationCommandOptionType::Subcommand)
    }

    /// Creates a subcommand group option.
    #[must_use]
    pub fn subcommand_group(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, enums::ApplicationCommandOptionType::SubcommandGroup)
    }

    /// Sets whether the option is required.
    #[must_use]
    pub const fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Adds a choice.
    #[must_use]
    pub fn with_choice(mut self, choice: CommandOptionChoice) -> Self {
        self.choices.push(choice);
        self
    }

    /// Adds a nested option (for Subcommand or SubcommandGroup).
    #[must_use]
    pub fn with_option(mut self, option: CommandOption) -> Self {
        self.options.push(option);
        self
    }

    /// Sets autocomplete support.
    #[must_use]
    pub const fn autocomplete(mut self, autocomplete: bool) -> Self {
        self.autocomplete = autocomplete;
        self
    }

    /// Sets string min and max length constraints.
    #[must_use]
    pub const fn with_length_bounds(mut self, min: Option<u16>, max: Option<u16>) -> Self {
        self.min_length = min;
        self.max_length = max;
        self
    }

    /// Adds a name localization.
    #[must_use]
    pub fn with_name_localization(mut self, locale: impl Into<String>, name: impl Into<String>) -> Self {
        self.name_localizations.insert(locale.into(), name.into());
        self
    }

    /// Adds a description localization.
    #[must_use]
    pub fn with_description_localization(
        mut self,
        locale: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.description_localizations.insert(locale.into(), description.into());
        self
    }

    /// Encodes the option to JSON.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert("name", Value::String(self.name.clone()));
        object.insert("description", Value::String(self.description.clone()));
        object.insert(
            "type",
            Value::Number(data::Number::from_u64(self.kind as u64)),
        );
        if self.required {
            object.insert("required", Value::Bool(true));
        }
        if self.autocomplete {
            object.insert("autocomplete", Value::Bool(true));
        }
        if !self.choices.is_empty() {
            object.insert(
                "choices",
                Value::Array(self.choices.iter().map(CommandOptionChoice::to_value).collect()),
            );
        }
        if !self.options.is_empty() {
            object.insert(
                "options",
                Value::Array(self.options.iter().map(CommandOption::to_value).collect()),
            );
        }
        if let Some(min) = self.min_value {
            if let Some(num) = data::Number::from_f64(min) {
                object.insert("min_value", Value::Number(num));
            }
        }
        if let Some(max) = self.max_value {
            if let Some(num) = data::Number::from_f64(max) {
                object.insert("max_value", Value::Number(num));
            }
        }
        if let Some(min) = self.min_length {
            object.insert("min_length", Value::Number(data::Number::from_u64(min as u64)));
        }
        if let Some(max) = self.max_length {
            object.insert("max_length", Value::Number(data::Number::from_u64(max as u64)));
        }
        if !self.name_localizations.is_empty() {
            let mut locs = Object::new();
            for (k, v) in &self.name_localizations {
                locs.insert(k, Value::String(v.clone()));
            }
            object.insert("name_localizations", Value::Object(locs));
        }
        if !self.description_localizations.is_empty() {
            let mut locs = Object::new();
            for (k, v) in &self.description_localizations {
                locs.insert(k, Value::String(v.clone()));
            }
            object.insert("description_localizations", Value::Object(locs));
        }
        Value::Object(object)
    }
}

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
    /// The command name (1-32 characters).
    pub name: String,
    /// The command description (1-100 characters for ChatInput; empty for User/Message).
    pub description: String,
    /// The command kind.
    pub kind: CommandKind,
    /// The command options as raw JSON values.
    pub options: Vec<Value>,
    /// Default member permissions bitfield (string of decimal integer).
    pub default_member_permissions: Option<String>,
    /// Whether the command is enabled in DMs (default true).
    pub dm_permission: Option<bool>,
    /// Name localizations map.
    pub name_localizations: HashMap<String, String>,
    /// Description localizations map.
    pub description_localizations: HashMap<String, String>,
    /// Whether the command is age-restricted / NSFW.
    pub nsfw: bool,
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
            default_member_permissions: None,
            dm_permission: None,
            name_localizations: HashMap::new(),
            description_localizations: HashMap::new(),
            nsfw: false,
        }
    }

    /// Creates a user context menu command.
    #[must_use]
    pub fn user_context_menu(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            kind: CommandKind::User,
            options: Vec::new(),
            default_member_permissions: None,
            dm_permission: None,
            name_localizations: HashMap::new(),
            description_localizations: HashMap::new(),
            nsfw: false,
        }
    }

    /// Creates a message context menu command.
    #[must_use]
    pub fn message_context_menu(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            kind: CommandKind::Message,
            options: Vec::new(),
            default_member_permissions: None,
            dm_permission: None,
            name_localizations: HashMap::new(),
            description_localizations: HashMap::new(),
            nsfw: false,
        }
    }

    /// Sets the command kind.
    #[must_use]
    pub const fn with_kind(mut self, kind: CommandKind) -> Self {
        self.kind = kind;
        self
    }

    /// Adds a raw JSON option.
    #[must_use]
    pub fn with_option(mut self, option: Value) -> Self {
        self.options.push(option);
        self
    }

    /// Adds a typed CommandOption.
    #[must_use]
    pub fn with_typed_option(mut self, option: CommandOption) -> Self {
        self.options.push(option.to_value());
        self
    }

    /// Sets default member permissions bitfield.
    #[must_use]
    pub fn with_default_member_permissions(mut self, perms: impl Into<String>) -> Self {
        self.default_member_permissions = Some(perms.into());
        self
    }

    /// Sets whether the command is allowed in DMs.
    #[must_use]
    pub const fn with_dm_permission(mut self, allowed: bool) -> Self {
        self.dm_permission = Some(allowed);
        self
    }

    /// Sets whether the command is age-restricted.
    #[must_use]
    pub const fn with_nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = nsfw;
        self
    }

    /// Adds a name localization.
    #[must_use]
    pub fn with_name_localization(mut self, locale: impl Into<String>, name: impl Into<String>) -> Self {
        self.name_localizations.insert(locale.into(), name.into());
        self
    }

    /// Adds a description localization.
    #[must_use]
    pub fn with_description_localization(
        mut self,
        locale: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.description_localizations.insert(locale.into(), description.into());
        self
    }

    /// Encodes the definition as a Discord-style JSON object.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert("name", Value::String(self.name.clone()));
        if self.kind == CommandKind::ChatInput {
            object.insert("description", Value::String(self.description.clone()));
        } else {
            object.insert("description", Value::String(String::new()));
        }
        object.insert("type", Value::Number((self.kind as u64 + 1).into()));
        if self.kind == CommandKind::ChatInput && !self.options.is_empty() {
            object.insert("options", Value::Array(self.options.clone()));
        }
        if let Some(perms) = &self.default_member_permissions {
            object.insert("default_member_permissions", Value::String(perms.clone()));
        }
        if let Some(dm) = self.dm_permission {
            object.insert("dm_permission", Value::Bool(dm));
        }
        if self.nsfw {
            object.insert("nsfw", Value::Bool(true));
        }
        if !self.name_localizations.is_empty() {
            let mut locs = Object::new();
            for (k, v) in &self.name_localizations {
                locs.insert(k, Value::String(v.clone()));
            }
            object.insert("name_localizations", Value::Object(locs));
        }
        if !self.description_localizations.is_empty() {
            let mut locs = Object::new();
            for (k, v) in &self.description_localizations {
                locs.insert(k, Value::String(v.clone()));
            }
            object.insert("description_localizations", Value::Object(locs));
        }
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
            if !current
                .commands
                .iter()
                .any(|item| item.name == command.name)
            {
                sync.upsert.push(command.clone());
            }
        }
        for command in &current.commands {
            if !desired
                .commands
                .iter()
                .any(|item| item.name == command.name)
            {
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

    /// Responds with a channel message immediately (Type 4).
    #[must_use]
    pub fn channel_message(data: Value) -> Self {
        Self::new(4).with_data(data)
    }

    /// Acknowledges an interaction and sends a message later (Type 5).
    #[must_use]
    pub const fn deferred_channel_message() -> Self {
        Self::new(5)
    }

    /// For components, acknowledges an interaction and edits the message later (Type 6).
    #[must_use]
    pub const fn deferred_update_message() -> Self {
        Self::new(6)
    }

    /// For components, edits the message the component was attached to (Type 7).
    #[must_use]
    pub fn update_message(data: Value) -> Self {
        Self::new(7).with_data(data)
    }

    /// Responds to an autocomplete interaction with suggested choices (Type 8).
    #[must_use]
    pub fn autocomplete(choices: Vec<CommandOptionChoice>) -> Self {
        let mut data = Object::new();
        data.insert(
            "choices",
            Value::Array(choices.into_iter().map(|c| c.to_value()).collect()),
        );
        Self::new(8).with_data(Value::Object(data))
    }

    /// Responds with a modal popup (Type 9).
    #[must_use]
    pub fn modal(data: Value) -> Self {
        Self::new(9).with_data(data)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_command_options_encode_to_json() {
        let opt = CommandOption::string("animal", "The animal to choose")
            .required(true)
            .autocomplete(true)
            .with_length_bounds(Some(1), Some(50))
            .with_choice(CommandOptionChoice::string("Dog", "dog"))
            .with_name_localization("fr", "animal_fr");

        let val = opt.to_value();
        match val {
            Value::Object(obj) => {
                assert_eq!(obj.get("name"), Some(&Value::String("animal".into())));
                assert_eq!(obj.get("type"), Some(&Value::Number(3_u64.into())));
                assert_eq!(obj.get("required"), Some(&Value::Bool(true)));
                assert_eq!(obj.get("autocomplete"), Some(&Value::Bool(true)));
                assert_eq!(obj.get("min_length"), Some(&Value::Number(1_u64.into())));
                assert_eq!(obj.get("max_length"), Some(&Value::Number(50_u64.into())));
                assert!(obj.contains_key("choices"));
                assert!(obj.contains_key("name_localizations"));
            }
            _ => panic!("expected object"),
        }
    }

    #[test]
    fn subcommands_and_groups_nesting() {
        let sub = CommandOption::subcommand("view", "View an item")
            .with_option(CommandOption::integer("id", "Item ID").required(true));

        let group = CommandOption::subcommand_group("item", "Item management").with_option(sub);

        let cmd = CommandDefinition::new("manage", "Management commands")
            .with_typed_option(group)
            .with_default_member_permissions("8")
            .with_dm_permission(false)
            .with_nsfw(true);

        let val = cmd.to_value();
        match val {
            Value::Object(obj) => {
                assert_eq!(obj.get("name"), Some(&Value::String("manage".into())));
                assert_eq!(
                    obj.get("default_member_permissions"),
                    Some(&Value::String("8".into()))
                );
                assert_eq!(obj.get("dm_permission"), Some(&Value::Bool(false)));
                assert_eq!(obj.get("nsfw"), Some(&Value::Bool(true)));
                assert!(obj.contains_key("options"));
            }
            _ => panic!("expected object"),
        }
    }

    #[test]
    fn context_menu_commands_have_no_options_and_empty_description() {
        let user_cmd = CommandDefinition::user_context_menu("Get User Avatar");
        assert_eq!(user_cmd.kind, CommandKind::User);
        assert!(user_cmd.description.is_empty());
        let val = user_cmd.to_value();
        match val {
            Value::Object(obj) => {
                assert_eq!(obj.get("type"), Some(&Value::Number(2_u64.into())));
                assert_eq!(obj.get("description"), Some(&Value::String(String::new())));
                assert!(!obj.contains_key("options"));
            }
            _ => panic!("expected object"),
        }

        let msg_cmd = CommandDefinition::message_context_menu("Bookmark Message");
        assert_eq!(msg_cmd.kind, CommandKind::Message);
        let val = msg_cmd.to_value();
        match val {
            Value::Object(obj) => {
                assert_eq!(obj.get("type"), Some(&Value::Number(3_u64.into())));
            }
            _ => panic!("expected object"),
        }
    }

    #[test]
    fn interaction_response_types_and_autocomplete() {
        let resp = InteractionResponse::autocomplete(vec![
            CommandOptionChoice::string("Option 1", "opt1"),
            CommandOptionChoice::string("Option 2", "opt2"),
        ]);
        assert_eq!(resp.response_type, 8);
        assert!(resp.data.is_some());

        let def_resp = InteractionResponse::deferred_channel_message();
        assert_eq!(def_resp.response_type, 5);
        assert!(def_resp.data.is_none());
    }
}
