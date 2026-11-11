//! Invite models.
//!
//! An [`Invite`] identifies a guild or group direct message by its code. When
//! the guild or channel is cached, the invite can hold the full model;
//! otherwise it holds the partial data Discord returns with the invite.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

use abc::{Asset, HasId, Snowflake};
use enums::{ChannelType, InviteFlags, InviteTarget, InviteType, NsfwLevel, VerificationLevel};

use crate::{Guild, GuildChannel, GuildScheduledEvent, PartialApplication, User};

const INVITE_BASE: &str = "https://discord.gg";

/// The partial channel data returned with an invite.
///
/// Equality and hashing are based on the channel's ID.
#[derive(Clone, Debug, Default)]
pub struct PartialInviteChannel {
    /// The channel's unique ID.
    pub id: Snowflake,
    /// The channel's name.
    pub name: String,
    /// The channel's type.
    pub channel_type: ChannelType,
}

impl PartialInviteChannel {
    /// Creates a partial invite channel.
    #[must_use]
    pub fn new(
        id: impl Into<Snowflake>,
        name: impl Into<String>,
        channel_type: ChannelType,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            channel_type,
        }
    }

    /// Returns the channel's type.
    #[must_use]
    pub const fn channel_type(&self) -> ChannelType {
        self.channel_type
    }

    /// Returns a string that mentions the channel.
    #[must_use]
    pub fn mention(&self) -> String {
        format!("<#{}>", self.id)
    }

    /// Returns the channel's creation time.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.id.created_at()
    }
}

impl fmt::Display for PartialInviteChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl PartialEq for PartialInviteChannel {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PartialInviteChannel {}

impl Hash for PartialInviteChannel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for PartialInviteChannel {
    fn id(&self) -> Snowflake {
        self.id
    }
}

/// The partial guild data returned with an invite.
///
/// Equality and hashing are based on the guild's ID.
#[derive(Clone, Debug, Default)]
pub struct PartialInviteGuild {
    /// The guild's unique ID.
    pub id: Snowflake,
    /// The guild's name.
    pub name: String,
    /// The guild's feature flags.
    pub features: Vec<String>,
    /// The guild's icon, if it has one.
    pub icon: Option<Asset>,
    /// The guild's banner, if it has one.
    pub banner: Option<Asset>,
    /// The guild's invite splash, if it has one.
    pub splash: Option<Asset>,
    /// The guild's verification level.
    pub verification_level: VerificationLevel,
    /// The guild's description, if it has one.
    pub description: Option<String>,
    /// The guild's vanity URL code, if it has one.
    pub vanity_url_code: Option<String>,
    /// The guild's NSFW level.
    pub nsfw_level: NsfwLevel,
    /// The number of boosts the guild currently has.
    pub premium_subscription_count: u64,
}

impl PartialInviteGuild {
    /// Creates a partial invite guild.
    #[must_use]
    pub fn new(id: impl Into<Snowflake>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            ..Self::default()
        }
    }

    /// Sets the guild's feature flags.
    #[must_use]
    pub fn with_features(mut self, features: Vec<String>) -> Self {
        self.features = features;
        self
    }

    /// Sets the guild's icon from its icon hash.
    #[must_use]
    pub fn with_icon_hash(mut self, icon_hash: &str) -> Self {
        self.icon = Some(Asset::from_guild_icon(self.id, icon_hash));
        self
    }

    /// Sets the guild's icon.
    #[must_use]
    pub fn with_icon(mut self, icon: Asset) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the guild's banner from its banner hash.
    #[must_use]
    pub fn with_banner_hash(mut self, banner_hash: &str) -> Self {
        self.banner = Some(Asset::from_guild_image(self.id, banner_hash, "banners"));
        self
    }

    /// Sets the guild's splash from its splash hash.
    #[must_use]
    pub fn with_splash_hash(mut self, splash_hash: &str) -> Self {
        self.splash = Some(Asset::from_guild_image(self.id, splash_hash, "splashes"));
        self
    }

    /// Sets the guild's verification level.
    #[must_use]
    pub fn with_verification_level(mut self, level: VerificationLevel) -> Self {
        self.verification_level = level;
        self
    }

    /// Sets the guild's description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the guild's vanity URL code.
    #[must_use]
    pub fn with_vanity_url_code(mut self, code: impl Into<String>) -> Self {
        self.vanity_url_code = Some(code.into());
        self
    }

