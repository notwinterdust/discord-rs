//! The `Guild` model.
//!
//! Guilds own roles and members. Until the channel and message models exist,
//! this type focuses on guild identity, settings, roles and members.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

use abc::{Asset, Colour, HasId, Snowflake};
use enums::{
    ChannelType, ContentFilter, MfaLevel, NotificationLevel, NsfwLevel, Permissions,
    VerificationLevel,
};

use crate::{GuildChannel, Member, Role};

/// Represents a Discord guild (server).
///
/// Equality and hashing are based on the guild's ID.
#[derive(Clone, Debug)]
pub struct Guild {
    /// The guild's unique ID.
    pub id: Snowflake,
    /// The guild's name.
    pub name: String,
    /// The guild's icon, if any.
    pub icon: Option<Asset>,
    /// The guild's banner, if any.
    pub banner: Option<Asset>,
    /// The guild's invite splash, if any.
    pub splash: Option<Asset>,
    /// The guild's discovery splash, if any.
    pub discovery_splash: Option<Asset>,
    /// The guild's description, if any.
    pub description: Option<String>,
    /// The ID of the guild's owner, if known.
    pub owner_id: Option<Snowflake>,
    /// The guild's roles, sorted from lowest to highest in the hierarchy.
    pub roles: Vec<Role>,
    /// The guild's cached members.
    pub members: Vec<Member>,
    /// The guild's cached channels.
    pub channels: Vec<GuildChannel>,
    /// The number of members in the guild, if known.
    pub member_count: Option<u64>,
    /// Whether the guild is unavailable.
    pub unavailable: bool,
    /// The guild's verification level.
    pub verification_level: VerificationLevel,
    /// The guild's default notification setting.
    pub default_notifications: NotificationLevel,
    /// The guild's explicit content filter level.
    pub explicit_content_filter: ContentFilter,
    /// The guild's multi-factor authentication requirement.
    pub mfa_level: MfaLevel,
    /// The guild's NSFW level.
    pub nsfw_level: NsfwLevel,
    /// The guild's premium tier, from 0 to 3.
    pub premium_tier: u8,
    /// The number of boosts the guild currently has.
    pub premium_subscription_count: u64,
    /// Whether the premium progress bar is enabled.
    pub premium_progress_bar_enabled: bool,
    /// The guild's preferred locale.
    pub preferred_locale: String,
    /// The guild's feature flags.
    pub features: Vec<String>,
    /// The guild's vanity URL code, if any.
    pub vanity_url_code: Option<String>,
    /// The number of seconds until an inactive member is moved to the AFK
    /// channel.
    pub afk_timeout: u64,
    /// The ID of the AFK channel, if any.
    pub afk_channel_id: Option<Snowflake>,
    /// The ID of the system channel, if any.
    pub system_channel_id: Option<Snowflake>,
    /// The ID of the rules channel, if any.
    pub rules_channel_id: Option<Snowflake>,
    /// The ID of the public updates channel, if any.
    pub public_updates_channel_id: Option<Snowflake>,
    /// The ID of the safety alerts channel, if any.
    pub safety_alerts_channel_id: Option<Snowflake>,
    /// Whether the guild widget is enabled.
    pub widget_enabled: bool,
    /// The ID of the widget channel, if any.
    pub widget_channel_id: Option<Snowflake>,
    /// The maximum number of presences for the guild, if known.
    pub max_presences: Option<u64>,
    /// The maximum number of members for the guild, if known.
    pub max_members: Option<u64>,
    /// The maximum number of users in a video channel, if known.
    pub max_video_channel_users: Option<u64>,
    /// The maximum number of users in a stage video channel, if known.
    pub max_stage_video_users: Option<u64>,
    /// The approximate number of members, when requested with counts.
    pub approximate_member_count: Option<u64>,
    /// The approximate number of active members, when requested with counts.
    pub approximate_presence_count: Option<u64>,
}

