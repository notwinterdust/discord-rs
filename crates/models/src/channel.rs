//! Guild channels and channel permission overwrites.
//!
//! A [`GuildChannel`] is a unified representation of Discord's guild channel
//! types. Fields that only apply to some channel types are still present; the
//! classification helpers indicate which fields are meaningful.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

use abc::{HasId, Snowflake};
use data::Value;
use enums::{
    ChannelFlags, ChannelType, ForumLayout, ForumSortOrder, Permissions, VideoQualityMode,
};

use crate::cast::{self, CastError};
use crate::{ForumTag, Guild, Member, Role, ThreadInfo, ThreadMember, ThreadMetadata};

/// The target kind of a channel permission overwrite.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PermissionOverwriteType {
    /// The overwrite applies to a role.
    #[default]
    Role = 0,
    /// The overwrite applies to a guild member.
    Member = 1,
}

impl PermissionOverwriteType {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Role),
            1 => Some(Self::Member),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Role => "role",
            Self::Member => "member",
        }
    }
}

impl fmt::Display for PermissionOverwriteType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A channel-specific allow/deny permission pair.
///
/// Unlike [`Permissions`], a permission in an overwrite has three states:
/// explicitly allowed, explicitly denied, or inherited. The inherited state is
/// represented by `None` when using [`PermissionOverwrite::get`] and
/// [`PermissionOverwrite::set`].
///
/// ```
///     use enums::Permissions;
///     use models::PermissionOverwrite;
///
///     let mut overwrite = PermissionOverwrite::new();
///     overwrite.set(Permissions::VIEW_CHANNEL, Some(true));
///     overwrite.set(Permissions::SEND_MESSAGES, Some(false));
///
///     assert_eq!(overwrite.get(Permissions::VIEW_CHANNEL), Some(true));
///     assert_eq!(overwrite.get(Permissions::SEND_MESSAGES), Some(false));
///     assert_eq!(overwrite.get(Permissions::MANAGE_GUILD), None);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PermissionOverwrite {
    allow: Permissions,
    deny: Permissions,
}

impl PermissionOverwrite {
    /// Creates an overwrite that allows and denies nothing.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            allow: Permissions::none(),
            deny: Permissions::none(),
        }
    }

    /// Creates an overwrite from an explicit allow and deny pair.
    ///
    /// Callers are expected to keep the two sets disjoint. Use
    /// [`PermissionOverwrite::set`] to set individual permissions with the
    /// correct precedence.
    #[must_use]
    pub const fn from_pair(allow: Permissions, deny: Permissions) -> Self {
        Self { allow, deny }
    }

    /// Returns the explicitly allowed permissions.
    #[must_use]
    pub const fn allow(&self) -> Permissions {
        self.allow
    }

    /// Returns the explicitly denied permissions.
    #[must_use]
    pub const fn deny(&self) -> Permissions {
        self.deny
    }

    /// Returns the `(allow, deny)` pair.
    #[must_use]
    pub const fn pair(&self) -> (Permissions, Permissions) {
        (self.allow, self.deny)
    }

    /// Returns whether the overwrite neither allows nor denies anything.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.allow.is_empty() && self.deny.is_empty()
    }

    /// Returns the state of a permission, or `None` if it is inherited.
    ///
    /// `permission` should normally contain a single flag. For a multi-flag
    /// mask, `Some(true)` and `Some(false)` are only returned when every bit in
    /// the mask has the same explicit state.
    #[must_use]
    pub fn get(&self, permission: Permissions) -> Option<bool> {
        if permission.is_empty() {
            return None;
        }

        if self.allow.intersection(permission) == permission {
            Some(true)
        } else if self.deny.intersection(permission) == permission {
            Some(false)
        } else {
            None
        }
    }

    /// Explicitly allows, explicitly denies, or inherits a permission.
    pub fn set(&mut self, permission: Permissions, value: Option<bool>) {
        match value {
            Some(true) => {
                self.deny.remove(permission);
                self.allow.insert(permission);
            }
            Some(false) => {
                self.allow.remove(permission);
                self.deny.insert(permission);
            }
            None => {
                self.allow.remove(permission);
                self.deny.remove(permission);
            }
        }
    }

    /// Explicitly allows every permission in `permissions`.
    pub fn allow_all(&mut self, permissions: Permissions) {
        self.deny.remove(permissions);
        self.allow.insert(permissions);
    }

    /// Explicitly denies every permission in `permissions`.
    pub fn deny_all(&mut self, permissions: Permissions) {
        self.allow.remove(permissions);
        self.deny.insert(permissions);
    }

    /// Removes every explicit state from `permissions`.
    pub fn clear(&mut self, permissions: Permissions) {
        self.allow.remove(permissions);
        self.deny.remove(permissions);
    }
}

impl From<(Permissions, Permissions)> for PermissionOverwrite {
    fn from((allow, deny): (Permissions, Permissions)) -> Self {
        Self::from_pair(allow, deny)
    }
}

impl From<PermissionOverwrite> for (Permissions, Permissions) {
    fn from(overwrite: PermissionOverwrite) -> Self {
        overwrite.pair()
    }
}

/// A single entry in a channel's permission overwrite list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PermissionOverwriteEntry {
    /// The role or member ID this entry applies to.
    pub target_id: Snowflake,
    /// Whether the target is a role or a member.
    pub kind: PermissionOverwriteType,
    /// The allow/deny pair.
    pub overwrite: PermissionOverwrite,
}

impl PermissionOverwriteEntry {
    /// Creates an overwrite entry for a role or member.
    #[must_use]
    pub const fn new(
        target_id: Snowflake,
        kind: PermissionOverwriteType,
        overwrite: PermissionOverwrite,
    ) -> Self {
        Self {
            target_id,
            kind,
            overwrite,
        }
    }

    /// Creates an overwrite entry for a role.
    #[must_use]
    pub const fn role(target_id: Snowflake, overwrite: PermissionOverwrite) -> Self {
        Self::new(target_id, PermissionOverwriteType::Role, overwrite)
    }

    /// Creates an overwrite entry for a member.
    #[must_use]
    pub const fn member(target_id: Snowflake, overwrite: PermissionOverwrite) -> Self {
        Self::new(target_id, PermissionOverwriteType::Member, overwrite)
    }

    /// Decodes a permission overwrite entry from a Discord overwrite object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "overwrite")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let target_id = cast::required_snowflake(object, path, "id")?;
        let kind = cast::required_enum(
            object,
            path,
            "type",
            "a permission overwrite type",
            PermissionOverwriteType::from_value,
        )?;
        let allow = Permissions::from_bits(
            cast::optional_permission_bits(object, path, "allow")?.unwrap_or(0),
        );
        let deny = Permissions::from_bits(
            cast::optional_permission_bits(object, path, "deny")?.unwrap_or(0),
        );
        Ok(Self::new(
            target_id,
            kind,
            PermissionOverwrite::from_pair(allow, deny),
        ))
    }

    /// Returns whether this entry applies to the given target.
    #[must_use]
    pub fn matches(&self, target_id: Snowflake, kind: PermissionOverwriteType) -> bool {
        self.target_id == target_id && self.kind == kind
    }

    /// Returns whether this entry applies to a role.
    #[must_use]
    pub const fn is_role(&self) -> bool {
        matches!(self.kind, PermissionOverwriteType::Role)
    }

    /// Returns whether this entry applies to a member.
    #[must_use]
    pub const fn is_member(&self) -> bool {
        matches!(self.kind, PermissionOverwriteType::Member)
    }
}

