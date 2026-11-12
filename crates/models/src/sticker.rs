//! Sticker models.
//!
//! [`StickerItem`] is the partial sticker attached to a message, while
//! [`Sticker`] carries the full sticker data and [`StickerPack`] groups the
//! standard stickers distributed by Discord.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

use abc::{Asset, CDN_BASE, HasId, Snowflake};
use data::Value;
use enums::{StickerFormatType, StickerType};

use crate::User;
use crate::cast::{self, CastError};

const STICKER_MEDIA_BASE: &str = "https://media.discordapp.net";

fn sticker_url(id: Snowflake, format: StickerFormatType) -> String {
    if format == StickerFormatType::Gif {
        format!("{STICKER_MEDIA_BASE}/stickers/{id}.gif")
    } else {
        format!("{CDN_BASE}/stickers/{id}.{}", format.file_extension())
    }
}

/// The partial sticker data attached to a message.
///
/// Equality and hashing are based on the sticker's ID.
#[derive(Clone, Debug, Default)]
pub struct StickerItem {
    /// The sticker's unique ID.
    pub id: Snowflake,
    /// The sticker's name.
    pub name: String,
    /// The sticker's file format.
    pub format: StickerFormatType,
}

impl StickerItem {
    /// Creates a sticker item with the PNG format.
    #[must_use]
    pub fn new(id: impl Into<Snowflake>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            format: StickerFormatType::default(),
        }
    }

    /// Sets the sticker's file format.
    #[must_use]
    pub fn with_format(mut self, format: StickerFormatType) -> Self {
        self.format = format;
        self
    }

    /// Decodes a sticker item from a Discord sticker item object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "sticker_item")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Ok(Self {
            id: cast::required_snowflake(object, path, "id")?,
            name: cast::required_string(object, path, "name")?.to_owned(),
            format: cast::required_enum(
                object,
                path,
                "format_type",
                "a sticker format",
                StickerFormatType::from_value,
            )?,
        })
    }

    /// Returns the URL of the sticker's image.
    #[must_use]
    pub fn url(&self) -> String {
        sticker_url(self.id, self.format)
    }

    /// Returns the sticker's creation time based on its ID.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.id.created_at()
    }
}

impl fmt::Display for StickerItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl PartialEq for StickerItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for StickerItem {}

impl Hash for StickerItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for StickerItem {
    fn id(&self) -> Snowflake {
        self.id
    }
}

/// A full sticker.
///
/// Standard stickers and guild stickers share this model. Fields that only
/// apply to one kind are optional; use [`Sticker::is_standard`] and
/// [`Sticker::is_guild`] to distinguish them.
///
/// Equality and hashing are based on the sticker's ID.
#[derive(Clone, Debug, Default)]
pub struct Sticker {
    /// The sticker's unique ID.
    pub id: Snowflake,
    /// The sticker's name.
    pub name: String,
    /// The sticker's description, if set.
    pub description: Option<String>,
    /// The sticker's file format.
    pub format: StickerFormatType,
    /// The sticker's type.
    pub sticker_type: StickerType,
    /// The sticker's autocomplete/suggestion tags.
    pub tags: Option<String>,
    /// The ID of the standard sticker pack, if applicable.
    pub pack_id: Option<Snowflake>,
    /// The sticker's sort order within its pack, if applicable.
    pub sort_value: Option<u32>,
    /// The ID of the guild that owns the sticker, if applicable.
    pub guild_id: Option<Snowflake>,
    /// Whether a guild sticker is available for use.
    pub available: Option<bool>,
    /// The user that created a guild sticker, if known.
    pub user: Option<User>,
}