impl Default for Guild {
    fn default() -> Self {
        Self {
            id: Snowflake::ZERO,
            name: String::new(),
            icon: None,
            banner: None,
            splash: None,
            discovery_splash: None,
            description: None,
            owner_id: None,
            roles: Vec::new(),
            members: Vec::new(),
            channels: Vec::new(),
            member_count: None,
            unavailable: false,
            verification_level: VerificationLevel::default(),
            default_notifications: NotificationLevel::default(),
            explicit_content_filter: ContentFilter::default(),
            mfa_level: MfaLevel::default(),
            nsfw_level: NsfwLevel::default(),
            premium_tier: 0,
            premium_subscription_count: 0,
            premium_progress_bar_enabled: false,
            preferred_locale: "en-US".to_owned(),
            features: Vec::new(),
            vanity_url_code: None,
            afk_timeout: 0,
            afk_channel_id: None,
            system_channel_id: None,
            rules_channel_id: None,
            public_updates_channel_id: None,
            safety_alerts_channel_id: None,
            widget_enabled: false,
            widget_channel_id: None,
            max_presences: None,
            max_members: None,
            max_video_channel_users: None,
            max_stage_video_users: None,
            approximate_member_count: None,
            approximate_presence_count: None,
        }
    }
}

impl Guild {
    /// Creates a guild with the fields Discord always provides.
    #[must_use]
    pub fn new(id: impl Into<Snowflake>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            ..Self::default()
        }
    }

    /// Sets the guild owner's ID.
    #[must_use]
    pub fn with_owner_id(mut self, owner_id: impl Into<Snowflake>) -> Self {
        self.owner_id = Some(owner_id.into());
        self
    }

    /// Sets the guild icon from a CDN hash.
    #[must_use]
    pub fn with_icon_hash(mut self, icon_hash: &str) -> Self {
        self.icon = Some(Asset::from_guild_icon(self.id, icon_hash));
        self
    }

    /// Sets the guild banner from a CDN hash.
    #[must_use]
    pub fn with_banner_hash(mut self, banner_hash: &str) -> Self {
        self.banner = Some(Asset::from_guild_image(self.id, banner_hash, "banners"));
        self
    }

    /// Sets the guild invite splash from a CDN hash.
    #[must_use]
    pub fn with_splash_hash(mut self, splash_hash: &str) -> Self {
        self.splash = Some(Asset::from_guild_image(self.id, splash_hash, "splashes"));
        self
    }

    /// Sets the guild discovery splash from a CDN hash.
    #[must_use]
    pub fn with_discovery_splash_hash(mut self, splash_hash: &str) -> Self {
        self.discovery_splash = Some(Asset::from_guild_image(
            self.id,
            splash_hash,
            "discovery-splashes",
        ));
        self
    }

    /// Sets the guild description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the guild's roles.
    ///
    /// The roles are sorted from lowest to highest in the hierarchy.
    #[must_use]
    pub fn with_roles(mut self, mut roles: Vec<Role>) -> Self {
        roles.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        self.roles = roles;
        self
    }

    /// Sets the guild's cached members.
    ///
    /// Each member's guild ID is updated to match this guild.
    #[must_use]
    pub fn with_members(mut self, mut members: Vec<Member>) -> Self {
        for member in &mut members {
            member.guild_id = self.id;
            for role in &mut member.roles {
                role.guild_id = self.id;
            }
        }
        self.members = members;
        self
    }

    /// Sets the guild's cached channels.
    ///
    /// Each channel's guild ID is updated to match this guild.
    #[must_use]
    pub fn with_channels(mut self, mut channels: Vec<GuildChannel>) -> Self {
        for channel in &mut channels {
            channel.guild_id = self.id;
        }
        self.channels = channels;
        self
    }

    /// Sets the known member count.
    #[must_use]
    pub fn with_member_count(mut self, member_count: u64) -> Self {
        self.member_count = Some(member_count);
        self
    }

    /// Sets the guild's verification level.
    #[must_use]
    pub fn with_verification_level(mut self, level: VerificationLevel) -> Self {
        self.verification_level = level;
        self
    }

    /// Sets the guild's default notification level.
    #[must_use]
    pub fn with_default_notifications(mut self, level: NotificationLevel) -> Self {
        self.default_notifications = level;
        self
    }

    /// Sets the guild's explicit content filter level.
    #[must_use]
    pub fn with_explicit_content_filter(mut self, filter: ContentFilter) -> Self {
        self.explicit_content_filter = filter;
        self
    }

    /// Sets the guild's MFA requirement.
    #[must_use]
    pub fn with_mfa_level(mut self, level: MfaLevel) -> Self {
        self.mfa_level = level;
        self
    }

    /// Sets the guild's NSFW level.
    #[must_use]
    pub fn with_nsfw_level(mut self, level: NsfwLevel) -> Self {
        self.nsfw_level = level;
        self
    }

    /// Sets the guild's premium tier.
    #[must_use]
    pub fn with_premium_tier(mut self, tier: u8) -> Self {
        self.premium_tier = tier;
        self
    }

    /// Sets the number of boosts the guild has.
    #[must_use]
    pub fn with_premium_subscription_count(mut self, count: u64) -> Self {
        self.premium_subscription_count = count;
        self
    }

    /// Sets whether the premium progress bar is enabled.
    #[must_use]
    pub fn with_premium_progress_bar_enabled(mut self, enabled: bool) -> Self {
        self.premium_progress_bar_enabled = enabled;
        self
    }

    /// Sets the guild's preferred locale.
    #[must_use]
    pub fn with_preferred_locale(mut self, locale: impl Into<String>) -> Self {
        self.preferred_locale = locale.into();
        self
    }

    /// Sets the guild's feature flags.
    #[must_use]
    pub fn with_features(mut self, features: Vec<String>) -> Self {
        self.features = features;
        self
    }

    /// Sets the guild's vanity URL code.
    #[must_use]
    pub fn with_vanity_url_code(mut self, code: impl Into<String>) -> Self {
        self.vanity_url_code = Some(code.into());
        self
    }

    /// Sets the AFK timeout in seconds.
    #[must_use]
    pub fn with_afk_timeout(mut self, seconds: u64) -> Self {
        self.afk_timeout = seconds;
        self
    }

    /// Sets the AFK channel ID.
    #[must_use]
    pub fn with_afk_channel_id(mut self, channel_id: impl Into<Snowflake>) -> Self {
        self.afk_channel_id = Some(channel_id.into());
        self
    }

    /// Sets the system channel ID.
    #[must_use]
    pub fn with_system_channel_id(mut self, channel_id: impl Into<Snowflake>) -> Self {
        self.system_channel_id = Some(channel_id.into());
        self
    }

    /// Sets the rules channel ID.
    #[must_use]
    pub fn with_rules_channel_id(mut self, channel_id: impl Into<Snowflake>) -> Self {
        self.rules_channel_id = Some(channel_id.into());
        self
    }

    /// Sets the public updates channel ID.
    #[must_use]
    pub fn with_public_updates_channel_id(mut self, channel_id: impl Into<Snowflake>) -> Self {
        self.public_updates_channel_id = Some(channel_id.into());
        self
    }

    /// Sets the safety alerts channel ID.
    #[must_use]
    pub fn with_safety_alerts_channel_id(mut self, channel_id: impl Into<Snowflake>) -> Self {
        self.safety_alerts_channel_id = Some(channel_id.into());
        self
    }

    /// Sets whether the guild widget is enabled.
    #[must_use]
    pub fn with_widget_enabled(mut self, enabled: bool) -> Self {
        self.widget_enabled = enabled;
        self
    }

    /// Sets the widget channel ID.
    #[must_use]
    pub fn with_widget_channel_id(mut self, channel_id: impl Into<Snowflake>) -> Self {
        self.widget_channel_id = Some(channel_id.into());
        self
    }

    /// Sets the maximum number of presences.
    #[must_use]
    pub fn with_max_presences(mut self, count: u64) -> Self {
        self.max_presences = Some(count);
        self
    }

    /// Sets the maximum number of members.
    #[must_use]
    pub fn with_max_members(mut self, count: u64) -> Self {
        self.max_members = Some(count);
        self
    }

    /// Sets the maximum number of users in a video channel.
    #[must_use]
    pub fn with_max_video_channel_users(mut self, count: u64) -> Self {
        self.max_video_channel_users = Some(count);
        self
    }

    /// Sets the maximum number of users in a stage video channel.
    #[must_use]
    pub fn with_max_stage_video_users(mut self, count: u64) -> Self {
        self.max_stage_video_users = Some(count);
        self
    }

    /// Sets the approximate member count.
    #[must_use]
    pub fn with_approximate_member_count(mut self, count: u64) -> Self {
        self.approximate_member_count = Some(count);
        self
    }

    /// Sets the approximate presence count.
    #[must_use]
    pub fn with_approximate_presence_count(mut self, count: u64) -> Self {
        self.approximate_presence_count = Some(count);
        self
    }

    /// Marks the guild as unavailable.
    #[must_use]
    pub fn with_unavailable(mut self, unavailable: bool) -> Self {
        self.unavailable = unavailable;
        self
    }

    /// Returns the guild's icon, if any.
    #[must_use]
    pub fn icon(&self) -> Option<&Asset> {
        self.icon.as_ref()
    }

    /// Returns the guild's banner, if any.
    #[must_use]
    pub fn banner(&self) -> Option<&Asset> {
        self.banner.as_ref()
    }

    /// Returns the guild's invite splash, if any.
    #[must_use]
    pub fn splash(&self) -> Option<&Asset> {
        self.splash.as_ref()
    }

    /// Returns the guild's discovery splash, if any.
    #[must_use]
    pub fn discovery_splash(&self) -> Option<&Asset> {
        self.discovery_splash.as_ref()
    }

    /// Returns the guild's roles, sorted from lowest to highest.
    #[must_use]
    pub fn roles(&self) -> &[Role] {
        &self.roles
    }

    /// Returns the role with the given ID, if it exists.
    #[must_use]
    pub fn get_role(&self, role_id: impl Into<Snowflake>) -> Option<&Role> {
        let role_id = role_id.into();
        self.roles.iter().find(|role| role.id == role_id)
    }

    /// Returns the guild's `@everyone` role, if it is cached.
    #[must_use]
    pub fn default_role(&self) -> Option<&Role> {
        self.get_role(self.id)
    }

    /// Returns the guild's premium subscriber role, if it exists.
    #[must_use]
    pub fn premium_subscriber_role(&self) -> Option<&Role> {
        self.roles.iter().find(|role| role.is_premium_subscriber())
    }

    /// Returns the guild's cached members.
    #[must_use]
    pub fn members(&self) -> &[Member] {
        &self.members
    }

    /// Returns the guild's cached channels.
    #[must_use]
    pub fn channels(&self) -> &[GuildChannel] {
        &self.channels
    }

    /// Returns the channel with the given ID, if it is cached.
    #[must_use]
    pub fn get_channel(&self, channel_id: impl Into<Snowflake>) -> Option<&GuildChannel> {
        let channel_id = channel_id.into();
        self.channels
            .iter()
            .find(|channel| channel.id == channel_id)
    }

    /// Returns every cached channel of the given type.
    pub fn channels_of_type(
        &self,
        channel_type: ChannelType,
    ) -> impl Iterator<Item = &GuildChannel> {
        self.channels
            .iter()
            .filter(move |channel| channel.channel_type == channel_type)
    }

    /// Returns every cached text channel.
    pub fn text_channels(&self) -> impl Iterator<Item = &GuildChannel> {
        self.channels_of_type(ChannelType::Text)
    }

    /// Returns every cached announcement channel.
    pub fn news_channels(&self) -> impl Iterator<Item = &GuildChannel> {
        self.channels_of_type(ChannelType::News)
    }

    /// Returns every cached voice channel.
    pub fn voice_channels(&self) -> impl Iterator<Item = &GuildChannel> {
        self.channels_of_type(ChannelType::Voice)
    }

    /// Returns every cached stage voice channel.
    pub fn stage_channels(&self) -> impl Iterator<Item = &GuildChannel> {
        self.channels_of_type(ChannelType::StageVoice)
    }

    /// Returns every cached forum channel.
    pub fn forum_channels(&self) -> impl Iterator<Item = &GuildChannel> {
        self.channels_of_type(ChannelType::Forum)
    }

    /// Returns every cached media channel.
    pub fn media_channels(&self) -> impl Iterator<Item = &GuildChannel> {
        self.channels_of_type(ChannelType::Media)
    }

    /// Returns every cached channel category.
    pub fn categories(&self) -> impl Iterator<Item = &GuildChannel> {
        self.channels_of_type(ChannelType::Category)
    }

    /// Returns the guild's AFK channel, if it is cached.
    #[must_use]
    pub fn afk_channel(&self) -> Option<&GuildChannel> {
        self.afk_channel_id.and_then(|id| self.get_channel(id))
    }

    /// Returns the guild's system channel, if it is cached.
    #[must_use]
    pub fn system_channel(&self) -> Option<&GuildChannel> {
        self.system_channel_id.and_then(|id| self.get_channel(id))
    }

    /// Returns the guild's rules channel, if it is cached.
    #[must_use]
    pub fn rules_channel(&self) -> Option<&GuildChannel> {
        self.rules_channel_id.and_then(|id| self.get_channel(id))
    }

    /// Returns the guild's public updates channel, if it is cached.
    #[must_use]
    pub fn public_updates_channel(&self) -> Option<&GuildChannel> {
        self.public_updates_channel_id
            .and_then(|id| self.get_channel(id))
    }

    /// Returns the guild's safety alerts channel, if it is cached.
    #[must_use]
    pub fn safety_alerts_channel(&self) -> Option<&GuildChannel> {
        self.safety_alerts_channel_id
            .and_then(|id| self.get_channel(id))
    }

    /// Returns the guild's widget channel, if it is cached.
    #[must_use]
    pub fn widget_channel(&self) -> Option<&GuildChannel> {
        self.widget_channel_id.and_then(|id| self.get_channel(id))
    }

    /// Returns the member with the given user ID, if they are cached.
    #[must_use]
    pub fn get_member(&self, user_id: impl Into<Snowflake>) -> Option<&Member> {
        let user_id = user_id.into();
        self.members.iter().find(|member| member.id() == user_id)
    }

    /// Returns the guild's owner, if they are cached.
    #[must_use]
    pub fn owner(&self) -> Option<&Member> {
        self.owner_id.and_then(|owner_id| self.get_member(owner_id))
    }

    /// Returns `true` if the given user ID is the guild owner.
    #[must_use]
    pub fn is_owner(&self, user_id: impl Into<Snowflake>) -> bool {
        self.owner_id == Some(user_id.into())
    }

    /// Returns the members who are currently boosting the guild.
    pub fn premium_subscribers(&self) -> impl Iterator<Item = &Member> {
        self.members
            .iter()
            .filter(|member| member.premium_since.is_some())
    }

    /// Resolves a member's roles against the guild's live role list.
    ///
    /// Members built without role snapshots resolve to an empty list.
    #[must_use]
    pub fn roles_for(&self, member: &Member) -> Vec<&Role> {
        member
            .roles()
            .iter()
            .filter_map(|role| self.get_role(role.id))
            .collect()
    }

    /// Returns the highest role a member has, resolved against this guild.
    #[must_use]
    pub fn top_role_for(&self, member: &Member) -> Option<&Role> {
        self.roles_for(member).last().copied()
    }

    /// Returns the colour rendered for a member by this guild.
    #[must_use]
    pub fn member_colour(&self, member: &Member) -> Colour {
        self.roles_for(member)
            .into_iter()
            .rev()
            .find(|role| !role.is_default() && role.colour().value() != 0)
            .map(Role::colour)
            .unwrap_or_default()
    }

    /// American spelling alias of [`Guild::member_colour`].
    #[must_use]
    pub fn member_color(&self, member: &Member) -> Colour {
        self.member_colour(member)
    }

    /// Computes a member's guild permissions using this guild's roles and
    /// owner.
    #[must_use]
    pub fn guild_permissions(&self, member: &Member) -> Permissions {
        if self.owner_id == Some(member.id()) {
            return Permissions::all();
        }

        let mut base = Permissions::none();
        for role in self.roles_for(member) {
            base.insert(role.permissions());
        }

        if base.administrator() {
            return Permissions::all();
        }

        if member.is_timed_out() {
            base = base.intersection(Permissions::timeout_mask());
        }

        base
    }

    /// Returns whether the guild is considered large.
    ///
    /// Discord treats guilds with at least 250 members as large.
    #[must_use]
    pub fn is_large(&self) -> Option<bool> {
        self.member_count.map(|count| count >= 250)
    }

    /// Returns the guild's creation time.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.id.created_at()
    }
}

