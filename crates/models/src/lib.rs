//! Discord API models.
//!
//! This crate will grow to contain `User`, `Guild`, `Channel`, `Message` and
//! the rest of Discord's API models. It currently covers applications, audit
//! logs, guilds, guild channels, permission overwrites, invites, messages,
//! components, embeds, forum tags, interactions, reactions, emojis, stickers,
//! polls, scheduled events, threads, webhooks, users, client users, guild roles
//! and guild members. Models can be decoded from [`data::Value`] trees with the
//! `from_value` constructors, which report the path of any malformed field.
//! Guild payloads decode their roles, members, channels, stickers and scheduled
//! events together, resolving member role IDs against the decoded roles.
//! Message payloads decode their attachments, embeds, components, polls,
//! reactions, stickers, references and interaction metadata.
//! Interaction payloads decode their command options, resolved objects, message
//! components and modal submissions according to the interaction type.

#![forbid(unsafe_code)]

mod application;
mod audit_log;
mod cast;
mod channel;
mod client_user;
mod components;
mod embed;
mod emoji;
mod forum;
mod guild;
mod interaction;
mod invite;
mod member;
mod message;
mod poll;
mod reaction;
mod role;
mod scheduled_event;
mod sticker;
mod thread;
mod user;
mod webhook;

pub use abc::{Asset, Color, Colour, HasId, Snowflake};
pub use application::PartialApplication;
pub use audit_log::{
    AuditLog, AuditLogChange, AuditLogChangeKey, AuditLogChangeValue, AuditLogEntry,
    AuditLogEntryInfo, AuditLogEntryTarget,
};
pub use cast::CastError;
pub use channel::{
    GuildChannel, PermissionOverwrite, PermissionOverwriteEntry, PermissionOverwriteType,
};
pub use client_user::ClientUser;
pub use components::{
    ActionRow, ActionRowChild, Button, SelectDefaultValue, SelectMenu, SelectOption, TextInput,
};
pub use embed::{
    Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedImage, EmbedProvider, EmbedThumbnail,
    EmbedVideo,
};
pub use emoji::PartialEmoji;
pub use enums::{
    ApplicationCommandOptionType, ApplicationCommandType, ApplicationIntegrationType,
    AttachmentFlags, ButtonStyle, ChannelFlags, ChannelType, ComponentType, ContentFilter,
    EmbedFlags, EmbedType, ForumLayout, ForumSortOrder, GuildScheduledEventEntityType,
    GuildScheduledEventStatus, InteractionContextType, InteractionResponseType, InteractionType,
    InviteFlags, InviteTarget, InviteType, MemberFlags, MessageFlags, MessageReferenceType,
    MessageType, MfaLevel, NotificationLevel, NsfwLevel, Permissions, PollLayoutType, PrivacyLevel,
    PublicUserFlags, ReactionType, RoleFlags, SelectDefaultValueType, StickerFormatType,
    StickerType, TextStyle, ThreadMemberFlags, VerificationLevel, VideoQualityMode, WebhookType,
};
pub use forum::ForumTag;
pub use guild::Guild;
pub use interaction::{
    ApplicationCommandData, ApplicationCommandOption, AutocompleteChoice, AutocompleteResponseData,
    CommandOptionValue, Interaction, InteractionActivityInstance, InteractionData,
    InteractionMessage, InteractionMessageData, InteractionResponse, InteractionResponseData,
    MessageComponentData, MessageInteraction, MessageInteractionMetadata, ModalResponseData,
    ModalSubmitComponent, ModalSubmitData, ResolvedData,
};
pub use invite::{Invite, InviteChannel, InviteGuild, PartialInviteChannel, PartialInviteGuild};
pub use member::Member;
pub use message::{Attachment, Message, MessageAuthor, MessageReference};
pub use poll::{Poll, PollAnswer, PollAnswerCount, PollMedia, PollResults};
pub use reaction::Reaction;
pub use role::{Role, RoleIcon, RoleTags};
pub use scheduled_event::{GuildScheduledEvent, ScheduledEventUser};
pub use sticker::{Sticker, StickerItem, StickerPack};
pub use thread::{ThreadInfo, ThreadMember, ThreadMetadata};
pub use user::User;
pub use webhook::{PartialWebhookChannel, PartialWebhookGuild, Webhook, WebhookMessage};
