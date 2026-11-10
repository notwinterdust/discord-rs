//! Discord API models.
//!
//! This crate will grow to contain `User`, `Guild`, `Channel`, `Message` and
//! the rest of Discord's API models. It currently covers guilds, guild
//! channels, permission overwrites, messages, components, embeds, reactions,
//! emojis, stickers, polls, users, client users, guild roles and guild
//! members.

#![forbid(unsafe_code)]

mod channel;
mod client_user;
mod components;
mod embed;
mod emoji;
mod guild;
mod member;
mod message;
mod poll;
mod reaction;
mod role;
mod sticker;
mod user;

pub use abc::{Asset, Color, Colour, HasId, Snowflake};
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
    AttachmentFlags, ButtonStyle, ChannelFlags, ChannelType, ComponentType, ContentFilter,
    EmbedFlags, EmbedType, ForumLayout, ForumSortOrder, MemberFlags, MessageFlags,
    MessageReferenceType, MessageType, MfaLevel, NotificationLevel, NsfwLevel, Permissions,
    PollLayoutType, PublicUserFlags, ReactionType, RoleFlags, SelectDefaultValueType,
    StickerFormatType, StickerType, TextStyle, VerificationLevel, VideoQualityMode,
};
pub use guild::Guild;
pub use member::Member;
pub use message::{Attachment, Message, MessageAuthor, MessageReference};
pub use poll::{Poll, PollAnswer, PollAnswerCount, PollMedia, PollResults};
pub use reaction::Reaction;
pub use role::{Role, RoleIcon, RoleTags};
pub use sticker::{Sticker, StickerItem, StickerPack};
pub use user::User;
