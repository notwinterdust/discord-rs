//! Embed models.
//!
//! Embeds are richly formatted messages sent alongside, or instead of, plain
//! text. [`Embed`] is the top-level model; the other types describe its
//! optional author, footer, media, provider and field sections.

use std::time::SystemTime;

use abc::Colour;
use data::{Object, Value};
use enums::{EmbedFlags, EmbedType};

use crate::cast::{self, CastError};

macro_rules! define_embed_media {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
        pub struct $name {
            /// The source URL of the media.
            pub url: String,
            /// A proxied URL of the media.
            pub proxy_url: Option<String>,
            /// The media's height in pixels.
            pub height: Option<u64>,
            /// The media's width in pixels.
            pub width: Option<u64>,
            /// The media's attachment flags.
            pub flags: EmbedFlags,
        }

        impl $name {
            /// Creates media with the given source URL.
            #[must_use]
            pub fn new(url: impl Into<String>) -> Self {
                Self {
                    url: url.into(),
                    ..Self::default()
                }
            }

            /// Sets the proxied URL.
            #[must_use]
            pub fn with_proxy_url(mut self, proxy_url: impl Into<String>) -> Self {
                self.proxy_url = Some(proxy_url.into());
                self
            }

            /// Sets the media's dimensions.
            #[must_use]
            pub fn with_dimensions(mut self, width: u64, height: u64) -> Self {
                self.width = Some(width);
                self.height = Some(height);
                self
            }

            /// Sets the media's attachment flags.
            #[must_use]
            pub fn with_flags(mut self, flags: EmbedFlags) -> Self {
                self.flags = flags;
                self
            }

            /// Decodes the media from a Discord embed media object.
            ///
            /// # Errors
            ///
            /// Returns [`CastError`] when a required field is missing or a
            /// field has an unexpected type.
            pub fn from_value(value: &Value) -> Result<Self, CastError> {
                Self::from_value_at(value, "media")
            }

            pub(crate) fn from_value_at(
                value: &Value,
                path: &str,
            ) -> Result<Self, CastError> {
                let object = cast::object(value, path)?;
                Self::from_object_at(object, path)
            }

            pub(crate) fn from_object_at(
                object: &Object,
                path: &str,
            ) -> Result<Self, CastError> {
                Ok(Self {
                    url: cast::required_string(object, path, "url")?.to_owned(),
                    proxy_url: cast::optional_string(object, path, "proxy_url")?
                        .map(str::to_owned),
                    height: cast::optional_u64(object, path, "height")?,
                    width: cast::optional_u64(object, path, "width")?,
                    flags: EmbedFlags::from_bits(
                        cast::optional_u64(object, path, "flags")?.unwrap_or(0),
                    ),
                })
            }
        }
    };
}

define_embed_media!(
    /// An image displayed in an embed.
    EmbedImage
);

define_embed_media!(
    /// A thumbnail displayed in an embed.
    EmbedThumbnail
);

define_embed_media!(
    /// A video attached to an embed by Discord.
    EmbedVideo
);

/// The footer of an embed.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct EmbedFooter {
    /// The footer text.
    pub text: String,
    /// The URL of the footer icon.
    pub icon_url: Option<String>,
    /// A proxied URL of the footer icon.
    pub proxy_icon_url: Option<String>,
}

impl EmbedFooter {
    /// Creates a footer with the given text.
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Self::default()
        }
    }

    /// Sets the footer icon URL.
    #[must_use]
    pub fn with_icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }

    /// Sets the proxied footer icon URL.
    #[must_use]
    pub fn with_proxy_icon_url(mut self, proxy_icon_url: impl Into<String>) -> Self {
        self.proxy_icon_url = Some(proxy_icon_url.into());
        self
    }

    /// Decodes a footer from a Discord embed footer object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "footer")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Self::from_object_at(object, path)
    }

    pub(crate) fn from_object_at(object: &Object, path: &str) -> Result<Self, CastError> {
        Ok(Self {
            text: cast::required_string(object, path, "text")?.to_owned(),
            icon_url: cast::optional_string(object, path, "icon_url")?.map(str::to_owned),
            proxy_icon_url: cast::optional_string(object, path, "proxy_icon_url")?
                .map(str::to_owned),
        })
    }
}