    /// Sets the guild's NSFW level.
    #[must_use]
    pub fn with_nsfw_level(mut self, level: NsfwLevel) -> Self {
        self.nsfw_level = level;
        self
    }

    /// Sets the number of boosts the guild currently has.
    #[must_use]
    pub fn with_premium_subscription_count(mut self, count: u64) -> Self {
        self.premium_subscription_count = count;
        self
    }

    /// Returns the guild's creation time.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.id.created_at()
    }

    /// Returns the guild's vanity invite URL, if it has one.
    #[must_use]
    pub fn vanity_url(&self) -> Option<String> {
        self.vanity_url_code
            .as_ref()
            .map(|code| format!("{INVITE_BASE}/{code}"))
    }

    /// Returns the guild's icon URL, if it has one.
    #[must_use]
    pub fn icon_url(&self) -> Option<&str> {
        self.icon.as_ref().map(Asset::url)
    }

    /// Returns the guild's banner URL, if it has one.
    #[must_use]
    pub fn banner_url(&self) -> Option<&str> {
        self.banner.as_ref().map(Asset::url)
    }

    /// Returns the guild's splash URL, if it has one.
    #[must_use]
    pub fn splash_url(&self) -> Option<&str> {
        self.splash.as_ref().map(Asset::url)
    }
}

impl fmt::Display for PartialInviteGuild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl PartialEq for PartialInviteGuild {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PartialInviteGuild {}

impl Hash for PartialInviteGuild {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for PartialInviteGuild {
    fn id(&self) -> Snowflake {
        self.id
    }
}

/// The guild an invite resolves to.
#[derive(Clone, Debug)]
pub enum InviteGuild {
    /// The partial guild data returned with the invite.
    Partial(Box<PartialInviteGuild>),
    /// The full guild, when it is cached.
    Full(Box<Guild>),
}

impl InviteGuild {
    /// Returns the guild's ID.
    #[must_use]
    pub fn id(&self) -> Snowflake {
        match self {
            Self::Partial(guild) => guild.id,
            Self::Full(guild) => guild.id,
        }
    }

    /// Returns the guild's name.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Partial(guild) => &guild.name,
            Self::Full(guild) => &guild.name,
        }
    }

    /// Returns the guild's icon, if it has one.
    #[must_use]
    pub fn icon(&self) -> Option<&Asset> {
        match self {
            Self::Partial(guild) => guild.icon.as_ref(),
            Self::Full(guild) => guild.icon.as_ref(),
        }
    }

    /// Returns the guild's creation time.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.id().created_at()
    }

    /// Returns `true` if this holds partial guild data.
    #[must_use]
    pub const fn is_partial(&self) -> bool {
        matches!(self, Self::Partial(_))
    }

    /// Returns `true` if this holds a full guild.
    #[must_use]
    pub const fn is_full(&self) -> bool {
        matches!(self, Self::Full(_))
    }

    /// Returns the partial guild data, if present.
    #[must_use]
    pub fn as_partial(&self) -> Option<&PartialInviteGuild> {
        match self {
            Self::Partial(guild) => Some(guild),
            Self::Full(_) => None,
        }
    }

    /// Returns the full guild, if present.
    #[must_use]
    pub fn as_guild(&self) -> Option<&Guild> {
        match self {
            Self::Partial(_) => None,
            Self::Full(guild) => Some(guild),
        }
    }

    /// Consumes the value and returns the full guild, if present.
    #[must_use]
    pub fn into_guild(self) -> Option<Guild> {
        match self {
            Self::Partial(_) => None,
            Self::Full(guild) => Some(*guild),
        }
    }
}

impl From<PartialInviteGuild> for InviteGuild {
    fn from(guild: PartialInviteGuild) -> Self {
        Self::Partial(Box::new(guild))
    }
}

impl From<Guild> for InviteGuild {
    fn from(guild: Guild) -> Self {
        Self::Full(Box::new(guild))
    }
}

impl fmt::Display for InviteGuild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl PartialEq for InviteGuild {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for InviteGuild {}

impl Hash for InviteGuild {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

/// The channel an invite resolves to.
#[derive(Clone, Debug)]
pub enum InviteChannel {
    /// The partial channel data returned with the invite.
    Partial(PartialInviteChannel),
    /// The full channel, when it is cached.
    Full(Box<GuildChannel>),
}

impl InviteChannel {
    /// Returns the channel's ID.
    #[must_use]
    pub fn id(&self) -> Snowflake {
        match self {
            Self::Partial(channel) => channel.id,
            Self::Full(channel) => channel.id,
        }
    }