impl Sticker {
    /// Creates a standard sticker with the PNG format.
    #[must_use]
    pub fn new(id: impl Into<Snowflake>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            ..Self::default()
        }
    }

    /// Creates a guild sticker.
    #[must_use]
    pub fn guild(
        id: impl Into<Snowflake>,
        name: impl Into<String>,
        guild_id: impl Into<Snowflake>,
    ) -> Self {
        Self {
            sticker_type: StickerType::Guild,
            guild_id: Some(guild_id.into()),
            available: Some(true),
            ..Self::new(id, name)
        }
    }

    /// Decodes a sticker from a Discord sticker object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "sticker")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let user_path = cast::field_path(path, "user");
        let user = cast::optional_object(object, path, "user")?
            .map(|user| User::from_object_at(user, &user_path))
            .transpose()?;
        Ok(Self {
            id: cast::required_snowflake(object, path, "id")?,
            name: cast::required_string(object, path, "name")?.to_owned(),
            description: cast::optional_string(object, path, "description")?.map(str::to_owned),
            format: cast::required_enum(
                object,
                path,
                "format_type",
                "a sticker format",
                StickerFormatType::from_value,
            )?,
            sticker_type: cast::required_enum(
                object,
                path,
                "type",
                "a sticker type",
                StickerType::from_value,
            )?,
            tags: cast::optional_string(object, path, "tags")?.map(str::to_owned),
            pack_id: cast::optional_snowflake(object, path, "pack_id")?,
            sort_value: cast::optional_u32(object, path, "sort_value")?,
            guild_id: cast::optional_snowflake(object, path, "guild_id")?,
            available: cast::optional(object, path, "available")
                .map(|(value, path)| cast::boolean(value, &path))
                .transpose()?,
            user,
        })
    }

    /// Sets the sticker's format.
    #[must_use]
    pub fn with_format(mut self, format: StickerFormatType) -> Self {
        self.format = format;
        self
    }

    /// Sets the sticker's type.
    #[must_use]
    pub fn with_sticker_type(mut self, sticker_type: StickerType) -> Self {
        self.sticker_type = sticker_type;
        self
    }

    /// Sets the sticker's description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the sticker's raw tag string.
    #[must_use]
    pub fn with_tags(mut self, tags: impl Into<String>) -> Self {
        self.tags = Some(tags.into());
        self
    }

    /// Sets the standard sticker pack ID.
    #[must_use]
    pub fn with_pack_id(mut self, pack_id: impl Into<Snowflake>) -> Self {
        self.pack_id = Some(pack_id.into());
        self
    }

    /// Sets the sticker's sort order within its pack.
    #[must_use]
    pub fn with_sort_value(mut self, sort_value: u32) -> Self {
        self.sort_value = Some(sort_value);
        self
    }

    /// Sets the user that created a guild sticker.
    #[must_use]
    pub fn with_user(mut self, user: User) -> Self {
        self.user = Some(user);
        self
    }

    /// Sets whether a guild sticker is available for use.
    #[must_use]
    pub fn with_available(mut self, available: bool) -> Self {
        self.available = Some(available);
        self
    }

    /// Returns `true` if this is a standard sticker.
    #[must_use]
    pub const fn is_standard(&self) -> bool {
        matches!(self.sticker_type, StickerType::Standard)
    }

    /// Returns `true` if this is a guild sticker.
    #[must_use]
    pub const fn is_guild(&self) -> bool {
        matches!(self.sticker_type, StickerType::Guild)
    }

    /// Returns whether the sticker is available for use.
    ///
    /// Guild stickers default to available when Discord omits the field.
    #[must_use]
    pub fn available(&self) -> bool {
        self.available.unwrap_or(true)
    }

    /// Returns the sticker's tags split on commas, trimmed and without empty
    /// entries.
    #[must_use]
    pub fn tag_list(&self) -> Vec<&str> {
        self.tags.as_deref().map_or_else(Vec::new, |tags| {
            tags.split(',')
                .map(str::trim)
                .filter(|tag| !tag.is_empty())
                .collect()
        })
    }

    /// Returns the URL of the sticker's image.
    #[must_use]
    pub fn url(&self) -> String {
        sticker_url(self.id, self.format)
    }

    /// Returns the sticker's creation time based on its ID.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.id.created_at()
    }
}

