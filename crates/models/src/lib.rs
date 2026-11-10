//! Discord API models.
//!
//! This crate will grow to contain `User`, `Guild`, `Channel`, `Message` and
//! the rest of Discord's API models. It currently covers guilds, guild
//! channels, permission overwrites, messages, reactions, emojis, users, client
//! users, guild roles and guild members.

#![forbid(unsafe_code)]

mod channel;
mod client_user;
mod emoji;
mod guild;
mod member;
mod message;
mod reaction;
mod role;
mod user;

pub use abc::{Asset, Color, Colour, HasId, Snowflake};
pub use channel::{
    GuildChannel, PermissionOverwrite, PermissionOverwriteEntry, PermissionOverwriteType,
};
pub use client_user::ClientUser;
pub use emoji::PartialEmoji;
pub use enums::{
    AttachmentFlags, ChannelFlags, ChannelType, ContentFilter, ForumLayout, ForumSortOrder,
    MemberFlags, MessageFlags, MessageReferenceType, MessageType, MfaLevel, NotificationLevel,
    NsfwLevel, Permissions, PublicUserFlags, ReactionType, RoleFlags, VerificationLevel,
    VideoQualityMode,
};
pub use guild::Guild;
pub use member::Member;
pub use message::{Attachment, Message, MessageAuthor, MessageReference};
pub use reaction::Reaction;
pub use role::{Role, RoleIcon, RoleTags};
pub use user::User;