impl fmt::Display for Guild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl PartialEq for Guild {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Guild {}

impl Hash for Guild {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for Guild {
    fn id(&self) -> Snowflake {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{Duration, SystemTime};

    use abc::{HasId, Snowflake};
    use enums::{ChannelType, Permissions, VerificationLevel};

    use super::Guild;
    use crate::{GuildChannel, Member, Role, RoleTags, User};

    fn sample_role(guild_id: u64, id: u64, name: &str, position: i64) -> Role {
        Role::new(guild_id, id, name).with_position(position)
    }

    fn sample_guild() -> Guild {
        let everyone = sample_role(100, 100, "@everyone", 0);
        let low = sample_role(100, 10, "Low", 1).with_colour(abc::Colour::RED);
        let high = sample_role(100, 20, "High", 2).with_colour(abc::Colour::BLUE);

        let owner = Member::new(User::new(1_u64, "owner"), 100_u64);
        let member = Member::new(User::new(2_u64, "member"), 100_u64)
            .with_roles(vec![low.clone(), high.clone()]);

        Guild::new(100_u64, "Test Guild")
            .with_owner_id(1_u64)
            .with_roles(vec![high, everyone, low])
            .with_members(vec![owner, member])
            .with_member_count(2)
    }

    #[test]
    fn constructor_defaults() {
        let guild = Guild::new(100_u64, "Test Guild");

        assert_eq!(guild.id, Snowflake::new(100));
        assert_eq!(guild.id(), Snowflake::new(100));
        assert_eq!(guild.name, "Test Guild");
        assert_eq!(guild.to_string(), "Test Guild");
        assert_eq!(guild.preferred_locale, "en-US");
        assert_eq!(guild.premium_tier, 0);
        assert!(!guild.widget_enabled);
        assert!(!guild.unavailable);
        assert_eq!(guild.verification_level, VerificationLevel::None);
        assert!(guild.roles().is_empty());
        assert!(guild.members().is_empty());
        assert!(guild.channels().is_empty());
        assert_eq!(guild.owner(), None);
        assert_eq!(guild.default_role(), None);
        assert_eq!(guild.is_large(), None);
    }

    #[test]
    fn roles_are_sorted_and_queryable() {
        let guild = sample_guild();

        assert_eq!(
            guild
                .roles()
                .iter()
                .map(|role| role.name.as_str())
                .collect::<Vec<_>>(),
            vec!["@everyone", "Low", "High"]
        );
        assert_eq!(
            guild.get_role(20_u64).map(|role| role.name.as_str()),
            Some("High")
        );
        assert!(guild.get_role(99_u64).is_none());
        assert_eq!(
            guild.default_role().map(|role| role.name.as_str()),
            Some("@everyone")
        );

        let boost = sample_role(100, 30, "Boost", 3).with_tags(RoleTags {
            premium_subscriber: true,
            ..RoleTags::default()
        });
        let guild = guild.with_roles(vec![boost]);
        assert_eq!(
            guild
                .premium_subscriber_role()
                .map(|role| role.name.as_str()),
            Some("Boost")
        );
    }

    #[test]
    fn members_owner_and_premium_subscribers() {
        let guild = sample_guild();

        assert_eq!(guild.members().len(), 2);
        assert_eq!(guild.get_member(2_u64).map(Member::name), Some("member"));
        assert!(guild.get_member(99_u64).is_none());
        assert_eq!(guild.owner().map(Member::name), Some("owner"));
        assert!(guild.is_owner(1_u64));
        assert!(!guild.is_owner(2_u64));
        assert_eq!(guild.is_large(), Some(false));

        let premium =
            Member::new(User::new(3_u64, "booster"), 100_u64).with_premium_since(SystemTime::now());
        let guild = guild.with_members(vec![premium]);
        assert_eq!(guild.members()[0].guild_id, Snowflake::new(100));
        assert_eq!(guild.premium_subscribers().count(), 1);
    }

    #[test]
    fn assets_use_cdn_urls() {
        let guild = Guild::new(100_u64, "Test")
            .with_icon_hash("icon")
            .with_banner_hash("banner")
            .with_splash_hash("splash")
            .with_discovery_splash_hash("discovery");

        assert_eq!(
            guild.icon().unwrap().url(),
            "https://cdn.discordapp.com/icons/100/icon.png?size=1024"
        );
        assert_eq!(
            guild.banner().unwrap().url(),
            "https://cdn.discordapp.com/banners/100/banner.png?size=1024"
        );
        assert_eq!(
            guild.splash().unwrap().url(),
            "https://cdn.discordapp.com/splashes/100/splash.png?size=1024"
        );
        assert_eq!(
            guild.discovery_splash().unwrap().url(),
            "https://cdn.discordapp.com/discovery-splashes/100/discovery.png?size=1024"
        );
    }

    #[test]
    fn resolves_member_roles_and_permissions() {
        let guild = sample_guild();
        let member = guild.get_member(2_u64).unwrap();

        assert_eq!(
            guild
                .roles_for(member)
                .iter()
                .map(|role| role.name.as_str())
                .collect::<Vec<_>>(),
            vec!["Low", "High"]
        );
        assert_eq!(
            guild.top_role_for(member).map(|role| role.name.as_str()),
            Some("High")
        );
        assert_eq!(guild.member_colour(member), abc::Colour::BLUE);
        assert_eq!(guild.member_color(member), abc::Colour::BLUE);

        let role = sample_role(100, 20, "High", 2)
            .with_permissions(Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES);
        let guild = sample_guild().with_roles(vec![role]);
        let member = guild.get_member(2_u64).unwrap();
        assert_eq!(
            guild.guild_permissions(member),
            Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES
        );

        let owner = guild.get_member(1_u64).unwrap();
        assert_eq!(guild.guild_permissions(owner), Permissions::all());

        let admin = sample_role(100, 20, "High", 2).with_permissions(Permissions::ADMINISTRATOR);
        let guild = sample_guild().with_roles(vec![admin]);
        let member = guild.get_member(2_u64).unwrap();
        assert_eq!(guild.guild_permissions(member), Permissions::all());

        let timed_out = Member::new(User::new(2_u64, "member"), 100_u64)
            .with_roles(vec![sample_role(100, 20, "High", 2)])
            .with_timed_out_until(SystemTime::now() + Duration::from_secs(60));
        let guild = sample_guild()
            .with_roles(vec![
                sample_role(100, 100, "@everyone", 0),
                sample_role(100, 20, "High", 2).with_permissions(
                    Permissions::VIEW_CHANNEL
                        | Permissions::READ_MESSAGE_HISTORY
                        | Permissions::SEND_MESSAGES,
                ),
            ])
            .with_members(vec![timed_out]);
        let member = guild.get_member(2_u64).unwrap();
        assert_eq!(
            guild.guild_permissions(member),
            Permissions::VIEW_CHANNEL | Permissions::READ_MESSAGE_HISTORY
        );
    }

    #[test]
    fn equality_and_hash_use_id_only() {
        let a = Guild::new(1_u64, "First");
        let b = Guild::new(1_u64, "Second").with_owner_id(9_u64);
        let c = Guild::new(2_u64, "First");

        assert_eq!(a, b);
        assert_ne!(a, c);

        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        a.hash(&mut hasher_a);
        b.hash(&mut hasher_b);
        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }

    #[test]
    fn settings_and_channels() {
        let guild = Guild::new(100_u64, "Test")
            .with_description("A description")
            .with_member_count(500)
            .with_verification_level(VerificationLevel::High)
            .with_premium_tier(2)
            .with_premium_subscription_count(10)
            .with_preferred_locale("fr")
            .with_features(vec!["COMMUNITY".to_owned()])
            .with_afk_channel_id(1_u64)
            .with_system_channel_id(2_u64)
            .with_widget_enabled(true)
            .with_widget_channel_id(3_u64)
            .with_unavailable(true);

        assert_eq!(guild.description.as_deref(), Some("A description"));
        assert_eq!(guild.member_count, Some(500));
        assert_eq!(guild.is_large(), Some(true));
        assert_eq!(guild.verification_level, VerificationLevel::High);
        assert_eq!(guild.premium_tier, 2);
        assert_eq!(guild.premium_subscription_count, 10);
        assert_eq!(guild.preferred_locale, "fr");
        assert_eq!(guild.features, vec!["COMMUNITY".to_owned()]);
        assert_eq!(guild.afk_channel_id, Some(Snowflake::new(1)));
        assert_eq!(guild.system_channel_id, Some(Snowflake::new(2)));
        assert!(guild.widget_enabled);
        assert_eq!(guild.widget_channel_id, Some(Snowflake::new(3)));
        assert!(guild.unavailable);
    }

    #[test]
    fn cached_channels_are_queryable() {
        let guild = Guild::new(100_u64, "Test")
            .with_afk_channel_id(2_u64)
            .with_system_channel_id(3_u64)
            .with_widget_channel_id(3_u64)
            .with_channels(vec![
                GuildChannel::new(1_u64, 1_u64, "Category", ChannelType::Category),
                GuildChannel::new(2_u64, 1_u64, "Voice", ChannelType::Voice),
                GuildChannel::new(3_u64, 1_u64, "General", ChannelType::Text),
                GuildChannel::new(4_u64, 1_u64, "News", ChannelType::News),
                GuildChannel::new(5_u64, 1_u64, "Stage", ChannelType::StageVoice),
                GuildChannel::new(6_u64, 1_u64, "Forum", ChannelType::Forum),
                GuildChannel::new(7_u64, 1_u64, "Media", ChannelType::Media),
            ]);

        assert_eq!(guild.channels().len(), 7);
        assert!(
            guild
                .channels()
                .iter()
                .all(|channel| channel.guild_id == guild.id)
        );
        assert_eq!(
            guild
                .get_channel(3_u64)
                .map(|channel| channel.name.as_str()),
            Some("General")
        );
        assert!(guild.get_channel(99_u64).is_none());

        assert_eq!(guild.categories().count(), 1);
        assert_eq!(guild.text_channels().count(), 1);
        assert_eq!(guild.news_channels().count(), 1);
        assert_eq!(guild.voice_channels().count(), 1);
        assert_eq!(guild.stage_channels().count(), 1);
        assert_eq!(guild.forum_channels().count(), 1);
        assert_eq!(guild.media_channels().count(), 1);
        assert_eq!(guild.channels_of_type(ChannelType::Category).count(), 1);

        assert_eq!(
            guild.afk_channel().map(|channel| channel.name.as_str()),
            Some("Voice")
        );
        assert_eq!(
            guild.system_channel().map(|channel| channel.name.as_str()),
            Some("General")
        );
        assert_eq!(
            guild.widget_channel().map(|channel| channel.name.as_str()),
            Some("General")
        );
        assert!(guild.rules_channel().is_none());
        assert!(guild.public_updates_channel().is_none());
        assert!(guild.safety_alerts_channel().is_none());
    }
}
