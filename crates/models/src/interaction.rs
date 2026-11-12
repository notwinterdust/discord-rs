//! Interaction models.
//!
//! An [`Interaction`] is sent when a user invokes an application command, uses
//! a message component or submits a modal. The associated [`InteractionData`]
//! describes what was invoked, while [`InteractionResponse`] describes the
//! response that should be sent back to Discord.

use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::time::SystemTime;

use abc::{HasId, Snowflake};
use data::{Object, Value};
use enums::{
    ApplicationCommandOptionType, ApplicationCommandType, ApplicationIntegrationType,
    ComponentType, InteractionContextType, InteractionResponseType, InteractionType, MessageFlags,
    Permissions,
};

use crate::cast::{self, CastError};
use crate::{
    ActionRow, Attachment, Embed, GuildChannel, Member, Message, MessageAuthor, Poll, Role, User,
};

/// A value supplied for an application command option.
#[derive(Clone, Debug, PartialEq)]
pub enum CommandOptionValue {
    /// A string value.
    String(String),
    /// An integer value.
    Integer(i64),
    /// A boolean value.
    Boolean(bool),
    /// A user, channel, role or mentionable ID.
    Snowflake(Snowflake),
    /// A floating-point value.
    Number(f64),
}

impl CommandOptionValue {
    /// Returns the value as a string, if it is one.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the value as an integer, if it is one.
    #[must_use]
    pub const fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the value as a boolean, if it is one.
    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the value as a snowflake, if it is one.
    #[must_use]
    pub const fn as_snowflake(&self) -> Option<Snowflake> {
        match self {
            Self::Snowflake(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the value as a float, if it is a number.
    #[must_use]
    pub const fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(value) => Some(*value),
            Self::Integer(value) => Some(*value as f64),
            _ => None,
        }
    }
}

impl From<&str> for CommandOptionValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<String> for CommandOptionValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<i64> for CommandOptionValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<bool> for CommandOptionValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<Snowflake> for CommandOptionValue {
    fn from(value: Snowflake) -> Self {
        Self::Snowflake(value)
    }
}

impl From<f64> for CommandOptionValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl fmt::Display for CommandOptionValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => f.write_str(value),
            Self::Integer(value) => write!(f, "{value}"),
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Snowflake(value) => write!(f, "{value}"),
            Self::Number(value) => write!(f, "{value}"),
        }
    }
}

/// An option supplied to an application command.
#[derive(Clone, Debug, PartialEq)]
pub struct ApplicationCommandOption {
    /// The option's name.
    pub name: String,
    /// The option's type.
    pub kind: ApplicationCommandOptionType,
    /// The option's supplied value, if it has one.
    pub value: Option<CommandOptionValue>,
    /// Nested options for subcommands and subcommand groups.
    pub options: Vec<Self>,
    /// Whether this option is the currently focused autocomplete option.
    pub focused: bool,
}

impl ApplicationCommandOption {
    /// Creates an option with the given name and type.
    #[must_use]
    pub fn new(name: impl Into<String>, kind: ApplicationCommandOptionType) -> Self {
        Self {
            name: name.into(),
            kind,
            value: None,
            options: Vec::new(),
            focused: false,
        }
    }

    /// Sets the option's value.
    #[must_use]
    pub fn with_value(mut self, value: impl Into<CommandOptionValue>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Sets the option's nested options.
    #[must_use]
    pub fn with_options(mut self, options: Vec<Self>) -> Self {
        self.options = options;
        self
    }

    /// Appends a nested option.
    #[must_use]
    pub fn add_option(mut self, option: Self) -> Self {
        self.options.push(option);
        self
    }

    /// Sets whether this is the focused autocomplete option.
    #[must_use]
    pub fn with_focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Returns the option's supplied value, if any.
    #[must_use]
    pub const fn value(&self) -> Option<&CommandOptionValue> {
        self.value.as_ref()
    }

    /// Returns `true` if the option contains nested options.
    #[must_use]
    pub const fn is_subcommand(&self) -> bool {
        self.kind.is_subcommand()
    }

    /// Returns the nested option with the given name, if it exists.
    #[must_use]
    pub fn option(&self, name: &str) -> Option<&Self> {
        self.find_option(name)
    }

    fn find_option(&self, name: &str) -> Option<&Self> {
        for option in &self.options {
            if option.name == name {
                return Some(option);
            }
            if let Some(found) = option.find_option(name) {
                return Some(found);
            }
        }
        None
    }
}

/// The users, members and other objects resolved from an interaction payload.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResolvedData {
    /// Resolved users, keyed by ID.
    pub users: HashMap<Snowflake, User>,
    /// Resolved guild members, keyed by user ID.
    pub members: HashMap<Snowflake, Member>,
    /// Resolved roles, keyed by ID.
    pub roles: HashMap<Snowflake, Role>,
    /// Resolved channels, keyed by ID.
    pub channels: HashMap<Snowflake, GuildChannel>,
    /// Resolved messages, keyed by ID.
    pub messages: HashMap<Snowflake, Message>,
    /// Resolved attachments, keyed by ID.
    pub attachments: HashMap<Snowflake, Attachment>,
}

impl ResolvedData {
    /// Creates an empty resolved-data set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds resolved users.
    #[must_use]
    pub fn with_users(mut self, users: impl IntoIterator<Item = User>) -> Self {
        for user in users {
            self.users.insert(user.id, user);
        }
        self
    }

    /// Adds resolved members.
    #[must_use]
    pub fn with_members(mut self, members: impl IntoIterator<Item = Member>) -> Self {
        for member in members {
            self.members.insert(member.id(), member);
        }
        self
    }

    /// Adds resolved roles.
    #[must_use]
    pub fn with_roles(mut self, roles: impl IntoIterator<Item = Role>) -> Self {
        for role in roles {
            self.roles.insert(role.id, role);
        }
        self
    }

    /// Adds resolved channels.
    #[must_use]
    pub fn with_channels(mut self, channels: impl IntoIterator<Item = GuildChannel>) -> Self {
        for channel in channels {
            self.channels.insert(channel.id, channel);
        }
        self
    }

    /// Adds resolved messages.
    #[must_use]
    pub fn with_messages(mut self, messages: impl IntoIterator<Item = Message>) -> Self {
        for message in messages {
            self.messages.insert(message.id, message);
        }
        self
    }

    /// Adds resolved attachments.
    #[must_use]
    pub fn with_attachments(mut self, attachments: impl IntoIterator<Item = Attachment>) -> Self {
        for attachment in attachments {
            self.attachments.insert(attachment.id, attachment);
        }
        self
    }

    /// Returns the resolved user with the given ID.
    #[must_use]
    pub fn user(&self, id: impl Into<Snowflake>) -> Option<&User> {
        self.users.get(&id.into())
    }

    /// Returns the resolved member with the given user ID.
    #[must_use]
    pub fn member(&self, id: impl Into<Snowflake>) -> Option<&Member> {
        self.members.get(&id.into())
    }

    /// Returns the resolved role with the given ID.
    #[must_use]
    pub fn role(&self, id: impl Into<Snowflake>) -> Option<&Role> {
        self.roles.get(&id.into())
    }

    /// Returns the resolved channel with the given ID.
    #[must_use]
    pub fn channel(&self, id: impl Into<Snowflake>) -> Option<&GuildChannel> {
        self.channels.get(&id.into())
    }

    /// Returns the resolved message with the given ID.
    #[must_use]
    pub fn message(&self, id: impl Into<Snowflake>) -> Option<&Message> {
        self.messages.get(&id.into())
    }

    /// Returns the resolved attachment with the given ID.
    #[must_use]
    pub fn attachment(&self, id: impl Into<Snowflake>) -> Option<&Attachment> {
        self.attachments.get(&id.into())
    }

    /// Returns `true` if no objects were resolved.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.users.is_empty()
            && self.members.is_empty()
            && self.roles.is_empty()
            && self.channels.is_empty()
            && self.messages.is_empty()
            && self.attachments.is_empty()
    }
}

