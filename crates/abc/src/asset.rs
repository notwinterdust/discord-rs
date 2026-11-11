//! Discord CDN assets.
//! This type only models the asset's location and metadata;
//! downloading the bytes belongs to the HTTP layer, which does not
//! exist yet.

use std::error::Error;
use std::fmt;
use std::str::FromStr;

use crate::Snowflake;

/// Base URL for Discord's CDN.
pub const CDN_BASE: &str = "https://cdn.discordapp.com";

/// Image formats accepted by the Discord CDN.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AssetFormat {
    /// WebP.
    Webp,
    /// JPEG.
    Jpeg,
    Jpg,
    /// PNG.
    Png,
    /// Animated GIF.
    Gif,
}

impl AssetFormat {
    /// Returns the lowercase format name used in URLs.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Webp => "webp",
            Self::Jpeg => "jpeg",
            Self::Jpg => "jpg",
            Self::Png => "png",
            Self::Gif => "gif",
        }
    }

    /// Returns `true` if the format can be used for a non-animated asset.
    #[must_use]
    pub const fn is_static(self) -> bool {
        !matches!(self, Self::Gif)
    }
}

impl fmt::Display for AssetFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for AssetFormat {
    type Err = AssetError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "webp" => Ok(Self::Webp),
            "jpeg" => Ok(Self::Jpeg),
            "jpg" => Ok(Self::Jpg),
            "png" => Ok(Self::Png),
            "gif" => Ok(Self::Gif),
            _ => Err(AssetError::InvalidFormat),
        }
    }
}

/// Error returned when an asset cannot be resized or converted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssetError {
    /// The requested size is not a power of two between 16 and 4096.
    InvalidSize(u32),
    /// The requested format is not valid for the asset.
    InvalidFormat,
}

impl fmt::Display for AssetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSize(size) => {
                write!(
                    f,
                    "size must be a power of two between 16 and 4096, got {size}"
                )
            }
            Self::InvalidFormat => f.write_str("format is not valid for this asset"),
        }
    }
}

impl Error for AssetError {}

/// Represents a CDN asset on Discord.
/// Equality and hashing are based on the URL.
#[derive(Clone, Debug)]
pub struct Asset {
    url: String,
    key: String,
    animated: bool,
}

impl Asset {
    /// Creates an asset from an existing CDN URL.
    #[must_use]
    pub fn new(url: impl Into<String>, key: impl Into<String>, animated: bool) -> Self {
        Self {
            url: url.into(),
            key: key.into(),
            animated,
        }
    }

    /// Creates the default avatar for one of Discord's six default colors.
    #[must_use]
    pub fn from_default_avatar(index: u8) -> Self {
        Self::new(
            format!("{CDN_BASE}/embed/avatars/{index}.png"),
            index.to_string(),
            false,
        )
    }

    /// Creates a user's global avatar.
    #[must_use]
    pub fn from_avatar(user_id: Snowflake, avatar_hash: &str) -> Self {
        let animated = avatar_hash.starts_with("a_");
        let format = if animated { "gif" } else { "png" };
        Self::new(
            format!("{CDN_BASE}/avatars/{user_id}/{avatar_hash}.{format}?size=1024"),
            avatar_hash,
            animated,
        )
    }

    /// Creates a member's guild-specific avatar.
    #[must_use]
    pub fn from_guild_avatar(guild_id: Snowflake, member_id: Snowflake, avatar_hash: &str) -> Self {
        let animated = avatar_hash.starts_with("a_");
        let format = if animated { "gif" } else { "png" };
        Self::new(
            format!(
                "{CDN_BASE}/guilds/{guild_id}/users/{member_id}/avatars/{avatar_hash}.{format}?size=1024"
            ),
            avatar_hash,
            animated,
        )
    }

    /// Creates a user's banner.
    #[must_use]
    pub fn from_user_banner(user_id: Snowflake, banner_hash: &str) -> Self {
        let animated = banner_hash.starts_with("a_");
        let format = if animated { "gif" } else { "png" };
        Self::new(
            format!("{CDN_BASE}/banners/{user_id}/{banner_hash}.{format}?size=512"),
            banner_hash,
            animated,
        )
    }

    /// Creates a member's guild-specific banner.
    #[must_use]
    pub fn from_guild_banner(guild_id: Snowflake, member_id: Snowflake, banner_hash: &str) -> Self {
        let animated = banner_hash.starts_with("a_");
        let format = if animated { "gif" } else { "png" };
        Self::new(
            format!(
                "{CDN_BASE}/guilds/{guild_id}/users/{member_id}/banners/{banner_hash}.{format}?size=1024"
            ),
            banner_hash,
            animated,
        )
    }

    /// Creates an avatar decoration preset asset.
    #[must_use]
    pub fn from_avatar_decoration(asset_hash: &str) -> Self {
        Self::new(
            format!("{CDN_BASE}/avatar-decoration-presets/{asset_hash}.png?size=96"),
            asset_hash,
            true,
        )
    }

