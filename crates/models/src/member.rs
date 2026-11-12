//! The `Member` model.
//!
//! a [`User`] plus guild-specific data. Discord's
//! model resolves roles through a live `Guild` cache; until the guild model
//! exists, this type stores the resolved [`Role`] snapshots that were supplied
//! when it was built.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

use abc::{Asset, Colour, HasId, Snowflake};
use data::{Object, Value};
use enums::{MemberFlags, Permissions, PublicUserFlags};

use crate::cast::{self, CastError};
use crate::{Role, RoleIcon, User};

/// Represents a Discord member of a guild.
/// Equality and hashing are based on the member's user ID
#[derive(Clone, Debug, Default)]
pub struct Member {
    /// The member's underlying user.
    pub user: User,
    /// The ID of the guild the member belongs to.
    pub guild_id: Snowflake,
    /// The guild-specific nickname, if set.
    pub nick: Option<String>,
    /// The member's roles, sorted from lowest to highest in the hierarchy.
    pub roles: Vec<Role>,
    /// When the member joined the guild, if available.
    pub joined_at: Option<SystemTime>,
    /// When the member started boosting the guild, if applicable.
    pub premium_since: Option<SystemTime>,
    /// Whether the member is pending membership verification.
    pub pending: bool,
    /// Whether the member is deafened in voice channels.
    pub deaf: bool,
    /// Whether the member is muted in voice channels.
    pub mute: bool,
    /// The member's guild member flags.
    pub flags: MemberFlags,
    /// Permissions resolved by Discord, usually from an interaction context.
    pub resolved_permissions: Option<Permissions>,
    /// The member's guild-specific avatar, if any.
    pub guild_avatar: Option<Asset>,
    /// The member's guild-specific banner, if any.
    pub guild_banner: Option<Asset>,
    /// When the member's timeout expires, if they are timed out.
    pub timed_out_until: Option<SystemTime>,
}

impl Member {
    /// Creates a member from a user and a guild ID.
    #[must_use]
    pub fn new(user: User, guild_id: impl Into<Snowflake>) -> Self {
        Self {
            user,
            guild_id: guild_id.into(),
            ..Self::default()
        }
    }

