//! Interaction enumerations.

use std::fmt;

/// The type of an interaction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum InteractionType {
    /// A ping sent by Discord to verify an endpoint.
    #[default]
    Ping = 1,
    /// An application command invocation.
    ApplicationCommand = 2,
    /// A message component interaction.
    MessageComponent = 3,
    /// An autocomplete interaction.
    Autocomplete = 4,
    /// A modal submission.
    ModalSubmit = 5,
}

impl InteractionType {
    /// Alias of [`InteractionType::MessageComponent`].
    pub const COMPONENT: Self = Self::MessageComponent;

    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Ping),
            2 => Some(Self::ApplicationCommand),
            3 => Some(Self::MessageComponent),
            4 => Some(Self::Autocomplete),
            5 => Some(Self::ModalSubmit),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ping => "ping",
            Self::ApplicationCommand => "application_command",
            Self::MessageComponent => "message_component",
            Self::Autocomplete => "autocomplete",
            Self::ModalSubmit => "modal_submit",
        }
    }

    /// Returns `true` if this interaction carries application command data.
    #[must_use]
    pub const fn is_application_command(self) -> bool {
        matches!(self, Self::ApplicationCommand | Self::Autocomplete)
    }
}

impl fmt::Display for InteractionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The type of response sent when acknowledging an interaction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum InteractionResponseType {
    /// Acknowledges a ping.
    #[default]
    Pong = 1,
    /// Sends a message in the interaction's channel.
    ChannelMessage = 4,
    /// Acknowledges the interaction and sends a message later.
    DeferredChannelMessage = 5,
    /// Acknowledges a component interaction and updates its message later.
    DeferredMessageUpdate = 6,
    /// Updates the message that triggered the interaction.
    MessageUpdate = 7,
    /// Returns autocomplete choices.
    AutocompleteResult = 8,
    /// Opens a modal.
    Modal = 9,
    /// Launches an activity.
    LaunchActivity = 12,
}

impl InteractionResponseType {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Pong),
            4 => Some(Self::ChannelMessage),
            5 => Some(Self::DeferredChannelMessage),
            6 => Some(Self::DeferredMessageUpdate),
            7 => Some(Self::MessageUpdate),
            8 => Some(Self::AutocompleteResult),
            9 => Some(Self::Modal),
            12 => Some(Self::LaunchActivity),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pong => "pong",
            Self::ChannelMessage => "channel_message",
            Self::DeferredChannelMessage => "deferred_channel_message",
            Self::DeferredMessageUpdate => "deferred_message_update",
            Self::MessageUpdate => "message_update",
            Self::AutocompleteResult => "autocomplete_result",
            Self::Modal => "modal",
            Self::LaunchActivity => "launch_activity",
        }
    }

    /// Returns `true` if this response acknowledges the interaction without
    /// sending a message immediately.
    #[must_use]
    pub const fn is_deferred(self) -> bool {
        matches!(
            self,
            Self::DeferredChannelMessage | Self::DeferredMessageUpdate
        )
    }

    /// Returns `true` if this response updates the interaction's message.
    #[must_use]
    pub const fn is_message_update(self) -> bool {
        matches!(self, Self::DeferredMessageUpdate | Self::MessageUpdate)
    }
}

impl fmt::Display for InteractionResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The context in which an application command can be used.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum InteractionContextType {
    /// The command can be used in guilds.
    #[default]
    Guild = 0,
    /// The command can be used in the bot's direct messages.
    BotDm = 1,
    /// The command can be used in direct messages and group direct messages.
    PrivateChannel = 2,
}

impl InteractionContextType {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Guild),
            1 => Some(Self::BotDm),
            2 => Some(Self::PrivateChannel),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Guild => "guild",
            Self::BotDm => "bot_dm",
            Self::PrivateChannel => "private_channel",
        }
    }

    /// Returns `true` if this context is outside a guild.
    #[must_use]
    pub const fn is_private(self) -> bool {
        matches!(self, Self::BotDm | Self::PrivateChannel)
    }
}

impl fmt::Display for InteractionContextType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The type of an application command.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ApplicationCommandType {
    /// A slash command.
    #[default]
    ChatInput = 1,
    /// A user context menu command.
    User = 2,
    /// A message context menu command.
    Message = 3,
}