/// A guild channel.
///
/// This is a unified model for text, voice, category, announcement, stage,
/// forum, media and thread channels. Fields that do not apply to the channel's
/// [`ChannelType`] keep their defaults. Thread-specific state is stored in
/// [`GuildChannel::thread`].
///
/// Equality and hashing are based on the channel's ID.
#[derive(Clone, Debug)]
pub struct GuildChannel {
    /// The channel's unique ID.
    pub id: Snowflake,
    /// The ID of the guild that owns the channel.
    pub guild_id: Snowflake,
    /// The channel's name.
    pub name: String,
    /// The channel's type.
    pub channel_type: ChannelType,
    /// The channel's position in the guild's channel list.
    pub position: i64,
    /// The ID of the category this channel belongs to, if any.
    pub category_id: Option<Snowflake>,
    /// The channel's permission overwrites.
    pub permission_overwrites: Vec<PermissionOverwriteEntry>,
    /// Whether the channel is age-restricted.
    pub nsfw: bool,
    /// The channel's flags.
    pub flags: ChannelFlags,
    /// The text channel's topic or the forum's guidelines.
    pub topic: Option<String>,
    /// The number of seconds members must wait between messages.
    pub slowmode_delay: u64,
    /// The default auto-archive duration for threads, in minutes.
    pub default_auto_archive_duration: u64,
    /// The default slowmode delay for threads, in seconds.
    pub default_thread_slowmode_delay: u64,
    /// The last message or thread ID, if known.
    pub last_message_id: Option<Snowflake>,
    /// The voice channel's bitrate, in bits per second.
    pub bitrate: u64,
    /// The voice channel's user limit, where `0` means unlimited.
    pub user_limit: u64,
    /// The voice channel's RTC region, or `None` for automatic selection.
    pub rtc_region: Option<String>,
    /// The voice channel's camera video quality.
    pub video_quality_mode: VideoQualityMode,
    /// The voice channel's status, if any.
    pub status: Option<String>,
    /// The forum's default layout.
    pub default_layout: ForumLayout,
    /// The forum's default sort order.
    pub default_sort_order: ForumSortOrder,
    /// The tags available in a forum or media channel.
    pub available_tags: Vec<ForumTag>,
    /// Thread-specific state, for thread channels.
    pub thread: Option<ThreadInfo>,
}

impl Default for GuildChannel {
    fn default() -> Self {
        Self {
            id: Snowflake::ZERO,
            guild_id: Snowflake::ZERO,
            name: String::new(),
            channel_type: ChannelType::default(),
            position: 0,
            category_id: None,
            permission_overwrites: Vec::new(),
            nsfw: false,
            flags: ChannelFlags::default(),
            topic: None,
            slowmode_delay: 0,
            default_auto_archive_duration: 1440,
            default_thread_slowmode_delay: 0,
            last_message_id: None,
            bitrate: 64_000,
            user_limit: 0,
            rtc_region: None,
            video_quality_mode: VideoQualityMode::default(),
            status: None,
            default_layout: ForumLayout::default(),
            default_sort_order: ForumSortOrder::default(),
            available_tags: Vec::new(),
            thread: None,
        }
    }
}

impl GuildChannel {
    /// Creates a guild channel with the fields Discord always provides.
    #[must_use]
    pub fn new(
        id: impl Into<Snowflake>,
        guild_id: impl Into<Snowflake>,
        name: impl Into<String>,
        channel_type: ChannelType,
    ) -> Self {
        Self {
            id: id.into(),
            guild_id: guild_id.into(),
            name: name.into(),
            channel_type,
            ..Self::default()
        }
    }