    /// Returns the channel's name.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Partial(channel) => &channel.name,
            Self::Full(channel) => &channel.name,
        }
    }

    /// Returns the channel's type.
    #[must_use]
    pub const fn channel_type(&self) -> ChannelType {
        match self {
            Self::Partial(channel) => channel.channel_type,
            Self::Full(channel) => channel.channel_type,
        }
    }

    /// Returns a string that mentions the channel.
    #[must_use]
    pub fn mention(&self) -> String {
        format!("<#{}>", self.id())
    }

    /// Returns the channel's creation time.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.id().created_at()
    }

    /// Returns `true` if this holds partial channel data.
    #[must_use]
    pub const fn is_partial(&self) -> bool {
        matches!(self, Self::Partial(_))
    }

    /// Returns `true` if this holds a full channel.
    #[must_use]
    pub const fn is_full(&self) -> bool {
        matches!(self, Self::Full(_))
    }

    /// Returns the partial channel data, if present.
    #[must_use]
    pub const fn as_partial(&self) -> Option<&PartialInviteChannel> {
        match self {
            Self::Partial(channel) => Some(channel),
            Self::Full(_) => None,
        }
    }

    /// Returns the full channel, if present.
    #[must_use]
    pub fn as_channel(&self) -> Option<&GuildChannel> {
        match self {
            Self::Partial(_) => None,
            Self::Full(channel) => Some(channel),
        }
    }

    /// Consumes the value and returns the full channel, if present.
    #[must_use]
    pub fn into_channel(self) -> Option<GuildChannel> {
        match self {
            Self::Partial(_) => None,
            Self::Full(channel) => Some(*channel),
        }
    }
}

impl From<PartialInviteChannel> for InviteChannel {
    fn from(channel: PartialInviteChannel) -> Self {
        Self::Partial(channel)
    }
}

impl From<GuildChannel> for InviteChannel {
    fn from(channel: GuildChannel) -> Self {
        Self::Full(Box::new(channel))
    }
}

