//! Application models.
//!
//! [`PartialApplication`] is the application data embedded in invite targets.
//! Full application metadata, bot users and application flags are deferred.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

use abc::{Asset, HasId, Snowflake};

/// The partial application data attached to an invite target.
///
/// Equality and hashing are based on the application's ID.
#[derive(Clone, Debug, Default)]
pub struct PartialApplication {
    /// The application's unique ID.
    pub id: Snowflake,
    /// The application's name.
    pub name: String,
    /// The application's description.
    pub description: String,
    /// The application's icon, if it has one.
    pub icon: Option<Asset>,
    /// The application's store cover image, if it has one.
    pub cover_image: Option<Asset>,
    /// The application's RPC origin URLs, if RPC is enabled.
    pub rpc_origins: Vec<String>,
    /// The hex-encoded key used to verify interactions.
    pub verify_key: String,
    /// The application's terms of service URL, if set.
    pub terms_of_service_url: Option<String>,
    /// The application's privacy policy URL, if set.
    pub privacy_policy_url: Option<String>,
    /// The approximate number of guilds the application is in.
    pub approximate_guild_count: u64,
    /// The application's OAuth2 redirect URIs.
    pub redirect_uris: Vec<String>,
    /// The URL that receives interactions, if configured.
    pub interactions_endpoint_url: Option<String>,
    /// The URL used for role connection verification, if configured.
    pub role_connections_verification_url: Option<String>,
}

impl PartialApplication {
    /// Creates an application with its ID and name.
    #[must_use]
    pub fn new(id: impl Into<Snowflake>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            ..Self::default()
        }
    }

    /// Sets the application's description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Sets the application's icon from its icon hash.
    #[must_use]
    pub fn with_icon_hash(mut self, icon_hash: &str) -> Self {
        self.icon = Some(Asset::from_application_icon(self.id, icon_hash));
        self
    }

    /// Sets the application's icon.
    #[must_use]
    pub fn with_icon(mut self, icon: Asset) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the application's cover image from its cover image hash.
    #[must_use]
    pub fn with_cover_image_hash(mut self, cover_image_hash: &str) -> Self {
        self.cover_image = Some(Asset::from_application_cover_image(
            self.id,
            cover_image_hash,
        ));
        self
    }

    /// Sets the application's cover image.
    #[must_use]
    pub fn with_cover_image(mut self, cover_image: Asset) -> Self {
        self.cover_image = Some(cover_image);
        self
    }

    /// Sets the application's RPC origin URLs.
    #[must_use]
    pub fn with_rpc_origins(mut self, rpc_origins: Vec<String>) -> Self {
        self.rpc_origins = rpc_origins;
        self
    }

    /// Sets the application's interaction verification key.
    #[must_use]
    pub fn with_verify_key(mut self, verify_key: impl Into<String>) -> Self {
        self.verify_key = verify_key.into();
        self
    }

    /// Sets the application's terms of service URL.
    #[must_use]
    pub fn with_terms_of_service_url(mut self, url: impl Into<String>) -> Self {
        self.terms_of_service_url = Some(url.into());
        self
    }

    /// Sets the application's privacy policy URL.
    #[must_use]
    pub fn with_privacy_policy_url(mut self, url: impl Into<String>) -> Self {
        self.privacy_policy_url = Some(url.into());
        self
    }

    /// Sets the approximate number of guilds the application is in.
    #[must_use]
    pub fn with_approximate_guild_count(mut self, count: u64) -> Self {
        self.approximate_guild_count = count;
        self
    }

    /// Sets the application's OAuth2 redirect URIs.
    #[must_use]
    pub fn with_redirect_uris(mut self, redirect_uris: Vec<String>) -> Self {
        self.redirect_uris = redirect_uris;
        self
    }

    /// Sets the URL that receives interactions.
    #[must_use]
    pub fn with_interactions_endpoint_url(mut self, url: impl Into<String>) -> Self {
        self.interactions_endpoint_url = Some(url.into());
        self
    }

    /// Sets the URL used for role connection verification.
    #[must_use]
    pub fn with_role_connections_verification_url(mut self, url: impl Into<String>) -> Self {
        self.role_connections_verification_url = Some(url.into());
        self
    }

    /// Returns the application's creation time.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.id.created_at()
    }

    /// Returns the application's icon URL, if it has one.
    #[must_use]
    pub fn icon_url(&self) -> Option<&str> {
        self.icon.as_ref().map(Asset::url)
    }

    /// Returns the application's cover image URL, if it has one.
    #[must_use]
    pub fn cover_image_url(&self) -> Option<&str> {
        self.cover_image.as_ref().map(Asset::url)
    }
}

impl fmt::Display for PartialApplication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl PartialEq for PartialApplication {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PartialApplication {}

impl Hash for PartialApplication {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for PartialApplication {
    fn id(&self) -> Snowflake {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use abc::{Asset, HasId, Snowflake};

    use super::PartialApplication;

    #[test]
    fn builders_and_assets() {
        let application = PartialApplication::new(10_u64, "Game")
            .with_description("A rich presence application")
            .with_icon_hash("icon_hash")
            .with_cover_image_hash("cover_hash")
            .with_rpc_origins(vec!["https://rpc.example".to_owned()])
            .with_verify_key("verify")
            .with_terms_of_service_url("https://example.com/tos")
            .with_privacy_policy_url("https://example.com/privacy")
            .with_approximate_guild_count(5)
            .with_redirect_uris(vec!["https://example.com/callback".to_owned()])
            .with_interactions_endpoint_url("https://example.com/interactions")
            .with_role_connections_verification_url("https://example.com/verify");

        assert_eq!(application.id, Snowflake::new(10));
        assert_eq!(application.id(), Snowflake::new(10));
        assert_eq!(application.name, "Game");
        assert_eq!(application.description, "A rich presence application");
        assert_eq!(
            application.icon,
            Some(Asset::from_application_icon(
                Snowflake::new(10),
                "icon_hash"
            ))
        );
        assert_eq!(
            application.cover_image,
            Some(Asset::from_application_cover_image(
                Snowflake::new(10),
                "cover_hash"
            ))
        );
        assert_eq!(
            application.icon_url(),
            Some(Asset::from_application_icon(Snowflake::new(10), "icon_hash").url())
        );
        assert_eq!(
            application.cover_image_url(),
            Some(Asset::from_application_cover_image(Snowflake::new(10), "cover_hash").url())
        );
        assert_eq!(application.verify_key, "verify");
        assert_eq!(application.approximate_guild_count, 5);
        assert_eq!(application.redirect_uris.len(), 1);
        assert_eq!(application.created_at(), Snowflake::new(10).created_at());
        assert_eq!(application.to_string(), "Game");
    }

    #[test]
    fn equality_and_hash_use_id() {
        let a = PartialApplication::new(1_u64, "A").with_description("first");
        let b = PartialApplication::new(1_u64, "B").with_approximate_guild_count(9);
        let c = PartialApplication::new(2_u64, "A");

        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