    /// Decodes a guild member from a Discord member object.
    ///
    /// Member payloads reference roles by ID, so the caller supplies the guild's
    /// known roles. Unknown IDs become placeholder roles with only an ID and
    /// the guild ID set.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(
        value: &Value,
        guild_id: impl Into<Snowflake>,
        roles: &[Role],
    ) -> Result<Self, CastError> {
        Self::from_value_at(value, "member", guild_id.into(), roles)
    }

    pub(crate) fn from_value_at(
        value: &Value,
        path: &str,
        guild_id: Snowflake,
        roles: &[Role],
    ) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let user_path = cast::field_path(path, "user");
        let user = User::from_object_at(cast::required_object(object, path, "user")?, &user_path)?;
        Self::from_object_with_user_at(object, path, guild_id, roles, user)
    }

    /// Decodes a member object that does not repeat the nested user object.
    pub(crate) fn from_object_with_user_at(
        object: &Object,
        path: &str,
        guild_id: Snowflake,
        roles: &[Role],
        user: User,
    ) -> Result<Self, CastError> {
        let mut member_roles = Vec::new();
        if let Some(role_ids) = cast::optional_array(object, path, "roles")? {
            let roles_path = cast::field_path(path, "roles");
            member_roles.reserve(role_ids.len());
            for (index, value) in role_ids.iter().enumerate() {
                let id = cast::snowflake(value, &cast::index_path(&roles_path, index))?;
                let role = roles
                    .iter()
                    .find(|role| role.id == id)
                    .cloned()
                    .unwrap_or_else(|| Role::new(guild_id, id, ""));
                member_roles.push(role);
            }
        }
        member_roles.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let guild_avatar = cast::optional_string(object, path, "avatar")?
            .map(|hash| Asset::from_guild_avatar(guild_id, user.id, hash));
        let guild_banner = cast::optional_string(object, path, "banner")?
            .map(|hash| Asset::from_guild_banner(guild_id, user.id, hash));

        Ok(Self {
            user,
            guild_id,
            nick: cast::optional_string(object, path, "nick")?.map(str::to_owned),
            roles: member_roles,
            joined_at: cast::optional_timestamp(object, path, "joined_at")?,
            premium_since: cast::optional_timestamp(object, path, "premium_since")?,
            pending: cast::optional_bool(object, path, "pending", false)?,
            deaf: cast::optional_bool(object, path, "deaf", false)?,
            mute: cast::optional_bool(object, path, "mute", false)?,
            flags: MemberFlags::from_bits(cast::optional_u64(object, path, "flags")?.unwrap_or(0)),
            resolved_permissions: cast::optional_permission_bits(object, path, "permissions")?
                .map(Permissions::from_bits),
            guild_avatar,
            guild_banner,
            timed_out_until: cast::optional_timestamp(
                object,
                path,
                "communication_disabled_until",
            )?,
        })
    }

    /// Sets the guild-specific nickname.
    #[must_use]
    pub fn with_nick(mut self, nick: impl Into<String>) -> Self {
        self.nick = Some(nick.into());
        self
    }

    /// Sets the member's roles.
    /// The roles are sorted from lowest to highest in the hierarchy.
    #[must_use]
    pub fn with_roles(mut self, mut roles: Vec<Role>) -> Self {
        roles.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        self.roles = roles;
        self
    }

    /// Sets when the member joined the guild.
    #[must_use]
    pub fn with_joined_at(mut self, joined_at: SystemTime) -> Self {
        self.joined_at = Some(joined_at);
        self
    }

    /// Sets when the member started boosting the guild.
    #[must_use]
    pub fn with_premium_since(mut self, premium_since: SystemTime) -> Self {
        self.premium_since = Some(premium_since);
        self
    }

    /// Sets whether the member is pending verification.
    #[must_use]
    pub fn with_pending(mut self, pending: bool) -> Self {
        self.pending = pending;
        self
    }

    /// Sets whether the member is deafened.
    #[must_use]
    pub fn with_deaf(mut self, deaf: bool) -> Self {
        self.deaf = deaf;
        self
    }

    /// Sets whether the member is muted.
    #[must_use]
    pub fn with_mute(mut self, mute: bool) -> Self {
        self.mute = mute;
        self
    }

    /// Sets the member's guild member flags.
    #[must_use]
    pub fn with_flags(mut self, flags: MemberFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Sets permissions resolved by Discord.
    #[must_use]
    pub fn with_resolved_permissions(mut self, permissions: Permissions) -> Self {
        self.resolved_permissions = Some(permissions);
        self
    }

    /// Sets the member's guild-specific avatar.
    #[must_use]
    pub fn with_guild_avatar(mut self, avatar: Asset) -> Self {
        self.guild_avatar = Some(avatar);
        self
    }

    /// Sets the member's guild-specific banner.
    #[must_use]
    pub fn with_guild_banner(mut self, banner: Asset) -> Self {
        self.guild_banner = Some(banner);
        self
    }

    /// Sets when the member's timeout expires.
    #[must_use]
    pub fn with_timed_out_until(mut self, timed_out_until: SystemTime) -> Self {
        self.timed_out_until = Some(timed_out_until);
        self
    }

    /// Returns the underlying user.
    #[must_use]
    pub fn user(&self) -> &User {
        &self.user
    }

    /// Returns the member's username.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.user.name
    }

    /// Returns the member's legacy discriminator.
    #[must_use]
    pub fn discriminator(&self) -> &str {
        &self.user.discriminator
    }

    /// Returns the member's global display name, if set.
    #[must_use]
    pub fn global_name(&self) -> Option<&str> {
        self.user.global_name.as_deref()
    }

    /// Returns `true` if the member is a bot account.
    #[must_use]
    pub fn bot(&self) -> bool {
        self.user.bot
    }

    /// Returns `true` if the member is a system account.
    #[must_use]
    pub fn system(&self) -> bool {
        self.user.system
    }

    /// Returns the member's public flags.
    #[must_use]
    pub fn public_flags(&self) -> PublicUserFlags {
        self.user.public_flags
    }

    /// Returns the member's uploaded avatar, if any.
    #[must_use]
    pub fn avatar(&self) -> Option<&Asset> {
        self.user.avatar()
    }

    /// Returns the member's roles, sorted from lowest to highest.
    #[must_use]
    pub fn roles(&self) -> &[Role] {
        &self.roles
    }

    /// Returns the IDs of the member's roles.
    pub fn role_ids(&self) -> impl Iterator<Item = Snowflake> + '_ {
        self.roles.iter().map(|role| role.id)
    }

    /// Returns the role with the given ID, if the member has it.
    #[must_use]
    pub fn get_role(&self, role_id: impl Into<Snowflake>) -> Option<&Role> {
        let role_id = role_id.into();
        self.roles.iter().find(|role| role.id == role_id)
    }

    /// Returns the colour Discord renders for this member.
    /// The highest non-default role with a colour wins
    #[must_use]
    pub fn colour(&self) -> Colour {
        self.roles
            .iter()
            .rev()
            .filter(|role| !role.is_default() && role.colour().value() != 0)
            .map(Role::colour)
            .next()
            .unwrap_or_default()
    }

    /// American spelling alias of [`Member::colour`].
    #[must_use]
    pub fn color(&self) -> Colour {
        self.colour()
    }

    /// Returns the icon displayed for this member, if any.
    #[must_use]
    pub fn display_icon(&self) -> Option<RoleIcon<'_>> {
        self.roles
            .iter()
            .rev()
            .filter(|role| !role.is_default())
            .find_map(Role::display_icon)
    }

    /// Returns the member's display name.
    /// The guild nickname takes precedence over the global name, which takes
    /// precedence over the username.
    #[must_use]
    pub fn display_name(&self) -> &str {
        self.nick
            .as_deref()
            .or(self.user.global_name.as_deref())
            .unwrap_or(&self.user.name)
    }

    /// Returns the avatar the member is displaying.
    /// The guild-specific avatar takes precedence over the user's global
    /// avatar, which takes precedence over the default avatar.
    #[must_use]
    pub fn display_avatar(&self) -> Asset {
        self.guild_avatar
            .clone()
            .unwrap_or_else(|| self.user.display_avatar())
    }

    /// Returns the member's guild-specific avatar, if any.
    #[must_use]
    pub fn guild_avatar(&self) -> Option<&Asset> {
        self.guild_avatar.as_ref()
    }

    /// Returns the banner the member is displaying, if any.
    /// The guild-specific banner takes precedence over the user's global
    /// banner.
    #[must_use]
    pub fn display_banner(&self) -> Option<&Asset> {
        self.guild_banner.as_ref().or(self.user.banner.as_ref())
    }

    /// Returns the member's guild-specific banner, if any.
    #[must_use]
    pub fn guild_banner(&self) -> Option<&Asset> {
        self.guild_banner.as_ref()
    }

    /// Returns the member's highest role.
    #[must_use]
    pub fn top_role(&self) -> Option<&Role> {
        self.roles.last()
    }

    /// Returns the permissions resolved by Discord, if any.
    #[must_use]
    pub fn resolved_permissions(&self) -> Option<Permissions> {
        self.resolved_permissions
    }

    /// Computes the member's guild permissions from their roles.
    ///
    /// `guild_owner_id` is passed explicitly until the `Guild` model exists.
    /// Ownership, the administrator implication and timeouts are handled
    #[must_use]
    pub fn guild_permissions(&self, guild_owner_id: impl Into<Snowflake>) -> Permissions {
        if guild_owner_id.into() == self.user.id {
            return Permissions::all();
        }

        let mut base = Permissions::none();
        for role in &self.roles {
            base.insert(role.permissions());
        }

        if base.administrator() {
            return Permissions::all();
        }

        if self.is_timed_out() {
            base = base.intersection(Permissions::timeout_mask());
        }

        base
    }

    /// Returns the member's guild member flags.
    #[must_use]
    pub fn flags(&self) -> MemberFlags {
        self.flags
    }

    /// Returns `true` if the member's timeout is still active.
    #[must_use]
    pub fn is_timed_out(&self) -> bool {
        self.timed_out_until
            .is_some_and(|until| until > SystemTime::now())
    }

    /// Returns the member's creation time, derived from their user ID.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.user.id.created_at()
    }

    /// Returns a string that mentions this member.
    #[must_use]
    pub fn mention(&self) -> String {
        self.user.mention()
    }

    /// Returns the member's default avatar.
    #[must_use]
    pub fn default_avatar(&self) -> Asset {
        self.user.default_avatar()
    }
}

impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.user.fmt(f)
    }
}

impl PartialEq for Member {
    fn eq(&self, other: &Self) -> bool {
        self.user == other.user
    }
}

impl Eq for Member {}

impl PartialEq<User> for Member {
    fn eq(&self, other: &User) -> bool {
        self.user == *other
    }
}

impl PartialEq<Member> for User {
    fn eq(&self, other: &Member) -> bool {
        *self == other.user
    }
}

impl Hash for Member {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.user.hash(state);
    }
}

impl HasId for Member {
    fn id(&self) -> Snowflake {
        self.user.id
    }
}

impl abc::User for Member {
    fn name(&self) -> &str {
        &self.user.name
    }

    fn discriminator(&self) -> &str {
        &self.user.discriminator
    }

    fn global_name(&self) -> Option<&str> {
        self.user.global_name.as_deref()
    }

    fn bot(&self) -> bool {
        self.user.bot
    }

    fn system(&self) -> bool {
        self.user.system
    }

    fn avatar(&self) -> Option<&Asset> {
        self.user.avatar()
    }

    fn avatar_decoration(&self) -> Option<&Asset> {
        self.user.avatar_decoration()
    }

    fn avatar_decoration_sku_id(&self) -> Option<Snowflake> {
        self.user.avatar_decoration_sku_id()
    }

