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
//! Invite, application and webhook payloads decode their partial guild,
//! channel, application and source models.
//! Audit log pages decode their entries, typed changes, extra information and
//! referenced users, webhooks, threads and scheduled events.
//! Gateway dispatches decode into typed [`Event`] values, preserving unknown
//! event names and their data.

#![forbid(unsafe_code)]

mod application;
mod audit_log;
mod automod;
mod cast;
mod channel;
mod client_user;
mod components;
mod embed;
mod emoji;
mod event;
mod forum;
mod guild;
mod interaction;
mod invite;
mod member;
mod message;
mod monetization;
mod onboarding;
mod poll;
mod presence;
mod reaction;
mod role;
mod scheduled_event;
mod soundboard;
mod stage_instance;
mod sticker;
mod template;
mod thread;
mod user;
mod webhook;

pub use abc::{Asset, Color, Colour, HasId, Snowflake};
pub use application::PartialApplication;
pub use audit_log::{
    AuditLog, AuditLogChange, AuditLogChangeKey, AuditLogChangeValue, AuditLogEntry,
    AuditLogEntryInfo, AuditLogEntryTarget,
};
pub use automod::{AutoModAction, AutoModActionMetadata, AutoModRule, AutoModTriggerMetadata};
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
pub use event::names;
pub use event::{
    CommandPermission, CommandPermissionsUpdate, Dispatch, Event, GuildMemberRemove,
    GuildMembersChunk, MessageDelete, MessageDeleteBulk, MessagePollVote, PresenceUpdate, RawEvent,
    ReactionEvent, ReactionRemoveAll, ReactionRemoveEmoji, ReadyEvent, RoleDelete, RoleEvent,
    ThreadDelete, TypingStart, UnavailableGuild, WebhooksUpdate,
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
pub use monetization::{Entitlement, Sku};
pub use onboarding::{
    Onboarding, OnboardingPrompt, OnboardingPromptOption, WelcomeScreen, WelcomeScreenChannel,
};
pub use poll::{Poll, PollAnswer, PollAnswerCount, PollMedia, PollResults};
pub use presence::{Activity, ClientStatus, Presence};
pub use reaction::Reaction;
pub use role::{Role, RoleIcon, RoleTags};
pub use scheduled_event::{GuildScheduledEvent, ScheduledEventUser};
pub use soundboard::SoundboardSound;
pub use stage_instance::{StageInstance, StagePrivacyLevel};
pub use sticker::{Sticker, StickerItem, StickerPack};
pub use template::GuildTemplate;
pub use thread::{ThreadInfo, ThreadMember, ThreadMetadata};
pub use user::User;
pub use webhook::{PartialWebhookChannel, PartialWebhookGuild, Webhook, WebhookMessage};
