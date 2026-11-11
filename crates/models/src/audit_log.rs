//! Audit log models.
//!
//! Audit logs record the actions taken in a guild. Each [`AuditLogEntry`]
//! describes one action, its target and the changes it made. Related users,
//! webhooks, threads and scheduled events are returned alongside the entries
//! in an [`AuditLog`].

use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

use abc::{HasId, Snowflake};
use enums::{AuditLogAction, AuditLogActionCategory, AuditLogActionTargetType};

use crate::{
    ForumTag, GuildChannel, GuildScheduledEvent, PermissionOverwrite, Role, User, Webhook,
};

/// The key naming a field changed by an audit log entry.
///
/// Unrecognised keys are preserved with [`AuditLogChangeKey::Unknown`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AuditLogChangeKey {
    /// The name of an object.
    Name,
    /// A description.
    Description,
    /// A preferred locale.
    PreferredLocale,
    /// A vanity URL code.
    VanityUrlCode,
    /// A channel topic.
    Topic,
    /// An invite code.
    Code,
    /// An allowed permission overwrite.
    Allow,
    /// A denied permission overwrite.
    Deny,
    /// A permission set.
    Permissions,
    /// A list of tags.
    Tags,
    /// A unicode emoji.
    UnicodeEmoji,
    /// An emoji name.
    EmojiName,
    /// A title.
    Title,
    /// A guild icon hash.
    IconHash,
    /// A guild splash hash.
    SplashHash,
    /// A guild discovery splash hash.
    DiscoverySplashHash,
    /// A guild banner hash.
    BannerHash,
    /// A user or webhook avatar hash.
    AvatarHash,
    /// An asset hash.
    Asset,
    /// An object ID.
    Id,
    /// A guild owner ID.
    OwnerId,
    /// An AFK channel ID.
    AfkChannelId,
    /// A rules channel ID.
    RulesChannelId,
    /// A public updates channel ID.
    PublicUpdatesChannelId,
    /// A widget channel ID.
    WidgetChannelId,
    /// A system channel ID.
    SystemChannelId,
    /// An application ID.
    ApplicationId,
    /// A channel ID.
    ChannelId,
    /// An invite inviter ID.
    InviterId,
    /// A guild ID.
    GuildId,
    /// A user ID.
    UserId,
    /// A soundboard sound ID.
    SoundId,
    /// Whether the widget is enabled.
    WidgetEnabled,
    /// Whether a channel is NSFW.
    Nsfw,
    /// Whether a role is hoisted.
    Hoist,
    /// Whether a role is mentionable.
    Mentionable,
    /// Whether a member is temporary.
    Temporary,
    /// Whether a member is deafened.
    Deaf,
    /// Whether a member is muted.
    Mute,
    /// A member nickname.
    Nick,
    /// Whether enabled emoticons are allowed.
    EnabledEmoticons,
    /// A voice region.
    Region,
    /// A voice channel RTC region.
    RtcRegion,
    /// Whether an object is available.
    Available,
    /// Whether a thread is archived.
    Archived,
    /// Whether a thread is locked.
    Locked,
    /// Whether an object is enabled.
    Enabled,
    /// Whether a select menu is single-select.
    SingleSelect,
    /// Whether an object is required.
    Required,
    /// Whether a member is in onboarding.
    InOnboarding,
    /// An AFK timeout.
    AfkTimeout,
    /// The number of days after which inactive members are pruned.
    PruneDeleteDays,
    /// A position in a list.
    Position,
    /// A voice channel bitrate.
    Bitrate,
    /// A channel rate limit.
    RateLimitPerUser,
    /// A role colour.
    Color,
    /// The maximum number of invite uses.
    MaxUses,
    /// The maximum age of an invite.
    MaxAge,
    /// A voice channel user limit.
    UserLimit,
    /// A thread auto-archive duration.
    AutoArchiveDuration,
    /// A guild default thread auto-archive duration.
    DefaultAutoArchiveDuration,
    /// A default thread rate limit.
    DefaultThreadRateLimitPerUser,
    /// When a member's communication is disabled until.
    CommunicationDisabledUntil,
    /// An object's flags.
    Flags,
    /// A voice channel volume.
    Volume,
    /// Roles added to a member.
    Add,
    /// Roles removed from a member.
    Remove,
    /// A guild MFA level.
    MfaLevel,
    /// A guild verification level.
    VerificationLevel,
    /// A guild explicit content filter level.
    ExplicitContentFilter,
    /// A guild default message notification level.
    DefaultMessageNotifications,
    /// A channel type.
    Type,
    /// An integration expire behaviour.
    ExpireBehavior,
    /// A voice channel video quality mode.
    VideoQualityMode,
    /// A channel's permission overwrites.
    PermissionOverwrites,
    /// A scheduled event privacy level.
    PrivacyLevel,
    /// A scheduled event status.
    Status,
    /// A scheduled event entity type.
    EntityType,
    /// Tags applied to a thread.
    AppliedTags,
    /// Default channel IDs for onboarding.
    DefaultChannelIds,
    /// The tags available in a forum or media channel.
    AvailableTags,
    /// A forum channel's default reaction emoji.
    DefaultReactionEmoji,
    /// An AutoMod trigger's metadata.
    TriggerMetadata,
    /// Onboarding prompts.
    Prompts,
    /// Onboarding prompt options.
    Options,
    /// A role's colours.
    Colors,
    /// A key this library does not recognise.
    Unknown(String),
}