    /// Decodes a guild channel from a Discord channel object.
    ///
    /// The payload's `guild_id` is used when present, otherwise the supplied
    /// `guild_id` is used.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value, guild_id: impl Into<Snowflake>) -> Result<Self, CastError> {
        Self::from_value_at(value, "channel", guild_id.into())
    }

    pub(crate) fn from_value_at(
        value: &Value,
        path: &str,
        guild_id: Snowflake,
    ) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let id = cast::required_snowflake(object, path, "id")?;
        let guild_id = cast::optional_snowflake(object, path, "guild_id")?.unwrap_or(guild_id);
        let name = cast::required_string(object, path, "name")?.to_owned();
        let channel_type = cast::required_enum(
            object,
            path,
            "type",
            "a channel type",
            ChannelType::from_value,
        )?;

        let overwrites_path = cast::field_path(path, "permission_overwrites");
        let mut permission_overwrites = Vec::new();
        if let Some(values) = cast::optional_array(object, path, "permission_overwrites")? {
            permission_overwrites.reserve(values.len());
            for (index, value) in values.iter().enumerate() {
                permission_overwrites.push(PermissionOverwriteEntry::from_value_at(
                    value,
                    &cast::index_path(&overwrites_path, index),
                )?);
            }
        }

        let parent_id = cast::optional_snowflake(object, path, "parent_id")?;
        let category_id = if channel_type.is_thread() {
            None
        } else {
            parent_id
        };

        let tags_path = cast::field_path(path, "available_tags");
        let mut available_tags = Vec::new();
        if let Some(values) = cast::optional_array(object, path, "available_tags")? {
            available_tags.reserve(values.len());
            for (index, value) in values.iter().enumerate() {
                available_tags.push(ForumTag::from_value_at(
                    value,
                    &cast::index_path(&tags_path, index),
                )?);
            }
        }

        let thread = if channel_type.is_thread() {
            Some(ThreadInfo::from_value_at(
                value,
                path,
                id,
                parent_id.unwrap_or_default(),
            )?)
        } else {
            None
        };

        Ok(Self {
            id,
            guild_id,
            name,
            channel_type,
            position: cast::optional_i64(object, path, "position")?.unwrap_or(0),
            category_id,
            permission_overwrites,
            nsfw: cast::optional_bool(object, path, "nsfw", false)?,
            flags: ChannelFlags::from_bits(cast::optional_u64(object, path, "flags")?.unwrap_or(0)),
            topic: cast::optional_string(object, path, "topic")?.map(str::to_owned),
            slowmode_delay: cast::optional_u64(object, path, "rate_limit_per_user")?.unwrap_or(0),
            default_auto_archive_duration: cast::optional_u64(
                object,
                path,
                "default_auto_archive_duration",
            )?
            .unwrap_or(1440),
            default_thread_slowmode_delay: cast::optional_u64(
                object,
                path,
                "default_thread_rate_limit_per_user",
            )?
            .unwrap_or(0),
            last_message_id: cast::optional_snowflake(object, path, "last_message_id")?,
            bitrate: cast::optional_u64(object, path, "bitrate")?.unwrap_or(64_000),
            user_limit: cast::optional_u64(object, path, "user_limit")?.unwrap_or(0),
            rtc_region: cast::optional_string(object, path, "rtc_region")?.map(str::to_owned),
            video_quality_mode: cast::optional_enum(
                object,
                path,
                "video_quality_mode",
                "a video quality mode",
                VideoQualityMode::from_value,
            )?
            .unwrap_or_default(),
            status: cast::optional_string(object, path, "status")?.map(str::to_owned),
            default_layout: cast::optional_enum(
                object,
                path,
                "default_forum_layout",
                "a forum layout",
                ForumLayout::from_value,
            )?
            .unwrap_or_default(),
            default_sort_order: cast::optional_enum(
                object,
                path,
                "default_sort_order",
                "a forum sort order",
                ForumSortOrder::from_value,
            )?
            .unwrap_or_default(),
            available_tags,
            thread,
        })
    }

    /// Sets the channel's position.
    #[must_use]
    pub fn with_position(mut self, position: i64) -> Self {
        self.position = position;
        self
    }

    /// Sets the ID of the category this channel belongs to.
    #[must_use]
    pub fn with_category_id(mut self, category_id: impl Into<Snowflake>) -> Self {
        self.category_id = Some(category_id.into());
        self
    }

    /// Sets the channel's permission overwrites.
    #[must_use]
    pub fn with_permission_overwrites(mut self, overwrites: Vec<PermissionOverwriteEntry>) -> Self {
        self.permission_overwrites = overwrites;
        self
    }

    /// Marks the channel as age-restricted.
    #[must_use]
    pub fn with_nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = nsfw;
        self
    }

    /// Sets the channel's flags.
    #[must_use]
    pub fn with_flags(mut self, flags: ChannelFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Sets the channel's topic or forum guidelines.
    #[must_use]
    pub fn with_topic(mut self, topic: impl Into<String>) -> Self {
        self.topic = Some(topic.into());
        self
    }

    /// Sets the channel's slowmode delay in seconds.
    #[must_use]
    pub fn with_slowmode_delay(mut self, seconds: u64) -> Self {
        self.slowmode_delay = seconds;
        self
    }

    /// Sets the default thread auto-archive duration in minutes.
    #[must_use]
    pub fn with_default_auto_archive_duration(mut self, minutes: u64) -> Self {
        self.default_auto_archive_duration = minutes;
        self
    }

    /// Sets the default thread slowmode delay in seconds.
    #[must_use]
    pub fn with_default_thread_slowmode_delay(mut self, seconds: u64) -> Self {
        self.default_thread_slowmode_delay = seconds;
        self
    }

    /// Sets the last known message or thread ID.
    #[must_use]
    pub fn with_last_message_id(mut self, message_id: impl Into<Snowflake>) -> Self {
        self.last_message_id = Some(message_id.into());
        self
    }

    /// Sets the voice channel bitrate.
    #[must_use]
    pub fn with_bitrate(mut self, bitrate: u64) -> Self {
        self.bitrate = bitrate;
        self
    }

    /// Sets the voice channel user limit.
    #[must_use]
    pub fn with_user_limit(mut self, user_limit: u64) -> Self {
        self.user_limit = user_limit;
        self
    }

    /// Sets the voice channel RTC region.
    #[must_use]
    pub fn with_rtc_region(mut self, region: impl Into<String>) -> Self {
        self.rtc_region = Some(region.into());
        self
    }

    /// Sets the voice channel camera video quality.
    #[must_use]
    pub fn with_video_quality_mode(mut self, mode: VideoQualityMode) -> Self {
        self.video_quality_mode = mode;
        self
    }

    /// Sets the voice channel status.
    #[must_use]
    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Sets the forum's default layout.
    #[must_use]
    pub fn with_default_layout(mut self, layout: ForumLayout) -> Self {
        self.default_layout = layout;
        self
    }

    /// Sets the forum's default sort order.
    #[must_use]
    pub fn with_default_sort_order(mut self, order: ForumSortOrder) -> Self {
        self.default_sort_order = order;
        self
    }

    /// Sets the tags available in a forum or media channel.
    #[must_use]
    pub fn with_available_tags(mut self, tags: Vec<ForumTag>) -> Self {
        self.available_tags = tags;
        self
    }

    /// Appends an available forum tag.
    #[must_use]
    pub fn add_available_tag(mut self, tag: ForumTag) -> Self {
        self.available_tags.push(tag);
        self
    }

    /// Sets the thread-specific state for a thread channel.
    #[must_use]
    pub fn with_thread_info(mut self, thread: ThreadInfo) -> Self {
        self.thread = Some(thread);
        self
    }

    /// Returns the channel's type.
    #[must_use]
    pub const fn channel_type(&self) -> ChannelType {
        self.channel_type
    }

    /// Returns `true` if the channel is a text channel.
    #[must_use]
    pub const fn is_text(&self) -> bool {
        matches!(self.channel_type, ChannelType::Text)
    }

    /// Returns `true` if the channel is an announcement channel.
    #[must_use]
    pub const fn is_news(&self) -> bool {
        matches!(self.channel_type, ChannelType::News)
    }

    /// Returns `true` if the channel is a voice channel.
    #[must_use]
    pub const fn is_voice(&self) -> bool {
        matches!(self.channel_type, ChannelType::Voice)
    }

    /// Returns `true` if the channel is a stage voice channel.
    #[must_use]
    pub const fn is_stage_voice(&self) -> bool {
        matches!(self.channel_type, ChannelType::StageVoice)
    }

    /// Returns `true` if the channel is a category.
    #[must_use]
    pub const fn is_category(&self) -> bool {
        matches!(self.channel_type, ChannelType::Category)
    }

    /// Returns `true` if the channel is a forum channel.
    #[must_use]
    pub const fn is_forum(&self) -> bool {
        matches!(self.channel_type, ChannelType::Forum)
    }

    /// Returns `true` if the channel is a media channel.
    #[must_use]
    pub const fn is_media(&self) -> bool {
        matches!(self.channel_type, ChannelType::Media)
    }

    /// Returns `true` if the channel supports forum tags.
    #[must_use]
    pub const fn supports_tags(&self) -> bool {
        matches!(self.channel_type, ChannelType::Forum | ChannelType::Media)
    }

    /// Returns the tags available in a forum or media channel.
    #[must_use]
    pub fn available_tags(&self) -> &[ForumTag] {
        &self.available_tags
    }

    /// Returns an available forum tag by ID, if it exists.
    #[must_use]
    pub fn available_tag(&self, tag_id: impl Into<Snowflake>) -> Option<&ForumTag> {
        let tag_id = tag_id.into();
        self.available_tags.iter().find(|tag| tag.id == tag_id)
    }

    /// Returns `true` if the channel has a tag with the given ID.
    #[must_use]
    pub fn has_tag(&self, tag_id: impl Into<Snowflake>) -> bool {
        self.available_tag(tag_id).is_some()
    }

    /// Returns the tag IDs applied to a thread.
    #[must_use]
    pub fn applied_tag_ids(&self) -> &[Snowflake] {
        self.thread
            .as_ref()
            .map_or(&[], |thread| thread.applied_tags.as_slice())
    }

    /// Returns the parent forum's tags applied to a thread.
    ///
    /// Returns an empty list when the thread has no applied tags or its parent
    /// channel is not cached.
    #[must_use]
    pub fn applied_tags<'a>(&'a self, guild: &'a Guild) -> Vec<&'a ForumTag> {
        let Some(thread) = self.thread.as_ref() else {
            return Vec::new();
        };
        let Some(parent) = self.thread_parent(guild) else {
            return Vec::new();
        };

        thread
            .applied_tags
            .iter()
            .filter_map(|tag_id| parent.available_tag(*tag_id))
            .collect()
    }

    /// Returns `true` if the channel is a thread.
    #[must_use]
    pub const fn is_thread(&self) -> bool {
        self.channel_type.is_thread()
    }

    /// Returns `true` if the channel is a thread created from an announcement
    /// channel.
    #[must_use]
    pub const fn is_news_thread(&self) -> bool {
        self.channel_type.is_news_thread()
    }

    /// Returns `true` if the channel is a public thread.
    #[must_use]
    pub const fn is_public_thread(&self) -> bool {
        self.channel_type.is_public_thread()
    }

    /// Returns `true` if the channel is a private thread.
    #[must_use]
    pub const fn is_private_thread(&self) -> bool {
        self.channel_type.is_private_thread()
    }

    /// Returns the channel's thread-specific state, if it is a thread.
    #[must_use]
    pub const fn thread_info(&self) -> Option<&ThreadInfo> {
        self.thread.as_ref()
    }

    /// Returns mutable access to the channel's thread-specific state.
    pub fn thread_info_mut(&mut self) -> Option<&mut ThreadInfo> {
        self.thread.as_mut()
    }

    /// Returns the thread's archive metadata, if it is a thread.
    #[must_use]
    pub fn thread_metadata(&self) -> Option<&ThreadMetadata> {
        self.thread.as_ref().map(|thread| &thread.metadata)
    }

    /// Returns the thread's cached members, or an empty slice for non-threads.
    #[must_use]
    pub fn thread_members(&self) -> &[ThreadMember] {
        self.thread
            .as_ref()
            .map_or(&[], |thread| thread.members.as_slice())
    }

    /// Returns a thread member by user ID, if it is cached.
    #[must_use]
    pub fn thread_member(&self, user_id: impl Into<Snowflake>) -> Option<&ThreadMember> {
        self.thread
            .as_ref()
            .and_then(|thread| thread.member(user_id))
    }

    /// Returns the thread's parent channel ID, if it is a thread.
    #[must_use]
    pub fn thread_parent_id(&self) -> Option<Snowflake> {
        self.thread.as_ref().map(|thread| thread.parent_id)
    }

    /// Returns the thread owner's user ID, if it is a thread.
    #[must_use]
    pub fn thread_owner_id(&self) -> Option<Snowflake> {
        self.thread.as_ref().map(|thread| thread.owner_id)
    }

    /// Returns `true` if the thread is archived.
    #[must_use]
    pub fn is_archived(&self) -> bool {
        self.thread.as_ref().is_some_and(ThreadInfo::is_archived)
    }

    /// Returns `true` if the thread is locked.
    #[must_use]
    pub fn is_locked(&self) -> bool {
        self.thread.as_ref().is_some_and(ThreadInfo::is_locked)
    }

    /// Returns `true` if non-moderators can invite other non-moderators.
    #[must_use]
    pub fn is_invitable(&self) -> bool {
        self.thread.as_ref().is_some_and(ThreadInfo::is_invitable)
    }

    /// Returns the thread's parent channel, if it is cached.
    #[must_use]
    pub fn thread_parent<'a>(&self, guild: &'a Guild) -> Option<&'a GuildChannel> {
        if !self.is_thread() {
            return None;
        }

        self.thread
            .as_ref()
            .and_then(|thread| guild.get_channel(thread.parent_id))
    }

    /// Returns `true` if the channel is age-restricted.
    #[must_use]
    pub const fn is_nsfw(&self) -> bool {
        self.nsfw
    }

    /// Returns the string that mentions the channel.
    #[must_use]
    pub fn mention(&self) -> String {
        format!("<#{}>", self.id)
    }

    /// Returns a URL that opens the channel in Discord.
    #[must_use]
    pub fn jump_url(&self) -> String {
        format!("https://discord.com/channels/{}/{}", self.guild_id, self.id)
    }

    /// Returns the channel's creation time.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.id.created_at()
    }

    /// Returns the channel's permission overwrites.
    #[must_use]
    pub fn overwrites(&self) -> &[PermissionOverwriteEntry] {
        &self.permission_overwrites
    }

    /// Returns the overwrite for a specific target, if one exists.
    #[must_use]
    pub fn overwrite(
        &self,
        target_id: impl Into<Snowflake>,
        kind: PermissionOverwriteType,
    ) -> Option<&PermissionOverwriteEntry> {
        let target_id = target_id.into();
        self.permission_overwrites
            .iter()
            .find(|entry| entry.matches(target_id, kind))
    }

    /// Returns the overwrite for a role, if one exists.
    #[must_use]
    pub fn overwrite_for_role(
        &self,
        role_id: impl Into<Snowflake>,
    ) -> Option<&PermissionOverwriteEntry> {
        self.overwrite(role_id, PermissionOverwriteType::Role)
    }

    /// Returns the overwrite for a member, if one exists.
    #[must_use]
    pub fn overwrite_for_member(
        &self,
        user_id: impl Into<Snowflake>,
    ) -> Option<&PermissionOverwriteEntry> {
        self.overwrite(user_id, PermissionOverwriteType::Member)
    }

    /// Adds or replaces an overwrite entry.
    ///
    /// If an entry for the same target and kind already exists, it is replaced.
    pub fn set_overwrite(&mut self, entry: PermissionOverwriteEntry) {
        if let Some(existing) = self
            .permission_overwrites
            .iter_mut()
            .find(|existing| existing.matches(entry.target_id, entry.kind))
        {
            *existing = entry;
        } else {
            self.permission_overwrites.push(entry);
        }
    }

    /// Removes the overwrite for a target, returning it if it existed.
    pub fn remove_overwrite(
        &mut self,
        target_id: impl Into<Snowflake>,
        kind: PermissionOverwriteType,
    ) -> Option<PermissionOverwriteEntry> {
        let target_id = target_id.into();
        let index = self
            .permission_overwrites
            .iter()
            .position(|entry| entry.matches(target_id, kind))?;
        Some(self.permission_overwrites.remove(index))
    }

    /// Returns the category this channel belongs to, if it is cached.
    ///
    /// Threads inherit the category of their parent channel.
    #[must_use]
    pub fn category<'a>(&self, guild: &'a Guild) -> Option<&'a GuildChannel> {
        if self.is_thread() {
            return self
                .thread_parent(guild)
                .and_then(|parent| parent.category(guild));
        }

        self.category_id
            .and_then(|category_id| guild.get_channel(category_id))
            .filter(|channel| channel.is_category())
    }

    /// Returns whether this channel's overwrites match its category's.
    ///
    /// Returns `false` if the channel has no category or the category is not
    /// cached.
    #[must_use]
    pub fn permissions_synced(&self, guild: &Guild) -> bool {
        let Some(category) = self.category(guild) else {
            return false;
        };

        self.same_overwrites(&category.permission_overwrites)
    }

    /// Resolves the permissions a role would have in this channel.
    #[must_use]
    pub fn permissions_for_role(&self, guild: &Guild, role: &Role) -> Permissions {
        if let Some(parent) = self.thread_parent(guild) {
            return apply_thread_permissions(parent.permissions_for_role(guild, role));
        }

        let mut base = self.base_permissions_for_role(guild, role);

        if base.administrator() {
            base = Permissions::all();
        } else {
            if let Some(entry) = self.overwrite_for_role(guild.id) {
                base = apply_overwrite(base, &entry.overwrite);
            }

            if !role.is_default()
                && let Some(entry) = self.overwrite_for_role(role.id)
            {
                base = apply_overwrite(base, &entry.overwrite);
            }
        }

        self.apply_channel_type_permissions(base)
    }

    /// Resolves the permissions a member would have in this channel.
    #[must_use]
    pub fn permissions_for(&self, guild: &Guild, member: &Member) -> Permissions {
        if let Some(parent) = self.thread_parent(guild) {
            return apply_thread_permissions(parent.permissions_for(guild, member));
        }

        let mut base = self.base_permissions_for_member(guild, member);

        if guild.is_owner(member.id()) || base.administrator() {
            base = Permissions::all();
        } else {
            if let Some(entry) = self.overwrite_for_role(guild.id) {
                base = apply_overwrite(base, &entry.overwrite);
            }

            let mut allows = Permissions::none();
            let mut denies = Permissions::none();

            for entry in &self.permission_overwrites {
                if !entry.is_role() || entry.target_id == guild.id {
                    continue;
                }

                if member.roles().iter().any(|role| role.id == entry.target_id) {
                    allows.insert(entry.overwrite.allow());
                    denies.insert(entry.overwrite.deny());
                }
            }

            base = apply_overwrite_pair(base, allows, denies);

            if let Some(entry) = self.overwrite_for_member(member.id()) {
                base = apply_overwrite(base, &entry.overwrite);
            }

            if member.is_timed_out() {
                base = base.intersection(Permissions::timeout_mask());
            }
        }

        self.apply_channel_type_permissions(base)
    }

    fn base_permissions_for_role(&self, guild: &Guild, role: &Role) -> Permissions {
        let Some(default_role) = guild.default_role() else {
            return Permissions::none();
        };

        default_role.permissions().union(role.permissions())
    }

    fn base_permissions_for_member(&self, guild: &Guild, member: &Member) -> Permissions {
        let Some(default_role) = guild.default_role() else {
            return Permissions::none();
        };

        let mut base = default_role.permissions();
        for role in guild.roles_for(member) {
            base.insert(role.permissions());
        }
        base
    }

    fn apply_channel_type_permissions(&self, base: Permissions) -> Permissions {
        let mut base = base;

        if self.channel_type.is_textual() {
            base = apply_implicit_permissions(base);
            return base.difference(Permissions::voice());
        }

        if self.is_voice() || self.is_stage_voice() {
            base = apply_implicit_permissions(base);
            if !base.connect() {
                let denied = Permissions::voice()
                    .union(Permissions::MANAGE_CHANNELS)
                    .union(Permissions::MANAGE_ROLES);
                base = base.difference(denied);
            }
        }

        base
    }

    fn same_overwrites(&self, other: &[PermissionOverwriteEntry]) -> bool {
        if self.permission_overwrites.len() != other.len() {
            return false;
        }

        self.permission_overwrites
            .iter()
            .all(|entry| other.iter().any(|candidate| candidate == entry))
    }
}