    /// Creates a guild icon.
    #[must_use]
    pub fn from_guild_icon(guild_id: Snowflake, icon_hash: &str) -> Self {
        let animated = icon_hash.starts_with("a_");
        let format = if animated { "gif" } else { "png" };
        Self::new(
            format!("{CDN_BASE}/icons/{guild_id}/{icon_hash}.{format}?size=1024"),
            icon_hash,
            animated,
        )
    }

    /// Creates a guild image from a CDN path.
    ///
    /// `path` is the CDN collection, such as `banners`, `splashes` or
    /// `discovery-splashes`.
    #[must_use]
    pub fn from_guild_image(guild_id: Snowflake, image_hash: &str, path: &str) -> Self {
        let animated = image_hash.starts_with("a_");
        let format = if animated { "gif" } else { "png" };
        Self::new(
            format!("{CDN_BASE}/{path}/{guild_id}/{image_hash}.{format}?size=1024"),
            image_hash,
            animated,
        )
    }

    /// Creates a role icon.
    #[must_use]
    pub fn from_role_icon(role_id: Snowflake, icon_hash: &str) -> Self {
        Self::new(
            format!("{CDN_BASE}/role-icons/{role_id}/{icon_hash}.png?size=1024"),
            icon_hash,
            false,
        )
    }

    /// Creates a sticker pack banner.
    #[must_use]
    pub fn from_sticker_banner(banner_id: Snowflake) -> Self {
        Self::new(
            format!("{CDN_BASE}/app-assets/710982414301790216/store/{banner_id}.png"),
            banner_id.to_string(),
            false,
        )
    }

    /// Creates an application's icon.
    #[must_use]
    pub fn from_application_icon(application_id: Snowflake, icon_hash: &str) -> Self {
        Self::new(
            format!("{CDN_BASE}/app-icons/{application_id}/{icon_hash}.png?size=1024"),
            icon_hash,
            false,
        )
    }

    /// Creates an application's store cover image.
    #[must_use]
    pub fn from_application_cover_image(application_id: Snowflake, cover_image_hash: &str) -> Self {
        Self::new(
            format!(
                "{CDN_BASE}/app-assets/{application_id}/store/{cover_image_hash}.png?size=1024"
            ),
            cover_image_hash,
            false,
        )
    }

    /// Returns the underlying CDN URL.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the identifying key of the asset.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns whether the asset is animated.
    #[must_use]
    pub fn is_animated(&self) -> bool {
        self.animated
    }

    /// Returns a new asset with the requested size.
    /// Sizes must be a power of two between 16 and 4096.
    pub fn with_size(&self, size: u32) -> Result<Self, AssetError> {
        self.replace(Some(size), None, None)
    }

    /// Returns a new asset with the requested format.
    /// Animated assets accept `gif`; static assets do not.
    pub fn with_format(&self, format: AssetFormat) -> Result<Self, AssetError> {
        self.replace(None, Some(format), None)
    }

    /// Returns a new asset with the requested static format.
    /// Animated assets are returned unchanged.
    pub fn with_static_format(&self, format: AssetFormat) -> Result<Self, AssetError> {
        self.replace(None, None, Some(format))
    }

    /// Returns a new asset with the given components replaced.
    /// `static_format` is preferred over `format` for non-animated assets.
    pub fn replace(
        &self,
        size: Option<u32>,
        format: Option<AssetFormat>,
        static_format: Option<AssetFormat>,
    ) -> Result<Self, AssetError> {
        let mut url = self.url.clone();

        if let Some(format) = format {
            if !self.animated && static_format.is_none() && !format.is_static() {
                return Err(AssetError::InvalidFormat);
            }
            url = with_extension(&url, format.as_str());
        }

        if let Some(static_format) = static_format
            && !self.animated
        {
            if !static_format.is_static() {
                return Err(AssetError::InvalidFormat);
            }
            url = with_extension(&url, static_format.as_str());
        }

        if let Some(size) = size {
            if !valid_icon_size(size) {
                return Err(AssetError::InvalidSize(size));
            }
            url = with_query_param(&url, "size", &size.to_string());
        }

        Ok(Self {
            url,
            key: self.key.clone(),
            animated: self.animated,
        })
    }
}

impl PartialEq for Asset {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

impl Eq for Asset {}

impl std::hash::Hash for Asset {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.url)
    }
}

impl AsRef<str> for Asset {
    fn as_ref(&self) -> &str {
        &self.url
    }
}

/// Returns `true` if `size` is an accepted Discord icon size.
#[must_use]
pub const fn valid_icon_size(size: u32) -> bool {
    size >= 16 && size <= 4096 && size.is_power_of_two()
}

