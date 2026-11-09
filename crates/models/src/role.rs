//! The `Role` model.
//!
//! Guild membership is represented by `guild_id`
//! rather than a full `Guild` reference, so roles can be used before the
//! guild model exists.

use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

use abc::{Asset, Colour, HasId, Snowflake};
use enums::{Permissions, RoleFlags};

/// Extra information attached to a managed role.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RoleTags {
    /// The bot's user ID that manages this role.
    pub bot_id: Option<Snowflake>,
    /// The integration ID that manages the role.
    pub integration_id: Option<Snowflake>,
    /// The ID of this role's subscription SKU and listing.
    pub subscription_listing_id: Option<Snowflake>,
    /// Whether this is the guild's premium subscriber ("boost") role.
    pub premium_subscriber: bool,
    /// Whether the role is available for purchase.
    pub available_for_purchase: bool,
    /// Whether the role is a guild's linked role.
    pub guild_connection: bool,
}

impl RoleTags {
    /// Creates an empty tag set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if the role is associated with a bot.
    #[must_use]
    pub const fn is_bot_managed(&self) -> bool {
        self.bot_id.is_some()
    }

    /// Returns `true` if the role is managed by an integration.
    #[must_use]
    pub const fn is_integration(&self) -> bool {
        self.integration_id.is_some()
    }

    /// Returns `true` if the role is the premium subscriber role.
    #[must_use]
    pub const fn is_premium_subscriber(&self) -> bool {
        self.premium_subscriber
    }

    /// Returns `true` if the role is available for purchase.
    #[must_use]
    pub const fn is_available_for_purchase(&self) -> bool {
        self.available_for_purchase
    }

    /// Returns `true` if the role is a guild's linked role.
    #[must_use]
    pub const fn is_guild_connection(&self) -> bool {
        self.guild_connection
    }
}

/// The icon displayed for a role.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoleIcon<'a> {
    /// A CDN asset.
    Asset(&'a Asset),
    /// A unicode emoji.
    Unicode(&'a str),
}

impl fmt::Display for RoleIcon<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Asset(asset) => asset.fmt(f),
            Self::Unicode(emoji) => f.write_str(emoji),
        }
    }
}

/// Represents a Discord role in a guild.
/// Equality and hashing are based on the role's ID
#[derive(Clone, Debug, Default)]
pub struct Role {
    /// The ID of the guild the role belongs to.
    pub guild_id: Snowflake,
    /// The role's unique ID.
    pub id: Snowflake,
    /// The role's name.
    pub name: String,
    /// The role's primary colour.
    pub colour: Colour,
    /// The role's secondary colour, when the role uses a gradient.
    pub secondary_colour: Option<Colour>,
    /// The role's tertiary colour, when the role uses a gradient.
    pub tertiary_colour: Option<Colour>,
    /// Whether the role is displayed separately from other members.
    pub hoist: bool,
    /// The role's position in the hierarchy. The bottom role is `0`.
    pub position: i64,
    /// The permissions granted by this role.
    pub permissions: Permissions,
    /// Whether the role is managed by an integration.
    pub managed: bool,
    /// Whether the role can be mentioned by users.
    pub mentionable: bool,
    /// The role's icon, if it has one.
    pub icon: Option<Asset>,
    /// The role's unicode emoji, if it has one.
    pub unicode_emoji: Option<String>,
    /// The role's flags.
    pub flags: RoleFlags,
    /// Extra information attached to a managed role.
    pub tags: Option<RoleTags>,
}