/// The author section of an embed.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct EmbedAuthor {
    /// The name of the author.
    pub name: String,
    /// The URL that the author name links to.
    pub url: Option<String>,
    /// The URL of the author icon.
    pub icon_url: Option<String>,
    /// A proxied URL of the author icon.
    pub proxy_icon_url: Option<String>,
}

impl EmbedAuthor {
    /// Creates an author with the given name.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    /// Sets the author's link URL.
    #[must_use]
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Sets the author icon URL.
    #[must_use]
    pub fn with_icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }

    /// Sets the proxied author icon URL.
    #[must_use]
    pub fn with_proxy_icon_url(mut self, proxy_icon_url: impl Into<String>) -> Self {
        self.proxy_icon_url = Some(proxy_icon_url.into());
        self
    }

    /// Decodes an author from a Discord embed author object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "author")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Self::from_object_at(object, path)
    }

    pub(crate) fn from_object_at(object: &Object, path: &str) -> Result<Self, CastError> {
        Ok(Self {
            name: cast::required_string(object, path, "name")?.to_owned(),
            url: cast::optional_string(object, path, "url")?.map(str::to_owned),
            icon_url: cast::optional_string(object, path, "icon_url")?.map(str::to_owned),
            proxy_icon_url: cast::optional_string(object, path, "proxy_icon_url")?
                .map(str::to_owned),
        })
    }
}

/// A single field inside an embed.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EmbedField {
    /// The field's name.
    pub name: String,
    /// The field's value.
    pub value: String,
    /// Whether the field is displayed inline with adjacent fields.
    pub inline: bool,
}

impl EmbedField {
    /// Creates a field that is displayed inline.
    #[must_use]
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            inline: true,
        }
    }

    /// Sets whether the field is displayed inline.
    #[must_use]
    pub fn with_inline(mut self, inline: bool) -> Self {
        self.inline = inline;
        self
    }

    /// Decodes a field from a Discord embed field object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "field")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Self::from_object_at(object, path)
    }

    pub(crate) fn from_object_at(object: &Object, path: &str) -> Result<Self, CastError> {
        Ok(Self {
            name: cast::required_string(object, path, "name")?.to_owned(),
            value: cast::required_string(object, path, "value")?.to_owned(),
            inline: cast::optional_bool(object, path, "inline", false)?,
        })
    }
}

/// The provider of an embed's content.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct EmbedProvider {
    /// The name of the provider.
    pub name: Option<String>,
    /// The URL of the provider.
    pub url: Option<String>,
}

impl EmbedProvider {
    /// Creates an empty provider.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            name: None,
            url: None,
        }
    }

    /// Sets the provider name.
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the provider URL.
    #[must_use]
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Decodes a provider from a Discord embed provider object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a field has an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "provider")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Self::from_object_at(object, path)
    }

    pub(crate) fn from_object_at(object: &Object, path: &str) -> Result<Self, CastError> {
        Ok(Self {
            name: cast::optional_string(object, path, "name")?.map(str::to_owned),
            url: cast::optional_string(object, path, "url")?.map(str::to_owned),
        })
    }
}

/// A rich embed attached to a message.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Embed {
    /// The embed's title.
    pub title: Option<String>,
    /// The embed's type.
    pub kind: EmbedType,
    /// The embed's description.
    pub description: Option<String>,
    /// The URL that the title links to.
    pub url: Option<String>,
    /// When the embed's content was created.
    pub timestamp: Option<SystemTime>,
    /// The embed's accent colour.
    pub colour: Option<Colour>,
    /// The embed's footer.
    pub footer: Option<EmbedFooter>,
    /// The embed's main image.
    pub image: Option<EmbedImage>,
    /// The embed's thumbnail.
    pub thumbnail: Option<EmbedThumbnail>,
    /// A video attached to the embed by Discord.
    pub video: Option<EmbedVideo>,
    /// The provider of the embed's content.
    pub provider: Option<EmbedProvider>,
    /// The embed's author.
    pub author: Option<EmbedAuthor>,
    /// The embed's fields.
    pub fields: Vec<EmbedField>,
    /// The embed's flags.
    pub flags: EmbedFlags,
}