/// The data attached to an application command interaction.
#[derive(Clone, Debug, PartialEq)]
pub struct ApplicationCommandData {
    /// The invoked command's ID.
    pub id: Snowflake,
    /// The invoked command's name.
    pub name: String,
    /// The command's type.
    pub kind: ApplicationCommandType,
    /// The guild in which the command was invoked, if any.
    pub guild_id: Option<Snowflake>,
    /// The user or message targeted by a context menu command.
    pub target_id: Option<Snowflake>,
    /// The options supplied to the command.
    pub options: Vec<ApplicationCommandOption>,
    /// Objects resolved from the command's options.
    pub resolved: Option<ResolvedData>,
}

impl ApplicationCommandData {
    /// Creates command data with the given ID, name and type.
    #[must_use]
    pub fn new(
        id: impl Into<Snowflake>,
        name: impl Into<String>,
        kind: ApplicationCommandType,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind,
            guild_id: None,
            target_id: None,
            options: Vec::new(),
            resolved: None,
        }
    }

    /// Sets the guild in which the command was invoked.
    #[must_use]
    pub fn with_guild_id(mut self, guild_id: impl Into<Snowflake>) -> Self {
        self.guild_id = Some(guild_id.into());
        self
    }

    /// Sets the context menu target ID.
    #[must_use]
    pub fn with_target_id(mut self, target_id: impl Into<Snowflake>) -> Self {
        self.target_id = Some(target_id.into());
        self
    }

    /// Sets the command's options.
    #[must_use]
    pub fn with_options(mut self, options: Vec<ApplicationCommandOption>) -> Self {
        self.options = options;
        self
    }

    /// Appends a command option.
    #[must_use]
    pub fn add_option(mut self, option: ApplicationCommandOption) -> Self {
        self.options.push(option);
        self
    }

    /// Sets the resolved objects.
    #[must_use]
    pub fn with_resolved(mut self, resolved: ResolvedData) -> Self {
        self.resolved = Some(resolved);
        self
    }

    /// Returns the top-level option with the given name.
    #[must_use]
    pub fn option(&self, name: &str) -> Option<&ApplicationCommandOption> {
        self.options.iter().find(|option| option.name == name)
    }

    /// Returns the option with the given name, including nested options.
    #[must_use]
    pub fn find_option(&self, name: &str) -> Option<&ApplicationCommandOption> {
        for option in &self.options {
            if option.name == name {
                return Some(option);
            }
            if let Some(found) = option.find_option(name) {
                return Some(found);
            }
        }
        None
    }
}

/// The data attached to a message component interaction.
#[derive(Clone, Debug, PartialEq)]
pub struct MessageComponentData {
    /// The custom ID of the component that was used.
    pub custom_id: String,
    /// The component's type.
    pub component_type: ComponentType,
    /// The values selected in a select menu.
    pub values: Vec<String>,
    /// Objects resolved from a select menu's values.
    pub resolved: Option<ResolvedData>,
}

impl MessageComponentData {
    /// Creates component data with the given custom ID and type.
    #[must_use]
    pub fn new(custom_id: impl Into<String>, component_type: ComponentType) -> Self {
        Self {
            custom_id: custom_id.into(),
            component_type,
            values: Vec::new(),
            resolved: None,
        }
    }

    /// Sets the selected values.
    #[must_use]
    pub fn with_values(mut self, values: Vec<String>) -> Self {
        self.values = values;
        self
    }

    /// Sets the resolved objects.
    #[must_use]
    pub fn with_resolved(mut self, resolved: ResolvedData) -> Self {
        self.resolved = Some(resolved);
        self
    }

    /// Returns the first selected value, if any.
    #[must_use]
    pub fn value(&self) -> Option<&str> {
        self.values.first().map(String::as_str)
    }

    /// Returns `true` if a button was used.
    #[must_use]
    pub const fn is_button(&self) -> bool {
        matches!(self.component_type, ComponentType::Button)
    }

    /// Returns `true` if a select menu was used.
    #[must_use]
    pub const fn is_select(&self) -> bool {
        self.component_type.is_select()
    }
}

/// A component received in a modal submission.
#[derive(Clone, Debug, PartialEq)]
pub enum ModalSubmitComponent {
    /// A row of submitted components.
    ActionRow {
        /// The row's components.
        components: Vec<Self>,
    },
    /// A submitted text input.
    TextInput {
        /// The input's custom ID.
        custom_id: String,
        /// The submitted value.
        value: String,
    },
    /// A submitted select menu.
    Select {
        /// The menu's custom ID.
        custom_id: String,
        /// The menu's type.
        kind: ComponentType,
        /// The submitted values.
        values: Vec<String>,
    },
    /// A submitted file upload.
    FileUpload {
        /// The upload's custom ID.
        custom_id: String,
        /// The submitted file IDs.
        values: Vec<String>,
    },
    /// A text display shown in the modal.
    TextDisplay {
        /// The displayed content.
        content: String,
    },
    /// A label wrapping another component.
    Label {
        /// The wrapped component.
        component: Box<Self>,
    },
    /// A submitted radio group.
    RadioGroup {
        /// The group's custom ID.
        custom_id: String,
        /// The group's component ID.
        id: u64,
        /// The selected option value, if any.
        value: Option<String>,
    },
    /// A submitted checkbox group.
    CheckboxGroup {
        /// The group's custom ID.
        custom_id: String,
        /// The group's component ID.
        id: u64,
        /// The selected option values.
        values: Vec<String>,
    },
    /// A submitted checkbox.
    Checkbox {
        /// The checkbox's custom ID.
        custom_id: String,
        /// The checkbox's component ID.
        id: u64,
        /// Whether the checkbox was checked.
        value: bool,
    },
}

impl ModalSubmitComponent {
    /// Returns the component's custom ID, if it has one.
    #[must_use]
    pub fn custom_id(&self) -> Option<&str> {
        match self {
            Self::ActionRow { .. } | Self::TextDisplay { .. } => None,
            Self::TextInput { custom_id, .. }
            | Self::Select { custom_id, .. }
            | Self::FileUpload { custom_id, .. }
            | Self::RadioGroup { custom_id, .. }
            | Self::CheckboxGroup { custom_id, .. }
            | Self::Checkbox { custom_id, .. } => Some(custom_id),
            Self::Label { component } => component.custom_id(),
        }
    }

    /// Returns the submitted values for select menus, file uploads and
    /// checkbox groups.
    #[must_use]
    pub fn values(&self) -> Option<&[String]> {
        match self {
            Self::Select { values, .. }
            | Self::FileUpload { values, .. }
            | Self::CheckboxGroup { values, .. } => Some(values),
            _ => None,
        }
    }

    /// Returns the submitted text value for a text input.
    #[must_use]
    pub fn text_value(&self) -> Option<&str> {
        match self {
            Self::TextInput { value, .. } => Some(value),
            _ => None,
        }
    }

    /// Returns the component with the given custom ID.
    #[must_use]
    pub fn find(&self, custom_id: &str) -> Option<&Self> {
        if self.custom_id() == Some(custom_id) {
            return Some(self);
        }

        match self {
            Self::ActionRow { components } => components
                .iter()
                .find_map(|component| component.find(custom_id)),
            Self::Label { component } => component.find(custom_id),
            _ => None,
        }
    }
}

/// The data attached to a modal submission.
#[derive(Clone, Debug, PartialEq)]
pub struct ModalSubmitData {
    /// The modal's custom ID.
    pub custom_id: String,
    /// The submitted components.
    pub components: Vec<ModalSubmitComponent>,
    /// Objects resolved from the modal's components.
    pub resolved: Option<ResolvedData>,
}

impl ModalSubmitData {
    /// Creates modal data with the given custom ID.
    #[must_use]
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            custom_id: custom_id.into(),
            components: Vec::new(),
            resolved: None,
        }
    }

    /// Sets the submitted components.
    #[must_use]
    pub fn with_components(mut self, components: Vec<ModalSubmitComponent>) -> Self {
        self.components = components;
        self
    }

    /// Appends a submitted component.
    #[must_use]
    pub fn add_component(mut self, component: ModalSubmitComponent) -> Self {
        self.components.push(component);
        self
    }

    /// Sets the resolved objects.
    #[must_use]
    pub fn with_resolved(mut self, resolved: ResolvedData) -> Self {
        self.resolved = Some(resolved);
        self
    }

    /// Returns the submitted component with the given custom ID.
    #[must_use]
    pub fn component(&self, custom_id: &str) -> Option<&ModalSubmitComponent> {
        self.components
            .iter()
            .find_map(|component| component.find(custom_id))
    }
}