impl ApplicationCommandType {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::ChatInput),
            2 => Some(Self::User),
            3 => Some(Self::Message),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChatInput => "chat_input",
            Self::User => "user",
            Self::Message => "message",
        }
    }

    /// Returns `true` if this is a context menu command.
    #[must_use]
    pub const fn is_context_menu(self) -> bool {
        matches!(self, Self::User | Self::Message)
    }
}

impl fmt::Display for ApplicationCommandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The type of an application command option.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ApplicationCommandOptionType {
    /// A subcommand.
    #[default]
    Subcommand = 1,
    /// A group of subcommands.
    SubcommandGroup = 2,
    /// A string value.
    String = 3,
    /// An integer value.
    Integer = 4,
    /// A boolean value.
    Boolean = 5,
    /// A user or member.
    User = 6,
    /// A channel.
    Channel = 7,
    /// A role.
    Role = 8,
    /// A user, member or role.
    Mentionable = 9,
    /// A floating-point value.
    Number = 10,
    /// An attachment.
    Attachment = 11,
}

impl ApplicationCommandOptionType {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Subcommand),
            2 => Some(Self::SubcommandGroup),
            3 => Some(Self::String),
            4 => Some(Self::Integer),
            5 => Some(Self::Boolean),
            6 => Some(Self::User),
            7 => Some(Self::Channel),
            8 => Some(Self::Role),
            9 => Some(Self::Mentionable),
            10 => Some(Self::Number),
            11 => Some(Self::Attachment),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Subcommand => "subcommand",
            Self::SubcommandGroup => "subcommand_group",
            Self::String => "string",
            Self::Integer => "integer",
            Self::Boolean => "boolean",
            Self::User => "user",
            Self::Channel => "channel",
            Self::Role => "role",
            Self::Mentionable => "mentionable",
            Self::Number => "number",
            Self::Attachment => "attachment",
        }
    }

    /// Returns `true` if this option contains other options.
    #[must_use]
    pub const fn is_subcommand(self) -> bool {
        matches!(self, Self::Subcommand | Self::SubcommandGroup)
    }

    /// Returns `true` if this option carries a value.
    #[must_use]
    pub const fn is_value(self) -> bool {
        !self.is_subcommand()
    }
}

impl fmt::Display for ApplicationCommandOptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The context in which an application is installed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ApplicationIntegrationType {
    /// The application is installed in a guild.
    #[default]
    GuildInstall = 0,
    /// The application is installed on a user account.
    UserInstall = 1,
}

impl ApplicationIntegrationType {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::GuildInstall),
            1 => Some(Self::UserInstall),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GuildInstall => "guild_install",
            Self::UserInstall => "user_install",
        }
    }
}