fn with_query_param(url: &str, key: &str, value: &str) -> String {
    let (base, query) = match url.split_once('?') {
        Some((base, query)) => (base, Some(query)),
        None => (url, None),
    };

    let mut pairs: Vec<String> = query
        .map(|query| {
            query
                .split('&')
                .filter(|pair| !pair.is_empty())
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default();

    let prefix = format!("{key}=");
    pairs.retain(|pair| !pair.starts_with(&prefix));
    pairs.push(format!("{prefix}{value}"));

    format!("{base}?{}", pairs.join("&"))
}

fn with_extension(url: &str, extension: &str) -> String {
    let (path, query) = match url.split_once('?') {
        Some((path, query)) => (path, Some(query)),
        None => (url, None),
    };

    let last_segment = path.rfind('/').map_or(0, |slash| slash + 1);
    let stem = match path.rfind('.') {
        Some(dot) if dot >= last_segment => &path[..dot],
        _ => path,
    };

    match query {
        Some(query) => format!("{stem}.{extension}?{query}"),
        None => format!("{stem}.{extension}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{Asset, AssetError, AssetFormat, valid_icon_size};
    use crate::Snowflake;

    #[test]
    fn avatar_urls() {
        let user_id = Snowflake::new(123456789012345678);

        let avatar = Asset::from_avatar(user_id, "abc123");
        assert_eq!(
            avatar.url(),
            "https://cdn.discordapp.com/avatars/123456789012345678/abc123.png?size=1024"
        );
        assert!(!avatar.is_animated());
        assert_eq!(avatar.key(), "abc123");

        let animated = Asset::from_avatar(user_id, "a_abc123");
        assert_eq!(
            animated.url(),
            "https://cdn.discordapp.com/avatars/123456789012345678/a_abc123.gif?size=1024"
        );
        assert!(animated.is_animated());
    }

    #[test]
    fn default_avatar_url() {
        let asset = Asset::from_default_avatar(3);
        assert_eq!(
            asset.url(),
            "https://cdn.discordapp.com/embed/avatars/3.png"
        );
        assert_eq!(asset.key(), "3");
    }

    #[test]
    fn sticker_banner_url() {
        let asset = Asset::from_sticker_banner(Snowflake::new(42));
        assert_eq!(
            asset.url(),
            "https://cdn.discordapp.com/app-assets/710982414301790216/store/42.png"
        );
        assert_eq!(asset.key(), "42");
        assert!(!asset.is_animated());
    }

    #[test]
    fn application_asset_urls() {
        let application_id = Snowflake::new(123456789012345678);

        let icon = Asset::from_application_icon(application_id, "abc123");
        assert_eq!(
            icon.url(),
            "https://cdn.discordapp.com/app-icons/123456789012345678/abc123.png?size=1024"
        );
        assert_eq!(icon.key(), "abc123");
        assert!(!icon.is_animated());

        let cover = Asset::from_application_cover_image(application_id, "cover123");
        assert_eq!(
            cover.url(),
            "https://cdn.discordapp.com/app-assets/123456789012345678/store/cover123.png?size=1024"
        );
    }

    #[test]
    fn sizes_are_validated_and_replaced() {
        assert!(valid_icon_size(16));
        assert!(valid_icon_size(4096));
        assert!(!valid_icon_size(15));
        assert!(!valid_icon_size(1000));
        assert!(!valid_icon_size(8192));

        let avatar = Asset::from_avatar(Snowflake::new(1), "abc");
        let resized = avatar.with_size(256).unwrap();

        assert_eq!(
            resized.url(),
            "https://cdn.discordapp.com/avatars/1/abc.png?size=256"
        );
        assert_eq!(avatar.with_size(1000), Err(AssetError::InvalidSize(1000)));
    }

    #[test]
    fn formats_are_validated() {
        let avatar = Asset::from_avatar(Snowflake::new(1), "abc");
        let webp = avatar.with_format(AssetFormat::Webp).unwrap();
        assert!(webp.url().ends_with("/abc.webp?size=1024"));
        assert_eq!(
            avatar.with_format(AssetFormat::Gif),
            Err(AssetError::InvalidFormat)
        );

        let animated = Asset::from_avatar(Snowflake::new(1), "a_abc");
        assert!(animated.with_format(AssetFormat::Gif).is_ok());
        assert_eq!(
            animated.with_static_format(AssetFormat::Webp).unwrap(),
            animated
        );
    }

    #[test]
    fn replace_applies_size_format_and_static_format() {
        let avatar = Asset::from_avatar(Snowflake::new(1), "abc");
        let asset = avatar
            .replace(Some(512), Some(AssetFormat::Jpeg), Some(AssetFormat::Webp))
            .unwrap();

        assert_eq!(
            asset.url(),
            "https://cdn.discordapp.com/avatars/1/abc.webp?size=512"
        );
    }

    #[test]
    fn equality_and_hash_are_url_based() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let a = Asset::new("https://example.com/a.png", "a", false);
        let b = Asset::new("https://example.com/a.png", "different", true);
        let c = Asset::new("https://example.com/c.png", "c", false);

        assert_eq!(a, b);
        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        a.hash(&mut hasher_a);
        b.hash(&mut hasher_b);
        assert_eq!(hasher_a.finish(), hasher_b.finish());
        assert_ne!(a, c);
    }
}