impl AuditLogChangeKey {
    /// Converts an API key to a known key, preserving unrecognised keys.
    #[must_use]
    pub fn from_key(key: &str) -> Self {
        match key {
            "name" => Self::Name,
            "description" => Self::Description,
            "preferred_locale" => Self::PreferredLocale,
            "vanity_url_code" => Self::VanityUrlCode,
            "topic" => Self::Topic,
            "code" => Self::Code,
            "allow" => Self::Allow,
            "deny" => Self::Deny,
            "permissions" => Self::Permissions,
            "tags" => Self::Tags,
            "unicode_emoji" => Self::UnicodeEmoji,
            "emoji_name" => Self::EmojiName,
            "title" => Self::Title,
            "icon_hash" => Self::IconHash,
            "splash_hash" => Self::SplashHash,
            "discovery_splash_hash" => Self::DiscoverySplashHash,
            "banner_hash" => Self::BannerHash,
            "avatar_hash" => Self::AvatarHash,
            "asset" => Self::Asset,
            "id" => Self::Id,
            "owner_id" => Self::OwnerId,
            "afk_channel_id" => Self::AfkChannelId,
            "rules_channel_id" => Self::RulesChannelId,
            "public_updates_channel_id" => Self::PublicUpdatesChannelId,
            "widget_channel_id" => Self::WidgetChannelId,
            "system_channel_id" => Self::SystemChannelId,
            "application_id" => Self::ApplicationId,
            "channel_id" => Self::ChannelId,
            "inviter_id" => Self::InviterId,
            "guild_id" => Self::GuildId,
            "user_id" => Self::UserId,
            "sound_id" => Self::SoundId,
            "widget_enabled" => Self::WidgetEnabled,
            "nsfw" => Self::Nsfw,
            "hoist" => Self::Hoist,
            "mentionable" => Self::Mentionable,
            "temporary" => Self::Temporary,
            "deaf" => Self::Deaf,
            "mute" => Self::Mute,
            "nick" => Self::Nick,
            "enabled_emoticons" => Self::EnabledEmoticons,
            "region" => Self::Region,
            "rtc_region" => Self::RtcRegion,
            "available" => Self::Available,
            "archived" => Self::Archived,
            "locked" => Self::Locked,
            "enabled" => Self::Enabled,
            "single_select" => Self::SingleSelect,
            "required" => Self::Required,
            "in_onboarding" => Self::InOnboarding,
            "afk_timeout" => Self::AfkTimeout,
            "prune_delete_days" => Self::PruneDeleteDays,
            "position" => Self::Position,
            "bitrate" => Self::Bitrate,
            "rate_limit_per_user" => Self::RateLimitPerUser,
            "color" => Self::Color,
            "max_uses" => Self::MaxUses,
            "max_age" => Self::MaxAge,
            "user_limit" => Self::UserLimit,
            "auto_archive_duration" => Self::AutoArchiveDuration,
            "default_auto_archive_duration" => Self::DefaultAutoArchiveDuration,
            "default_thread_rate_limit_per_user" => Self::DefaultThreadRateLimitPerUser,
            "communication_disabled_until" => Self::CommunicationDisabledUntil,
            "flags" => Self::Flags,
            "volume" => Self::Volume,
            "$add" => Self::Add,
            "$remove" => Self::Remove,
            "mfa_level" => Self::MfaLevel,
            "verification_level" => Self::VerificationLevel,
            "explicit_content_filter" => Self::ExplicitContentFilter,
            "default_message_notifications" => Self::DefaultMessageNotifications,
            "type" => Self::Type,
            "expire_behavior" => Self::ExpireBehavior,
            "video_quality_mode" => Self::VideoQualityMode,
            "permission_overwrites" => Self::PermissionOverwrites,
            "privacy_level" => Self::PrivacyLevel,
            "status" => Self::Status,
            "entity_type" => Self::EntityType,
            "applied_tags" => Self::AppliedTags,
            "default_channel_ids" => Self::DefaultChannelIds,
            "available_tags" => Self::AvailableTags,
            "default_reaction_emoji" => Self::DefaultReactionEmoji,
            "trigger_metadata" => Self::TriggerMetadata,
            "prompts" => Self::Prompts,
            "options" => Self::Options,
            "colors" => Self::Colors,
            other => Self::Unknown(other.to_owned()),
        }
    }