/// The data attached to an interaction.
#[derive(Clone, Debug, PartialEq)]
pub enum InteractionData {
    /// Application command data, used by command and autocomplete interactions.
    ApplicationCommand(Box<ApplicationCommandData>),
    /// Message component data.
    MessageComponent(MessageComponentData),
    /// Modal submission data.
    ModalSubmit(ModalSubmitData),
}

impl InteractionData {
    /// Returns the application command data, if present.
    #[must_use]
    pub fn application_command(&self) -> Option<&ApplicationCommandData> {
        match self {
            Self::ApplicationCommand(data) => Some(data),
            _ => None,
        }
    }

    /// Returns the message component data, if present.
    #[must_use]
    pub const fn message_component(&self) -> Option<&MessageComponentData> {
        match self {
            Self::MessageComponent(data) => Some(data),
            _ => None,
        }
    }

    /// Returns the modal submission data, if present.
    #[must_use]
    pub const fn modal_submit(&self) -> Option<&ModalSubmitData> {
        match self {
            Self::ModalSubmit(data) => Some(data),
            _ => None,
        }
    }

    /// Returns the resolved objects, if the data has any.
    #[must_use]
    pub fn resolved(&self) -> Option<&ResolvedData> {
        match self {
            Self::ApplicationCommand(data) => data.resolved.as_ref(),
            Self::MessageComponent(data) => data.resolved.as_ref(),
            Self::ModalSubmit(data) => data.resolved.as_ref(),
        }
    }

    /// Returns the invoked command's name, if this is command data.
    #[must_use]
    pub fn command_name(&self) -> Option<&str> {
        self.application_command().map(|data| data.name.as_str())
    }

    /// Returns the component or modal custom ID, if the data has one.
    #[must_use]
    pub fn custom_id(&self) -> Option<&str> {
        match self {
            Self::ApplicationCommand(_) => None,
            Self::MessageComponent(data) => Some(&data.custom_id),
            Self::ModalSubmit(data) => Some(&data.custom_id),
        }
    }
}

/// An interaction sent to an application.
///
/// Equality and hashing are based on the interaction's ID.
#[derive(Clone, Debug)]
pub struct Interaction {
    /// The interaction's unique ID.
    pub id: Snowflake,
    /// The interaction's type.
    pub kind: InteractionType,
    /// The application the interaction belongs to.
    pub application_id: Snowflake,
    /// The interaction's data, when the type carries any.
    pub data: Option<InteractionData>,
    /// The ID of the guild the interaction was sent from, if any.
    pub guild_id: Option<Snowflake>,
    /// The ID of the channel the interaction was sent from, if known.
    pub channel_id: Option<Snowflake>,
    /// The user or member that invoked the interaction.
    pub user: Option<MessageAuthor>,
    /// The token used to respond to the interaction.
    pub token: String,
    /// The interaction payload version.
    pub version: u8,
    /// The invoking user's locale.
    pub locale: Option<String>,
    /// The guild's preferred locale, if the interaction came from a guild.
    pub guild_locale: Option<String>,
    /// The application's resolved permissions.
    pub app_permissions: Option<Permissions>,
    /// The context in which the interaction was invoked.
    pub context: Option<InteractionContextType>,
    /// The maximum attachment size accepted by the application.
    pub attachment_size_limit: Option<u64>,
    /// The SKU IDs the invoking user has entitlements for.
    pub entitlement_sku_ids: Vec<Snowflake>,
    /// The integration owners that authorized the interaction.
    pub authorizing_integration_owners: HashMap<ApplicationIntegrationType, Snowflake>,
}

impl Interaction {
    /// Creates an interaction with the fields Discord always provides.
    #[must_use]
    pub fn new(
        id: impl Into<Snowflake>,
        kind: InteractionType,
        application_id: impl Into<Snowflake>,
        token: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            application_id: application_id.into(),
            data: None,
            guild_id: None,
            channel_id: None,
            user: None,
            token: token.into(),
            version: 1,
            locale: None,
            guild_locale: None,
            app_permissions: None,
            context: None,
            attachment_size_limit: None,
            entitlement_sku_ids: Vec::new(),
            authorizing_integration_owners: HashMap::new(),
        }
    }

    /// Sets the interaction's data.
    #[must_use]
    pub fn with_data(mut self, data: InteractionData) -> Self {
        self.data = Some(data);
        self
    }

    /// Sets the guild ID.
    #[must_use]
    pub fn with_guild_id(mut self, guild_id: impl Into<Snowflake>) -> Self {
        self.guild_id = Some(guild_id.into());
        self
    }

    /// Sets the channel ID.
    #[must_use]
    pub fn with_channel_id(mut self, channel_id: impl Into<Snowflake>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    /// Sets the invoking user.
    #[must_use]
    pub fn with_user(mut self, user: impl Into<MessageAuthor>) -> Self {
        self.user = Some(user.into());
        self
    }

    /// Sets the invoking guild member.
    #[must_use]
    pub fn with_member(mut self, member: Member) -> Self {
        self.user = Some(MessageAuthor::from_member(member));
        self
    }

    /// Sets the payload version.
    #[must_use]
    pub fn with_version(mut self, version: u8) -> Self {
        self.version = version;
        self
    }

    /// Sets the invoking user's locale.
    #[must_use]
    pub fn with_locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    /// Sets the guild's preferred locale.
    #[must_use]
    pub fn with_guild_locale(mut self, guild_locale: impl Into<String>) -> Self {
        self.guild_locale = Some(guild_locale.into());
        self
    }

    /// Sets the application's resolved permissions.
    #[must_use]
    pub fn with_app_permissions(mut self, permissions: Permissions) -> Self {
        self.app_permissions = Some(permissions);
        self
    }

    /// Sets the interaction context.
    #[must_use]
    pub fn with_context(mut self, context: InteractionContextType) -> Self {
        self.context = Some(context);
        self
    }

    /// Sets the maximum attachment size.
    #[must_use]
    pub fn with_attachment_size_limit(mut self, limit: u64) -> Self {
        self.attachment_size_limit = Some(limit);
        self
    }

    /// Sets the entitlement SKU IDs.
    #[must_use]
    pub fn with_entitlement_sku_ids(mut self, sku_ids: Vec<Snowflake>) -> Self {
        self.entitlement_sku_ids = sku_ids;
        self
    }

    /// Adds an integration owner.
    #[must_use]
    pub fn with_authorizing_integration_owner(
        mut self,
        kind: ApplicationIntegrationType,
        owner_id: impl Into<Snowflake>,
    ) -> Self {
        self.authorizing_integration_owners
            .insert(kind, owner_id.into());
        self
    }

    /// Returns the interaction's creation time.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.id.created_at()
    }

    /// Returns the invoking user or member.
    #[must_use]
    pub const fn user(&self) -> Option<&MessageAuthor> {
        self.user.as_ref()
    }

    /// Returns the interaction's data, if present.
    #[must_use]
    pub const fn data(&self) -> Option<&InteractionData> {
        self.data.as_ref()
    }

    /// Returns the invoked command's name, if this is command data.
    #[must_use]
    pub fn command_name(&self) -> Option<&str> {
        self.data.as_ref().and_then(InteractionData::command_name)
    }

    /// Returns the component or modal custom ID, if present.
    #[must_use]
    pub fn custom_id(&self) -> Option<&str> {
        self.data.as_ref().and_then(InteractionData::custom_id)
    }

    /// Returns the resolved objects attached to the interaction data.
    #[must_use]
    pub fn resolved(&self) -> Option<&ResolvedData> {
        self.data.as_ref().and_then(InteractionData::resolved)
    }

    /// Returns `true` if this is a ping.
    #[must_use]
    pub const fn is_ping(&self) -> bool {
        matches!(self.kind, InteractionType::Ping)
    }

    /// Returns `true` if this is an application command.
    #[must_use]
    pub const fn is_application_command(&self) -> bool {
        matches!(self.kind, InteractionType::ApplicationCommand)
    }

    /// Returns `true` if this is an autocomplete interaction.
    #[must_use]
    pub const fn is_autocomplete(&self) -> bool {
        matches!(self.kind, InteractionType::Autocomplete)
    }

    /// Returns `true` if this is a message component interaction.
    #[must_use]
    pub const fn is_component(&self) -> bool {
        matches!(self.kind, InteractionType::MessageComponent)
    }

    /// Returns `true` if this is a modal submission.
    #[must_use]
    pub const fn is_modal_submit(&self) -> bool {
        matches!(self.kind, InteractionType::ModalSubmit)
    }
}