impl Role {
    /// Creates a role with the fields Discord always provides.
    #[must_use]
    pub fn new(
        guild_id: impl Into<Snowflake>,
        id: impl Into<Snowflake>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            guild_id: guild_id.into(),
            id: id.into(),
            name: name.into(),
            ..Self::default()
        }
    }

    /// Sets the primary colour.
    #[must_use]
    pub fn with_colour(mut self, colour: Colour) -> Self {
        self.colour = colour;
        self
    }

    /// Sets the secondary colour.
    #[must_use]
    pub fn with_secondary_colour(mut self, colour: Colour) -> Self {
        self.secondary_colour = Some(colour);
        self
    }

    /// Sets the tertiary colour.
    #[must_use]
    pub fn with_tertiary_colour(mut self, colour: Colour) -> Self {
        self.tertiary_colour = Some(colour);
        self
    }

    /// Sets whether the role is hoisted.
    #[must_use]
    pub fn with_hoist(mut self, hoist: bool) -> Self {
        self.hoist = hoist;
        self
    }

    /// Sets the role's hierarchy position.
    #[must_use]
    pub fn with_position(mut self, position: i64) -> Self {
        self.position = position;
        self
    }

    /// Sets the permissions granted by the role.
    #[must_use]
    pub fn with_permissions(mut self, permissions: Permissions) -> Self {
        self.permissions = permissions;
        self
    }

    /// Sets whether the role is managed by an integration.
    #[must_use]
    pub fn with_managed(mut self, managed: bool) -> Self {
        self.managed = managed;
        self
    }

    /// Sets whether the role is mentionable.
    #[must_use]
    pub fn with_mentionable(mut self, mentionable: bool) -> Self {
        self.mentionable = mentionable;
        self
    }

    /// Sets the role icon from a CDN icon hash.
    #[must_use]
    pub fn with_icon_hash(mut self, icon_hash: &str) -> Self {
        self.icon = Some(Asset::from_role_icon(self.id, icon_hash));
        self
    }

    /// Sets the role icon asset.
    #[must_use]
    pub fn with_icon(mut self, icon: Asset) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the role's unicode emoji.
    #[must_use]
    pub fn with_unicode_emoji(mut self, emoji: impl Into<String>) -> Self {
        self.unicode_emoji = Some(emoji.into());
        self
    }

    /// Sets the role flags.
    #[must_use]
    pub fn with_flags(mut self, flags: RoleFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Sets the role tags.
    #[must_use]
    pub fn with_tags(mut self, tags: RoleTags) -> Self {
        self.tags = Some(tags);
        self
    }

    /// Returns `true` if this is the guild's `@everyone` role.
    #[must_use]
    pub fn is_default(&self) -> bool {
        self.id == self.guild_id
    }

    /// Returns `true` if the role is associated with a bot.
    #[must_use]
    pub fn is_bot_managed(&self) -> bool {
        self.tags.as_ref().is_some_and(RoleTags::is_bot_managed)
    }

    /// Returns `true` if the role is managed by an integration.
    #[must_use]
    pub fn is_integration(&self) -> bool {
        self.tags.as_ref().is_some_and(RoleTags::is_integration)
    }

    /// Returns `true` if the role is the premium subscriber role.
    #[must_use]
    pub fn is_premium_subscriber(&self) -> bool {
        self.tags
            .as_ref()
            .is_some_and(RoleTags::is_premium_subscriber)
    }

    /// Returns `true` if the role is available for purchase.
    #[must_use]
    pub fn is_available_for_purchase(&self) -> bool {
        self.tags
            .as_ref()
            .is_some_and(RoleTags::is_available_for_purchase)
    }

    /// Returns `true` if the role is a guild's linked role.
    #[must_use]
    pub fn is_guild_connection(&self) -> bool {
        self.tags
            .as_ref()
            .is_some_and(RoleTags::is_guild_connection)
    }

    /// Returns the role's primary colour.
    #[must_use]
    pub fn colour(&self) -> Colour {
        self.colour
    }

    /// American spelling alias of [`Role::colour`].
    #[must_use]
    pub fn color(&self) -> Colour {
        self.colour
    }

    /// Returns the role's secondary colour, if any.
    #[must_use]
    pub fn secondary_colour(&self) -> Option<Colour> {
        self.secondary_colour
    }

    /// American spelling alias of [`Role::secondary_colour`].
    #[must_use]
    pub fn secondary_color(&self) -> Option<Colour> {
        self.secondary_colour
    }

    /// Returns the role's tertiary colour, if any.
    #[must_use]
    pub fn tertiary_colour(&self) -> Option<Colour> {
        self.tertiary_colour
    }

    /// American spelling alias of [`Role::tertiary_colour`].
    #[must_use]
    pub fn tertiary_color(&self) -> Option<Colour> {
        self.tertiary_colour
    }

    /// Returns the role's permissions.
    #[must_use]
    pub fn permissions(&self) -> Permissions {
        self.permissions
    }

    /// Returns the role's icon, if any.
    #[must_use]
    pub fn icon(&self) -> Option<&Asset> {
        self.icon.as_ref()
    }

    /// Returns the icon displayed for this role.
    /// The CDN icon takes precedence over the unicode emoji.
    #[must_use]
    pub fn display_icon(&self) -> Option<RoleIcon<'_>> {
        self.icon
            .as_ref()
            .map(RoleIcon::Asset)
            .or_else(|| self.unicode_emoji.as_deref().map(RoleIcon::Unicode))
    }

    /// Returns the role's flags.
    #[must_use]
    pub fn flags(&self) -> RoleFlags {
        self.flags
    }

    /// Returns the role's creation time.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.id.created_at()
    }

    /// Returns a string that mentions this role.
    #[must_use]
    pub fn mention(&self) -> String {
        format!("<@&{}>", self.id)
    }

    /// Compares two roles in the hierarchy.
    /// Returns `None` when the roles belong to different guilds
    #[must_use]
    pub fn hierarchy_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.guild_id != other.guild_id {
            return None;
        }

        // The @everyone role is always the lowest role in the hierarchy.
        if self.id == self.guild_id {
            return Some(if other.id == self.guild_id {
                Ordering::Equal
            } else {
                Ordering::Less
            });
        }

        match self.position.cmp(&other.position) {
            Ordering::Equal => Some(other.id.cmp(&self.id)),
            ordering => Some(ordering),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl PartialEq for Role {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Role {}

impl PartialOrd for Role {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hierarchy_cmp(other)
    }
}

impl Hash for Role {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for Role {
    fn id(&self) -> Snowflake {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use abc::{Asset, Colour, HasId, Snowflake};
    use enums::{Permissions, RoleFlags};

    use super::{Role, RoleIcon, RoleTags};

    fn role(guild_id: u64, id: u64, name: &str, position: i64) -> Role {
        Role::new(guild_id, id, name).with_position(position)
    }

    #[test]
    fn constructor_and_builders() {
        let icon = Asset::from_role_icon(Snowflake::new(45), "hash");
        let tags = RoleTags {
            bot_id: Some(Snowflake::new(99)),
            ..RoleTags::default()
        };
        let role = Role::new(1_u64, 45_u64, "Moderator")
            .with_colour(Colour::BLURPLE)
            .with_secondary_colour(Colour::RED)
            .with_tertiary_colour(Colour::GREEN)
            .with_hoist(true)
            .with_position(5)
            .with_permissions(Permissions::ADMINISTRATOR)
            .with_managed(true)
            .with_mentionable(true)
            .with_icon(icon.clone())
            .with_unicode_emoji("🛡")
            .with_flags(RoleFlags::IN_PROMPT)
            .with_tags(tags.clone());

        assert_eq!(role.guild_id, Snowflake::new(1));
        assert_eq!(role.id(), Snowflake::new(45));
        assert_eq!(role.name, "Moderator");
        assert_eq!(role.colour(), Colour::BLURPLE);
        assert_eq!(role.color(), Colour::BLURPLE);
        assert_eq!(role.secondary_colour(), Some(Colour::RED));
        assert_eq!(role.secondary_color(), Some(Colour::RED));
        assert_eq!(role.tertiary_colour(), Some(Colour::GREEN));
        assert_eq!(role.tertiary_color(), Some(Colour::GREEN));
        assert!(role.hoist);
        assert_eq!(role.position, 5);
        assert_eq!(role.permissions(), Permissions::ADMINISTRATOR);
        assert!(role.managed);
        assert!(role.mentionable);
        assert_eq!(role.icon(), Some(&icon));
        assert_eq!(role.unicode_emoji.as_deref(), Some("🛡"));
        assert_eq!(role.flags(), RoleFlags::IN_PROMPT);
        assert_eq!(role.tags, Some(tags));
        assert!(role.is_bot_managed());
    }

    #[test]
    fn display_icon_prefers_asset() {
        let role = Role::new(1_u64, 2_u64, "Role").with_unicode_emoji("🔥");
        assert_eq!(role.display_icon(), Some(RoleIcon::Unicode("🔥")));
        assert_eq!(role.display_icon().unwrap().to_string(), "🔥");

        let role = role.with_icon_hash("icon-hash");
        let icon = role.display_icon().unwrap();
        assert!(matches!(icon, RoleIcon::Asset(_)));
        assert_eq!(
            icon.to_string(),
            "https://cdn.discordapp.com/role-icons/2/icon-hash.png?size=1024"
        );
    }

    #[test]
    fn default_role_and_tags() {
        let everyone = Role::new(1_u64, 1_u64, "@everyone");
        assert!(everyone.is_default());
        assert!(!everyone.is_bot_managed());
        assert!(!everyone.is_integration());
        assert!(!everyone.is_premium_subscriber());
        assert!(!everyone.is_available_for_purchase());
        assert!(!everyone.is_guild_connection());

        let tags = RoleTags {
            integration_id: Some(Snowflake::new(3)),
            premium_subscriber: true,
            available_for_purchase: true,
            guild_connection: true,
            ..RoleTags::default()
        };
        let role = Role::new(1_u64, 2_u64, "Boost").with_tags(tags);
        assert!(role.is_integration());
        assert!(role.is_premium_subscriber());
        assert!(role.is_available_for_purchase());
        assert!(role.is_guild_connection());
    }

    #[test]
    fn mention_and_display() {
        let role = Role::new(1_u64, 45_u64, "Moderator");
        assert_eq!(role.mention(), "<@&45>");
        assert_eq!(role.to_string(), "Moderator");
    }

    #[test]
    fn hierarchy_comparison() {
        let everyone = role(1, 1, "@everyone", 0);
        let low = role(1, 10, "Low", 1);
        let high = role(1, 20, "High", 2);
        let same_position = role(1, 30, "Same position", 2);

        assert_eq!(everyone.hierarchy_cmp(&low), Some(Ordering::Less));
        assert_eq!(low.hierarchy_cmp(&everyone), Some(Ordering::Greater));
        assert_eq!(low.hierarchy_cmp(&high), Some(Ordering::Less));
        assert!(high > low);
        // At equal positions the higher ID is the lower role.
        assert_eq!(high.hierarchy_cmp(&same_position), Some(Ordering::Greater));
        assert_eq!(everyone.hierarchy_cmp(&everyone), Some(Ordering::Equal));

        let other_guild = role(2, 10, "Other", 1);
        assert_eq!(low.hierarchy_cmp(&other_guild), None);
        assert!(low.partial_cmp(&other_guild).is_none());
    }

    #[test]
    fn equality_and_hash_use_id_only() {
        let a = Role::new(1_u64, 10_u64, "First");
        let b = Role::new(2_u64, 10_u64, "Second").with_position(9);
        let c = Role::new(1_u64, 11_u64, "First");

        assert_eq!(a, b);
        assert_ne!(a, c);

        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        a.hash(&mut hasher_a);
        b.hash(&mut hasher_b);
        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }
}
