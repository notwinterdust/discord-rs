//! Channel-level enumerations and flags.

use std::fmt;

/// The kind of channel a [`ChannelType`] describes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ChannelType {
    /// A guild text channel.
    #[default]
    Text = 0,
    /// A direct message between two users.
    Private = 1,
    /// A guild voice channel.
    Voice = 2,
    /// A group direct message.
    Group = 3,
    /// A guild channel category.
    Category = 4,
    /// A guild announcement channel.
    News = 5,
    /// A thread created from an announcement channel.
    NewsThread = 10,
    /// A public thread.
    PublicThread = 11,
    /// A private thread.
    PrivateThread = 12,
    /// A guild stage voice channel.
    StageVoice = 13,
    /// A guild forum channel.
    Forum = 15,
    /// A guild media channel.
    Media = 16,
}

impl ChannelType {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Text),
            1 => Some(Self::Private),
            2 => Some(Self::Voice),
            3 => Some(Self::Group),
            4 => Some(Self::Category),
            5 => Some(Self::News),
            10 => Some(Self::NewsThread),
            11 => Some(Self::PublicThread),
            12 => Some(Self::PrivateThread),
            13 => Some(Self::StageVoice),
            15 => Some(Self::Forum),
            16 => Some(Self::Media),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Private => "private",
            Self::Voice => "voice",
            Self::Group => "group",
            Self::Category => "category",
            Self::News => "news",
            Self::NewsThread => "news_thread",
            Self::PublicThread => "public_thread",
            Self::PrivateThread => "private_thread",
            Self::StageVoice => "stage_voice",
            Self::Forum => "forum",
            Self::Media => "media",
        }
    }

    /// Returns `true` if this is a guild channel type.
    #[must_use]
    pub const fn is_guild(self) -> bool {
        !matches!(self, Self::Private | Self::Group)
    }

    /// Returns `true` if this is a direct or group message type.
    #[must_use]
    pub const fn is_dm(self) -> bool {
        matches!(self, Self::Private | Self::Group)
    }

    /// Returns `true` if this is any thread type.
    #[must_use]
    pub const fn is_thread(self) -> bool {
        matches!(
            self,
            Self::NewsThread | Self::PublicThread | Self::PrivateThread
        )
    }

    /// Returns `true` if this type can contain text messages or threads.
    #[must_use]
    pub const fn is_textual(self) -> bool {
        matches!(
            self,
            Self::Text
                | Self::News
                | Self::NewsThread
                | Self::PublicThread
                | Self::PrivateThread
                | Self::Forum
                | Self::Media
        )
    }
}

impl fmt::Display for ChannelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The camera video quality used by a voice channel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum VideoQualityMode {
    /// Discord chooses the quality automatically.
    #[default]
    Auto = 1,
    /// The channel uses full quality.
    Full = 2,
}

impl VideoQualityMode {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Auto),
            2 => Some(Self::Full),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Full => "full",
        }
    }
}

impl fmt::Display for VideoQualityMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The default layout of a forum channel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ForumLayout {
    /// The guild has not chosen a layout.
    #[default]
    NotSet = 0,
    /// The forum displays threads as a list.
    ListView = 1,
    /// The forum displays threads as a gallery.
    GalleryView = 2,
}

impl ForumLayout {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::NotSet),
            1 => Some(Self::ListView),
            2 => Some(Self::GalleryView),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotSet => "not_set",
            Self::ListView => "list_view",
            Self::GalleryView => "gallery_view",
        }
    }
}

impl fmt::Display for ForumLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The default sort order of a forum channel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ForumSortOrder {
    /// Sort threads by their latest activity.
    #[default]
    LatestActivity = 0,
    /// Sort threads by their creation date.
    CreationDate = 1,
}

impl ForumSortOrder {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::LatestActivity),
            1 => Some(Self::CreationDate),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LatestActivity => "latest_activity",
            Self::CreationDate => "creation_date",
        }
    }
}