impl fmt::Display for InviteChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl PartialEq for InviteChannel {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for InviteChannel {}

impl Hash for InviteChannel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

/// A guild or group direct message invite.
///
/// Equality and hashing are based on the invite's code.
#[derive(Clone, Debug, Default)]
pub struct Invite {
    /// The URL fragment used for the invite.
    pub code: String,
    /// The invite's type.
    pub kind: InviteType,
    /// The guild the invite is for, if any.
    pub guild: Option<InviteGuild>,
    /// The channel the invite is for, if any.
    pub channel: Option<InviteChannel>,
    /// The user that created the invite, if known.
    pub inviter: Option<User>,
    /// The target of a voice channel invite.
    pub target_type: InviteTarget,
    /// The user whose stream is targeted, if any.
    pub target_user: Option<User>,
    /// The embedded application targeted by the invite, if any.
    pub target_application: Option<PartialApplication>,
    /// The ID of the scheduled event associated with the invite, if any.
    pub scheduled_event_id: Option<Snowflake>,
    /// The scheduled event associated with the invite, if provided.
    pub scheduled_event: Option<Box<GuildScheduledEvent>>,
    /// The approximate number of members in the guild, if known.
    pub approximate_member_count: Option<u64>,
    /// The approximate number of members currently active, if known.
    pub approximate_presence_count: Option<u64>,
    /// How many times the invite has been used, if known.
    pub uses: Option<u64>,
    /// How many times the invite can be used, if known.
    pub max_uses: Option<u64>,
    /// How long before the invite expires, in seconds, if known.
    pub max_age: Option<u64>,
    /// Whether the invite grants temporary membership, if known.
    pub temporary: Option<bool>,
    /// Whether the invite has been revoked, if known.
    pub revoked: Option<bool>,
    /// When the invite was created, if known.
    pub created_at: Option<SystemTime>,
    /// When the invite expires, or `None` if it never expires.
    pub expires_at: Option<SystemTime>,
    /// The invite's flags.
    pub flags: InviteFlags,
}

impl Invite {
    /// Creates an invite with the given code.
    #[must_use]
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            ..Self::default()
        }
    }

    /// Sets the invite's type.
    #[must_use]
    pub fn with_type(mut self, kind: InviteType) -> Self {
        self.kind = kind;
        self
    }

    /// Sets the guild the invite is for.
    #[must_use]
    pub fn with_guild(mut self, guild: impl Into<InviteGuild>) -> Self {
        self.guild = Some(guild.into());
        self
    }

    /// Sets the channel the invite is for.
    #[must_use]
    pub fn with_channel(mut self, channel: impl Into<InviteChannel>) -> Self {
        self.channel = Some(channel.into());
        self
    }

    /// Sets the user that created the invite.
    #[must_use]
    pub fn with_inviter(mut self, inviter: User) -> Self {
        self.inviter = Some(inviter);
        self
    }

    /// Sets the invite's target type.
    #[must_use]
    pub fn with_target_type(mut self, target: InviteTarget) -> Self {
        self.target_type = target;
        self
    }

    /// Sets the user whose stream is targeted.
    #[must_use]
    pub fn with_target_user(mut self, target_user: User) -> Self {
        self.target_user = Some(target_user);
        self
    }

    /// Sets the embedded application targeted by the invite.
    #[must_use]
    pub fn with_target_application(mut self, application: PartialApplication) -> Self {
        self.target_application = Some(application);
        self
    }

    /// Sets the scheduled event associated with the invite.
    #[must_use]
    pub fn with_scheduled_event_id(mut self, event_id: impl Into<Snowflake>) -> Self {
        self.scheduled_event_id = Some(event_id.into());
        self
    }

    /// Sets the scheduled event associated with the invite.
    ///
    /// The invite's scheduled event ID is updated from the event.
    #[must_use]
    pub fn with_scheduled_event(mut self, event: GuildScheduledEvent) -> Self {
        self.scheduled_event_id = Some(event.id);
        self.scheduled_event = Some(Box::new(event));
        self
    }

    /// Sets the approximate member and presence counts.
    #[must_use]
    pub fn with_approximate_counts(mut self, members: u64, presences: u64) -> Self {
        self.approximate_member_count = Some(members);
        self.approximate_presence_count = Some(presences);
        self
    }

    /// Sets how many times the invite has been used.
    #[must_use]
    pub fn with_uses(mut self, uses: u64) -> Self {
        self.uses = Some(uses);
        self
    }

    /// Sets how many times the invite can be used.
    #[must_use]
    pub fn with_max_uses(mut self, max_uses: u64) -> Self {
        self.max_uses = Some(max_uses);
        self
    }

    /// Sets how long before the invite expires, in seconds.
    #[must_use]
    pub fn with_max_age(mut self, max_age: u64) -> Self {
        self.max_age = Some(max_age);
        self
    }

    /// Sets whether the invite grants temporary membership.
    #[must_use]
    pub fn with_temporary(mut self, temporary: bool) -> Self {
        self.temporary = Some(temporary);
        self
    }

    /// Sets whether the invite has been revoked.
    #[must_use]
    pub fn with_revoked(mut self, revoked: bool) -> Self {
        self.revoked = Some(revoked);
        self
    }

    /// Sets when the invite was created.
    #[must_use]
    pub fn with_created_at(mut self, created_at: SystemTime) -> Self {
        self.created_at = Some(created_at);
        self
    }

    /// Sets when the invite expires.
    #[must_use]
    pub fn with_expires_at(mut self, expires_at: SystemTime) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    /// Sets the invite's flags.
    #[must_use]
    pub fn with_flags(mut self, flags: InviteFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Returns the invite's code.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.code
    }

    /// Returns the invite URL.
    #[must_use]
    pub fn url(&self) -> String {
        let mut url = format!("{INVITE_BASE}/{}", self.code);
        if let Some(event_id) = self.scheduled_event_id {
            url.push_str("?event=");
            url.push_str(&event_id.to_string());
        }
        url
    }

    /// Returns the ID of the guild the invite is for, if known.
    #[must_use]
    pub fn guild_id(&self) -> Option<Snowflake> {
        self.guild.as_ref().map(InviteGuild::id)
    }

    /// Returns the ID of the channel the invite is for, if known.
    #[must_use]
    pub fn channel_id(&self) -> Option<Snowflake> {
        self.channel.as_ref().map(InviteChannel::id)
    }

    /// Returns `true` if this is a guild invite.
    #[must_use]
    pub const fn is_guild(&self) -> bool {
        self.kind.is_guild()
    }

    /// Returns `true` if this is a group direct message invite.
    #[must_use]
    pub const fn is_group_dm(&self) -> bool {
        self.kind.is_group_dm()
    }

    /// Returns `true` if this is a friend invite.
    #[must_use]
    pub const fn is_friend(&self) -> bool {
        self.kind.is_friend()
    }

    /// Returns `true` if the invite has expired at `now`.
    ///
    /// Invites without an expiration time never expire.
    #[must_use]
    pub fn is_expired_at(&self, now: SystemTime) -> bool {
        self.expires_at.is_some_and(|expires_at| now >= expires_at)
    }
}

