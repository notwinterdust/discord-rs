//! The `User` model.
//! Guild-specific and collectible fields will be added with
//! the `Member` model.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

use abc::{Asset, Colour, HasId, Snowflake};
use data::{Object, Value};
use enums::PublicUserFlags;

use crate::cast::{self, CastError};

/// Represents a Discord user.
/// Equality and hashing are based on the user's ID.
#[derive(Clone, Debug, Default)]
pub struct User {
    /// The user's unique ID.
    pub id: Snowflake,
    /// The user's username.
    pub name: String,
    /// The user's legacy discriminator (`"0"` for new usernames).
    pub discriminator: String,
    /// The user's global display name, if set.
    pub global_name: Option<String>,
    /// The user's uploaded avatar, if any.
    pub avatar: Option<Asset>,
    /// Whether the user is a bot account.
    pub bot: bool,
    /// Whether the user is a system account.
    pub system: bool,
    /// The user's banner, when available.
    pub banner: Option<Asset>,
    /// The user's accent colour, when available.
    pub accent_colour: Option<Colour>,
    /// The user's avatar decoration, when available.
    pub avatar_decoration: Option<Asset>,
    /// The SKU ID of the user's avatar decoration, when available.
    pub avatar_decoration_sku_id: Option<Snowflake>,
    /// The user's public flags.
    pub public_flags: PublicUserFlags,
}

