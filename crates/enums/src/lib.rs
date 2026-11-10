//! Discord enumerations and bitfields.
//! The first inhabitants of this crate are the two flag types used everywhere
//! in a Discord client: [`Intents`] for the gateway and [`Permissions`] for
//! guild and channel authorization.

#![forbid(unsafe_code)]

mod channel;
mod embed;
mod guild;
mod intents;
mod member_flags;
mod message;
mod permissions;
mod public_user_flags;
mod role_flags;
mod sticker;

pub use channel::{ChannelFlags, ChannelType, ForumLayout, ForumSortOrder, VideoQualityMode};
pub use embed::{EmbedFlags, EmbedType};
pub use guild::{ContentFilter, MfaLevel, NotificationLevel, NsfwLevel, VerificationLevel};
pub use intents::Intents;
pub use member_flags::MemberFlags;
pub use message::{AttachmentFlags, MessageFlags, MessageReferenceType, MessageType, ReactionType};
pub use permissions::Permissions;
pub use public_user_flags::PublicUserFlags;
pub use role_flags::RoleFlags;
pub use sticker::{StickerFormatType, StickerType};