impl fmt::Display for Invite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.url())
    }
}

impl PartialEq for Invite {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl Eq for Invite {}

impl Hash for Invite {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};

    use abc::{Asset, HasId, Snowflake};
    use enums::{ChannelType, InviteFlags, InviteTarget, InviteType, NsfwLevel, VerificationLevel};

    use super::{Invite, InviteChannel, InviteGuild, PartialInviteChannel, PartialInviteGuild};
    use crate::{Guild, GuildChannel, GuildScheduledEvent, PartialApplication, User};

    #[test]
    fn partial_invite_models() {
        let channel = PartialInviteChannel::new(10_u64, "general", ChannelType::Text);
        assert_eq!(channel.id, Snowflake::new(10));
        assert_eq!(channel.id(), Snowflake::new(10));
        assert_eq!(channel.name, "general");
        assert_eq!(channel.channel_type(), ChannelType::Text);
        assert_eq!(channel.mention(), "<#10>");
        assert_eq!(channel.created_at(), Snowflake::new(10).created_at());
        assert_eq!(channel.to_string(), "general");
        assert_eq!(
            channel,
            PartialInviteChannel::new(10_u64, "other", ChannelType::Voice)
        );

        let guild = PartialInviteGuild::new(20_u64, "Guild")
            .with_features(vec!["COMMUNITY".to_owned()])
            .with_icon_hash("icon")
            .with_banner_hash("banner")
            .with_splash_hash("splash")
            .with_verification_level(VerificationLevel::High)
            .with_description("A guild")
            .with_vanity_url_code("vanity")
            .with_nsfw_level(NsfwLevel::Explicit)
            .with_premium_subscription_count(4);

        assert_eq!(guild.id, Snowflake::new(20));
        assert_eq!(guild.id(), Snowflake::new(20));
        assert_eq!(guild.name, "Guild");
        assert_eq!(guild.features, vec!["COMMUNITY".to_owned()]);
        assert_eq!(guild.verification_level, VerificationLevel::High);
        assert_eq!(guild.description.as_deref(), Some("A guild"));
        assert_eq!(guild.nsfw_level, NsfwLevel::Explicit);
        assert_eq!(guild.premium_subscription_count, 4);
        assert_eq!(guild.created_at(), Snowflake::new(20).created_at());
        assert_eq!(
            guild.vanity_url().as_deref(),
            Some("https://discord.gg/vanity")
        );
        assert_eq!(
            guild.icon_url(),
            Some(Asset::from_guild_icon(Snowflake::new(20), "icon").url())
        );
        assert_eq!(
            guild.banner_url(),
            Some(Asset::from_guild_image(Snowflake::new(20), "banner", "banners").url())
        );
        assert_eq!(
            guild.splash_url(),
            Some(Asset::from_guild_image(Snowflake::new(20), "splash", "splashes").url())
        );
        assert_eq!(guild.to_string(), "Guild");

        let bare = PartialInviteGuild::new(21_u64, "Bare");
        assert!(bare.vanity_url().is_none());
        assert!(bare.icon_url().is_none());
        assert!(bare.banner_url().is_none());
        assert!(bare.splash_url().is_none());
    }

    #[test]
    fn invite_guild_and_channel_wrap_partial_or_full_models() {
        let partial_guild = PartialInviteGuild::new(20_u64, "Partial");
        let guild = InviteGuild::from(partial_guild.clone());
        assert!(guild.is_partial());
        assert!(!guild.is_full());
        assert_eq!(guild.id(), Snowflake::new(20));
        assert_eq!(guild.name(), "Partial");
        assert_eq!(guild.as_partial(), Some(&partial_guild));
        assert!(guild.as_guild().is_none());
        assert!(guild.into_guild().is_none());

        let full_guild = Guild::new(20_u64, "Full").with_description("cached");
        let guild = InviteGuild::from(full_guild);
        assert!(guild.is_full());
        assert!(!guild.is_partial());
        assert_eq!(guild.name(), "Full");
        assert!(guild.as_partial().is_none());
        assert_eq!(
            guild.as_guild().map(|guild| guild.name.as_str()),
            Some("Full")
        );
        assert_eq!(guild.to_string(), "Full");
        assert!(guild.into_guild().is_some());

        let partial_channel = PartialInviteChannel::new(10_u64, "partial", ChannelType::Text);
        let channel = InviteChannel::from(partial_channel.clone());
        assert!(channel.is_partial());
        assert_eq!(channel.id(), Snowflake::new(10));
        assert_eq!(channel.name(), "partial");
        assert_eq!(channel.channel_type(), ChannelType::Text);
        assert_eq!(channel.mention(), "<#10>");
        assert_eq!(channel.as_partial(), Some(&partial_channel));
        assert!(channel.as_channel().is_none());
        assert!(channel.into_channel().is_none());

        let full_channel = GuildChannel::new(10_u64, 20_u64, "full", ChannelType::Voice);
        let channel = InviteChannel::from(full_channel);
        assert!(channel.is_full());
        assert_eq!(channel.name(), "full");
        assert_eq!(channel.channel_type(), ChannelType::Voice);
        assert_eq!(channel.to_string(), "full");
        assert!(channel.as_channel().is_some());
        assert!(channel.into_channel().is_some());
    }

