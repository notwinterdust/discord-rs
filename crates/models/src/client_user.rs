//! The `ClientUser` model.
//!
//! a regular [`User`] plus the fields that are
//! only available for the account the client is logged in as.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

use abc::{Asset, HasId, Snowflake};

use crate::User;

/// Represents the user the client is logged in as.
/// Equality and hashing are based on the underlying user's ID,
#[derive(Clone, Debug, Default)]
pub struct ClientUser {
    /// The underlying user data.
    pub user: User,
    /// Whether the account's email is verified.
    pub verified: bool,
    /// Whether the account has multi-factor authentication enabled.
    pub mfa_enabled: bool,
    /// The IETF language tag the account is using, if reported.
    pub locale: Option<String>,
}

impl ClientUser {
    /// Creates a client user from regular user data.
    #[must_use]
    pub fn new(user: User) -> Self {
        Self {
            user,
            verified: false,
            mfa_enabled: false,
            locale: None,
        }
    }

    /// Sets whether the account's email is verified.
    #[must_use]
    pub fn with_verified(mut self, verified: bool) -> Self {
        self.verified = verified;
        self
    }

    /// Sets whether the account has multi-factor authentication enabled.
    #[must_use]
    pub fn with_mfa_enabled(mut self, mfa_enabled: bool) -> Self {
        self.mfa_enabled = mfa_enabled;
        self
    }

    /// Sets the account's locale.
    #[must_use]
    pub fn with_locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    /// Returns the underlying user.
    #[must_use]
    pub fn user(&self) -> &User {
        &self.user
    }

    /// Returns the underlying user mutably.
    pub fn user_mut(&mut self) -> &mut User {
        &mut self.user
    }

    /// Consumes the client user and returns the underlying user.
    #[must_use]
    pub fn into_user(self) -> User {
        self.user
    }

    /// Returns the client user's ID.
    #[must_use]
    pub fn id(&self) -> Snowflake {
        self.user.id
    }
}

impl Deref for ClientUser {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.user
    }
}

impl From<User> for ClientUser {
    fn from(user: User) -> Self {
        Self::new(user)
    }
}

impl fmt::Display for ClientUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.user.fmt(f)
    }
}

impl PartialEq for ClientUser {
    fn eq(&self, other: &Self) -> bool {
        self.user == other.user
    }
}

impl Eq for ClientUser {}

impl Hash for ClientUser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.user.hash(state);
    }
}

impl HasId for ClientUser {
    fn id(&self) -> Snowflake {
        self.user.id
    }
}

impl abc::User for ClientUser {
    fn name(&self) -> &str {
        self.user.name()
    }

    fn discriminator(&self) -> &str {
        self.user.discriminator()
    }

    fn global_name(&self) -> Option<&str> {
        self.user.global_name()
    }

    fn bot(&self) -> bool {
        self.user.bot()
    }

    fn system(&self) -> bool {
        self.user.system()
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
        self.user.display_name()
    }

    fn default_avatar(&self) -> Asset {
        self.user.default_avatar()
    }

    fn display_avatar(&self) -> Asset {
        self.user.display_avatar()
    }

    fn mention(&self) -> String {
        self.user.mention()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use abc::Snowflake;

    use super::ClientUser;
    use crate::User;

    fn user() -> User {
        User::new(123_u64, "winter").with_global_name("Winter")
    }

    #[test]
    fn constructor_defaults() {
        let client = ClientUser::new(user());

        assert_eq!(client.id(), Snowflake::new(123));
        assert_eq!(client.user.name, "winter");
        assert!(!client.verified);
        assert!(!client.mfa_enabled);
        assert_eq!(client.locale, None);
        assert_eq!(client.to_string(), "winter");
    }

    #[test]
    fn builders_and_delegation() {
        let client = ClientUser::new(user())
            .with_verified(true)
            .with_mfa_enabled(true)
            .with_locale("en-US");

        assert!(client.verified);
        assert!(client.mfa_enabled);
        assert_eq!(client.locale.as_deref(), Some("en-US"));
        assert_eq!(client.display_name(), "Winter");
        assert_eq!(client.mention(), "<@123>");
        assert_eq!(client.user().name, "winter");
        assert_eq!(client.id(), Snowflake::new(123));
    }

    #[test]
    fn equality_and_hash_use_user_id_only() {
        let a = ClientUser::new(User::new(1_u64, "first"));
        let b = ClientUser::new(User::new(1_u64, "second").with_bot(true)).with_verified(true);
        let c = ClientUser::new(User::new(2_u64, "first"));

        assert_eq!(a, b);
        assert_ne!(a, c);

        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        a.hash(&mut hasher_a);
        b.hash(&mut hasher_b);
        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }

    #[test]
    fn conversion_round_trip() {
        let user = user();
        let client = ClientUser::from(user.clone());

        assert_eq!(client.into_user(), user);
    }
}