impl PartialEq for Interaction {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Interaction {}

impl Hash for Interaction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for Interaction {
    fn id(&self) -> Snowflake {
        self.id
    }
}

/// A message sent as an interaction response.
///
/// Interaction messages behave like ordinary messages; this wrapper makes their
/// origin explicit.
#[derive(Clone, Debug)]
pub struct InteractionMessage {
    /// The underlying message.
    pub message: Message,
}

impl InteractionMessage {
    /// Creates an interaction message from a message.
    #[must_use]
    pub const fn new(message: Message) -> Self {
        Self { message }
    }

    /// Returns the underlying message.
    #[must_use]
    pub const fn as_message(&self) -> &Message {
        &self.message
    }

    /// Consumes the wrapper and returns the underlying message.
    #[must_use]
    pub fn into_message(self) -> Message {
        self.message
    }
}

impl From<Message> for InteractionMessage {
    fn from(message: Message) -> Self {
        Self::new(message)
    }
}

impl Deref for InteractionMessage {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

impl PartialEq for InteractionMessage {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message
    }
}

impl Eq for InteractionMessage {}

impl Hash for InteractionMessage {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.message.hash(state);
    }
}

impl HasId for InteractionMessage {
    fn id(&self) -> Snowflake {
        self.message.id
    }
}

/// The interaction that a message is a response to.
///
/// Equality and hashing are based on the interaction's ID.
#[derive(Clone, Debug)]
pub struct MessageInteraction {
    /// The interaction's ID.
    pub id: Snowflake,
    /// The interaction's type.
    pub kind: InteractionType,
    /// The interaction's name.
    pub name: String,
    /// The user or member that invoked the interaction.
    pub user: MessageAuthor,
}

impl MessageInteraction {
    /// Creates a message interaction.
    #[must_use]
    pub fn new(
        id: impl Into<Snowflake>,
        kind: InteractionType,
        name: impl Into<String>,
        user: impl Into<MessageAuthor>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            name: name.into(),
            user: user.into(),
        }
    }

    /// Decodes a message interaction from a Discord message interaction
    /// object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "interaction")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Self::from_object_at(object, path)
    }

    pub(crate) fn from_object_at(object: &Object, path: &str) -> Result<Self, CastError> {
        let user_path = cast::field_path(path, "user");
        let user = User::from_object_at(cast::required_object(object, path, "user")?, &user_path)?;
        Ok(Self {
            id: cast::required_snowflake(object, path, "id")?,
            kind: cast::required_enum(
                object,
                path,
                "type",
                "an interaction type",
                InteractionType::from_value,
            )?,
            name: cast::required_string(object, path, "name")?.to_owned(),
            user: MessageAuthor::from_user(user),
        })
    }

    /// Returns the interaction's creation time.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.id.created_at()
    }
}

impl PartialEq for MessageInteraction {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for MessageInteraction {}

impl Hash for MessageInteraction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for MessageInteraction {
    fn id(&self) -> Snowflake {
        self.id
    }
}

/// Detailed interaction metadata attached to a message.
///
/// Equality and hashing are based on the interaction's ID.
#[derive(Clone, Debug)]
pub struct MessageInteractionMetadata {
    /// The interaction's ID.
    pub id: Snowflake,
    /// The interaction's type.
    pub kind: InteractionType,
    /// The user that invoked the interaction.
    pub user: User,
    /// The integration owners that authorized the interaction.
    pub authorizing_integration_owners: HashMap<ApplicationIntegrationType, Snowflake>,
    /// The ID of the original response message, if this is a follow-up.
    pub original_response_message_id: Option<Snowflake>,
    /// The ID of the message containing the interactive components, if any.
    pub interacted_message_id: Option<Snowflake>,
    /// The metadata of the modal submission that triggered this interaction.
    pub modal_interaction: Option<Box<Self>>,
    /// The user targeted by a user context menu command, if any.
    pub target_user: Option<User>,
    /// The message targeted by a message context menu command, if any.
    pub target_message_id: Option<Snowflake>,
}

impl MessageInteractionMetadata {
    /// Creates metadata for an interaction.
    #[must_use]
    pub fn new(id: impl Into<Snowflake>, kind: InteractionType, user: User) -> Self {
        Self {
            id: id.into(),
            kind,
            user,
            authorizing_integration_owners: HashMap::new(),
            original_response_message_id: None,
            interacted_message_id: None,
            modal_interaction: None,
            target_user: None,
            target_message_id: None,
        }
    }

    /// Decodes message interaction metadata from a Discord object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "interaction_metadata")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Self::from_object_at(object, path)
    }

    pub(crate) fn from_object_at(object: &Object, path: &str) -> Result<Self, CastError> {
        let user_path = cast::field_path(path, "user");
        let user = User::from_object_at(cast::required_object(object, path, "user")?, &user_path)?;

        let mut authorizing_integration_owners = HashMap::new();
        if let Some(owners) = cast::optional_object(object, path, "authorizing_integration_owners")?
        {
            let owners_path = cast::field_path(path, "authorizing_integration_owners");
            for (kind, value) in owners.iter() {
                let Ok(kind) = kind.parse::<u8>() else {
                    continue;
                };
                let Some(kind) = ApplicationIntegrationType::from_value(kind) else {
                    continue;
                };
                let owner_id =
                    cast::snowflake(value, &cast::field_path(&owners_path, kind.as_str()))?;
                authorizing_integration_owners.insert(kind, owner_id);
            }
        }

        let modal_interaction = match cast::optional_object(object, path, "modal_interaction")? {
            Some(modal) => {
                let modal_path = cast::field_path(path, "modal_interaction");
                Some(Box::new(Self::from_object_at(modal, &modal_path)?))
            }
            None => None,
        };

        let target_user_path = cast::field_path(path, "target_user");
        let target_user = cast::optional_object(object, path, "target_user")?
            .map(|user| User::from_object_at(user, &target_user_path))
            .transpose()?;

        Ok(Self {
            id: cast::required_snowflake(object, path, "id")?,
            kind: cast::required_enum(
                object,
                path,
                "type",
                "an interaction type",
                InteractionType::from_value,
            )?,
            user,
            authorizing_integration_owners,
            original_response_message_id: cast::optional_snowflake(
                object,
                path,
                "original_response_message_id",
            )?,
            interacted_message_id: cast::optional_snowflake(object, path, "interacted_message_id")?,
            modal_interaction,
            target_user,
            target_message_id: cast::optional_snowflake(object, path, "target_message_id")?,
        })
    }

    /// Adds an integration owner.
    #[must_use]
    pub fn with_authorizing_integration_owner(
        mut self,
        kind: ApplicationIntegrationType,
        owner_id: impl Into<Snowflake>,
    ) -> Self {
        self.authorizing_integration_owners
            .insert(kind, owner_id.into());
        self
    }

    /// Sets the original response message ID.
    #[must_use]
    pub fn with_original_response_message_id(mut self, message_id: impl Into<Snowflake>) -> Self {
        self.original_response_message_id = Some(message_id.into());
        self
    }

    /// Sets the interacted message ID.
    #[must_use]
    pub fn with_interacted_message_id(mut self, message_id: impl Into<Snowflake>) -> Self {
        self.interacted_message_id = Some(message_id.into());
        self
    }

    /// Sets the triggering modal interaction metadata.
    #[must_use]
    pub fn with_modal_interaction(mut self, metadata: Self) -> Self {
        self.modal_interaction = Some(Box::new(metadata));
        self
    }