fn apply_overwrite(base: Permissions, overwrite: &PermissionOverwrite) -> Permissions {
    apply_overwrite_pair(base, overwrite.allow(), overwrite.deny())
}

fn apply_overwrite_pair(base: Permissions, allow: Permissions, deny: Permissions) -> Permissions {
    base.difference(deny).union(allow)
}

fn apply_thread_permissions(mut base: Permissions) -> Permissions {
    if !base.contains(Permissions::SEND_MESSAGES_IN_THREADS) {
        base.remove(
            Permissions::SEND_TTS_MESSAGES
                | Permissions::MENTION_EVERYONE
                | Permissions::EMBED_LINKS
                | Permissions::ATTACH_FILES,
        );
    }

    if !base.contains(Permissions::VIEW_CHANNEL) {
        base = base.difference(Permissions::all_channel());
    }

    base
}

fn apply_implicit_permissions(mut base: Permissions) -> Permissions {
    if !base.send_messages() {
        base.remove(
            Permissions::SEND_TTS_MESSAGES
                | Permissions::MENTION_EVERYONE
                | Permissions::EMBED_LINKS
                | Permissions::ATTACH_FILES,
        );
    }

    if !base.read_messages() {
        base = base.difference(Permissions::all_channel());
    }

    base
}

impl fmt::Display for GuildChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl PartialEq for GuildChannel {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for GuildChannel {}

impl Hash for GuildChannel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for GuildChannel {
    fn id(&self) -> Snowflake {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use abc::{HasId, Snowflake};
    use data::json;
    use enums::{
        ChannelFlags, ChannelType, ForumLayout, ForumSortOrder, Permissions, VideoQualityMode,
    };

    use super::{
        GuildChannel, PermissionOverwrite, PermissionOverwriteEntry, PermissionOverwriteType,
    };
    use crate::{
        ForumTag, Guild, Member, PartialEmoji, Role, ThreadInfo, ThreadMember, ThreadMetadata, User,
    };

    fn role(guild_id: u64, id: u64, name: &str, position: i64) -> Role {
        Role::new(guild_id, id, name).with_position(position)
    }

    fn sample_guild() -> Guild {
        let everyone = role(100, 100, "@everyone", 0).with_permissions(Permissions::VIEW_CHANNEL);
        let member_role = role(100, 20, "Member", 2).with_permissions(Permissions::SEND_MESSAGES);
        let owner = Member::new(User::new(1_u64, "owner"), 100_u64);
        let member =
            Member::new(User::new(2_u64, "member"), 100_u64).with_roles(vec![member_role.clone()]);

        Guild::new(100_u64, "Test Guild")
            .with_owner_id(1_u64)
            .with_roles(vec![everyone, member_role])
            .with_members(vec![owner, member])
    }

    #[test]
    fn permission_overwrite_tristate() {
        let mut overwrite = PermissionOverwrite::new();
        assert!(overwrite.is_empty());
        assert_eq!(overwrite.get(Permissions::VIEW_CHANNEL), None);

        overwrite.set(Permissions::VIEW_CHANNEL, Some(true));
        overwrite.set(Permissions::SEND_MESSAGES, Some(false));

        assert_eq!(overwrite.get(Permissions::VIEW_CHANNEL), Some(true));
        assert_eq!(overwrite.get(Permissions::SEND_MESSAGES), Some(false));
        assert_eq!(overwrite.get(Permissions::MANAGE_GUILD), None);
        assert_eq!(overwrite.allow(), Permissions::VIEW_CHANNEL);
        assert_eq!(overwrite.deny(), Permissions::SEND_MESSAGES);
        assert_eq!(
            overwrite.pair(),
            (Permissions::VIEW_CHANNEL, Permissions::SEND_MESSAGES)
        );

        overwrite.set(Permissions::VIEW_CHANNEL, Some(false));
        assert_eq!(overwrite.get(Permissions::VIEW_CHANNEL), Some(false));
        assert_eq!(overwrite.allow(), Permissions::none());

        overwrite.set(Permissions::VIEW_CHANNEL, None);
        assert_eq!(overwrite.get(Permissions::VIEW_CHANNEL), None);

        overwrite.allow_all(Permissions::BAN_MEMBERS);
        overwrite.deny_all(Permissions::KICK_MEMBERS);
        assert_eq!(overwrite.get(Permissions::BAN_MEMBERS), Some(true));
        assert_eq!(overwrite.get(Permissions::KICK_MEMBERS), Some(false));

        overwrite.clear(Permissions::BAN_MEMBERS | Permissions::KICK_MEMBERS);
        assert_eq!(overwrite.get(Permissions::BAN_MEMBERS), None);
        assert_eq!(overwrite.get(Permissions::KICK_MEMBERS), None);
        assert_eq!(overwrite.get(Permissions::SEND_MESSAGES), Some(false));

        overwrite.clear(Permissions::SEND_MESSAGES);
        assert!(overwrite.is_empty());
    }

    #[test]
    fn overwrite_entries_and_types() {
        let overwrite =
            PermissionOverwrite::from_pair(Permissions::VIEW_CHANNEL, Permissions::SEND_MESSAGES);
        let role_entry = PermissionOverwriteEntry::role(Snowflake::new(20), overwrite);
        let member_entry = PermissionOverwriteEntry::member(Snowflake::new(2), overwrite);

        assert!(role_entry.is_role());
        assert!(!role_entry.is_member());
        assert!(role_entry.matches(Snowflake::new(20), PermissionOverwriteType::Role));
        assert!(!role_entry.matches(Snowflake::new(20), PermissionOverwriteType::Member));
        assert!(member_entry.is_member());

        assert_eq!(
            PermissionOverwriteType::from_value(0),
            Some(PermissionOverwriteType::Role)
        );
        assert_eq!(
            PermissionOverwriteType::from_value(1),
            Some(PermissionOverwriteType::Member)
        );
        assert_eq!(PermissionOverwriteType::from_value(2), None);
        assert_eq!(PermissionOverwriteType::Member.to_string(), "member");

        let mut channel = GuildChannel::new(10_u64, 100_u64, "general", ChannelType::Text);
        assert_eq!(channel.overwrites(), &[]);
        channel.set_overwrite(role_entry);
        channel.set_overwrite(member_entry);

        assert_eq!(channel.overwrites().len(), 2);
        assert!(channel.overwrite_for_role(20_u64).is_some());
        assert!(channel.overwrite_for_member(2_u64).is_some());

        let replacement = PermissionOverwriteEntry::role(
            Snowflake::new(20),
            PermissionOverwrite::from_pair(Permissions::SEND_MESSAGES, Permissions::none()),
        );
        channel.set_overwrite(replacement);
        assert_eq!(channel.overwrites().len(), 2);
        assert_eq!(
            channel
                .overwrite_for_role(20_u64)
                .unwrap()
                .overwrite
                .allow(),
            Permissions::SEND_MESSAGES
        );

        assert_eq!(
            channel
                .remove_overwrite(20_u64, PermissionOverwriteType::Role)
                .unwrap()
                .target_id,
            Snowflake::new(20)
        );
        assert!(channel.overwrite_for_role(20_u64).is_none());
        assert_eq!(channel.overwrites().len(), 1);
    }

    #[test]
    fn channel_builders_and_classification() {
        let channel = GuildChannel::new(10_u64, 100_u64, "general", ChannelType::Text)
            .with_position(3)
            .with_category_id(5_u64)
            .with_nsfw(true)
            .with_flags(ChannelFlags::PINNED)
            .with_topic("A topic")
            .with_slowmode_delay(10)
            .with_default_auto_archive_duration(60)
            .with_default_thread_slowmode_delay(5)
            .with_last_message_id(99_u64)
            .with_bitrate(128_000)
            .with_user_limit(10)
            .with_rtc_region("us-east")
            .with_video_quality_mode(VideoQualityMode::Full)
            .with_status("Status")
            .with_default_layout(ForumLayout::GalleryView)
            .with_default_sort_order(ForumSortOrder::CreationDate);

        assert_eq!(channel.id, Snowflake::new(10));
        assert_eq!(channel.id(), Snowflake::new(10));
        assert_eq!(channel.guild_id, Snowflake::new(100));
        assert_eq!(channel.channel_type(), ChannelType::Text);
        assert_eq!(channel.position, 3);
        assert_eq!(channel.category_id, Some(Snowflake::new(5)));
        assert!(channel.is_nsfw());
        assert!(channel.flags.pinned());
        assert_eq!(channel.topic.as_deref(), Some("A topic"));
        assert_eq!(channel.slowmode_delay, 10);
        assert_eq!(channel.default_auto_archive_duration, 60);
        assert_eq!(channel.default_thread_slowmode_delay, 5);
        assert_eq!(channel.last_message_id, Some(Snowflake::new(99)));
        assert_eq!(channel.bitrate, 128_000);
        assert_eq!(channel.user_limit, 10);
        assert_eq!(channel.rtc_region.as_deref(), Some("us-east"));
        assert_eq!(channel.video_quality_mode, VideoQualityMode::Full);
        assert_eq!(channel.status.as_deref(), Some("Status"));
        assert_eq!(channel.default_layout, ForumLayout::GalleryView);
        assert_eq!(channel.default_sort_order, ForumSortOrder::CreationDate);

        assert!(channel.is_text());
        assert!(!channel.is_voice());
        assert!(!channel.is_category());
        assert!(!channel.is_forum());
        assert!(!channel.is_media());
        assert!(!channel.is_thread());
        assert_eq!(channel.mention(), "<#10>");
        assert_eq!(channel.jump_url(), "https://discord.com/channels/100/10");
        assert_eq!(channel.to_string(), "general");
        assert_eq!(channel.created_at(), Snowflake::new(10).created_at());
    }

    #[test]
    fn channel_type_helpers() {
        let voice = GuildChannel::new(10_u64, 100_u64, "voice", ChannelType::Voice);
        let stage = GuildChannel::new(11_u64, 100_u64, "stage", ChannelType::StageVoice);
        let category = GuildChannel::new(12_u64, 100_u64, "category", ChannelType::Category);
        let forum = GuildChannel::new(13_u64, 100_u64, "forum", ChannelType::Forum);
        let media = GuildChannel::new(14_u64, 100_u64, "media", ChannelType::Media);
        let thread = GuildChannel::new(15_u64, 100_u64, "thread", ChannelType::PublicThread);
        let news = GuildChannel::new(16_u64, 100_u64, "news", ChannelType::News);

        assert!(voice.is_voice());
        assert!(stage.is_stage_voice());
        assert!(category.is_category());
        assert!(forum.is_forum());
        assert!(media.is_media());
        assert!(thread.is_thread());
        assert!(news.is_news());
    }

    #[test]
    fn category_and_sync_helpers() {
        let category = GuildChannel::new(5_u64, 100_u64, "Category", ChannelType::Category)
            .with_permission_overwrites(vec![PermissionOverwriteEntry::role(
                Snowflake::new(20),
                PermissionOverwrite::from_pair(Permissions::VIEW_CHANNEL, Permissions::none()),
            )]);
        let channel = GuildChannel::new(10_u64, 100_u64, "general", ChannelType::Text)
            .with_category_id(5_u64)
            .with_permission_overwrites(category.overwrites().to_vec());
        let guild = sample_guild().with_channels(vec![category.clone(), channel]);

        let channel = guild.get_channel(10_u64).unwrap();
        assert_eq!(
            channel
                .category(&guild)
                .map(|category| category.name.as_str()),
            Some("Category")
        );
        assert!(channel.permissions_synced(&guild));

        let unsynced =
            GuildChannel::new(11_u64, 100_u64, "other", ChannelType::Text).with_category_id(5_u64);
        let guild = guild.with_channels(vec![category, unsynced]);
        assert!(
            !guild
                .get_channel(11_u64)
                .unwrap()
                .permissions_synced(&guild)
        );

        let no_category = GuildChannel::new(12_u64, 100_u64, "bare", ChannelType::Text);
        let guild = guild.with_channels(vec![no_category]);
        assert!(
            !guild
                .get_channel(12_u64)
                .unwrap()
                .permissions_synced(&guild)
        );
    }

    #[test]
    fn resolves_member_permissions() {
        let guild = sample_guild();
        let member = guild.get_member(2_u64).unwrap();
        let channel = GuildChannel::new(10_u64, 100_u64, "general", ChannelType::Text);

        let permissions = channel.permissions_for(&guild, member);
        assert!(permissions.view_channel());
        assert!(permissions.send_messages());

        let member_role = role(100, 20, "Member", 2)
            .with_permissions(Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES);
        let guild = sample_guild().with_roles(vec![role(100, 100, "@everyone", 0), member_role]);
        let member = guild.get_member(2_u64).unwrap();

        let channel = channel.with_permission_overwrites(vec![
            PermissionOverwriteEntry::role(
                Snowflake::new(100),
                PermissionOverwrite::from_pair(Permissions::none(), Permissions::MANAGE_GUILD),
            ),
            PermissionOverwriteEntry::role(
                Snowflake::new(20),
                PermissionOverwrite::from_pair(
                    Permissions::READ_MESSAGE_HISTORY,
                    Permissions::SEND_MESSAGES,
                ),
            ),
            PermissionOverwriteEntry::member(
                Snowflake::new(2),
                PermissionOverwrite::from_pair(Permissions::BAN_MEMBERS, Permissions::none()),
            ),
        ]);

        let permissions = channel.permissions_for(&guild, member);
        assert!(permissions.view_channel());
        assert!(!permissions.send_messages());
        assert!(permissions.read_message_history());
        assert!(permissions.ban_members());
        assert!(!permissions.manage_guild());

        let owner = guild.get_member(1_u64).unwrap();
        let owner_permissions = channel.permissions_for(&guild, owner);
        assert!(owner_permissions.view_channel());
        assert!(owner_permissions.send_messages());
        assert!(!owner_permissions.connect());

        let admin = role(100, 30, "Admin", 3)
            .with_permissions(Permissions::ADMINISTRATOR | Permissions::VIEW_CHANNEL);
        let guild = guild.with_roles(vec![role(100, 100, "@everyone", 0), admin]);
        let guild = guild.with_members(vec![
            Member::new(User::new(2_u64, "member"), 100_u64)
                .with_roles(vec![role(100, 30, "Admin", 3)]),
        ]);
        let member = guild.get_member(2_u64).unwrap();
        let permissions = channel.permissions_for(&guild, member);
        assert!(permissions.administrator());
        assert!(permissions.manage_guild());
    }

    #[test]
    fn thread_channels_expose_state_and_inherit_permissions() {
        let timestamp = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000);
        let category = GuildChannel::new(90_u64, 100_u64, "category", ChannelType::Category);
        let parent = GuildChannel::new(10_u64, 100_u64, "general", ChannelType::Text)
            .with_category_id(90_u64)
            .with_permission_overwrites(vec![PermissionOverwriteEntry::role(
                Snowflake::new(20),
                PermissionOverwrite::from_pair(Permissions::none(), Permissions::SEND_MESSAGES),
            )]);
        let member =
            ThreadMember::new(2_u64, 11_u64, timestamp).with_flags(enums::ThreadMemberFlags::IN);
        let info = ThreadInfo::new(10_u64, 1_u64)
            .with_metadata(
                ThreadMetadata::new(ThreadMetadata::ONE_HOUR, timestamp)
                    .with_archived(true)
                    .with_locked(true),
            )
            .with_counts(3, 1)
            .with_members(vec![member.clone()]);
        let thread = GuildChannel::new(11_u64, 100_u64, "topic", ChannelType::PublicThread)
            .with_thread_info(info);

        let guild = sample_guild().with_channels(vec![category, parent, thread]);
        let thread = guild.get_channel(11_u64).unwrap();

        assert!(thread.is_thread());
        assert!(thread.is_public_thread());
        assert!(!thread.is_private_thread());
        assert!(!thread.is_news_thread());
        assert_eq!(thread.thread_parent_id(), Some(Snowflake::new(10)));
        assert_eq!(thread.thread_owner_id(), Some(Snowflake::new(1)));
        assert!(thread.is_archived());
        assert!(thread.is_locked());
        assert!(thread.is_invitable());
        assert_eq!(thread.thread_members(), &[member]);
        assert_eq!(
            thread.thread_member(2_u64).map(|member| member.joined_at),
            Some(timestamp)
        );
        assert!(thread.thread_member(3_u64).is_none());
        assert_eq!(
            thread.thread_parent(&guild).map(|parent| parent.id),
            Some(Snowflake::new(10))
        );
        assert_eq!(
            thread.category(&guild).map(|category| category.id),
            Some(Snowflake::new(90))
        );
        assert_eq!(
            thread
                .thread_metadata()
                .map(|metadata| metadata.auto_archive_duration),
            Some(ThreadMetadata::ONE_HOUR)
        );

        let member = guild.get_member(2_u64).unwrap();
        let permissions = thread.permissions_for(&guild, member);
        assert!(permissions.view_channel());
        assert!(!permissions.send_messages());
        assert!(!permissions.embed_links());

        let role = guild.get_role(20_u64).unwrap();
        let permissions = thread.permissions_for_role(&guild, role);
        assert!(permissions.view_channel());
        assert!(!permissions.send_messages());

        let plain = GuildChannel::new(12_u64, 100_u64, "plain", ChannelType::Text);
        assert!(!plain.is_archived());
        assert!(!plain.is_locked());
        assert!(!plain.is_invitable());
        assert!(plain.thread_members().is_empty());
        assert!(plain.thread_info().is_none());
        assert_eq!(plain.thread_parent(&guild), None);
        assert_eq!(plain.thread_parent_id(), None);
        assert_eq!(plain.thread_owner_id(), None);
    }