impl fmt::Display for Sticker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl PartialEq for Sticker {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Sticker {}

impl Hash for Sticker {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for Sticker {
    fn id(&self) -> Snowflake {
        self.id
    }
}

/// A pack of standard stickers.
///
/// Equality and hashing are based on the pack's ID.
#[derive(Clone, Debug, Default)]
pub struct StickerPack {
    /// The pack's unique ID.
    pub id: Snowflake,
    /// The pack's name.
    pub name: String,
    /// The pack's description.
    pub description: String,
    /// The SKU ID of the pack.
    pub sku_id: Snowflake,
    /// The ID of the sticker used as the pack's cover.
    pub cover_sticker_id: Option<Snowflake>,
    /// The stickers in the pack.
    pub stickers: Vec<Sticker>,
    /// The ID of the pack's banner asset, if set.
    pub banner_asset_id: Option<Snowflake>,
}

impl StickerPack {
    /// Creates a sticker pack with no stickers.
    #[must_use]
    pub fn new(
        id: impl Into<Snowflake>,
        name: impl Into<String>,
        description: impl Into<String>,
        sku_id: impl Into<Snowflake>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            sku_id: sku_id.into(),
            ..Self::default()
        }
    }

    /// Sets the stickers in the pack.
    #[must_use]
    pub fn with_stickers(mut self, stickers: Vec<Sticker>) -> Self {
        self.stickers = stickers;
        self
    }

    /// Sets the cover sticker ID.
    #[must_use]
    pub fn with_cover_sticker_id(mut self, cover_sticker_id: impl Into<Snowflake>) -> Self {
        self.cover_sticker_id = Some(cover_sticker_id.into());
        self
    }

    /// Sets the banner asset ID.
    #[must_use]
    pub fn with_banner_asset_id(mut self, banner_asset_id: impl Into<Snowflake>) -> Self {
        self.banner_asset_id = Some(banner_asset_id.into());
        self
    }

    /// Returns the sticker used as the pack's cover, if it is in the pack.
    #[must_use]
    pub fn cover_sticker(&self) -> Option<&Sticker> {
        let cover_sticker_id = self.cover_sticker_id?;
        self.stickers
            .iter()
            .find(|sticker| sticker.id == cover_sticker_id)
    }