    /// Returns the key used by the API.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Name => "name",
            Self::Description => "description",
            Self::PreferredLocale => "preferred_locale",
            Self::VanityUrlCode => "vanity_url_code",
            Self::Topic => "topic",
            Self::Code => "code",
            Self::Allow => "allow",
            Self::Deny => "deny",
            Self::Permissions => "permissions",
            Self::Tags => "tags",
            Self::UnicodeEmoji => "unicode_emoji",
            Self::EmojiName => "emoji_name",
            Self::Title => "title",
            Self::IconHash => "icon_hash",
            Self::SplashHash => "splash_hash",
            Self::DiscoverySplashHash => "discovery_splash_hash",
            Self::BannerHash => "banner_hash",
            Self::AvatarHash => "avatar_hash",
            Self::Asset => "asset",
            Self::Id => "id",
            Self::OwnerId => "owner_id",
            Self::AfkChannelId => "afk_channel_id",
            Self::RulesChannelId => "rules_channel_id",
            Self::PublicUpdatesChannelId => "public_updates_channel_id",
            Self::WidgetChannelId => "widget_channel_id",
            Self::SystemChannelId => "system_channel_id",
            Self::ApplicationId => "application_id",
            Self::ChannelId => "channel_id",
            Self::InviterId => "inviter_id",
            Self::GuildId => "guild_id",
            Self::UserId => "user_id",
            Self::SoundId => "sound_id",
            Self::WidgetEnabled => "widget_enabled",
            Self::Nsfw => "nsfw",
            Self::Hoist => "hoist",
            Self::Mentionable => "mentionable",
            Self::Temporary => "temporary",
            Self::Deaf => "deaf",
            Self::Mute => "mute",
            Self::Nick => "nick",
            Self::EnabledEmoticons => "enabled_emoticons",
            Self::Region => "region",
            Self::RtcRegion => "rtc_region",
            Self::Available => "available",
            Self::Archived => "archived",
            Self::Locked => "locked",
            Self::Enabled => "enabled",
            Self::SingleSelect => "single_select",
            Self::Required => "required",
            Self::InOnboarding => "in_onboarding",
            Self::AfkTimeout => "afk_timeout",
            Self::PruneDeleteDays => "prune_delete_days",
            Self::Position => "position",
            Self::Bitrate => "bitrate",
            Self::RateLimitPerUser => "rate_limit_per_user",
            Self::Color => "color",
            Self::MaxUses => "max_uses",
            Self::MaxAge => "max_age",
            Self::UserLimit => "user_limit",
            Self::AutoArchiveDuration => "auto_archive_duration",
            Self::DefaultAutoArchiveDuration => "default_auto_archive_duration",
            Self::DefaultThreadRateLimitPerUser => "default_thread_rate_limit_per_user",
            Self::CommunicationDisabledUntil => "communication_disabled_until",
            Self::Flags => "flags",
            Self::Volume => "volume",
            Self::Add => "$add",
            Self::Remove => "$remove",
            Self::MfaLevel => "mfa_level",
            Self::VerificationLevel => "verification_level",
            Self::ExplicitContentFilter => "explicit_content_filter",
            Self::DefaultMessageNotifications => "default_message_notifications",
            Self::Type => "type",
            Self::ExpireBehavior => "expire_behavior",
            Self::VideoQualityMode => "video_quality_mode",
            Self::PermissionOverwrites => "permission_overwrites",
            Self::PrivacyLevel => "privacy_level",
            Self::Status => "status",
            Self::EntityType => "entity_type",
            Self::AppliedTags => "applied_tags",
            Self::DefaultChannelIds => "default_channel_ids",
            Self::AvailableTags => "available_tags",
            Self::DefaultReactionEmoji => "default_reaction_emoji",
            Self::TriggerMetadata => "trigger_metadata",
            Self::Prompts => "prompts",
            Self::Options => "options",
            Self::Colors => "colors",
            Self::Unknown(key) => key,
        }
    }
}

impl fmt::Display for AuditLogChangeKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for AuditLogChangeKey {
    fn from(key: &str) -> Self {
        Self::from_key(key)
    }
}

impl From<String> for AuditLogChangeKey {
    fn from(key: String) -> Self {
        Self::from_key(&key)
    }
}

/// The value of a field changed by an audit log entry.
///
/// Discord uses different JSON shapes for different keys, so the value is
/// represented as an enum rather than a single scalar type.
#[derive(Clone, Debug, PartialEq)]
pub enum AuditLogChangeValue {
    /// A text value.
    Text(String),
    /// An integer value.
    Integer(i64),
    /// A floating-point value.
    Float(f64),
    /// A boolean value.
    Boolean(bool),
    /// A snowflake ID.
    Snowflake(Snowflake),
    /// A list of snowflake IDs.
    Snowflakes(Vec<Snowflake>),
    /// A list of text values.
    Strings(Vec<String>),
    /// A list of roles.
    Roles(Vec<Role>),
    /// A list of permission overwrites.
    Overwrites(Vec<PermissionOverwrite>),
    /// A list of forum tags.
    ForumTags(Vec<ForumTag>),
}