impl fmt::Display for ForumSortOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Wraps the Discord channel flag bitfield.
///
/// These flags appear on guild channels and threads.
///
/// ```
///     use enums::ChannelFlags;
///
///     let flags = ChannelFlags::PINNED;
///     assert!(flags.pinned());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ChannelFlags(u64);

impl ChannelFlags {
    /// No flags.
    pub const NONE: Self = Self(0);

    /// The thread is pinned to its forum or media channel.
    pub const PINNED: Self = Self(1 << 1);
    /// A tag must be selected when creating a thread in this forum.
    pub const REQUIRE_TAG: Self = Self(1 << 4);
    /// Embedded media download options are hidden in this media channel.
    pub const HIDE_MEDIA_DOWNLOAD_OPTIONS: Self = Self(1 << 15);

    /// Every known channel flag.
    pub const ALL: Self =
        Self(Self::PINNED.0 | Self::REQUIRE_TAG.0 | Self::HIDE_MEDIA_DOWNLOAD_OPTIONS.0);

    /// Creates an empty flag set.
    #[must_use]
    pub const fn none() -> Self {
        Self::NONE
    }

    /// Creates a set with every known flag enabled.
    #[must_use]
    pub const fn all() -> Self {
        Self::ALL
    }

    /// Returns the raw bitfield.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Wraps a raw bitfield without validating it.
    #[must_use]
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Wraps a raw bitfield, dropping bits not defined by this crate.
    #[must_use]
    pub const fn from_bits_truncate(bits: u64) -> Self {
        Self(bits & Self::ALL.0)
    }

    /// Returns `true` if no flag is set.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if all flags in `other` are enabled in `self`.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Returns `true` if at least one flag in `other` is enabled in `self`.
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }

    /// Enables the flags in `other`.
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    /// Disables the flags in `other`.
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }

    /// Toggles the flags in `other`.
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.0;
    }

    /// Enables or disables the flags in `other`.
    pub fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    /// Returns the union of both sets.
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Returns the intersection of both sets.
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Returns the flags in `self` that are not in `other`.
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// Returns the flags enabled in exactly one of the two sets.
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    /// Returns every known flag that is not enabled in `self`.
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0 & Self::ALL.0)
    }

    /// Returns whether the thread is pinned to a forum or media channel.
    #[must_use]
    pub const fn pinned(self) -> bool {
        self.contains(Self::PINNED)
    }

    /// Enables or disables the pinned flag.
    pub fn set_pinned(&mut self, value: bool) {
        self.set(Self::PINNED, value);
    }

    /// Returns whether a tag is required when creating a forum thread.
    #[must_use]
    pub const fn require_tag(self) -> bool {
        self.contains(Self::REQUIRE_TAG)
    }

    /// Enables or disables the required tag flag.
    pub fn set_require_tag(&mut self, value: bool) {
        self.set(Self::REQUIRE_TAG, value);
    }

    /// Returns whether embedded media download options are hidden.
    #[must_use]
    pub const fn hide_media_download_options(self) -> bool {
        self.contains(Self::HIDE_MEDIA_DOWNLOAD_OPTIONS)
    }

    /// Enables or disables hidden media download options.
    pub fn set_hide_media_download_options(&mut self, value: bool) {
        self.set(Self::HIDE_MEDIA_DOWNLOAD_OPTIONS, value);
    }
}

impl core::ops::BitOr for ChannelFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl core::ops::BitOrAssign for ChannelFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.insert(rhs);
    }
}

impl core::ops::BitAnd for ChannelFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl core::ops::BitAndAssign for ChannelFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl core::ops::BitXor for ChannelFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl core::ops::BitXorAssign for ChannelFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.toggle(rhs);
    }
}