    /// Sets the targeted user.
    #[must_use]
    pub fn with_target_user(mut self, user: User) -> Self {
        self.target_user = Some(user);
        self
    }

    /// Sets the targeted message ID.
    #[must_use]
    pub fn with_target_message_id(mut self, message_id: impl Into<Snowflake>) -> Self {
        self.target_message_id = Some(message_id.into());
        self
    }

    /// Returns the metadata's creation time.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.id.created_at()
    }
}

impl PartialEq for MessageInteractionMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for MessageInteractionMetadata {}

impl Hash for MessageInteractionMetadata {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for MessageInteractionMetadata {
    fn id(&self) -> Snowflake {
        self.id
    }
}

/// The message payload of an interaction response.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InteractionMessageData {
    /// The response's text content, if any.
    pub content: Option<String>,
    /// Whether the response should be read as text-to-speech.
    pub tts: bool,
    /// The response's embeds.
    pub embeds: Vec<Embed>,
    /// The response's component rows.
    pub components: Vec<ActionRow>,
    /// The response's poll, if any.
    pub poll: Option<Poll>,
    /// The response's message flags.
    pub flags: MessageFlags,
}

impl InteractionMessageData {
    /// Creates an empty message response.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the response's content.
    #[must_use]
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Sets the text-to-speech state.
    #[must_use]
    pub fn with_tts(mut self, tts: bool) -> Self {
        self.tts = tts;
        self
    }

    /// Sets the response's embeds.
    #[must_use]
    pub fn with_embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.embeds = embeds;
        self
    }

    /// Sets the response's component rows.
    #[must_use]
    pub fn with_components(mut self, components: Vec<ActionRow>) -> Self {
        self.components = components;
        self
    }

    /// Sets the response's poll.
    #[must_use]
    pub fn with_poll(mut self, poll: Poll) -> Self {
        self.poll = Some(poll);
        self
    }

    /// Sets the response's message flags.
    #[must_use]
    pub fn with_flags(mut self, flags: MessageFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Sets whether the response is ephemeral.
    #[must_use]
    pub fn with_ephemeral(mut self, ephemeral: bool) -> Self {
        self.flags.set(MessageFlags::EPHEMERAL, ephemeral);
        self
    }

    /// Returns `true` if the response is ephemeral.
    #[must_use]
    pub const fn is_ephemeral(&self) -> bool {
        self.flags.contains(MessageFlags::EPHEMERAL)
    }
}

/// The modal payload of an interaction response.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ModalResponseData {
    /// The modal's custom ID.
    pub custom_id: String,
    /// The modal's title.
    pub title: String,
    /// The modal's components.
    pub components: Vec<ActionRow>,
}

impl ModalResponseData {
    /// Creates a modal with the given custom ID and title.
    #[must_use]
    pub fn new(custom_id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            custom_id: custom_id.into(),
            title: title.into(),
            components: Vec::new(),
        }
    }

    /// Sets the modal's components.
    #[must_use]
    pub fn with_components(mut self, components: Vec<ActionRow>) -> Self {
        self.components = components;
        self
    }
}

/// A choice returned by an autocomplete response.
#[derive(Clone, Debug, PartialEq)]
pub struct AutocompleteChoice {
    /// The choice's display name.
    pub name: String,
    /// The value sent to the bot.
    pub value: CommandOptionValue,
}

impl AutocompleteChoice {
    /// Creates an autocomplete choice.
    #[must_use]
    pub fn new(name: impl Into<String>, value: impl Into<CommandOptionValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

/// The autocomplete payload of an interaction response.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AutocompleteResponseData {
    /// The choices shown to the user.
    pub choices: Vec<AutocompleteChoice>,
}

impl AutocompleteResponseData {
    /// Creates an autocomplete response without choices.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the response's choices.
    #[must_use]
    pub fn with_choices(mut self, choices: Vec<AutocompleteChoice>) -> Self {
        self.choices = choices;
        self
    }

    /// Appends a choice.
    #[must_use]
    pub fn add_choice(mut self, choice: AutocompleteChoice) -> Self {
        self.choices.push(choice);
        self
    }
}

/// An activity instance launched by an interaction response.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct InteractionActivityInstance {
    /// The activity instance ID.
    pub id: String,
}

impl InteractionActivityInstance {
    /// Creates an activity instance.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

impl fmt::Display for InteractionActivityInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.id)
    }
}

/// The data sent with an interaction response.
#[derive(Clone, Debug, PartialEq)]
pub enum InteractionResponseData {
    /// A message response.
    Message(InteractionMessageData),
    /// A modal response.
    Modal(ModalResponseData),
    /// An autocomplete response.
    Autocomplete(AutocompleteResponseData),
    /// A launched activity.
    LaunchActivity(InteractionActivityInstance),
}

impl InteractionResponseData {
    /// Returns the message payload, if present.
    #[must_use]
    pub const fn message(&self) -> Option<&InteractionMessageData> {
        match self {
            Self::Message(data) => Some(data),
            _ => None,
        }
    }

    /// Returns the modal payload, if present.
    #[must_use]
    pub const fn modal(&self) -> Option<&ModalResponseData> {
        match self {
            Self::Modal(data) => Some(data),
            _ => None,
        }
    }

    /// Returns the autocomplete payload, if present.
    #[must_use]
    pub const fn autocomplete(&self) -> Option<&AutocompleteResponseData> {
        match self {
            Self::Autocomplete(data) => Some(data),
            _ => None,
        }
    }

    /// Returns the activity instance, if present.
    #[must_use]
    pub const fn activity(&self) -> Option<&InteractionActivityInstance> {
        match self {
            Self::LaunchActivity(activity) => Some(activity),
            _ => None,
        }
    }
}

impl From<InteractionMessageData> for InteractionResponseData {
    fn from(data: InteractionMessageData) -> Self {
        Self::Message(data)
    }
}

impl From<ModalResponseData> for InteractionResponseData {
    fn from(data: ModalResponseData) -> Self {
        Self::Modal(data)
    }
}

impl From<AutocompleteResponseData> for InteractionResponseData {
    fn from(data: AutocompleteResponseData) -> Self {
        Self::Autocomplete(data)
    }
}

impl From<InteractionActivityInstance> for InteractionResponseData {
    fn from(activity: InteractionActivityInstance) -> Self {
        Self::LaunchActivity(activity)
    }
}

/// The response to an interaction.
#[derive(Clone, Debug, PartialEq)]
pub struct InteractionResponse {
    /// The response type.
    pub kind: InteractionResponseType,
    /// The response's data, if the type carries any.
    pub data: Option<InteractionResponseData>,
}

impl InteractionResponse {
    /// Creates a response of the given type.
    #[must_use]
    pub const fn new(kind: InteractionResponseType) -> Self {
        Self { kind, data: None }
    }

    /// Creates a pong response.
    #[must_use]
    pub const fn pong() -> Self {
        Self::new(InteractionResponseType::Pong)
    }

    /// Creates an immediate channel message response.
    #[must_use]
    pub fn channel_message(data: InteractionMessageData) -> Self {
        Self::new(InteractionResponseType::ChannelMessage)
            .with_data(InteractionResponseData::Message(data))
    }

    /// Creates a deferred channel message response.
    #[must_use]
    pub const fn deferred_channel_message() -> Self {
        Self::new(InteractionResponseType::DeferredChannelMessage)
    }

    /// Creates a deferred message update response.
    #[must_use]
    pub const fn deferred_message_update() -> Self {
        Self::new(InteractionResponseType::DeferredMessageUpdate)
    }

    /// Creates an immediate message update response.
    #[must_use]
    pub fn message_update(data: InteractionMessageData) -> Self {
        Self::new(InteractionResponseType::MessageUpdate)
            .with_data(InteractionResponseData::Message(data))
    }

    /// Creates an autocomplete response.
    #[must_use]
    pub fn autocomplete(data: AutocompleteResponseData) -> Self {
        Self::new(InteractionResponseType::AutocompleteResult)
            .with_data(InteractionResponseData::Autocomplete(data))
    }

    /// Creates a modal response.
    #[must_use]
    pub fn modal(data: ModalResponseData) -> Self {
        Self::new(InteractionResponseType::Modal).with_data(InteractionResponseData::Modal(data))
    }