    #[test]
    fn forum_tags_and_thread_applied_tags() {
        let bug = ForumTag::new("Bug")
            .with_id(1_u64)
            .with_emoji(PartialEmoji::unicode("🐛"));
        let question = ForumTag::new("Question").with_id(2_u64);
        let forum = GuildChannel::new(10_u64, 100_u64, "forum", ChannelType::Forum)
            .with_available_tags(vec![bug.clone()])
            .add_available_tag(question.clone());

        assert!(forum.supports_tags());
        assert!(!forum.is_thread());
        assert_eq!(forum.available_tags(), &[bug.clone(), question]);
        assert_eq!(
            forum.available_tag(1_u64).map(|tag| tag.name.as_str()),
            Some("Bug")
        );
        assert!(forum.has_tag(2_u64));
        assert!(!forum.has_tag(3_u64));
        assert!(forum.available_tag(3_u64).is_none());

        let thread = GuildChannel::new(11_u64, 100_u64, "topic", ChannelType::PublicThread)
            .with_thread_info(
                ThreadInfo::new(10_u64, 1_u64)
                    .add_applied_tag(1_u64)
                    .add_applied_tag(3_u64),
            );
        let guild = sample_guild().with_channels(vec![forum, thread]);
        let thread = guild.get_channel(11_u64).unwrap();

        assert_eq!(
            thread.applied_tag_ids(),
            &[Snowflake::new(1), Snowflake::new(3)]
        );
        let applied = thread.applied_tags(&guild);
        assert_eq!(applied.len(), 1);
        assert_eq!(applied[0].name, "Bug");
        assert_eq!(applied[0].to_string(), "Bug");

        let plain = GuildChannel::new(12_u64, 100_u64, "plain", ChannelType::Text);
        assert!(!plain.supports_tags());
        assert!(plain.available_tags().is_empty());
        assert!(plain.applied_tag_ids().is_empty());
        assert!(plain.applied_tags(&guild).is_empty());
    }