    fn display_name(&self) -> &str {
        Member::display_name(self)
    }

    fn default_avatar(&self) -> Asset {
        Member::default_avatar(self)
    }

    fn display_avatar(&self) -> Asset {
        Member::display_avatar(self)
    }

    fn mention(&self) -> String {
        Member::mention(self)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use abc::{Asset, Colour, HasId, Snowflake};
    use data::json;
    use enums::{MemberFlags, Permissions, PublicUserFlags};

    use super::Member;
    use crate::{Role, RoleIcon, User};

    fn sample_role(guild_id: u64, id: u64, name: &str, position: i64) -> Role {
        Role::new(guild_id, id, name).with_position(position)
    }

    fn sample_member() -> Member {
        Member::new(
            User::new(42_u64, "winter").with_global_name("Winter"),
            100_u64,
        )
    }

    #[test]
    fn constructor_and_display_name_precedence() {
        let member = sample_member();
        assert_eq!(member.id(), Snowflake::new(42));
        assert_eq!(member.guild_id, Snowflake::new(100));
        assert_eq!(member.display_name(), "Winter");

        let member = sample_member().with_nick("Frost");
        assert_eq!(member.display_name(), "Frost");

        let member = Member::new(User::new(42_u64, "winter"), 100_u64);
        assert_eq!(member.display_name(), "winter");
    }

    #[test]
    fn mention_and_display() {
        let member = sample_member();
        assert_eq!(member.mention(), "<@42>");
        assert_eq!(member.to_string(), "winter");
    }

    #[test]
    fn guild_avatar_and_banner_take_precedence() {
        let guild_avatar =
            Asset::from_guild_avatar(Snowflake::new(100), Snowflake::new(42), "guild");
        let guild_banner =
            Asset::from_guild_banner(Snowflake::new(100), Snowflake::new(42), "banner");
        let user_avatar = Asset::from_avatar(Snowflake::new(42), "global");
        let user_banner = Asset::from_user_banner(Snowflake::new(42), "global-banner");

        let member = Member::new(
            User::new(42_u64, "winter").with_avatar(user_avatar.clone()),
            100_u64,
        )
        .with_guild_avatar(guild_avatar.clone())
        .with_guild_banner(guild_banner.clone());

        assert_eq!(member.avatar(), Some(&user_avatar));
        assert_eq!(member.guild_avatar(), Some(&guild_avatar));
        assert_eq!(member.display_avatar(), guild_avatar);
        assert_eq!(member.guild_banner(), Some(&guild_banner));
        assert_eq!(member.display_banner(), Some(&guild_banner));

        let member = Member::new(
            User::new(42_u64, "winter")
                .with_avatar(user_avatar.clone())
                .with_banner(user_banner.clone()),
            100_u64,
        );
        assert_eq!(member.display_avatar(), user_avatar);
        assert_eq!(member.display_banner(), Some(&user_banner));
    }

    #[test]
    fn roles_are_sorted_and_resolved() {
        let low = sample_role(100, 10, "Low", 1);
        let high = sample_role(100, 20, "High", 2);
        let member = sample_member().with_roles(vec![high.clone(), low.clone()]);

        assert_eq!(member.roles(), &[low.clone(), high.clone()]);
        assert_eq!(
            member.role_ids().collect::<Vec<_>>(),
            vec![Snowflake::new(10), Snowflake::new(20)]
        );
        assert!(member.get_role(10_u64).is_some());
        assert!(member.get_role(99_u64).is_none());
        assert_eq!(member.top_role(), Some(&high));
    }

    #[test]
    fn colour_uses_highest_coloured_role() {
        let everyone = sample_role(100, 100, "@everyone", 0);
        let red = sample_role(100, 10, "Red", 1).with_colour(Colour::RED);
        let colourless = sample_role(100, 20, "Colourless", 2);
        let blue = sample_role(100, 30, "Blue", 3).with_colour(Colour::BLUE);

        let member = sample_member().with_roles(vec![everyone, red, colourless, blue]);
        assert_eq!(member.colour(), Colour::BLUE);
        assert_eq!(member.color(), Colour::BLUE);

        let member = sample_member().with_roles(vec![
            sample_role(100, 100, "@everyone", 0).with_colour(Colour::GREEN),
            sample_role(100, 10, "Colourless", 1),
        ]);
        assert_eq!(member.colour(), Colour::default());
    }

    #[test]
    fn display_icon_uses_highest_role_icon() {
        let lower = sample_role(100, 10, "Lower", 1).with_unicode_emoji("🔥");
        let higher = sample_role(100, 20, "Higher", 2);
        let member = sample_member().with_roles(vec![lower, higher]);

        assert_eq!(member.display_icon(), Some(RoleIcon::Unicode("🔥")));

        let member = sample_member().with_roles(vec![sample_role(100, 10, "Plain", 1)]);
        assert_eq!(member.display_icon(), None);
    }

    #[test]
    fn guild_permissions_consider_owner_admin_and_timeout() {
        let role = sample_role(100, 10, "Role", 1).with_permissions(
            Permissions::VIEW_CHANNEL
                | Permissions::READ_MESSAGE_HISTORY
                | Permissions::SEND_MESSAGES,
        );

        let member = sample_member().with_roles(vec![role.clone()]);
        assert_eq!(
            member.guild_permissions(999_u64),
            Permissions::VIEW_CHANNEL
                | Permissions::READ_MESSAGE_HISTORY
                | Permissions::SEND_MESSAGES
        );
        assert_eq!(member.guild_permissions(42_u64), Permissions::all());

        let admin = sample_role(100, 11, "Admin", 1).with_permissions(Permissions::ADMINISTRATOR);
        let member = sample_member().with_roles(vec![admin]);
        assert_eq!(member.guild_permissions(999_u64), Permissions::all());

        let member = sample_member()
            .with_roles(vec![role])
            .with_timed_out_until(SystemTime::now() + Duration::from_secs(60));
        assert_eq!(
            member.guild_permissions(999_u64),
            Permissions::VIEW_CHANNEL | Permissions::READ_MESSAGE_HISTORY
        );
    }

    #[test]
    fn timeout_and_resolved_permissions() {
        let member = sample_member();
        assert!(!member.is_timed_out());

        let member = member.with_timed_out_until(SystemTime::now() - Duration::from_secs(60));
        assert!(!member.is_timed_out());

        let member = member.with_timed_out_until(SystemTime::now() + Duration::from_secs(60));
        assert!(member.is_timed_out());
        assert_eq!(member.resolved_permissions(), None);

        let member = member.with_resolved_permissions(Permissions::ADMINISTRATOR);
        assert_eq!(
            member.resolved_permissions(),
            Some(Permissions::ADMINISTRATOR)
        );
    }

    #[test]
    fn flags_and_user_fields() {
        let user = User::new(42_u64, "winter")
            .with_bot(true)
            .with_public_flags(PublicUserFlags::VERIFIED_BOT);
        let member =
            Member::new(user, 100_u64).with_flags(MemberFlags::DID_REJOIN | MemberFlags::GUEST);

        assert!(member.bot());
        assert!(!member.system());
        assert_eq!(member.public_flags(), PublicUserFlags::VERIFIED_BOT);
        assert_eq!(member.flags(), MemberFlags::DID_REJOIN | MemberFlags::GUEST);
        assert!(member.flags().did_rejoin());
        assert_eq!(member.user().name, "winter");
    }

    #[test]
    fn equality_and_hash_use_user_id_only() {
        let a = Member::new(User::new(1_u64, "first"), 100_u64);
        let b = Member::new(User::new(1_u64, "second").with_bot(true), 200_u64).with_nick("nick");
        let c = Member::new(User::new(2_u64, "first"), 100_u64);

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a == b.user);
        assert!(a.user == b);

        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        a.hash(&mut hasher_a);
        b.hash(&mut hasher_b);
        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }

    #[test]
    fn trait_accessors_use_guild_specific_values() {
        use abc::User as UserTrait;

        let member =
            sample_member()
                .with_nick("Frost")
                .with_guild_avatar(Asset::from_guild_avatar(
                    Snowflake::new(100),
                    Snowflake::new(42),
                    "guild",
                ));

        assert_eq!(UserTrait::display_name(&member), "Frost");
        assert!(
            UserTrait::display_avatar(&member)
                .url()
                .contains("/guilds/100/users/42/avatars/guild.png")
        );
        assert_eq!(UserTrait::mention(&member), "<@42>");
        assert_eq!(UserTrait::name(&member), "winter");
    }

    #[test]
    fn decodes_a_full_member_payload() {
        let value = json::from_str(
            r#"{
                "user": {
                    "id": "42",
                    "username": "winter",
                    "global_name": "Winter",
                    "bot": false
                },
                "nick": "Frost",
                "roles": ["10", "20"],
                "joined_at": "2021-08-01T12:34:56+00:00",
                "premium_since": "2022-01-02T03:04:05.500Z",
                "pending": true,
                "deaf": true,
                "mute": true,
                "flags": 1,
                "permissions": "8",
                "avatar": "guildavatar",
                "banner": "guildbanner",
                "communication_disabled_until": "2023-01-01T00:00:00Z"
            }"#,
        )
        .unwrap();