impl fmt::Display for ApplicationIntegrationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ApplicationCommandOptionType, ApplicationCommandType, ApplicationIntegrationType,
        InteractionContextType, InteractionResponseType, InteractionType,
    };

    #[test]
    fn interaction_type_values() {
        let values = [
            (1, InteractionType::Ping),
            (2, InteractionType::ApplicationCommand),
            (3, InteractionType::MessageComponent),
            (4, InteractionType::Autocomplete),
            (5, InteractionType::ModalSubmit),
        ];

        for (value, kind) in values {
            assert_eq!(InteractionType::from_value(value), Some(kind));
            assert_eq!(kind.value(), value);
        }

        assert_eq!(InteractionType::from_value(0), None);
        assert_eq!(InteractionType::from_value(6), None);
        assert_eq!(InteractionType::default(), InteractionType::Ping);
        assert_eq!(
            InteractionType::COMPONENT,
            InteractionType::MessageComponent
        );
        assert!(InteractionType::ApplicationCommand.is_application_command());
        assert!(InteractionType::Autocomplete.is_application_command());
        assert!(!InteractionType::ModalSubmit.is_application_command());
        assert_eq!(
            InteractionType::MessageComponent.to_string(),
            "message_component"
        );
    }

    #[test]
    fn interaction_response_type_values() {
        let values = [
            (1, InteractionResponseType::Pong),
            (4, InteractionResponseType::ChannelMessage),
            (5, InteractionResponseType::DeferredChannelMessage),
            (6, InteractionResponseType::DeferredMessageUpdate),
            (7, InteractionResponseType::MessageUpdate),
            (8, InteractionResponseType::AutocompleteResult),
            (9, InteractionResponseType::Modal),
            (12, InteractionResponseType::LaunchActivity),
        ];

        for (value, kind) in values {
            assert_eq!(InteractionResponseType::from_value(value), Some(kind));
            assert_eq!(kind.value(), value);
        }

        assert_eq!(InteractionResponseType::from_value(2), None);
        assert_eq!(InteractionResponseType::from_value(3), None);
        assert_eq!(InteractionResponseType::from_value(10), None);
        assert_eq!(InteractionResponseType::from_value(11), None);
        assert_eq!(
            InteractionResponseType::default(),
            InteractionResponseType::Pong
        );
        assert!(InteractionResponseType::DeferredChannelMessage.is_deferred());
        assert!(InteractionResponseType::DeferredMessageUpdate.is_message_update());
        assert!(!InteractionResponseType::MessageUpdate.is_deferred());
        assert_eq!(
            InteractionResponseType::AutocompleteResult.to_string(),
            "autocomplete_result"
        );
    }

    #[test]
    fn context_and_command_types() {
        let contexts = [
            (0, InteractionContextType::Guild),
            (1, InteractionContextType::BotDm),
            (2, InteractionContextType::PrivateChannel),
        ];

        for (value, context) in contexts {
            assert_eq!(InteractionContextType::from_value(value), Some(context));
            assert_eq!(context.value(), value);
        }

        assert_eq!(InteractionContextType::from_value(3), None);
        assert_eq!(
            InteractionContextType::default(),
            InteractionContextType::Guild
        );
        assert!(!InteractionContextType::Guild.is_private());
        assert!(InteractionContextType::BotDm.is_private());
        assert_eq!(InteractionContextType::BotDm.to_string(), "bot_dm");

        let commands = [
            (1, ApplicationCommandType::ChatInput),
            (2, ApplicationCommandType::User),
            (3, ApplicationCommandType::Message),
        ];

        for (value, kind) in commands {
            assert_eq!(ApplicationCommandType::from_value(value), Some(kind));
            assert_eq!(kind.value(), value);
        }

        assert_eq!(ApplicationCommandType::from_value(4), None);
        assert_eq!(
            ApplicationCommandType::default(),
            ApplicationCommandType::ChatInput
        );
        assert!(!ApplicationCommandType::ChatInput.is_context_menu());
        assert!(ApplicationCommandType::User.is_context_menu());
        assert_eq!(ApplicationCommandType::User.to_string(), "user");
    }

    #[test]
    fn option_and_integration_types() {
        let options = [
            (1, ApplicationCommandOptionType::Subcommand),
            (2, ApplicationCommandOptionType::SubcommandGroup),
            (3, ApplicationCommandOptionType::String),
            (4, ApplicationCommandOptionType::Integer),
            (5, ApplicationCommandOptionType::Boolean),
            (6, ApplicationCommandOptionType::User),
            (7, ApplicationCommandOptionType::Channel),
            (8, ApplicationCommandOptionType::Role),
            (9, ApplicationCommandOptionType::Mentionable),
            (10, ApplicationCommandOptionType::Number),
            (11, ApplicationCommandOptionType::Attachment),
        ];

        for (value, kind) in options {
            assert_eq!(ApplicationCommandOptionType::from_value(value), Some(kind));
            assert_eq!(kind.value(), value);
        }

        assert_eq!(ApplicationCommandOptionType::from_value(0), None);
        assert_eq!(ApplicationCommandOptionType::from_value(12), None);
        assert_eq!(
            ApplicationCommandOptionType::default(),
            ApplicationCommandOptionType::Subcommand
        );
        assert!(ApplicationCommandOptionType::Subcommand.is_subcommand());
        assert!(!ApplicationCommandOptionType::String.is_subcommand());
        assert!(ApplicationCommandOptionType::String.is_value());
        assert_eq!(
            ApplicationCommandOptionType::SubcommandGroup.to_string(),
            "subcommand_group"
        );

        let integrations = [
            (0, ApplicationIntegrationType::GuildInstall),
            (1, ApplicationIntegrationType::UserInstall),
        ];

        for (value, kind) in integrations {
            assert_eq!(ApplicationIntegrationType::from_value(value), Some(kind));
            assert_eq!(kind.value(), value);
        }

        assert_eq!(ApplicationIntegrationType::from_value(2), None);
        assert_eq!(
            ApplicationIntegrationType::default(),
            ApplicationIntegrationType::GuildInstall
        );
        assert_eq!(
            ApplicationIntegrationType::UserInstall.to_string(),
            "user_install"
        );
    }
}