    #[test]
    fn invite_builders_and_url() {
        let created = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000);
        let expires = SystemTime::UNIX_EPOCH + Duration::from_secs(2_000);
        let inviter = User::new(1_u64, "inviter");
        let target_user = User::new(2_u64, "target");
        let application = PartialApplication::new(3_u64, "App");
        let event = GuildScheduledEvent::new(30_u64, 20_u64, "Event", SystemTime::UNIX_EPOCH);
        let invite = Invite::new("abc")
            .with_type(InviteType::Guild)
            .with_guild(PartialInviteGuild::new(20_u64, "Guild"))
            .with_channel(PartialInviteChannel::new(
                10_u64,
                "general",
                ChannelType::Text,
            ))
            .with_inviter(inviter.clone())
            .with_target_type(InviteTarget::Stream)
            .with_target_user(target_user.clone())
            .with_target_application(application.clone())
            .with_scheduled_event(event)
            .with_approximate_counts(42, 7)
            .with_uses(5)
            .with_max_uses(10)
            .with_max_age(3600)
            .with_temporary(true)
            .with_revoked(false)
            .with_created_at(created)
            .with_expires_at(expires)
            .with_flags(InviteFlags::GUEST);

        assert_eq!(invite.code, "abc");
        assert_eq!(invite.id(), "abc");
        assert_eq!(invite.kind, InviteType::Guild);
        assert!(invite.is_guild());
        assert!(!invite.is_group_dm());
        assert_eq!(invite.guild_id(), Some(Snowflake::new(20)));
        assert_eq!(invite.channel_id(), Some(Snowflake::new(10)));
        assert_eq!(invite.inviter, Some(inviter));
        assert_eq!(invite.target_type, InviteTarget::Stream);
        assert_eq!(invite.target_user, Some(target_user));
        assert_eq!(invite.target_application, Some(application));
        assert_eq!(invite.scheduled_event_id, Some(Snowflake::new(30)));
        assert_eq!(
            invite
                .scheduled_event
                .as_deref()
                .map(|event| event.name.as_str()),
            Some("Event")
        );
        assert_eq!(invite.approximate_member_count, Some(42));
        assert_eq!(invite.approximate_presence_count, Some(7));
        assert_eq!(invite.uses, Some(5));
        assert_eq!(invite.max_uses, Some(10));
        assert_eq!(invite.max_age, Some(3600));
        assert_eq!(invite.temporary, Some(true));
        assert_eq!(invite.revoked, Some(false));
        assert_eq!(invite.created_at, Some(created));
        assert_eq!(invite.expires_at, Some(expires));
        assert!(invite.flags.guest());
        assert!(!invite.is_expired_at(expires - Duration::from_secs(1)));
        assert!(invite.is_expired_at(expires));
        assert_eq!(invite.url(), "https://discord.gg/abc?event=30");
        assert_eq!(invite.to_string(), "https://discord.gg/abc?event=30");

        let group = Invite::new("group").with_type(InviteType::GroupDm);
        assert!(group.is_group_dm());
        assert!(!group.is_guild());
        assert_eq!(group.guild_id(), None);
        assert_eq!(group.channel_id(), None);
        assert_eq!(group.url(), "https://discord.gg/group");
        assert!(!group.is_expired_at(SystemTime::UNIX_EPOCH));

        assert_eq!(Invite::new("abc"), invite);
        assert_ne!(Invite::new("other"), invite);
    }
}
