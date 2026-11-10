//! Discord API models.
//!
//! This crate will grow to contain `User`, `Guild`, `Channel`, `Message` and
//! the rest of Discord's API models. It currently covers guilds, guild
//! channels, permission overwrites, messages, components, embeds, forum tags,
//! interactions, reactions, emojis, stickers, polls, threads, users, client
//! users, guild roles and guild members.

#![forbid(unsafe_code)]

mod channel;
mod client_user;
mod components;
mod embed;
mod emoji;
mod forum;
mod guild;
mod interaction;
mod member;
mod message;
mod poll;
mod reaction;
mod role;
mod sticker;
mod thread;
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
    ApplicationCommandOptionType, ApplicationCommandType, ApplicationIntegrationType,
    AttachmentFlags, ButtonStyle, ChannelFlags, ChannelType, ComponentType, ContentFilter,
    EmbedFlags, EmbedType, ForumLayout, ForumSortOrder, InteractionContextType,
    InteractionResponseType, InteractionType, MemberFlags, MessageFlags, MessageReferenceType,
    MessageType, MfaLevel, NotificationLevel, NsfwLevel, Permissions, PollLayoutType,
    PublicUserFlags, ReactionType, RoleFlags, SelectDefaultValueType, StickerFormatType,
    StickerType, TextStyle, ThreadMemberFlags, VerificationLevel, VideoQualityMode,
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
pub use member::Member;
pub use message::{Attachment, Message, MessageAuthor, MessageReference};
pub use poll::{Poll, PollAnswer, PollAnswerCount, PollMedia, PollResults};
pub use reaction::Reaction;
pub use role::{Role, RoleIcon, RoleTags};
pub use sticker::{Sticker, StickerItem, StickerPack};
pub use thread::{ThreadInfo, ThreadMember, ThreadMetadata};
pub use user::User;