impl User {
    /// Creates a user with the fields Discord always provides.
    /// The discriminator defaults to `"0"`, matching new Discord usernames.
    #[must_use]
    pub fn new(id: impl Into<Snowflake>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            discriminator: "0".to_owned(),
            ..Self::default()
        }
    }

    /// Decodes a user from a Discord user object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "user")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Self::from_object_at(object, path)
    }

    pub(crate) fn from_object_at(object: &Object, path: &str) -> Result<Self, CastError> {
        let id = cast::required_snowflake(object, path, "id")?;
        let name = cast::required_string(object, path, "username")?.to_owned();
        let discriminator = cast::optional_string(object, path, "discriminator")?
            .unwrap_or("0")
            .to_owned();
        let global_name = cast::optional_string(object, path, "global_name")?.map(str::to_owned);
        let avatar =
            cast::optional_string(object, path, "avatar")?.map(|hash| Asset::from_avatar(id, hash));
        let bot = cast::optional_bool(object, path, "bot", false)?;
        let system = cast::optional_bool(object, path, "system", false)?;
        let banner = cast::optional_string(object, path, "banner")?
            .map(|hash| Asset::from_user_banner(id, hash));
        let accent_colour = cast::optional_colour(object, path, "accent_color")?;

        let decoration_path = cast::field_path(path, "avatar_decoration_data");
        let (avatar_decoration, avatar_decoration_sku_id) =
            match cast::optional_object(object, path, "avatar_decoration_data")? {
                Some(decoration) => (
                    cast::optional_string(decoration, &decoration_path, "asset")?
                        .map(Asset::from_avatar_decoration),
                    cast::optional_snowflake(decoration, &decoration_path, "sku_id")?,
                ),
                None => (
                    cast::optional_string(object, path, "avatar_decoration")?
                        .map(Asset::from_avatar_decoration),
                    None,
                ),
            };

        let public_flags = PublicUserFlags::from_bits(
            cast::optional_u64(object, path, "public_flags")?.unwrap_or(0),
        );

        Ok(Self {
            id,
            name,
            discriminator,
            global_name,
            avatar,
            bot,
            system,
            banner,
            accent_colour,
            avatar_decoration,
            avatar_decoration_sku_id,
            public_flags,
        })
    }

    /// Sets the legacy discriminator.
    #[must_use]
    pub fn with_discriminator(mut self, discriminator: impl Into<String>) -> Self {
        self.discriminator = discriminator.into();
        self
    }

    /// Sets the global display name.
    #[must_use]
    pub fn with_global_name(mut self, global_name: impl Into<String>) -> Self {
        self.global_name = Some(global_name.into());
        self
    }

    /// Sets the uploaded avatar.
    #[must_use]
    pub fn with_avatar(mut self, avatar: Asset) -> Self {
        self.avatar = Some(avatar);
        self
    }

    /// Marks the user as a bot account.
    #[must_use]
    pub fn with_bot(mut self, bot: bool) -> Self {
        self.bot = bot;
        self
    }

    /// Marks the user as a system account.
    #[must_use]
    pub fn with_system(mut self, system: bool) -> Self {
        self.system = system;
        self
    }

    /// Sets the user's banner.
    #[must_use]
    pub fn with_banner(mut self, banner: Asset) -> Self {
        self.banner = Some(banner);
        self
    }

    /// Sets the user's accent colour.
    #[must_use]
    pub fn with_accent_colour(mut self, accent_colour: Colour) -> Self {
        self.accent_colour = Some(accent_colour);
        self
    }

    /// Sets the user's avatar decoration.
    #[must_use]
    pub fn with_avatar_decoration(mut self, avatar_decoration: Asset) -> Self {
        self.avatar_decoration = Some(avatar_decoration);
        self
    }

    /// Sets the avatar decoration SKU ID.
    #[must_use]
    pub fn with_avatar_decoration_sku_id(mut self, sku_id: impl Into<Snowflake>) -> Self {
        self.avatar_decoration_sku_id = Some(sku_id.into());
        self
    }

    /// Sets the user's public flags.
    #[must_use]
    pub fn with_public_flags(mut self, public_flags: PublicUserFlags) -> Self {
        self.public_flags = public_flags;
        self
    }

    /// Returns the user's uploaded avatar, if any.
    #[must_use]
    pub fn avatar(&self) -> Option<&Asset> {
        self.avatar.as_ref()
    }

    /// Returns the user's avatar decoration, if any.
    #[must_use]
    pub fn avatar_decoration(&self) -> Option<&Asset> {
        self.avatar_decoration.as_ref()
    }

    /// Returns the avatar decoration SKU ID, if any.
    #[must_use]
    pub fn avatar_decoration_sku_id(&self) -> Option<Snowflake> {
        self.avatar_decoration_sku_id
    }

    /// Returns the user's display name.
    /// The global name takes precedence over the username.
    #[must_use]
    pub fn display_name(&self) -> &str {
        <Self as abc::User>::display_name(self)
    }

    /// Returns the default avatar for this user.
    #[must_use]
    pub fn default_avatar(&self) -> Asset {
        <Self as abc::User>::default_avatar(self)
    }

    /// Returns the avatar the user is displaying.
    #[must_use]
    pub fn display_avatar(&self) -> Asset {
        <Self as abc::User>::display_avatar(self)
    }

    /// Returns a string that mentions this user.
    #[must_use]
    pub fn mention(&self) -> String {
        <Self as abc::User>::mention(self)
    }

    /// Returns the user's creation time.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.id.created_at()
    }

    /// Returns the user's accent colour.
    /// American spelling alias of [`User::accent_colour`].
    #[must_use]
    pub fn accent_color(&self) -> Option<Colour> {
        self.accent_colour
    }

    /// Returns the user's accent colour.
    #[must_use]
    pub fn accent_colour(&self) -> Option<Colour> {
        self.accent_colour
    }

    /// Returns the colour used to render this user.
    /// Regular users always render with the default colour
    #[must_use]
    pub fn colour(&self) -> Colour {
        Colour::default()
    }

    /// American spelling alias of [`User::colour`].
    #[must_use]
    pub fn color(&self) -> Colour {
        self.colour()
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.discriminator == "0" {
            f.write_str(&self.name)
        } else {
            write!(f, "{}#{}", self.name, self.discriminator)
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}

impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for User {
    fn id(&self) -> Snowflake {
        self.id
    }
}

impl abc::User for User {
    fn name(&self) -> &str {
        &self.name
    }

    fn discriminator(&self) -> &str {
        &self.discriminator
    }

    fn global_name(&self) -> Option<&str> {
        self.global_name.as_deref()
    }

    fn bot(&self) -> bool {
        self.bot
    }

    fn system(&self) -> bool {
        self.system
    }

    fn avatar(&self) -> Option<&Asset> {
        self.avatar.as_ref()
    }

    fn avatar_decoration(&self) -> Option<&Asset> {
        self.avatar_decoration.as_ref()
    }

    fn avatar_decoration_sku_id(&self) -> Option<Snowflake> {
        self.avatar_decoration_sku_id
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use abc::{Asset, Colour, HasId, Snowflake};
    use data::json;
    use enums::PublicUserFlags;

    use super::User;

    #[test]
    fn constructor_uses_modern_username_defaults() {
        let user = User::new(123_u64, "winter");

        assert_eq!(user.id, Snowflake::new(123));
        assert_eq!(user.name, "winter");
        assert_eq!(user.discriminator, "0");
        assert_eq!(user.global_name, None);
        assert_eq!(user.avatar, None);
        assert!(!user.bot);
        assert!(!user.system);
        assert_eq!(user.public_flags, PublicUserFlags::none());
        assert_eq!(user.to_string(), "winter");
    }

    #[test]
    fn display_name_prefers_global_name() {
        let user = User::new(1_u64, "winter").with_global_name("Winter");

        assert_eq!(user.display_name(), "Winter");
        assert_eq!(User::new(1_u64, "winter").display_name(), "winter");
    }

    #[test]
    fn mention_uses_snowflake() {
        let user = User::new(123_u64, "winter");
        assert_eq!(user.mention(), "<@123>");
    }

    #[test]
    fn default_avatar_uses_modern_index() {
        let user = User::new(Snowflake::new(12345_u64 << 22), "winter");
        assert_eq!(
            user.default_avatar().url(),
            "https://cdn.discordapp.com/embed/avatars/3.png"
        );
    }

    #[test]
    fn default_avatar_uses_legacy_discriminator() {
        let user = User::new(1_u64, "winter").with_discriminator("1234");
        assert_eq!(
            user.default_avatar().url(),
            "https://cdn.discordapp.com/embed/avatars/4.png"
        );
    }

    #[test]
    fn display_avatar_prefers_uploaded_avatar() {
        let avatar = Asset::from_avatar(Snowflake::new(1), "abc");
        let user = User::new(1_u64, "winter").with_avatar(avatar.clone());

        assert_eq!(user.avatar(), Some(&avatar));
        assert_eq!(user.display_avatar(), avatar);
    }

    #[test]
    fn equality_and_hash_use_id_only() {
        let a = User::new(1_u64, "first");
        let b = User::new(1_u64, "second").with_bot(true);
        let c = User::new(2_u64, "first");

        assert_eq!(a, b);
        assert_ne!(a, c);

        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        a.hash(&mut hasher_a);
        b.hash(&mut hasher_b);
        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }

    #[test]
    fn display_shows_legacy_discriminator() {
        let user = User::new(1_u64, "winter").with_discriminator("1234");
        assert_eq!(user.to_string(), "winter#1234");
    }

    #[test]
    fn colour_aliases_and_accent_colour() {
        let user = User::new(1_u64, "winter").with_accent_colour(Colour::BLURPLE);

        assert_eq!(user.accent_colour(), Some(Colour::BLURPLE));
        assert_eq!(user.accent_color(), Some(Colour::BLURPLE));
        assert_eq!(user.colour(), Colour::default());
        assert_eq!(user.color(), Colour::default());
    }

    #[test]
    fn has_id_and_trait_accessors() {
        use abc::User as UserTrait;

        let user = User::new(1_u64, "winter")
            .with_global_name("Winter")
            .with_avatar_decoration(Asset::from_avatar_decoration("decoration"));

        assert_eq!(user.id(), Snowflake::new(1));
        assert_eq!(UserTrait::display_name(&user), "Winter");
        assert_eq!(UserTrait::mention(&user), "<@1>");
        assert!(user.avatar_decoration().is_some());
    }

    #[test]
    fn decodes_a_full_user_payload() {
        let value = json::from_str(
            r#"{
                "id": "123456789012345678",
                "username": "winter",
                "discriminator": "1234",
                "global_name": "Winter",
                "avatar": "avatarhash",
                "bot": true,
                "system": true,
                "banner": "bannerhash",
                "accent_color": 5793266,
                "avatar_decoration_data": {
                    "asset": "decorationhash",
                    "sku_id": "42"
                },
                "public_flags": 65536
            }"#,
        )
        .unwrap();

        let user = User::from_value(&value).unwrap();

        assert_eq!(user.id, Snowflake::new(123_456_789_012_345_678));
        assert_eq!(user.name, "winter");
        assert_eq!(user.discriminator, "1234");
        assert_eq!(user.global_name.as_deref(), Some("Winter"));
        assert_eq!(user.avatar.as_ref().map(Asset::key), Some("avatarhash"));
        assert!(user.bot);
        assert!(user.system);
        assert_eq!(user.banner.as_ref().map(Asset::key), Some("bannerhash"));
        assert_eq!(user.accent_colour, Some(Colour::new(5_793_266)));
        assert_eq!(
            user.avatar_decoration.as_ref().map(Asset::key),
            Some("decorationhash")
        );
        assert_eq!(user.avatar_decoration_sku_id, Some(Snowflake::new(42)));
        assert_eq!(user.public_flags, PublicUserFlags::VERIFIED_BOT);
        assert!(user.avatar.as_ref().unwrap().url().contains("/avatars/"));
        assert!(user.banner.as_ref().unwrap().url().contains("/banners/"));
    }

    #[test]
    fn decodes_minimal_and_legacy_user_payloads() {
        let value = json::from_str(r#"{"id":1,"username":"winter"}"#).unwrap();
        let user = User::from_value(&value).unwrap();
        assert_eq!(user.id, Snowflake::new(1));
        assert_eq!(user.discriminator, "0");
        assert_eq!(user.global_name, None);
        assert_eq!(user.avatar, None);
        assert!(!user.bot);
        assert!(!user.system);
        assert_eq!(user.public_flags, PublicUserFlags::none());

        let legacy =
            json::from_str(r#"{"id":"1","username":"winter","avatar_decoration":"legacy"}"#)
                .unwrap();
        let user = User::from_value(&legacy).unwrap();
        assert_eq!(
            user.avatar_decoration.as_ref().map(Asset::key),
            Some("legacy")
        );
        assert_eq!(user.avatar_decoration_sku_id, None);
    }

    #[test]
    fn rejects_malformed_user_payloads() {
        let missing = json::from_str(r#"{"username":"winter"}"#).unwrap();
        let error = User::from_value(&missing).unwrap_err();
        assert_eq!(error.to_string(), "missing field `user.id`");

        let wrong_type = json::from_str(r#"{"id":1,"username":2}"#).unwrap();
        let error = User::from_value(&wrong_type).unwrap_err();
        assert_eq!(error.to_string(), "field `user.username` must be a string");

        let invalid_id = json::from_str(r#"{"id":"nope","username":"winter"}"#).unwrap();
        let error = User::from_value(&invalid_id).unwrap_err();
        assert_eq!(error.to_string(), "field `user.id` must be a snowflake");

        let invalid_colour =
            json::from_str(r#"{"id":1,"username":"winter","accent_color":16777216}"#).unwrap();
        let error = User::from_value(&invalid_colour).unwrap_err();
        assert_eq!(
            error.to_string(),
            "field `user.accent_color` must be a 24-bit colour"
        );
    }
}