    /// Creates an activity launch response.
    #[must_use]
    pub fn launch_activity(activity_instance_id: impl Into<String>) -> Self {
        Self::new(InteractionResponseType::LaunchActivity).with_data(
            InteractionResponseData::LaunchActivity(InteractionActivityInstance::new(
                activity_instance_id,
            )),
        )
    }

    /// Sets the response's data.
    #[must_use]
    pub fn with_data(mut self, data: impl Into<InteractionResponseData>) -> Self {
        self.data = Some(data.into());
        self
    }

    /// Returns `true` if the response defers the interaction.
    #[must_use]
    pub const fn is_deferred(&self) -> bool {
        self.kind.is_deferred()
    }

    /// Returns `true` if the response updates the interaction's message.
    #[must_use]
    pub const fn is_message_update(&self) -> bool {
        self.kind.is_message_update()
    }

    /// Returns the message payload, if present.
    #[must_use]
    pub fn message_data(&self) -> Option<&InteractionMessageData> {
        self.data
            .as_ref()
            .and_then(InteractionResponseData::message)
    }

    /// Returns the modal payload, if present.
    #[must_use]
    pub fn modal_data(&self) -> Option<&ModalResponseData> {
        self.data.as_ref().and_then(InteractionResponseData::modal)
    }

    /// Returns the autocomplete payload, if present.
    #[must_use]
    pub fn autocomplete_data(&self) -> Option<&AutocompleteResponseData> {
        self.data
            .as_ref()
            .and_then(InteractionResponseData::autocomplete)
    }

