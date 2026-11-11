//! Discord enumerations and bitfields.
//! The first inhabitants of this crate are the two flag types used everywhere
//! in a Discord client: [`Intents`] for the gateway and [`Permissions`] for
//! guild and channel authorization.

#![forbid(unsafe_code)]

mod audit_log;
mod channel;
mod components;
mod embed;
mod guild;
mod intents;
mod interaction;
mod invite;
mod member_flags;
mod message;
mod permissions;
mod poll;
mod public_user_flags;
mod role_flags;
mod scheduled_event;
mod sticker;
mod thread;
mod webhook;

pub use audit_log::{AuditLogAction, AuditLogActionCategory, AuditLogActionTargetType};
pub use channel::{ChannelFlags, ChannelType, ForumLayout, ForumSortOrder, VideoQualityMode};
pub use components::{ButtonStyle, ComponentType, SelectDefaultValueType, TextStyle};
pub use embed::{EmbedFlags, EmbedType};
pub use guild::{
    ContentFilter, MfaLevel, NotificationLevel, NsfwLevel, PrivacyLevel, VerificationLevel,
};
pub use intents::Intents;
pub use interaction::{
    ApplicationCommandOptionType, ApplicationCommandType, ApplicationIntegrationType,
    InteractionContextType, InteractionResponseType, InteractionType,
};
pub use invite::{InviteFlags, InviteTarget, InviteType};
pub use member_flags::MemberFlags;
pub use message::{AttachmentFlags, MessageFlags, MessageReferenceType, MessageType, ReactionType};
pub use permissions::Permissions;
pub use poll::PollLayoutType;
pub use public_user_flags::PublicUserFlags;
pub use role_flags::RoleFlags;
pub use scheduled_event::{GuildScheduledEventEntityType, GuildScheduledEventStatus};
pub use sticker::{StickerFormatType, StickerType};
pub use thread::ThreadMemberFlags;
pub use webhook::WebhookType;