impl core::ops::Not for ChannelFlags {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl From<u64> for ChannelFlags {
    fn from(bits: u64) -> Self {
        Self::from_bits(bits)
    }
}

impl From<ChannelFlags> for u64 {
    fn from(flags: ChannelFlags) -> Self {
        flags.bits()
    }
}

#[cfg(test)]
mod tests {
    use super::{ChannelFlags, ChannelType, ForumLayout, ForumSortOrder, VideoQualityMode};

    #[test]
    fn channel_type_round_trips() {
        let values = [0, 1, 2, 3, 4, 5, 10, 11, 12, 13, 15, 16];

        for value in values {
            let channel_type = ChannelType::from_value(value).unwrap();
            assert_eq!(channel_type.value(), value);
        }

        assert_eq!(ChannelType::from_value(6), None);
        assert_eq!(ChannelType::from_value(14), None);
        assert_eq!(ChannelType::default(), ChannelType::Text);
    }

    #[test]
    fn channel_type_classification() {
        assert!(ChannelType::Text.is_guild());
        assert!(!ChannelType::Private.is_guild());
        assert!(!ChannelType::Group.is_guild());
        assert!(ChannelType::Private.is_dm());
        assert!(ChannelType::Group.is_dm());
        assert!(ChannelType::PublicThread.is_thread());
        assert!(!ChannelType::Text.is_thread());
        assert!(ChannelType::Forum.is_textual());
        assert!(ChannelType::Media.is_textual());
        assert!(!ChannelType::Voice.is_textual());
    }

    #[test]
    fn channel_type_names() {
        assert_eq!(ChannelType::NewsThread.to_string(), "news_thread");
        assert_eq!(ChannelType::StageVoice.to_string(), "stage_voice");
        assert_eq!(ChannelType::Media.to_string(), "media");
    }

    #[test]
    fn auxiliary_enums_round_trip() {
        assert_eq!(
            VideoQualityMode::from_value(1),
            Some(VideoQualityMode::Auto)
        );
        assert_eq!(
            VideoQualityMode::from_value(2),
            Some(VideoQualityMode::Full)
        );
        assert_eq!(VideoQualityMode::from_value(3), None);
        assert_eq!(VideoQualityMode::default(), VideoQualityMode::Auto);

        assert_eq!(ForumLayout::from_value(2), Some(ForumLayout::GalleryView));
        assert_eq!(ForumLayout::from_value(3), None);
        assert_eq!(
            ForumSortOrder::from_value(1),
            Some(ForumSortOrder::CreationDate)
        );
        assert_eq!(ForumSortOrder::from_value(2), None);

        assert_eq!(VideoQualityMode::Full.to_string(), "full");
        assert_eq!(ForumLayout::ListView.to_string(), "list_view");
        assert_eq!(
            ForumSortOrder::LatestActivity.to_string(),
            "latest_activity"
        );
    }

    #[test]
    fn channel_flags() {
        assert_eq!(ChannelFlags::PINNED.bits(), 1 << 1);
        assert_eq!(ChannelFlags::REQUIRE_TAG.bits(), 1 << 4);
        assert_eq!(ChannelFlags::HIDE_MEDIA_DOWNLOAD_OPTIONS.bits(), 1 << 15);
        assert_eq!(
            ChannelFlags::all().bits(),
            ChannelFlags::PINNED.bits()
                | ChannelFlags::REQUIRE_TAG.bits()
                | ChannelFlags::HIDE_MEDIA_DOWNLOAD_OPTIONS.bits()
        );

        let mut flags = ChannelFlags::none();
        assert!(flags.is_empty());
        assert!(!flags.pinned());

        flags.set_pinned(true);
        flags.set_require_tag(true);
        assert!(flags.pinned());
        assert!(flags.require_tag());
        assert_eq!(flags, ChannelFlags::PINNED | ChannelFlags::REQUIRE_TAG);

        flags.set_pinned(false);
        assert!(!flags.pinned());
        assert_eq!(!ChannelFlags::none(), ChannelFlags::all());
    }
}