    /// Returns `true` if the response is an ephemeral message.
    #[must_use]
    pub fn is_ephemeral(&self) -> bool {
        self.message_data()
            .is_some_and(InteractionMessageData::is_ephemeral)
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use abc::{HasId, Snowflake};
    use data::json;
    use enums::{
        ApplicationCommandOptionType, ApplicationCommandType, ApplicationIntegrationType,
        ComponentType, InteractionContextType, InteractionResponseType, InteractionType,
        MessageFlags, Permissions,
    };

    use super::{
        ApplicationCommandData, ApplicationCommandOption, AutocompleteChoice,
        AutocompleteResponseData, CommandOptionValue, Interaction, InteractionActivityInstance,
        InteractionData, InteractionMessage, InteractionMessageData, InteractionResponse,
        InteractionResponseData, MessageComponentData, MessageInteraction,
        MessageInteractionMetadata, ModalResponseData, ModalSubmitComponent, ModalSubmitData,
        ResolvedData,
    };
    use crate::{
        ActionRow, Button, Embed, GuildChannel, Member, Message, PartialEmoji, Poll, Role, User,
    };

    #[test]
    fn command_option_values_and_lookup() {
        let option =
            ApplicationCommandOption::new("group", ApplicationCommandOptionType::SubcommandGroup)
                .add_option(
                    ApplicationCommandOption::new("sub", ApplicationCommandOptionType::Subcommand)
                        .add_option(
                            ApplicationCommandOption::new(
                                "name",
                                ApplicationCommandOptionType::String,
                            )
                            .with_value("winter"),
                        )
                        .add_option(
                            ApplicationCommandOption::new(
                                "count",
                                ApplicationCommandOptionType::Integer,
                            )
                            .with_value(3_i64),
                        ),
                )
                .with_focused(true);

        assert_eq!(option.name, "group");
        assert_eq!(option.kind, ApplicationCommandOptionType::SubcommandGroup);
        assert!(option.is_subcommand());
        assert!(option.focused);
        assert_eq!(option.value(), None);
        assert_eq!(
            option.option("name").and_then(|option| option.value()),
            Some(&CommandOptionValue::String("winter".to_owned()))
        );
        assert_eq!(
            option
                .option("count")
                .and_then(|option| option.value())
                .and_then(CommandOptionValue::as_i64),
            Some(3)
        );
        assert!(option.option("missing").is_none());

        let string = CommandOptionValue::String("text".to_owned());
        assert_eq!(string.as_str(), Some("text"));
        assert_eq!(string.as_i64(), None);
        assert_eq!(string.to_string(), "text");
        assert_eq!(CommandOptionValue::from("str").as_str(), Some("str"));
        assert_eq!(
            CommandOptionValue::from("owned".to_owned()).as_str(),
            Some("owned")
        );
        assert_eq!(CommandOptionValue::from(7_i64).as_i64(), Some(7));
        assert_eq!(CommandOptionValue::from(true).as_bool(), Some(true));
        assert_eq!(
            CommandOptionValue::from(Snowflake::new(8)).as_snowflake(),
            Some(Snowflake::new(8))
        );
        assert_eq!(CommandOptionValue::from(1.5).as_f64(), Some(1.5));
        assert_eq!(CommandOptionValue::from(2_i64).as_f64(), Some(2.0));
        assert_eq!(CommandOptionValue::from(false).to_string(), "false");
        assert_eq!(CommandOptionValue::from(Snowflake::new(9)).to_string(), "9");
        assert_eq!(CommandOptionValue::from(2.5).to_string(), "2.5");
    }

    #[test]
    fn resolved_data_helpers() {
        let user = User::new(1_u64, "winter");
        let member = Member::new(user.clone(), 2_u64);
        let role = Role {
            id: Snowflake::new(3),
            name: "role".to_owned(),
            ..Role::default()
        };
        let channel = GuildChannel {
            id: Snowflake::new(4),
            name: "general".to_owned(),
            ..GuildChannel::default()
        };
        let message = Message::new(5_u64, 4_u64, user.clone());
        let attachment = crate::Attachment::new(6_u64, "file.txt", 1, "url", "proxy");

        let resolved = ResolvedData::new()
            .with_users(vec![user.clone()])
            .with_members(vec![member.clone()])
            .with_roles(vec![role.clone()])
            .with_channels(vec![channel.clone()])
            .with_messages(vec![message.clone()])
            .with_attachments(vec![attachment.clone()]);

        assert!(!resolved.is_empty());
        assert_eq!(resolved.user(1_u64), Some(&user));
        assert_eq!(resolved.member(1_u64), Some(&member));
        assert_eq!(resolved.role(3_u64), Some(&role));
        assert_eq!(resolved.channel(4_u64), Some(&channel));
        assert_eq!(resolved.message(5_u64), Some(&message));
        assert_eq!(resolved.attachment(6_u64), Some(&attachment));
        assert!(resolved.user(99_u64).is_none());
        assert!(ResolvedData::new().is_empty());
    }

    #[test]
    fn application_command_data_and_options() {
        let resolved = ResolvedData::new().with_users(vec![User::new(2_u64, "target")]);
        let data = ApplicationCommandData::new(1_u64, "greet", ApplicationCommandType::ChatInput)
            .with_guild_id(3_u64)
            .with_options(vec![
                ApplicationCommandOption::new("name", ApplicationCommandOptionType::String)
                    .with_value("winter"),
                ApplicationCommandOption::new("nested", ApplicationCommandOptionType::Subcommand)
                    .add_option(
                        ApplicationCommandOption::new(
                            "deep",
                            ApplicationCommandOptionType::Boolean,
                        )
                        .with_value(true),
                    ),
            ])
            .with_resolved(resolved.clone())
            .with_target_id(4_u64);

        assert_eq!(data.id, Snowflake::new(1));
        assert_eq!(data.name, "greet");
        assert_eq!(data.kind, ApplicationCommandType::ChatInput);
        assert_eq!(data.guild_id, Some(Snowflake::new(3)));
        assert_eq!(data.target_id, Some(Snowflake::new(4)));
        assert_eq!(data.resolved, Some(resolved));
        assert_eq!(
            data.option("name")
                .and_then(|option| option.value())
                .and_then(CommandOptionValue::as_str),
            Some("winter")
        );
        assert!(data.option("deep").is_none());
        assert!(data.find_option("deep").is_some());

        let interaction_data = InteractionData::ApplicationCommand(Box::new(data));
        assert_eq!(interaction_data.command_name(), Some("greet"));
        assert_eq!(interaction_data.custom_id(), None);
        assert!(interaction_data.application_command().is_some());
        assert!(interaction_data.message_component().is_none());
        assert!(interaction_data.modal_submit().is_none());
        assert!(interaction_data.resolved().is_some());
    }

    #[test]
    fn component_and_modal_data() {
        let component = MessageComponentData::new("choose", ComponentType::StringSelect)
            .with_values(vec!["one".to_owned(), "two".to_owned()])
            .with_resolved(ResolvedData::new().with_users(vec![User::new(1_u64, "winter")]));

        assert_eq!(component.custom_id, "choose");
        assert_eq!(component.value(), Some("one"));
        assert!(component.is_select());
        assert!(!component.is_button());

        let button = MessageComponentData::new("confirm", ComponentType::Button);
        assert!(button.is_button());
        assert!(!button.is_select());
        assert_eq!(button.value(), None);

        let modal = ModalSubmitData::new("form")
            .with_components(vec![ModalSubmitComponent::ActionRow {
                components: vec![
                    ModalSubmitComponent::TextInput {
                        custom_id: "name".to_owned(),
                        value: "winter".to_owned(),
                    },
                    ModalSubmitComponent::Select {
                        custom_id: "colour".to_owned(),
                        kind: ComponentType::StringSelect,
                        values: vec!["blue".to_owned()],
                    },
                ],
            }])
            .add_component(ModalSubmitComponent::Label {
                component: Box::new(ModalSubmitComponent::Checkbox {
                    custom_id: "agree".to_owned(),
                    id: 9,
                    value: true,
                }),
            });

        assert_eq!(modal.custom_id, "form");
        assert_eq!(
            modal
                .component("name")
                .and_then(ModalSubmitComponent::text_value),
            Some("winter")
        );
        assert_eq!(
            modal
                .component("colour")
                .and_then(ModalSubmitComponent::values),
            Some(["blue".to_owned()].as_slice())
        );
        assert_eq!(
            modal
                .component("agree")
                .and_then(ModalSubmitComponent::custom_id),
            Some("agree")
        );
        assert!(modal.component("missing").is_none());
        assert_eq!(
            ModalSubmitComponent::TextDisplay {
                content: "hello".to_owned()
            }
            .custom_id(),
            None
        );
        assert_eq!(
            ModalSubmitComponent::FileUpload {
                custom_id: "files".to_owned(),
                values: vec!["1".to_owned()]
            }
            .values(),
            Some(["1".to_owned()].as_slice())
        );

        let data = InteractionData::MessageComponent(component);
        assert_eq!(data.command_name(), None);
        assert_eq!(data.custom_id(), Some("choose"));
        assert!(data.application_command().is_none());

        let modal_data = InteractionData::ModalSubmit(modal);
        assert_eq!(modal_data.custom_id(), Some("form"));
        assert!(modal_data.modal_submit().is_some());
    }

    #[test]
    fn interaction_builders_and_helpers() {
        let user = User::new(1_u64, "winter").with_global_name("Winter");
        let data = InteractionData::MessageComponent(MessageComponentData::new(
            "confirm",
            ComponentType::Button,
        ));
        let interaction =
            Interaction::new(10_u64, InteractionType::MessageComponent, 20_u64, "token")
                .with_data(data)
                .with_guild_id(30_u64)
                .with_channel_id(40_u64)
                .with_user(user)
                .with_version(1)
                .with_locale("en-US")
                .with_guild_locale("en-GB")
                .with_app_permissions(Permissions::VIEW_CHANNEL)
                .with_context(InteractionContextType::Guild)
                .with_attachment_size_limit(1024)
                .with_entitlement_sku_ids(vec![Snowflake::new(50)])
                .with_authorizing_integration_owner(
                    ApplicationIntegrationType::GuildInstall,
                    60_u64,
                );

        assert_eq!(interaction.id, Snowflake::new(10));
        assert_eq!(interaction.id(), Snowflake::new(10));
        assert_eq!(interaction.application_id, Snowflake::new(20));
        assert_eq!(interaction.token, "token");
        assert_eq!(interaction.version, 1);
        assert_eq!(interaction.guild_id, Some(Snowflake::new(30)));
        assert_eq!(interaction.channel_id, Some(Snowflake::new(40)));
        assert_eq!(
            interaction.user().map(abc::User::display_name),
            Some("Winter")
        );
        assert_eq!(interaction.custom_id(), Some("confirm"));
        assert_eq!(interaction.command_name(), None);
        assert!(interaction.resolved().is_none());
        assert!(interaction.is_component());
        assert!(!interaction.is_application_command());
        assert!(!interaction.is_autocomplete());
        assert!(!interaction.is_modal_submit());
        assert!(!interaction.is_ping());
        assert_eq!(interaction.locale.as_deref(), Some("en-US"));
        assert_eq!(interaction.guild_locale.as_deref(), Some("en-GB"));
        assert_eq!(interaction.app_permissions, Some(Permissions::VIEW_CHANNEL));
        assert_eq!(interaction.context, Some(InteractionContextType::Guild));
        assert_eq!(interaction.attachment_size_limit, Some(1024));
        assert_eq!(interaction.entitlement_sku_ids, vec![Snowflake::new(50)]);
        assert_eq!(
            interaction
                .authorizing_integration_owners
                .get(&ApplicationIntegrationType::GuildInstall),
            Some(&Snowflake::new(60))
        );
        assert!(interaction.created_at() > SystemTime::UNIX_EPOCH);

        let same = Interaction::new(10_u64, InteractionType::Ping, 99_u64, "other");
        assert_eq!(interaction, same);
        assert_eq!(interaction, same.clone());

        let member = Member::new(User::new(2_u64, "member"), 30_u64);
        let with_member =
            Interaction::new(11_u64, InteractionType::ApplicationCommand, 20_u64, "t")
                .with_member(member);
        assert!(with_member.user().is_some_and(|user| user.is_member()));
        assert!(with_member.is_application_command());
        assert!(!with_member.is_ping());

        let ping = Interaction::new(12_u64, InteractionType::Ping, 20_u64, "t");
        assert!(ping.is_ping());
        assert!(ping.data().is_none());

        let autocomplete = Interaction::new(13_u64, InteractionType::Autocomplete, 20_u64, "t")
            .with_data(InteractionData::ApplicationCommand(Box::new(
                ApplicationCommandData::new(1_u64, "cmd", ApplicationCommandType::ChatInput),
            )));
        assert!(autocomplete.is_autocomplete());
        assert_eq!(autocomplete.command_name(), Some("cmd"));

        let modal = Interaction::new(14_u64, InteractionType::ModalSubmit, 20_u64, "t");
        assert!(modal.is_modal_submit());
    }

    #[test]
    fn message_models_and_equality() {
        let user = User::new(1_u64, "winter");
        let message_interaction = MessageInteraction::new(
            10_u64,
            InteractionType::ApplicationCommand,
            "greet",
            user.clone(),
        );

        assert_eq!(message_interaction.id(), Snowflake::new(10));
        assert_eq!(message_interaction.name, "greet");
        assert_eq!(message_interaction.user.id(), Snowflake::new(1));
        assert!(message_interaction.created_at() > SystemTime::UNIX_EPOCH);

        let same = MessageInteraction::new(
            10_u64,
            InteractionType::MessageComponent,
            "other",
            user.clone(),
        );
        assert_eq!(message_interaction, same);

        let modal_metadata =
            MessageInteractionMetadata::new(20_u64, InteractionType::ModalSubmit, user.clone())
                .with_original_response_message_id(30_u64)
                .with_target_message_id(31_u64);
        let metadata = MessageInteractionMetadata::new(
            19_u64,
            InteractionType::ApplicationCommand,
            user.clone(),
        )
        .with_authorizing_integration_owner(ApplicationIntegrationType::UserInstall, 2_u64)
        .with_original_response_message_id(30_u64)
        .with_interacted_message_id(40_u64)
        .with_modal_interaction(modal_metadata)
        .with_target_user(User::new(3_u64, "target"));

        assert_eq!(metadata.id(), Snowflake::new(19));
        assert_eq!(
            metadata.original_response_message_id,
            Some(Snowflake::new(30))
        );
        assert_eq!(metadata.interacted_message_id, Some(Snowflake::new(40)));
        assert_eq!(
            metadata
                .authorizing_integration_owners
                .get(&ApplicationIntegrationType::UserInstall),
            Some(&Snowflake::new(2))
        );
        assert_eq!(
            metadata.modal_interaction.as_ref().map(|modal| modal.id),
            Some(Snowflake::new(20))
        );
        assert_eq!(
            metadata.target_user.as_ref().map(|user| user.id),
            Some(Snowflake::new(3))
        );
        assert_eq!(metadata.target_message_id, None);
        assert!(metadata.created_at() > SystemTime::UNIX_EPOCH);

        let same = MessageInteractionMetadata::new(
            19_u64,
            InteractionType::MessageComponent,
            User::new(99_u64, "other"),
        );
        assert_eq!(metadata, same);

        let message = Message::new(50_u64, 60_u64, user.clone());
        let interaction_message = InteractionMessage::new(message.clone());
        assert_eq!(interaction_message.id(), Snowflake::new(50));
        assert_eq!(interaction_message.as_message(), &message);
        assert_eq!(*interaction_message, message);
        assert_eq!(
            InteractionMessage::from(message.clone()).into_message(),
            message
        );
    }

    #[test]
    fn interaction_message_data_and_responses() {
        let data = InteractionMessageData::new()
            .with_content("hello")
            .with_tts(true)
            .with_embeds(vec![Embed::new()])
            .with_components(vec![ActionRow::new(vec![
                Button::new(enums::ButtonStyle::Primary)
                    .with_custom_id("confirm")
                    .into(),
            ])])
            .with_poll(Poll::new("Favourite?"))
            .with_flags(MessageFlags::SUPPRESS_NOTIFICATIONS)
            .with_ephemeral(true);

        assert_eq!(data.content.as_deref(), Some("hello"));
        assert!(data.tts);
        assert_eq!(data.embeds.len(), 1);
        assert_eq!(data.components.len(), 1);
        assert!(data.poll.is_some());
        assert!(data.is_ephemeral());
        assert!(data.flags.contains(MessageFlags::SUPPRESS_NOTIFICATIONS));

        let response = InteractionResponse::channel_message(data.clone());
        assert_eq!(response.kind, InteractionResponseType::ChannelMessage);
        assert_eq!(response.message_data(), Some(&data));
        assert!(!response.is_deferred());
        assert!(!response.is_message_update());
        assert!(response.is_ephemeral());

        let deferred = InteractionResponse::deferred_channel_message();
        assert!(deferred.is_deferred());
        assert!(!deferred.is_ephemeral());
        assert!(deferred.message_data().is_none());

        let update = InteractionResponse::message_update(InteractionMessageData::new());
        assert!(update.is_message_update());
        assert!(InteractionResponse::deferred_message_update().is_deferred());

        let pong = InteractionResponse::pong();
        assert_eq!(pong.kind, InteractionResponseType::Pong);
        assert!(pong.data.is_none());

        let autocomplete = AutocompleteResponseData::new()
            .add_choice(AutocompleteChoice::new("One", 1_i64))
            .with_choices(vec![AutocompleteChoice::new("Two", "two")]);
        let response = InteractionResponse::autocomplete(autocomplete);
        assert_eq!(response.kind, InteractionResponseType::AutocompleteResult);
        assert_eq!(
            response.autocomplete_data().map(|data| data.choices.len()),
            Some(1)
        );
        assert_eq!(
            response
                .autocomplete_data()
                .and_then(|data| data.choices.first())
                .map(|choice| choice.value.as_str()),
            Some(Some("two"))
        );

        let modal = ModalResponseData::new("form", "Form");
        let response = InteractionResponse::modal(modal.clone());
        assert_eq!(response.kind, InteractionResponseType::Modal);
        assert_eq!(response.modal_data(), Some(&modal));

        let activity = InteractionActivityInstance::new("activity");
        let response = InteractionResponse::launch_activity("activity");
        assert_eq!(response.kind, InteractionResponseType::LaunchActivity);
        assert_eq!(
            response
                .data
                .as_ref()
                .and_then(InteractionResponseData::activity),
            Some(&activity)
        );
        assert_eq!(activity.to_string(), "activity");
        assert_eq!(
            InteractionResponseData::from(activity.clone()),
            InteractionResponseData::LaunchActivity(activity)
        );

        assert_eq!(
            InteractionResponse::new(InteractionResponseType::Modal)
                .with_data(InteractionResponseData::Modal(modal)),
            InteractionResponse::modal(ModalResponseData::new("form", "Form"))
        );
    }

    #[test]
    fn message_component_data_resolved_and_emoji() {
        let emoji = PartialEmoji::unicode("✅");
        assert_eq!(emoji.to_string(), "✅");

        let data = MessageComponentData::new("select", ComponentType::ChannelSelect).with_resolved(
            ResolvedData::new().with_channels(vec![GuildChannel {
                id: Snowflake::new(1),
                name: "general".to_owned(),
                ..GuildChannel::default()
            }]),
        );
        assert!(data.is_select());
        assert_eq!(
            data.resolved
                .as_ref()
                .and_then(|resolved| resolved.channel(1_u64))
                .map(|channel| channel.name.as_str()),
            Some("general")
        );
    }

    #[test]
    fn decodes_message_interactions_and_metadata() {
        let interaction = MessageInteraction::from_value(
            &json::from_str(
                r#"{
                    "id": "50",
                    "type": 3,
                    "name": "confirm",
                    "user": {"id": "1", "username": "winter"}
                }"#,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(interaction.id, Snowflake::new(50));
        assert_eq!(interaction.kind, InteractionType::MessageComponent);
        assert_eq!(interaction.name, "confirm");
        assert_eq!(interaction.user.id(), Snowflake::new(1));

        let metadata = MessageInteractionMetadata::from_value(
            &json::from_str(
                r#"{
                    "id": "60",
                    "type": 5,
                    "user": {"id": "1", "username": "winter"},
                    "authorizing_integration_owners": {"0": "100", "1": "1", "9": "7"},
                    "original_response_message_id": "61",
                    "interacted_message_id": "62",
                    "target_user": {"id": "2", "username": "target"},
                    "target_message_id": "63",
                    "modal_interaction": {
                        "id": "59",
                        "type": 3,
                        "user": {"id": "1", "username": "winter"}
                    }
                }"#,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(metadata.kind, InteractionType::ModalSubmit);
        assert_eq!(metadata.authorizing_integration_owners.len(), 2);
        assert_eq!(
            metadata
                .authorizing_integration_owners
                .get(&ApplicationIntegrationType::GuildInstall),
            Some(&Snowflake::new(100))
        );
        assert_eq!(
            metadata
                .authorizing_integration_owners
                .get(&ApplicationIntegrationType::UserInstall),
            Some(&Snowflake::new(1))
        );
        assert_eq!(
            metadata.original_response_message_id,
            Some(Snowflake::new(61))
        );
        assert_eq!(metadata.interacted_message_id, Some(Snowflake::new(62)));
        assert_eq!(
            metadata.target_user.as_ref().map(|user| user.id),
            Some(Snowflake::new(2))
        );
        assert_eq!(metadata.target_message_id, Some(Snowflake::new(63)));
        let modal = metadata.modal_interaction.as_ref().unwrap();
        assert_eq!(modal.id, Snowflake::new(59));
        assert_eq!(modal.kind, InteractionType::MessageComponent);
        assert!(modal.modal_interaction.is_none());

        let missing_user = MessageInteraction::from_value(
            &json::from_str(r#"{"id":"50","type":2,"name":"command"}"#).unwrap(),
        )
        .unwrap_err();
        assert_eq!(missing_user.to_string(), "missing field `interaction.user`");

        let bad_owner = MessageInteractionMetadata::from_value(
            &json::from_str(
                r#"{"id":"60","type":2,"user":{"id":"1","username":"winter"},"authorizing_integration_owners":{"0":"nope"}}"#,
            )
            .unwrap(),
        )
        .unwrap_err();
        assert_eq!(
            bad_owner.to_string(),
            "field `interaction_metadata.authorizing_integration_owners.guild_install` must be a snowflake"
        );
    }
}