impl AuditLogChangeValue {
    /// Returns the text value, if this is a text value.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Text(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the integer value, if this is an integer value.
    #[must_use]
    pub const fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the floating-point value, if this is a floating-point value.
    #[must_use]
    pub const fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the boolean value, if this is a boolean value.
    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the snowflake value, if this is a single snowflake.
    #[must_use]
    pub const fn as_snowflake(&self) -> Option<Snowflake> {
        match self {
            Self::Snowflake(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the snowflake list, if this is a list of snowflakes.
    #[must_use]
    pub fn as_snowflakes(&self) -> Option<&[Snowflake]> {
        match self {
            Self::Snowflakes(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the role list, if this is a list of roles.
    #[must_use]
    pub fn as_roles(&self) -> Option<&[Role]> {
        match self {
            Self::Roles(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the overwrite list, if this is a list of permission overwrites.
    #[must_use]
    pub fn as_overwrites(&self) -> Option<&[PermissionOverwrite]> {
        match self {
            Self::Overwrites(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the forum tag list, if this is a list of forum tags.
    #[must_use]
    pub fn as_forum_tags(&self) -> Option<&[ForumTag]> {
        match self {
            Self::ForumTags(value) => Some(value),
            _ => None,
        }
    }
}

/// A single field change recorded by an audit log entry.
#[derive(Clone, Debug, PartialEq)]
pub struct AuditLogChange {
    /// The field that changed.
    pub key: AuditLogChangeKey,
    /// The new value, if the field was set.
    pub new_value: Option<AuditLogChangeValue>,
    /// The old value, if the field had a value before the change.
    pub old_value: Option<AuditLogChangeValue>,
}

impl AuditLogChange {
    /// Creates a change for the given key.
    #[must_use]
    pub fn new(key: impl Into<AuditLogChangeKey>) -> Self {
        Self {
            key: key.into(),
            new_value: None,
            old_value: None,
        }
    }

    /// Sets the new value.
    #[must_use]
    pub fn with_new_value(mut self, value: AuditLogChangeValue) -> Self {
        self.new_value = Some(value);
        self
    }

    /// Sets the old value.
    #[must_use]
    pub fn with_old_value(mut self, value: AuditLogChangeValue) -> Self {
        self.old_value = Some(value);
        self
    }

    /// Returns the new text value, if this is a text change.
    #[must_use]
    pub fn new_text(&self) -> Option<&str> {
        self.new_value
            .as_ref()
            .and_then(AuditLogChangeValue::as_str)
    }

    /// Returns the old text value, if this is a text change.
    #[must_use]
    pub fn old_text(&self) -> Option<&str> {
        self.old_value
            .as_ref()
            .and_then(AuditLogChangeValue::as_str)
    }
}

impl fmt::Display for AuditLogChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.key.as_str())
    }
}

/// Extra information attached to an audit log entry.
///
/// Which fields are present depends on the entry's action.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AuditLogEntryInfo {
    /// The number of days after which deleted members would have been pruned.
    pub delete_member_days: Option<String>,
    /// The number of members removed by a prune.
    pub members_removed: Option<String>,
    /// The channel the action was taken in, if any.
    pub channel_id: Option<Snowflake>,
    /// The message the action was taken on, if any.
    pub message_id: Option<Snowflake>,
    /// A count associated with the action.
    pub count: Option<String>,
    /// An object ID associated with the action.
    pub id: Option<Snowflake>,
    /// The type of the action, as a string.
    pub entry_type: Option<String>,
    /// The name of a role associated with the action.
    pub role_name: Option<String>,
    /// The application associated with the action, if any.
    pub application_id: Option<Snowflake>,
    /// The guild associated with the action, if any.
    pub guild_id: Option<Snowflake>,
    /// The name of an AutoMod rule associated with the action.
    pub auto_moderation_rule_name: Option<String>,
    /// The trigger type of an AutoMod rule associated with the action.
    pub auto_moderation_rule_trigger_type: Option<String>,
    /// The integration type associated with the action.
    pub integration_type: Option<String>,
}

impl AuditLogEntryInfo {
    /// Sets the channel the action was taken in.
    #[must_use]
    pub fn with_channel_id(mut self, channel_id: impl Into<Snowflake>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    /// Sets the message the action was taken on.
    #[must_use]
    pub fn with_message_id(mut self, message_id: impl Into<Snowflake>) -> Self {
        self.message_id = Some(message_id.into());
        self
    }

    /// Sets the count associated with the action.
    #[must_use]
    pub fn with_count(mut self, count: impl Into<String>) -> Self {
        self.count = Some(count.into());
        self
    }

    /// Sets the object ID associated with the action.
    #[must_use]
    pub fn with_id(mut self, id: impl Into<Snowflake>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the role name associated with the action.
    #[must_use]
    pub fn with_role_name(mut self, role_name: impl Into<String>) -> Self {
        self.role_name = Some(role_name.into());
        self
    }
}

/// One action recorded in a guild's audit log.
///
/// Equality and hashing are based on the entry's ID.
#[derive(Clone, Debug)]
pub struct AuditLogEntry {
    /// The entry's unique ID.
    pub id: Snowflake,
    /// The action that was performed.
    pub action: AuditLogAction,
    /// The ID of the user that performed the action, if known.
    pub user_id: Option<Snowflake>,
    /// The ID of the object the action targeted, if any.
    pub target_id: Option<Snowflake>,
    /// The changes made by the action.
    pub changes: Vec<AuditLogChange>,
    /// Extra information about the action.
    pub options: Option<AuditLogEntryInfo>,
    /// The reason supplied for the action, if any.
    pub reason: Option<String>,
}

impl AuditLogEntry {
    /// Creates an entry for the given action.
    #[must_use]
    pub fn new(id: impl Into<Snowflake>, action: AuditLogAction) -> Self {
        Self {
            id: id.into(),
            action,
            user_id: None,
            target_id: None,
            changes: Vec::new(),
            options: None,
            reason: None,
        }
    }

    /// Sets the user that performed the action.
    #[must_use]
    pub fn with_user_id(mut self, user_id: impl Into<Snowflake>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// Sets the object the action targeted.
    #[must_use]
    pub fn with_target_id(mut self, target_id: impl Into<Snowflake>) -> Self {
        self.target_id = Some(target_id.into());
        self
    }

    /// Sets the changes made by the action.
    #[must_use]
    pub fn with_changes(mut self, changes: Vec<AuditLogChange>) -> Self {
        self.changes = changes;
        self
    }

    /// Adds a change made by the action.
    #[must_use]
    pub fn with_change(mut self, change: AuditLogChange) -> Self {
        self.changes.push(change);
        self
    }

    /// Sets the entry's extra information.
    #[must_use]
    pub fn with_options(mut self, options: AuditLogEntryInfo) -> Self {
        self.options = Some(options);
        self
    }

    /// Sets the reason supplied for the action.
    #[must_use]
    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Returns the time the entry was created.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.id.created_at()
    }

    /// Returns the category of the action, if it has one.
    #[must_use]
    pub const fn category(&self) -> Option<AuditLogActionCategory> {
        self.action.category()
    }

    /// Returns the kind of object the action targets, if known.
    #[must_use]
    pub const fn target_type(&self) -> Option<AuditLogActionTargetType> {
        self.action.target_type()
    }

    /// Returns `true` if the action created an object.
    #[must_use]
    pub const fn is_create(&self) -> bool {
        self.action.is_create()
    }

    /// Returns `true` if the action updated an object.
    #[must_use]
    pub const fn is_update(&self) -> bool {
        self.action.is_update()
    }

    /// Returns `true` if the action deleted an object.
    #[must_use]
    pub const fn is_delete(&self) -> bool {
        self.action.is_delete()
    }

    /// Returns the change for the given key, if the entry has one.
    #[must_use]
    pub fn change(&self, key: impl Into<AuditLogChangeKey>) -> Option<&AuditLogChange> {
        let key = key.into();
        self.changes.iter().find(|change| change.key == key)
    }

    /// Returns the new value of the given key, if the entry has one.
    #[must_use]
    pub fn new_value(&self, key: impl Into<AuditLogChangeKey>) -> Option<&AuditLogChangeValue> {
        self.change(key)
            .and_then(|change| change.new_value.as_ref())
    }

    /// Returns the old value of the given key, if the entry has one.
    #[must_use]
    pub fn old_value(&self, key: impl Into<AuditLogChangeKey>) -> Option<&AuditLogChangeValue> {
        self.change(key)
            .and_then(|change| change.old_value.as_ref())
    }

    /// Resolves the entry's target against the audit log's cached objects.
    ///
    /// Returns `None` when the entry has no target ID. Targets that this
    /// library cannot resolve are returned as
    /// [`AuditLogEntryTarget::Unknown`].
    #[must_use]
    pub fn target<'a>(&self, log: &'a AuditLog) -> Option<AuditLogEntryTarget<'a>> {
        let target_id = self.target_id?;
        let target = match self.target_type()? {
            AuditLogActionTargetType::Guild => AuditLogEntryTarget::Guild(target_id),
            AuditLogActionTargetType::Channel => log
                .get_channel(target_id)
                .map_or(AuditLogEntryTarget::Unknown(target_id), |channel| {
                    AuditLogEntryTarget::Channel(channel)
                }),
            AuditLogActionTargetType::Thread => log
                .get_thread(target_id)
                .map_or(AuditLogEntryTarget::Unknown(target_id), |thread| {
                    AuditLogEntryTarget::Thread(thread)
                }),
            AuditLogActionTargetType::User => log
                .user(target_id)
                .map_or(AuditLogEntryTarget::Unknown(target_id), |user| {
                    AuditLogEntryTarget::User(user)
                }),
            AuditLogActionTargetType::Webhook => log
                .get_webhook(target_id)
                .map_or(AuditLogEntryTarget::Unknown(target_id), |webhook| {
                    AuditLogEntryTarget::Webhook(webhook)
                }),
            AuditLogActionTargetType::GuildScheduledEvent => log
                .get_scheduled_event(target_id)
                .map_or(AuditLogEntryTarget::Unknown(target_id), |event| {
                    AuditLogEntryTarget::ScheduledEvent(event)
                }),
            _ => AuditLogEntryTarget::Unknown(target_id),
        };
        Some(target)
    }
}

impl fmt::Display for AuditLogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.action, self.id)
    }
}

impl PartialEq for AuditLogEntry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for AuditLogEntry {}

impl Hash for AuditLogEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for AuditLogEntry {
    fn id(&self) -> Snowflake {
        self.id
    }
}

/// A resolved audit log entry target.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuditLogEntryTarget<'a> {
    /// The guild itself.
    Guild(Snowflake),
    /// A cached guild channel.
    Channel(&'a GuildChannel),
    /// A cached thread channel.
    Thread(&'a GuildChannel),
    /// A cached user.
    User(&'a User),
    /// A cached webhook.
    Webhook(&'a Webhook),
    /// A cached scheduled event.
    ScheduledEvent(&'a GuildScheduledEvent),
    /// A target that is not cached, or whose kind this library does not model.
    Unknown(Snowflake),
}

impl AuditLogEntryTarget<'_> {
    /// Returns the target's ID.
    #[must_use]
    pub fn id(&self) -> Snowflake {
        match self {
            Self::Guild(id) | Self::Unknown(id) => *id,
            Self::Channel(channel) => channel.id,
            Self::Thread(thread) => thread.id,
            Self::User(user) => user.id,
            Self::Webhook(webhook) => webhook.id,
            Self::ScheduledEvent(event) => event.id,
        }
    }
}

/// A page of a guild's audit log.
#[derive(Clone, Debug, Default)]
pub struct AuditLog {
    /// The entries in the page.
    pub entries: Vec<AuditLogEntry>,
    /// The users referenced by the entries.
    pub users: Vec<User>,
    /// The webhooks referenced by the entries.
    pub webhooks: Vec<Webhook>,
    /// The threads referenced by the entries.
    pub threads: Vec<GuildChannel>,
    /// The scheduled events referenced by the entries.
    pub scheduled_events: Vec<GuildScheduledEvent>,
}

impl AuditLog {
    /// Creates an empty audit log page.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
            users: Vec::new(),
            webhooks: Vec::new(),
            threads: Vec::new(),
            scheduled_events: Vec::new(),
        }
    }

    /// Sets the page's entries.
    #[must_use]
    pub fn with_entries(mut self, entries: Vec<AuditLogEntry>) -> Self {
        self.entries = entries;
        self
    }

    /// Sets the users referenced by the page.
    #[must_use]
    pub fn with_users(mut self, users: Vec<User>) -> Self {
        self.users = users;
        self
    }

    /// Sets the webhooks referenced by the page.
    #[must_use]
    pub fn with_webhooks(mut self, webhooks: Vec<Webhook>) -> Self {
        self.webhooks = webhooks;
        self
    }

    /// Sets the threads referenced by the page.
    #[must_use]
    pub fn with_threads(mut self, threads: Vec<GuildChannel>) -> Self {
        self.threads = threads;
        self
    }

    /// Sets the scheduled events referenced by the page.
    #[must_use]
    pub fn with_scheduled_events(mut self, scheduled_events: Vec<GuildScheduledEvent>) -> Self {
        self.scheduled_events = scheduled_events;
        self
    }

    /// Returns the entry with the given ID, if it is in the page.
    #[must_use]
    pub fn get_entry(&self, entry_id: impl Into<Snowflake>) -> Option<&AuditLogEntry> {
        let entry_id = entry_id.into();
        self.entries.iter().find(|entry| entry.id == entry_id)
    }

    /// Returns the user with the given ID, if they are in the page.
    #[must_use]
    pub fn user(&self, user_id: impl Into<Snowflake>) -> Option<&User> {
        let user_id = user_id.into();
        self.users.iter().find(|user| user.id == user_id)
    }

    /// Returns the webhook with the given ID, if it is in the page.
    #[must_use]
    pub fn get_webhook(&self, webhook_id: impl Into<Snowflake>) -> Option<&Webhook> {
        let webhook_id = webhook_id.into();
        self.webhooks
            .iter()
            .find(|webhook| webhook.id == webhook_id)
    }

    /// Returns the channel with the given ID, if it is in the page's threads.
    #[must_use]
    pub fn get_channel(&self, channel_id: impl Into<Snowflake>) -> Option<&GuildChannel> {
        let channel_id = channel_id.into();
        self.threads.iter().find(|channel| channel.id == channel_id)
    }

    /// Returns the thread with the given ID, if it is in the page.
    #[must_use]
    pub fn get_thread(&self, thread_id: impl Into<Snowflake>) -> Option<&GuildChannel> {
        let thread_id = thread_id.into();
        self.threads.iter().find(|thread| thread.id == thread_id)
    }

    /// Returns the scheduled event with the given ID, if it is in the page.
    #[must_use]
    pub fn get_scheduled_event(
        &self,
        event_id: impl Into<Snowflake>,
    ) -> Option<&GuildScheduledEvent> {
        let event_id = event_id.into();
        self.scheduled_events
            .iter()
            .find(|event| event.id == event_id)
    }

    /// Returns every entry that performed the given action.
    pub fn entries_for_action(
        &self,
        action: AuditLogAction,
    ) -> impl Iterator<Item = &AuditLogEntry> {
        self.entries
            .iter()
            .filter(move |entry| entry.action == action)
    }

    /// Returns every entry performed by the given user.
    pub fn entries_by_user(
        &self,
        user_id: impl Into<Snowflake>,
    ) -> impl Iterator<Item = &AuditLogEntry> {
        let user_id = user_id.into();
        self.entries
            .iter()
            .filter(move |entry| entry.user_id == Some(user_id))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use abc::{HasId, Snowflake};
    use enums::{AuditLogAction, AuditLogActionCategory, AuditLogActionTargetType};

    use super::{
        AuditLog, AuditLogChange, AuditLogChangeKey, AuditLogChangeValue, AuditLogEntry,
        AuditLogEntryInfo, AuditLogEntryTarget,
    };
    use crate::{
        ForumTag, GuildChannel, GuildScheduledEvent, PermissionOverwrite, Role, User, Webhook,
    };

    fn hash<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn change_keys_round_trip() {
        let keys = [
            ("name", AuditLogChangeKey::Name),
            (
                "permission_overwrites",
                AuditLogChangeKey::PermissionOverwrites,
            ),
            ("$add", AuditLogChangeKey::Add),
            ("$remove", AuditLogChangeKey::Remove),
            ("colors", AuditLogChangeKey::Colors),
        ];
        for (raw, key) in keys {
            assert_eq!(AuditLogChangeKey::from_key(raw), key);
            assert_eq!(key.as_str(), raw);
            assert_eq!(key.to_string(), raw);
        }

        let unknown = AuditLogChangeKey::from_key("future_field");
        assert_eq!(unknown, AuditLogChangeKey::Unknown("future_field".into()));
        assert_eq!(unknown.as_str(), "future_field");
        assert_eq!(AuditLogChangeKey::from("name"), AuditLogChangeKey::Name);
        assert_eq!(
            AuditLogChangeKey::from(String::from("topic")),
            AuditLogChangeKey::Topic
        );
    }

    #[test]
    fn change_builders_and_value_accessors() {
        let role = Role::new(1_u64, 2_u64, "Role");
        let tag = ForumTag::new("tag").with_id(3_u64);
        let overwrite = PermissionOverwrite::new();
        let change = AuditLogChange::new("name")
            .with_new_value(AuditLogChangeValue::Text("new".into()))
            .with_old_value(AuditLogChangeValue::Text("old".into()));

        assert_eq!(change.key, AuditLogChangeKey::Name);
        assert_eq!(change.new_text(), Some("new"));
        assert_eq!(change.old_text(), Some("old"));
        assert_eq!(change.to_string(), "name");
        assert!(AuditLogChange::new("topic").new_text().is_none());

        assert_eq!(AuditLogChangeValue::Integer(7).as_integer(), Some(7));
        assert_eq!(AuditLogChangeValue::Float(1.5).as_float(), Some(1.5));
        assert_eq!(AuditLogChangeValue::Boolean(true).as_bool(), Some(true));
        assert_eq!(
            AuditLogChangeValue::Snowflake(Snowflake::new(4)).as_snowflake(),
            Some(Snowflake::new(4))
        );
        assert_eq!(
            AuditLogChangeValue::Snowflakes(vec![Snowflake::new(4)])
                .as_snowflakes()
                .map(<[Snowflake]>::len),
            Some(1)
        );
        assert_eq!(
            AuditLogChangeValue::Roles(vec![role])
                .as_roles()
                .map(<[Role]>::len),
            Some(1)
        );
        assert_eq!(
            AuditLogChangeValue::Overwrites(vec![overwrite])
                .as_overwrites()
                .map(<[PermissionOverwrite]>::len),
            Some(1)
        );
        assert_eq!(
            AuditLogChangeValue::ForumTags(vec![tag])
                .as_forum_tags()
                .map(<[ForumTag]>::len),
            Some(1)
        );
        assert_eq!(AuditLogChangeValue::Strings(Vec::new()).as_str(), None);
    }

    #[test]
    fn entries_store_changes_and_delegate_action_helpers() {
        let change =
            AuditLogChange::new("name").with_new_value(AuditLogChangeValue::Text("renamed".into()));
        let options = AuditLogEntryInfo::default()
            .with_channel_id(5_u64)
            .with_message_id(6_u64)
            .with_count("2")
            .with_id(7_u64)
            .with_role_name("Role");
        let entry = AuditLogEntry::new(10_u64, AuditLogAction::ChannelUpdate)
            .with_user_id(1_u64)
            .with_target_id(5_u64)
            .with_changes(vec![change.clone()])
            .with_change(AuditLogChange::new("topic"))
            .with_options(options.clone())
            .with_reason("because");

        assert_eq!(entry.id, Snowflake::new(10));
        assert_eq!(entry.id(), Snowflake::new(10));
        assert_eq!(entry.created_at(), Snowflake::new(10).created_at());
        assert_eq!(entry.user_id, Some(Snowflake::new(1)));
        assert_eq!(entry.target_id, Some(Snowflake::new(5)));
        assert_eq!(entry.changes.len(), 2);
        assert_eq!(entry.change("name"), Some(&change));
        assert_eq!(
            entry
                .new_value("name")
                .and_then(AuditLogChangeValue::as_str),
            Some("renamed")
        );
        assert_eq!(entry.old_value("name"), None);
        assert_eq!(entry.change("missing"), None);
        assert_eq!(entry.options, Some(options));
        assert_eq!(
            entry.options.as_ref().and_then(|info| info.channel_id),
            Some(Snowflake::new(5))
        );
        assert_eq!(
            entry
                .options
                .as_ref()
                .and_then(|info| info.count.as_deref()),
            Some("2")
        );
        assert_eq!(
            entry
                .options
                .as_ref()
                .and_then(|info| info.role_name.as_deref()),
            Some("Role")
        );
        assert_eq!(entry.reason.as_deref(), Some("because"));
        assert_eq!(entry.category(), Some(AuditLogActionCategory::Update));
        assert_eq!(entry.target_type(), Some(AuditLogActionTargetType::Channel));
        assert!(entry.is_update());
        assert!(!entry.is_create());
        assert!(!entry.is_delete());
        assert_eq!(entry.to_string(), "channel_update (10)");
    }

    #[test]
    fn entries_use_id_for_equality_and_hashing() {
        let first = AuditLogEntry::new(10_u64, AuditLogAction::Kick).with_reason("first");
        let same = AuditLogEntry::new(10_u64, AuditLogAction::Ban).with_reason("other");
        let other = AuditLogEntry::new(11_u64, AuditLogAction::Kick);

        assert_eq!(first, same);
        assert_ne!(first, other);
        assert_eq!(hash(&first), hash(&same));
        assert_ne!(hash(&first), hash(&other));
        assert_eq!(
            AuditLogEntry::new(1_u64, AuditLogAction::Kick).category(),
            None
        );
    }

    #[test]
    fn log_accessors_and_target_resolution() {
        let user = User::new(1_u64, "moderator");
        let webhook = Webhook::new(2_u64);
        let channel = GuildChannel::new(3_u64, 100_u64, "general", enums::ChannelType::Text);
        let event =
            GuildScheduledEvent::new(4_u64, 100_u64, "Event", std::time::SystemTime::UNIX_EPOCH);
        let entries = vec![
            AuditLogEntry::new(10_u64, AuditLogAction::ChannelUpdate)
                .with_user_id(1_u64)
                .with_target_id(3_u64),
            AuditLogEntry::new(11_u64, AuditLogAction::WebhookCreate)
                .with_user_id(1_u64)
                .with_target_id(2_u64),
            AuditLogEntry::new(12_u64, AuditLogAction::ScheduledEventCreate).with_target_id(4_u64),
            AuditLogEntry::new(13_u64, AuditLogAction::ThreadUpdate).with_target_id(3_u64),
            AuditLogEntry::new(14_u64, AuditLogAction::GuildUpdate).with_target_id(100_u64),
            AuditLogEntry::new(15_u64, AuditLogAction::RoleUpdate).with_target_id(7_u64),
            AuditLogEntry::new(16_u64, AuditLogAction::Kick),
        ];
        let log = AuditLog::new()
            .with_entries(entries)
            .with_users(vec![user.clone()])
            .with_webhooks(vec![webhook.clone()])
            .with_threads(vec![channel.clone()])
            .with_scheduled_events(vec![event.clone()]);

        assert_eq!(
            log.get_entry(10_u64).map(|entry| entry.id),
            Some(Snowflake::new(10))
        );
        assert!(log.get_entry(99_u64).is_none());
        assert_eq!(log.user(1_u64), Some(&user));
        assert!(log.user(99_u64).is_none());
        assert_eq!(log.get_webhook(2_u64), Some(&webhook));
        assert_eq!(log.get_channel(3_u64), Some(&channel));
        assert_eq!(log.get_thread(3_u64), Some(&channel));
        assert_eq!(log.get_scheduled_event(4_u64), Some(&event));
        assert_eq!(
            log.entries_for_action(AuditLogAction::ChannelUpdate)
                .count(),
            1
        );
        assert_eq!(log.entries_by_user(1_u64).count(), 2);
        assert_eq!(log.entries_by_user(99_u64).count(), 0);

        let channel_entry = log.get_entry(10_u64).unwrap();
        assert_eq!(
            channel_entry.target(&log),
            Some(AuditLogEntryTarget::Channel(&channel))
        );
        assert_eq!(
            log.get_entry(11_u64).unwrap().target(&log),
            Some(AuditLogEntryTarget::Webhook(&webhook))
        );
        assert_eq!(
            log.get_entry(12_u64).unwrap().target(&log),
            Some(AuditLogEntryTarget::ScheduledEvent(&event))
        );
        assert_eq!(
            log.get_entry(13_u64).unwrap().target(&log),
            Some(AuditLogEntryTarget::Thread(&channel))
        );
        assert_eq!(
            log.get_entry(14_u64).unwrap().target(&log),
            Some(AuditLogEntryTarget::Guild(Snowflake::new(100)))
        );
        assert_eq!(
            log.get_entry(15_u64).unwrap().target(&log),
            Some(AuditLogEntryTarget::Unknown(Snowflake::new(7)))
        );
        assert_eq!(log.get_entry(16_u64).unwrap().target(&log), None);
        assert_eq!(
            AuditLogEntryTarget::Guild(Snowflake::new(1)).id(),
            Snowflake::new(1)
        );
        assert_eq!(
            AuditLogEntryTarget::Channel(&channel).id(),
            Snowflake::new(3)
        );
        assert_eq!(AuditLogEntryTarget::User(&user).id(), Snowflake::new(1));
        assert_eq!(
            AuditLogEntryTarget::Webhook(&webhook).id(),
            Snowflake::new(2)
        );
        assert_eq!(
            AuditLogEntryTarget::ScheduledEvent(&event).id(),
            Snowflake::new(4)
        );
        assert_eq!(
            AuditLogEntryTarget::Unknown(Snowflake::new(9)).id(),
            Snowflake::new(9)
        );
    }
}
