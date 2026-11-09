//! Discord API models.
//!
//! This crate will grow to contain `User`, `Guild`, `Channel`, `Message` and
//! the rest of Discord's API models. It currently covers guilds, guild
//! channels and permission overwrites, users, client users, guild roles and
//! guild members.

#![forbid(unsafe_code)]

mod channel;
mod client_user;
mod guild;
mod member;
mod role;
mod user;

pub use abc::{Asset, Color, Colour, HasId, Snowflake};
pub use channel::{
    GuildChannel, PermissionOverwrite, PermissionOverwriteEntry, PermissionOverwriteType,
};
pub use client_user::ClientUser;
pub use enums::{
    ChannelFlags, ChannelType, ContentFilter, ForumLayout, ForumSortOrder, MemberFlags, MfaLevel,
    NotificationLevel, NsfwLevel, Permissions, PublicUserFlags, RoleFlags, VerificationLevel,
    VideoQualityMode,
};
pub use guild::Guild;
pub use member::Member;
pub use role::{Role, RoleIcon, RoleTags};
pub use user::User;