    #[test]
    fn timeout_masks_member_permissions() {
        let guild = sample_guild();
        let member_role = role(100, 20, "Member", 2).with_permissions(
            Permissions::VIEW_CHANNEL
                | Permissions::READ_MESSAGE_HISTORY
                | Permissions::SEND_MESSAGES
                | Permissions::BAN_MEMBERS,
        );
        let guild = guild.with_roles(vec![role(100, 100, "@everyone", 0), member_role.clone()]);
        let member = Member::new(User::new(2_u64, "member"), 100_u64)
            .with_roles(vec![member_role])
            .with_timed_out_until(SystemTime::now() + Duration::from_secs(60));
        let guild = guild.with_members(vec![member]);
        let member = guild.get_member(2_u64).unwrap();
        let channel = GuildChannel::new(10_u64, 100_u64, "general", ChannelType::Text);

        let permissions = channel.permissions_for(&guild, member);
        assert!(permissions.view_channel());
        assert!(permissions.read_message_history());
        assert!(!permissions.send_messages());
        assert!(!permissions.ban_members());
    }

    #[test]
    fn resolves_role_permissions() {
        let guild = sample_guild();
        let member_role = guild.get_role(20_u64).unwrap();
        let channel = GuildChannel::new(10_u64, 100_u64, "general", ChannelType::Text)
            .with_permission_overwrites(vec![PermissionOverwriteEntry::role(
                Snowflake::new(20),
                PermissionOverwrite::from_pair(Permissions::none(), Permissions::SEND_MESSAGES),
            )]);

        let permissions = channel.permissions_for_role(&guild, member_role);
        assert!(permissions.view_channel());
        assert!(!permissions.send_messages());

        let everyone = guild.default_role().unwrap();
        let permissions = channel.permissions_for_role(&guild, everyone);
        assert!(permissions.view_channel());
        assert!(!permissions.send_messages());
    }