    /// Returns the pack's banner asset, if it has one.
    #[must_use]
    pub fn banner(&self) -> Option<Asset> {
        self.banner_asset_id.map(Asset::from_sticker_banner)
    }
}

impl fmt::Display for StickerPack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl PartialEq for StickerPack {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for StickerPack {}

impl Hash for StickerPack {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for StickerPack {
    fn id(&self) -> Snowflake {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use abc::{HasId, Snowflake};
    use data::json;
    use enums::{StickerFormatType, StickerType};

    use super::{Sticker, StickerItem, StickerPack};
    use crate::User;

    #[test]
    fn sticker_item_urls_and_display() {
        let png = StickerItem::new(10_u64, "wave");
        assert_eq!(png.id(), Snowflake::new(10));
        assert_eq!(png.url(), "https://cdn.discordapp.com/stickers/10.png");
        assert_eq!(png.created_at(), Snowflake::new(10).created_at());
        assert_eq!(png.to_string(), "wave");

        let gif = StickerItem::new(11_u64, "dance").with_format(StickerFormatType::Gif);
        assert_eq!(gif.url(), "https://media.discordapp.net/stickers/11.gif");

        let lottie = StickerItem::new(12_u64, "spin").with_format(StickerFormatType::Lottie);
        assert_eq!(lottie.url(), "https://cdn.discordapp.com/stickers/12.json");
    }

    #[test]
    fn standard_sticker_builders_and_tags() {
        let sticker = Sticker::new(20_u64, "wave")
            .with_format(StickerFormatType::Apng)
            .with_description("A waving hand")
            .with_tags("wave, hello ,")
            .with_pack_id(30_u64)
            .with_sort_value(4);

        assert_eq!(sticker.id, Snowflake::new(20));
        assert_eq!(sticker.name, "wave");
        assert_eq!(sticker.format, StickerFormatType::Apng);
        assert_eq!(sticker.description.as_deref(), Some("A waving hand"));
        assert_eq!(sticker.tags.as_deref(), Some("wave, hello ,"));
        assert_eq!(sticker.tag_list(), vec!["wave", "hello"]);
        assert_eq!(sticker.pack_id, Some(Snowflake::new(30)));
        assert_eq!(sticker.sort_value, Some(4));
        assert_eq!(sticker.sticker_type, StickerType::Standard);
        assert!(sticker.is_standard());
        assert!(!sticker.is_guild());
        assert!(sticker.available());
        assert_eq!(sticker.to_string(), "wave");
        assert_eq!(sticker.url(), "https://cdn.discordapp.com/stickers/20.png");
        assert_eq!(sticker.created_at(), Snowflake::new(20).created_at());
    }

    #[test]
    fn guild_sticker_builders() {
        let user = User::new(1_u64, "winter");
        let sticker = Sticker::guild(21_u64, "party", 100_u64)
            .with_format(StickerFormatType::Gif)
            .with_tags("🎉")
            .with_user(user.clone())
            .with_available(false);

        assert_eq!(sticker.sticker_type, StickerType::Guild);
        assert!(sticker.is_guild());
        assert!(!sticker.is_standard());
        assert_eq!(sticker.guild_id, Some(Snowflake::new(100)));
        assert_eq!(sticker.user, Some(user));
        assert!(!sticker.available());
        assert_eq!(sticker.tag_list(), vec!["🎉"]);
        assert_eq!(
            sticker.url(),
            "https://media.discordapp.net/stickers/21.gif"
        );

        let default_available = Sticker::guild(22_u64, "party", 100_u64);
        assert!(default_available.available());
        assert!(default_available.tags.is_none());
        assert!(default_available.tag_list().is_empty());
    }

    #[test]
    fn sticker_pack_cover_and_banner() {
        let cover = Sticker::new(40_u64, "cover");
        let other = Sticker::new(41_u64, "other");
        let pack = StickerPack::new(50_u64, "Pack", "A pack", 60_u64)
            .with_stickers(vec![cover, other])
            .with_cover_sticker_id(40_u64)
            .with_banner_asset_id(70_u64);

        assert_eq!(pack.id(), Snowflake::new(50));
        assert_eq!(pack.name, "Pack");
        assert_eq!(pack.description, "A pack");
        assert_eq!(pack.sku_id, Snowflake::new(60));
        assert_eq!(pack.stickers.len(), 2);
        assert_eq!(
            pack.cover_sticker().map(|sticker| sticker.name.as_str()),
            Some("cover")
        );
        assert_eq!(
            pack.banner().map(|banner| banner.url().to_owned()),
            Some(
                "https://cdn.discordapp.com/app-assets/710982414301790216/store/70.png".to_owned()
            )
        );
        assert_eq!(pack.to_string(), "Pack");

        let missing_cover =
            StickerPack::new(51_u64, "Empty", "", 61_u64).with_cover_sticker_id(99_u64);
        assert!(missing_cover.cover_sticker().is_none());
        assert!(missing_cover.banner().is_none());
    }

    #[test]
    fn sticker_equality_and_hash_use_id() {
        let a = Sticker::new(10_u64, "first");
        let b = Sticker::new(10_u64, "second");
        let c = Sticker::new(11_u64, "first");

        assert_eq!(a, b);
        assert_ne!(a, c);

        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        a.hash(&mut hasher_a);
        b.hash(&mut hasher_b);
        assert_eq!(hasher_a.finish(), hasher_b.finish());

        assert_eq!(StickerItem::new(10_u64, "a"), StickerItem::new(10_u64, "b"));
        assert_eq!(
            StickerPack::new(10_u64, "a", "", 1_u64),
            StickerPack::new(10_u64, "b", "", 2_u64)
        );
    }

    #[test]
    fn decodes_sticker_objects() {
        let value = json::from_str(
            r#"{
                "id": "10",
                "name": "wave",
                "description": "A waving hand",
                "format_type": 2,
                "type": 2,
                "tags": "wave, hello",
                "pack_id": "50",
                "sort_value": 7,
                "guild_id": "100",
                "available": true,
                "user": {"id": "1", "username": "winter"}
            }"#,
        )
        .unwrap();

        let sticker = Sticker::from_value(&value).unwrap();
        assert_eq!(sticker.id(), Snowflake::new(10));
        assert_eq!(sticker.name, "wave");
        assert_eq!(sticker.description.as_deref(), Some("A waving hand"));
        assert_eq!(sticker.format, StickerFormatType::Apng);
        assert_eq!(sticker.sticker_type, StickerType::Guild);
        assert_eq!(sticker.tags.as_deref(), Some("wave, hello"));
        assert_eq!(sticker.pack_id, Some(Snowflake::new(50)));
        assert_eq!(sticker.sort_value, Some(7));
        assert_eq!(sticker.guild_id, Some(Snowflake::new(100)));
        assert_eq!(sticker.available, Some(true));
        assert_eq!(
            sticker.user.as_ref().map(|user| user.id),
            Some(Snowflake::new(1))
        );

        let minimal = Sticker::from_value(
            &json::from_str(r#"{"id":"11","name":"dance","format_type":1,"type":1}"#).unwrap(),
        )
        .unwrap();
        assert_eq!(minimal.format, StickerFormatType::Png);
        assert_eq!(minimal.sticker_type, StickerType::Standard);
        assert_eq!(minimal.description, None);
        assert_eq!(minimal.pack_id, None);
        assert_eq!(minimal.available, None);
        assert_eq!(minimal.user, None);

        let missing_name = Sticker::from_value(
            &json::from_str(r#"{"id":"10","format_type":1,"type":1}"#).unwrap(),
        )
        .unwrap_err();
        assert_eq!(missing_name.to_string(), "missing field `sticker.name`");

        let bad_format = Sticker::from_value(
            &json::from_str(r#"{"id":"10","name":"wave","format_type":9,"type":1}"#).unwrap(),
        )
        .unwrap_err();
        assert_eq!(
            bad_format.to_string(),
            "field `sticker.format_type` must be a sticker format"
        );

        let bad_type = Sticker::from_value(
            &json::from_str(r#"{"id":"10","name":"wave","format_type":1,"type":"guild"}"#).unwrap(),
        )
        .unwrap_err();
        assert_eq!(
            bad_type.to_string(),
            "field `sticker.type` must be a non-negative integer"
        );

        let bad_user = Sticker::from_value(
            &json::from_str(
                r#"{"id":"10","name":"wave","format_type":1,"type":1,"user":{"id":"nope","username":"winter"}}"#,
            )
            .unwrap(),
        )
        .unwrap_err();
        assert_eq!(
            bad_user.to_string(),
            "field `sticker.user.id` must be a snowflake"
        );
    }

    #[test]
    fn decodes_sticker_items() {
        let item = StickerItem::from_value(
            &json::from_str(r#"{"id":"10","name":"wave","format_type":4}"#).unwrap(),
        )
        .unwrap();
        assert_eq!(item.id, Snowflake::new(10));
        assert_eq!(item.name, "wave");
        assert_eq!(item.format, StickerFormatType::Gif);
        assert_eq!(item.url(), "https://media.discordapp.net/stickers/10.gif");

        let missing_format =
            StickerItem::from_value(&json::from_str(r#"{"id":"10","name":"wave"}"#).unwrap())
                .unwrap_err();
        assert_eq!(
            missing_format.to_string(),
            "missing field `sticker_item.format_type`"
        );
    }
}