        let roles = vec![
            Role::new(100_u64, 20_u64, "Higher").with_position(2),
            Role::new(100_u64, 10_u64, "Lower").with_position(1),
        ];
        let member = Member::from_value(&value, 100_u64, &roles).unwrap();

        assert_eq!(member.user.id, Snowflake::new(42));
        assert_eq!(member.guild_id, Snowflake::new(100));
        assert_eq!(member.nick.as_deref(), Some("Frost"));
        assert_eq!(member.roles.len(), 2);
        assert_eq!(member.roles[0].name, "Lower");
        assert_eq!(member.roles[1].name, "Higher");
        assert_eq!(
            member.joined_at,
            Some(UNIX_EPOCH + Duration::new(1_627_821_296, 0))
        );
        assert_eq!(
            member.premium_since,
            Some(UNIX_EPOCH + Duration::new(1_641_092_645, 500_000_000))
        );
        assert!(member.pending);
        assert!(member.deaf);
        assert!(member.mute);
        assert_eq!(member.flags, MemberFlags::DID_REJOIN);
        assert_eq!(
            member.resolved_permissions,
            Some(Permissions::ADMINISTRATOR)
        );
        assert_eq!(
            member.guild_avatar.as_ref().map(Asset::key),
            Some("guildavatar")
        );
        assert_eq!(
            member.guild_banner.as_ref().map(Asset::key),
            Some("guildbanner")
        );
        assert_eq!(
            member.timed_out_until,
            Some(UNIX_EPOCH + Duration::new(1_672_531_200, 0))
        );
    }

    #[test]
    fn member_role_ids_resolve_against_supplied_roles() {
        let value = json::from_str(
            r#"{
                "user": {"id": "42", "username": "winter"},
                "roles": ["10", "999", "20"]
            }"#,
        )
        .unwrap();
        let roles = vec![
            Role::new(100_u64, 10_u64, "Lower").with_position(1),
            Role::new(100_u64, 20_u64, "Higher").with_position(2),
        ];

        let member = Member::from_value(&value, 100_u64, &roles).unwrap();

        assert_eq!(member.roles.len(), 3);
        assert_eq!(member.roles[0].id, Snowflake::new(999));
        assert_eq!(member.roles[0].name, "");
        assert_eq!(member.roles[0].guild_id, Snowflake::new(100));
        assert_eq!(member.roles[1].id, Snowflake::new(10));
        assert_eq!(member.roles[2].id, Snowflake::new(20));
        assert_eq!(member.nick, None);
        assert_eq!(member.joined_at, None);
        assert_eq!(member.guild_avatar, None);
    }

    #[test]
    fn decodes_minimal_members_and_reports_errors() {
        let value = json::from_str(r#"{"user":{"id":"42","username":"winter"}}"#).unwrap();
        let member = Member::from_value(&value, 100_u64, &[]).unwrap();
        assert!(member.roles.is_empty());
        assert!(!member.pending);
        assert!(!member.deaf);
        assert!(!member.mute);
        assert_eq!(member.flags, MemberFlags::NONE);
        assert_eq!(member.resolved_permissions, None);
        assert_eq!(member.timed_out_until, None);

        let missing_user = json::from_str(r#"{"nick":"Frost"}"#).unwrap();
        let error = Member::from_value(&missing_user, 100_u64, &[]).unwrap_err();
        assert_eq!(error.to_string(), "missing field `member.user`");

        let bad_user = json::from_str(r#"{"user":"42"}"#).unwrap();
        let error = Member::from_value(&bad_user, 100_u64, &[]).unwrap_err();
        assert_eq!(error.to_string(), "field `member.user` must be an object");

        let bad_role =
            json::from_str(r#"{"user":{"id":"42","username":"winter"},"roles":[true]}"#).unwrap();
        let error = Member::from_value(&bad_role, 100_u64, &[]).unwrap_err();
        assert_eq!(
            error.to_string(),
            "field `member.roles[0]` must be a snowflake"
        );

        let bad_timestamp =
            json::from_str(r#"{"user":{"id":"42","username":"winter"},"joined_at":"yesterday"}"#)
                .unwrap();
        let error = Member::from_value(&bad_timestamp, 100_u64, &[]).unwrap_err();
        assert_eq!(
            error.to_string(),
            "field `member.joined_at` must be a valid ISO 8601 timestamp"
        );
    }
}