    #[test]
    fn voice_and_category_permissions() {
        let guild = sample_guild();
        let member = guild.get_member(2_u64).unwrap();

        let voice = GuildChannel::new(10_u64, 100_u64, "voice", ChannelType::Voice);
        let permissions = voice.permissions_for(&guild, member);
        assert!(permissions.view_channel());
        assert!(!permissions.connect());
        assert!(!permissions.manage_channels());
        assert!(!permissions.manage_roles());

        let category = GuildChannel::new(11_u64, 100_u64, "category", ChannelType::Category);
        let permissions = category.permissions_for(&guild, member);
        assert!(permissions.view_channel());
        assert!(permissions.send_messages());
    }

    #[test]
    fn equality_and_hash_use_id_only() {
        let a = GuildChannel::new(1_u64, 100_u64, "First", ChannelType::Text);
        let b = GuildChannel::new(1_u64, 100_u64, "Second", ChannelType::Voice).with_position(9);
        let c = GuildChannel::new(2_u64, 100_u64, "First", ChannelType::Text);

        assert_eq!(a, b);
        assert_ne!(a, c);

        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        a.hash(&mut hasher_a);
        b.hash(&mut hasher_b);
        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }

    #[test]
    fn decodes_permission_overwrite_objects() {
        let role = json::from_str(r#"{"id":"20","type":0,"allow":"1024","deny":"8"}"#).unwrap();
        let overwrite = PermissionOverwriteEntry::from_value(&role).unwrap();
        assert_eq!(overwrite.target_id, Snowflake::new(20));
        assert!(overwrite.is_role());
        assert_eq!(overwrite.overwrite.allow(), Permissions::VIEW_CHANNEL);
        assert_eq!(overwrite.overwrite.deny(), Permissions::ADMINISTRATOR);

        let member = json::from_str(r#"{"id":2,"type":1}"#).unwrap();
        let overwrite = PermissionOverwriteEntry::from_value(&member).unwrap();
        assert!(overwrite.is_member());
        assert!(overwrite.overwrite.is_empty());

        let unknown = json::from_str(r#"{"id":"20","type":9}"#).unwrap();
        let error = PermissionOverwriteEntry::from_value(&unknown).unwrap_err();
        assert_eq!(
            error.to_string(),
            "field `overwrite.type` must be a permission overwrite type"
        );

        let missing = json::from_str(r#"{"type":0}"#).unwrap();
        let error = PermissionOverwriteEntry::from_value(&missing).unwrap_err();
        assert_eq!(error.to_string(), "missing field `overwrite.id`");
    }

    #[test]
    fn decodes_a_text_channel_payload() {
        let value = json::from_str(
            r#"{
                "id": "10",
                "guild_id": "100",
                "name": "general",
                "type": 0,
                "position": 3,
                "parent_id": "5",
                "permission_overwrites": [
                    {"id": "20", "type": 0, "allow": "1024", "deny": "8"}
                ],
                "nsfw": true,
                "flags": 2,
                "topic": "hello",
                "rate_limit_per_user": 10,
                "default_auto_archive_duration": 4320,
                "default_thread_rate_limit_per_user": 5,
                "last_message_id": "99",
                "bitrate": 128000,
                "user_limit": 25,
                "rtc_region": "us-east",
                "video_quality_mode": 2,
                "status": "live",
                "default_forum_layout": 1,
                "default_sort_order": 1
            }"#,
        )
        .unwrap();

        let channel = GuildChannel::from_value(&value, 1_u64).unwrap();

        assert_eq!(channel.id, Snowflake::new(10));
        assert_eq!(channel.guild_id, Snowflake::new(100));
        assert_eq!(channel.name, "general");
        assert_eq!(channel.channel_type, ChannelType::Text);
        assert_eq!(channel.position, 3);
        assert_eq!(channel.category_id, Some(Snowflake::new(5)));
        assert_eq!(channel.permission_overwrites.len(), 1);
        assert!(channel.nsfw);
        assert_eq!(channel.flags, ChannelFlags::PINNED);
        assert_eq!(channel.topic.as_deref(), Some("hello"));
        assert_eq!(channel.slowmode_delay, 10);
        assert_eq!(channel.default_auto_archive_duration, 4320);
        assert_eq!(channel.default_thread_slowmode_delay, 5);
        assert_eq!(channel.last_message_id, Some(Snowflake::new(99)));
        assert_eq!(channel.bitrate, 128_000);
        assert_eq!(channel.user_limit, 25);
        assert_eq!(channel.rtc_region.as_deref(), Some("us-east"));
        assert_eq!(channel.video_quality_mode, VideoQualityMode::Full);
        assert_eq!(channel.status.as_deref(), Some("live"));
        assert_eq!(channel.default_layout, ForumLayout::ListView);
        assert_eq!(channel.default_sort_order, ForumSortOrder::CreationDate);
        assert_eq!(channel.thread, None);
        assert!(channel.is_text());
    }

    #[test]
    fn decodes_a_thread_channel_payload() {
        let value = json::from_str(
            r#"{
                "id": "11",
                "guild_id": "100",
                "name": "topic",
                "type": 11,
                "parent_id": "10",
                "thread_metadata": {
                    "archived": true,
                    "auto_archive_duration": 60,
                    "archive_timestamp": "2021-08-01T12:34:56Z",
                    "archiver_id": "1",
                    "locked": true,
                    "invitable": false,
                    "create_timestamp": "2021-07-01T00:00:00Z"
                },
                "owner_id": "2",
                "message_count": 3,
                "member_count": 1,
                "total_message_sent": 4,
                "applied_tags": ["7"],
                "members": [
                    {
                        "id": "11",
                        "user_id": "2",
                        "join_timestamp": "2021-08-02T00:00:00Z",
                        "flags": 1
                    }
                ],
                "thread_member": {
                    "id": "11",
                    "user_id": "3",
                    "join_timestamp": "2021-08-03T00:00:00Z"
                },
                "last_pin_timestamp": "2021-08-04T00:00:00Z",
                "newly_created": true
            }"#,
        )
        .unwrap();

        let channel = GuildChannel::from_value(&value, 1_u64).unwrap();

        assert_eq!(channel.channel_type, ChannelType::PublicThread);
        assert_eq!(channel.category_id, None);
        assert!(channel.is_thread());
        assert!(channel.is_public_thread());

        let thread = channel.thread.as_ref().unwrap();
        assert_eq!(thread.parent_id, Snowflake::new(10));
        assert_eq!(thread.owner_id, Snowflake::new(2));
        assert!(thread.metadata.archived);
        assert!(thread.metadata.locked);
        assert!(!thread.metadata.invitable);
        assert_eq!(thread.metadata.auto_archive_duration, 60);
        assert_eq!(
            thread.metadata.archive_timestamp,
            UNIX_EPOCH + Duration::new(1_627_821_296, 0)
        );
        assert_eq!(thread.metadata.archiver_id, Some(Snowflake::new(1)));
        assert_eq!(
            thread.metadata.create_timestamp,
            Some(UNIX_EPOCH + Duration::new(1_625_097_600, 0))
        );
        assert_eq!(thread.message_count, 3);
        assert_eq!(thread.member_count, 1);
        assert_eq!(thread.total_message_sent, 4);
        assert_eq!(thread.applied_tags, [Snowflake::new(7)]);
        assert_eq!(thread.members.len(), 1);
        assert_eq!(thread.members[0].id, Snowflake::new(2));
        assert_eq!(thread.members[0].thread_id, Snowflake::new(11));
        assert_eq!(
            thread.members[0].joined_at,
            UNIX_EPOCH + Duration::new(1_627_862_400, 0)
        );
        assert!(thread.members[0].is_in_thread());
        assert_eq!(thread.me.as_ref().map(|me| me.id), Some(Snowflake::new(3)));
        assert_eq!(
            thread.last_pin_timestamp,
            Some(UNIX_EPOCH + Duration::new(1_628_035_200, 0))
        );
        assert!(thread.newly_created);
        assert_eq!(channel.thread_parent_id(), Some(Snowflake::new(10)));
    }

    #[test]
    fn decodes_forum_channels_and_tags() {
        let value = json::from_str(
            r#"{
                "id": "20",
                "guild_id": "100",
                "name": "forum",
                "type": 15,
                "default_forum_layout": 2,
                "default_sort_order": 1,
                "available_tags": [
                    {
                        "id": "7",
                        "name": "Bug",
                        "moderated": true,
                        "emoji": {"id": "8", "name": "bug", "animated": true}
                    },
                    {
                        "id": "9",
                        "name": "Idea",
                        "emoji": {"name": "💡"}
                    }
                ]
            }"#,
        )
        .unwrap();

        let channel = GuildChannel::from_value(&value, 1_u64).unwrap();

        assert!(channel.is_forum());
        assert_eq!(channel.default_layout, ForumLayout::GalleryView);
        assert_eq!(channel.default_sort_order, ForumSortOrder::CreationDate);
        assert_eq!(channel.available_tags.len(), 2);
        assert_eq!(channel.available_tags[0].name, "Bug");
        assert!(channel.available_tags[0].moderated);
        let emoji = channel.available_tags[0].emoji.as_ref().unwrap();
        assert_eq!(emoji.id, Some(Snowflake::new(8)));
        assert_eq!(emoji.name.as_deref(), Some("bug"));
        assert!(emoji.animated);
        assert!(channel.has_tag(9_u64));
        assert_eq!(
            channel.available_tags[1]
                .emoji
                .as_ref()
                .and_then(|emoji| emoji.name.as_deref()),
            Some("💡")
        );
    }

    #[test]
    fn rejects_malformed_channel_payloads() {
        let missing_type = json::from_str(r#"{"id":"10","name":"general"}"#).unwrap();
        let error = GuildChannel::from_value(&missing_type, 1_u64).unwrap_err();
        assert_eq!(error.to_string(), "missing field `channel.type`");

        let unknown_type = json::from_str(r#"{"id":"10","name":"general","type":99}"#).unwrap();
        let error = GuildChannel::from_value(&unknown_type, 1_u64).unwrap_err();
        assert_eq!(
            error.to_string(),
            "field `channel.type` must be a channel type"
        );

        let bad_overwrite = json::from_str(
            r#"{"id":"10","name":"general","type":0,"permission_overwrites":[{"id":"1","type":"role"}]}"#,
        )
        .unwrap();
        let error = GuildChannel::from_value(&bad_overwrite, 1_u64).unwrap_err();
        assert_eq!(
            error.to_string(),
            "field `channel.permission_overwrites[0].type` must be a non-negative integer"
        );

        let bad_thread = json::from_str(
            r#"{"id":"10","name":"thread","type":11,"thread_metadata":{"archived":true}}"#,
        )
        .unwrap();
        let error = GuildChannel::from_value(&bad_thread, 1_u64).unwrap_err();
        assert_eq!(
            error.to_string(),
            "missing field `channel.thread_metadata.archive_timestamp`"
        );
    }
}