impl Embed {
    /// Creates an empty rich embed.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Decodes an embed from a Discord embed object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a field has an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "embed")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Self::from_object_at(object, path)
    }

    pub(crate) fn from_object_at(object: &Object, path: &str) -> Result<Self, CastError> {
        let footer_path = cast::field_path(path, "footer");
        let footer = cast::optional_object(object, path, "footer")?
            .map(|footer| EmbedFooter::from_object_at(footer, &footer_path))
            .transpose()?;

        let image_path = cast::field_path(path, "image");
        let image = cast::optional_object(object, path, "image")?
            .map(|image| EmbedImage::from_object_at(image, &image_path))
            .transpose()?;

        let thumbnail_path = cast::field_path(path, "thumbnail");
        let thumbnail = cast::optional_object(object, path, "thumbnail")?
            .map(|thumbnail| EmbedThumbnail::from_object_at(thumbnail, &thumbnail_path))
            .transpose()?;

        let video_path = cast::field_path(path, "video");
        let video = cast::optional_object(object, path, "video")?
            .map(|video| EmbedVideo::from_object_at(video, &video_path))
            .transpose()?;

        let provider_path = cast::field_path(path, "provider");
        let provider = cast::optional_object(object, path, "provider")?
            .map(|provider| EmbedProvider::from_object_at(provider, &provider_path))
            .transpose()?;

        let author_path = cast::field_path(path, "author");
        let author = cast::optional_object(object, path, "author")?
            .map(|author| EmbedAuthor::from_object_at(author, &author_path))
            .transpose()?;

        let fields = cast::optional_array_of(object, path, "fields", EmbedField::from_value_at)?
            .unwrap_or_default();

        Ok(Self {
            title: cast::optional_string(object, path, "title")?.map(str::to_owned),
            kind: cast::optional_named_enum(
                object,
                path,
                "type",
                "an embed type",
                EmbedType::from_name,
            )?
            .unwrap_or_default(),
            description: cast::optional_string(object, path, "description")?.map(str::to_owned),
            url: cast::optional_string(object, path, "url")?.map(str::to_owned),
            timestamp: cast::optional_timestamp(object, path, "timestamp")?,
            colour: cast::optional_colour(object, path, "color")?,
            footer,
            image,
            thumbnail,
            video,
            provider,
            author,
            fields,
            flags: EmbedFlags::from_bits(cast::optional_u64(object, path, "flags")?.unwrap_or(0)),
        })
    }

    /// Sets the title.
    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the embed type.
    #[must_use]
    pub fn with_kind(mut self, kind: EmbedType) -> Self {
        self.kind = kind;
        self
    }

    /// Sets the description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the URL that the title links to.
    #[must_use]
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Sets when the embed's content was created.
    #[must_use]
    pub fn with_timestamp(mut self, timestamp: SystemTime) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    /// Sets the accent colour.
    #[must_use]
    pub fn with_colour(mut self, colour: Colour) -> Self {
        self.colour = Some(colour);
        self
    }

    /// American spelling of [`Embed::with_colour`].
    #[must_use]
    pub fn with_color(self, color: Colour) -> Self {
        self.with_colour(color)
    }

    /// Sets the footer.
    #[must_use]
    pub fn with_footer(mut self, footer: EmbedFooter) -> Self {
        self.footer = Some(footer);
        self
    }

    /// Sets the main image.
    #[must_use]
    pub fn with_image(mut self, image: EmbedImage) -> Self {
        self.image = Some(image);
        self
    }

    /// Sets the thumbnail.
    #[must_use]
    pub fn with_thumbnail(mut self, thumbnail: EmbedThumbnail) -> Self {
        self.thumbnail = Some(thumbnail);
        self
    }

    /// Sets the video.
    #[must_use]
    pub fn with_video(mut self, video: EmbedVideo) -> Self {
        self.video = Some(video);
        self
    }

    /// Sets the provider.
    #[must_use]
    pub fn with_provider(mut self, provider: EmbedProvider) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Sets the author.
    #[must_use]
    pub fn with_author(mut self, author: EmbedAuthor) -> Self {
        self.author = Some(author);
        self
    }

    /// Replaces the embed's fields.
    #[must_use]
    pub fn with_fields(mut self, fields: Vec<EmbedField>) -> Self {
        self.fields = fields;
        self
    }

    /// Appends a field.
    #[must_use]
    pub fn add_field(mut self, field: EmbedField) -> Self {
        self.fields.push(field);
        self
    }

    /// Inserts a field at `index`, clamped to the current field count.
    #[must_use]
    pub fn insert_field(mut self, index: usize, field: EmbedField) -> Self {
        let index = index.min(self.fields.len());
        self.fields.insert(index, field);
        self
    }

    /// Removes the field at `index`, ignoring an out-of-range index.
    #[must_use]
    pub fn remove_field(mut self, index: usize) -> Self {
        if index < self.fields.len() {
            self.fields.remove(index);
        }
        self
    }

    /// Removes every field.
    #[must_use]
    pub fn clear_fields(mut self) -> Self {
        self.fields.clear();
        self
    }

    /// Sets the embed flags.
    #[must_use]
    pub fn with_flags(mut self, flags: EmbedFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Returns the total number of characters counted against Discord's
    /// 6000-character embed limit.
    #[must_use]
    pub fn len(&self) -> usize {
        let mut total = self.title.as_ref().map_or(0, |text| text.chars().count())
            + self
                .description
                .as_ref()
                .map_or(0, |text| text.chars().count());

        for field in &self.fields {
            total += field.name.chars().count() + field.value.chars().count();
        }

        if let Some(footer) = &self.footer {
            total += footer.text.chars().count();
        }

        if let Some(author) = &self.author {
            total += author.name.chars().count();
        }

        total
    }

    /// Returns `true` if the embed has no content set.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.title.is_none()
            && self.description.is_none()
            && self.url.is_none()
            && self.timestamp.is_none()
            && self.colour.is_none()
            && self.footer.is_none()
            && self.image.is_none()
            && self.thumbnail.is_none()
            && self.video.is_none()
            && self.provider.is_none()
            && self.author.is_none()
            && self.fields.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use abc::Colour;
    use data::json;
    use enums::{EmbedFlags, EmbedType};

    use super::{
        Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedImage, EmbedProvider, EmbedThumbnail,
        EmbedVideo,
    };

    #[test]
    fn embed_builders_and_helpers() {
        let timestamp = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let embed = Embed::new()
            .with_title("Title")
            .with_description("Description")
            .with_url("https://example.com")
            .with_timestamp(timestamp)
            .with_colour(Colour::BLURPLE)
            .with_footer(
                EmbedFooter::new("footer")
                    .with_icon_url("https://example.com/footer.png")
                    .with_proxy_icon_url("https://proxy.example.com/footer.png"),
            )
            .with_image(
                EmbedImage::new("https://example.com/image.png")
                    .with_proxy_url("https://proxy.example.com/image.png")
                    .with_dimensions(640, 480)
                    .with_flags(EmbedFlags::CONTAINS_EXPLICIT_MEDIA),
            )
            .with_thumbnail(EmbedThumbnail::new("https://example.com/thumb.png"))
            .with_video(EmbedVideo::new("https://example.com/video.mp4"))
            .with_provider(
                EmbedProvider::new()
                    .with_name("Provider")
                    .with_url("https://provider.example.com"),
            )
            .with_author(
                EmbedAuthor::new("Author")
                    .with_url("https://example.com/author")
                    .with_icon_url("https://example.com/author.png")
                    .with_proxy_icon_url("https://proxy.example.com/author.png"),
            )
            .with_fields(vec![EmbedField::new("Name", "Value")])
            .with_flags(EmbedFlags::CONTAINS_EXPLICIT_MEDIA);

        assert_eq!(embed.title.as_deref(), Some("Title"));
        assert_eq!(embed.kind, enums::EmbedType::Rich);
        assert_eq!(embed.description.as_deref(), Some("Description"));
        assert_eq!(embed.url.as_deref(), Some("https://example.com"));
        assert_eq!(embed.timestamp, Some(timestamp));
        assert_eq!(embed.colour, Some(Colour::BLURPLE));
        assert_eq!(
            embed.footer.as_ref().map(|footer| footer.text.as_str()),
            Some("footer")
        );
        assert_eq!(
            embed.image.as_ref().and_then(|image| image.width),
            Some(640)
        );
        assert!(
            embed
                .image
                .as_ref()
                .is_some_and(|image| image.flags.contains_explicit_media())
        );
        assert!(embed.thumbnail.is_some());
        assert!(embed.video.is_some());
        assert_eq!(
            embed
                .provider
                .as_ref()
                .and_then(|provider| provider.name.as_deref()),
            Some("Provider")
        );
        assert_eq!(
            embed.author.as_ref().map(|author| author.name.as_str()),
            Some("Author")
        );
        assert_eq!(embed.fields.len(), 1);
        assert!(embed.flags.contains_explicit_media());
    }

    #[test]
    fn embed_kind_and_colour_aliases() {
        let embed = Embed::new()
            .with_kind(enums::EmbedType::Article)
            .with_color(Colour::RED);

        assert_eq!(embed.kind, enums::EmbedType::Article);
        assert_eq!(embed.colour, Some(Colour::RED));
    }

    #[test]
    fn field_editing() {
        let embed = Embed::new()
            .add_field(EmbedField::new("first", "1"))
            .add_field(EmbedField::new("second", "2").with_inline(false))
            .insert_field(1, EmbedField::new("middle", "1.5"))
            .remove_field(0);

        assert_eq!(embed.fields.len(), 2);
        assert_eq!(embed.fields[0].name, "middle");
        assert_eq!(embed.fields[1].name, "second");
        assert!(!embed.fields[1].inline);

        let embed = embed.clear_fields().remove_field(10);
        assert!(embed.fields.is_empty());

        let clamped = Embed::new().insert_field(5, EmbedField::new("only", "1"));
        assert_eq!(clamped.fields.len(), 1);
        assert_eq!(clamped.fields[0].name, "only");
    }

    #[test]
    fn length_and_emptiness() {
        assert!(Embed::new().is_empty());
        assert_eq!(Embed::new().len(), 0);

        let embed = Embed::new()
            .with_title("Title")
            .with_description("🎉 description")
            .with_author(EmbedAuthor::new("Author"))
            .with_footer(EmbedFooter::new("Footer"))
            .add_field(EmbedField::new("Name", "Value"));

        assert!(!embed.is_empty());
        assert_eq!(
            embed.len(),
            "Title".len() + "🎉 description".chars().count() + 6 + 6 + 4 + 5
        );

        assert!(!Embed::new().with_image(EmbedImage::new("url")).is_empty());
    }

    #[test]
    fn equality_compares_content() {
        let a = Embed::new().with_title("Title").with_colour(Colour::BLUE);
        let b = Embed::new().with_title("Title").with_colour(Colour::BLUE);
        let c = Embed::new().with_title("Other").with_colour(Colour::BLUE);

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, Embed::new().with_title("Title"));
    }

    #[test]
    fn decodes_full_and_minimal_embeds() {
        let value = json::from_str(
            r#"{
                "title": "Title",
                "type": "article",
                "description": "Description",
                "url": "https://example.com",
                "timestamp": "2021-08-01T12:34:56Z",
                "color": 5793266,
                "footer": {
                    "text": "Footer",
                    "icon_url": "https://example.com/icon.png",
                    "proxy_icon_url": "https://example.com/proxy-icon.png"
                },
                "image": {
                    "url": "https://example.com/image.png",
                    "proxy_url": "https://example.com/proxy-image.png",
                    "height": 100,
                    "width": 200,
                    "flags": 16
                },
                "thumbnail": {"url": "https://example.com/thumb.png"},
                "video": {"url": "https://example.com/video.mp4"},
                "provider": {"name": "Provider", "url": "https://provider.example"},
                "author": {
                    "name": "Author",
                    "url": "https://author.example",
                    "icon_url": "https://author.example/icon.png",
                    "proxy_icon_url": "https://author.example/proxy.png"
                },
                "fields": [
                    {"name": "First", "value": "One", "inline": true},
                    {"name": "Second", "value": "Two"}
                ],
                "flags": 32
            }"#,
        )
        .unwrap();

        let embed = Embed::from_value(&value).unwrap();
        assert_eq!(embed.title.as_deref(), Some("Title"));
        assert_eq!(embed.kind, EmbedType::Article);
        assert_eq!(embed.description.as_deref(), Some("Description"));
        assert_eq!(embed.url.as_deref(), Some("https://example.com"));
        assert_eq!(
            embed.timestamp,
            Some(UNIX_EPOCH + Duration::new(1_627_821_296, 0))
        );
        assert_eq!(embed.colour, Some(Colour::new(5_793_266)));
        assert_eq!(
            embed.footer.as_ref().map(|footer| footer.text.as_str()),
            Some("Footer")
        );
        let image = embed.image.as_ref().unwrap();
        assert_eq!(image.url, "https://example.com/image.png");
        assert_eq!(image.height, Some(100));
        assert_eq!(image.width, Some(200));
        assert_eq!(image.flags, EmbedFlags::CONTAINS_EXPLICIT_MEDIA);
        assert_eq!(
            embed.thumbnail.as_ref().map(|thumb| thumb.url.as_str()),
            Some("https://example.com/thumb.png")
        );
        assert_eq!(
            embed.video.as_ref().map(|video| video.url.as_str()),
            Some("https://example.com/video.mp4")
        );
        assert_eq!(
            embed
                .provider
                .as_ref()
                .and_then(|provider| provider.name.as_deref()),
            Some("Provider")
        );
        assert_eq!(
            embed.author.as_ref().map(|author| author.name.as_str()),
            Some("Author")
        );
        assert_eq!(embed.fields.len(), 2);
        assert!(embed.fields[0].inline);
        assert!(!embed.fields[1].inline);
        assert_eq!(embed.flags, EmbedFlags::CONTENT_INVENTORY_ENTRY);

        let minimal = Embed::from_value(&json::from_str("{}").unwrap()).unwrap();
        assert_eq!(minimal.kind, EmbedType::Rich);
        assert_eq!(minimal.title, None);
        assert_eq!(minimal.footer, None);
        assert!(minimal.fields.is_empty());
        assert!(minimal.flags.is_empty());

        let bad_type =
            Embed::from_value(&json::from_str(r#"{"type":"unknown"}"#).unwrap()).unwrap_err();
        assert_eq!(
            bad_type.to_string(),
            "field `embed.type` must be an embed type"
        );

        let bad_field =
            Embed::from_value(&json::from_str(r#"{"fields":[{"name":"First"}]}"#).unwrap())
                .unwrap_err();
        assert_eq!(
            bad_field.to_string(),
            "missing field `embed.fields[0].value`"
        );

        let bad_footer =
            Embed::from_value(&json::from_str(r#"{"footer":{"icon_url":"x"}}"#).unwrap())
                .unwrap_err();
        assert_eq!(bad_footer.to_string(), "missing field `embed.footer.text`");
    }
}
