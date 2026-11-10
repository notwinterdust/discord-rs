//! Emoji models.
//!
//! [`PartialEmoji`] represents both custom and Unicode emojis. It is the
//! representation used by reactions, message components and forum tags.

use std::convert::Infallible;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::time::SystemTime;

use abc::{CDN_BASE, Snowflake};

/// A Unicode emoji or a reference to a custom emoji.
#[derive(Clone, Debug, Default)]
pub struct PartialEmoji {
    /// The custom emoji name, or the Unicode codepoint for a Unicode emoji.
    pub name: Option<String>,
    /// The custom emoji ID, or `None` for a Unicode emoji.
    pub id: Option<Snowflake>,
    /// Whether the custom emoji is animated.
    pub animated: bool,
}

impl PartialEmoji {
    /// Creates a Unicode emoji.
    #[must_use]
    pub fn unicode(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            id: None,
            animated: false,
        }
    }

    /// Creates a custom emoji.
    #[must_use]
    pub fn custom(id: impl Into<Snowflake>, name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            id: Some(id.into()),
            animated: false,
        }
    }

    /// Sets whether the custom emoji is animated.
    #[must_use]
    pub fn with_animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Parses a Discord emoji representation.
    ///
    /// The accepted custom formats are `name:id`, `<:name:id>`, `a:name:id`
    /// and `<a:name:id>`. Any other value is treated as a Unicode emoji.
    #[must_use]
    pub fn parse(value: &str) -> Self {
        let trimmed = value
            .strip_prefix('<')
            .and_then(|value| value.strip_suffix('>'))
            .unwrap_or(value);
        let (animated, rest) = trimmed
            .strip_prefix("a:")
            .map_or((false, trimmed), |rest| (true, rest));

        if let Some((name, id)) = rest.rsplit_once(':')
            && !name.is_empty()
            && let Ok(id) = id.parse::<u64>()
        {
            return Self::custom(Snowflake::new(id), name).with_animated(animated);
        }

        Self::unicode(value)
    }

    /// Returns whether this is a custom emoji.
    #[must_use]
    pub const fn is_custom_emoji(&self) -> bool {
        self.id.is_some()
    }

    /// Returns whether this is a Unicode emoji.
    #[must_use]
    pub const fn is_unicode_emoji(&self) -> bool {
        self.id.is_none()
    }

    /// Returns the custom emoji's CDN URL, or `None` for a Unicode emoji.
    #[must_use]
    pub fn url(&self) -> Option<String> {
        let id = self.id?;
        let format = if self.animated {
            "webp?animated=true"
        } else {
            "png"
        };
        Some(format!("{CDN_BASE}/emojis/{id}.{format}"))
    }

    /// Returns the emoji's creation time, or `None` for a Unicode emoji.
    #[must_use]
    pub fn created_at(&self) -> Option<SystemTime> {
        self.id.map(Snowflake::created_at)
    }

    /// Returns the value used when adding this emoji as a reaction.
    #[must_use]
    pub fn as_reaction(&self) -> String {
        match (self.name.as_deref(), self.id) {
            (_, None) => self.name.clone().unwrap_or_default(),
            (Some(name), Some(id)) => format!("{name}:{id}"),
            (None, Some(id)) => id.to_string(),
        }
    }
}

impl FromStr for PartialEmoji {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(value))
    }
}

impl fmt::Display for PartialEmoji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name.as_deref().filter(|name| !name.is_empty());
        let name = name.unwrap_or("_");

        match self.id {
            None => f.write_str(name),
            Some(id) if self.animated => write!(f, "<a:{name}:{id}>"),
            Some(id) => write!(f, "<:{name}:{id}>"),
        }
    }
}

impl PartialEq for PartialEmoji {
    fn eq(&self, other: &Self) -> bool {
        match self.id {
            Some(id) => other.id == Some(id),
            None => other.id.is_none() && self.name == other.name,
        }
    }
}

impl Eq for PartialEmoji {}

impl Hash for PartialEmoji {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.id {
            Some(id) => id.hash(state),
            None => self.name.hash(state),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use abc::Snowflake;

    use super::PartialEmoji;

    #[test]
    fn unicode_emoji() {
        let emoji = PartialEmoji::unicode("🎉");

        assert_eq!(emoji.name.as_deref(), Some("🎉"));
        assert_eq!(emoji.id, None);
        assert!(emoji.is_unicode_emoji());
        assert!(!emoji.is_custom_emoji());
        assert_eq!(emoji.url(), None);
        assert_eq!(emoji.created_at(), None);
        assert_eq!(emoji.as_reaction(), "🎉");
        assert_eq!(emoji.to_string(), "🎉");
    }

    #[test]
    fn custom_emoji() {
        let emoji = PartialEmoji::custom(10_u64, "wave").with_animated(true);

        assert_eq!(emoji.id, Some(Snowflake::new(10)));
        assert_eq!(emoji.name.as_deref(), Some("wave"));
        assert!(emoji.is_custom_emoji());
        assert!(!emoji.is_unicode_emoji());
        assert_eq!(
            emoji.url().as_deref(),
            Some("https://cdn.discordapp.com/emojis/10.webp?animated=true")
        );
        assert_eq!(emoji.created_at(), Some(Snowflake::new(10).created_at()));
        assert_eq!(emoji.as_reaction(), "wave:10");
        assert_eq!(emoji.to_string(), "<a:wave:10>");

        let static_emoji = PartialEmoji::custom(11_u64, "wave");
        assert_eq!(static_emoji.to_string(), "<:wave:11>");
        assert_eq!(
            static_emoji.url().as_deref(),
            Some("https://cdn.discordapp.com/emojis/11.png")
        );
    }

    #[test]
    fn parses_custom_emoji_formats() {
        let emoji = PartialEmoji::parse("<:wave:10>");
        assert_eq!(emoji, PartialEmoji::custom(10_u64, "wave"));

        let animated = PartialEmoji::parse("<a:wave:10>");
        assert_eq!(
            animated,
            PartialEmoji::custom(10_u64, "wave").with_animated(true)
        );
        assert!(animated.animated);

        assert_eq!(
            PartialEmoji::parse("wave:10"),
            PartialEmoji::custom(10_u64, "wave")
        );
        assert_eq!(
            PartialEmoji::parse("a:wave:10"),
            PartialEmoji::custom(10_u64, "wave").with_animated(true)
        );

        let unicode = PartialEmoji::parse("🎉");
        assert_eq!(unicode, PartialEmoji::unicode("🎉"));
        assert!(unicode.is_unicode_emoji());

        let invalid = PartialEmoji::parse("name:not-an-id");
        assert!(invalid.is_unicode_emoji());
        assert_eq!(invalid.name.as_deref(), Some("name:not-an-id"));

        let parsed: PartialEmoji = "<:wave:10>".parse().unwrap();
        assert_eq!(parsed, PartialEmoji::custom(10_u64, "wave"));
    }

    #[test]
    fn equality_and_hash_use_name_or_id() {
        let unicode_a = PartialEmoji::unicode("🎉");
        let unicode_b = PartialEmoji::unicode("🎉");
        let unicode_c = PartialEmoji::unicode("🎊");
        assert_eq!(unicode_a, unicode_b);
        assert_ne!(unicode_a, unicode_c);

        let custom_a = PartialEmoji::custom(10_u64, "wave");
        let custom_b = PartialEmoji::custom(10_u64, "different-name");
        let custom_c = PartialEmoji::custom(11_u64, "wave");
        assert_eq!(custom_a, custom_b);
        assert_ne!(custom_a, custom_c);
        assert_ne!(custom_a, unicode_a);

        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        custom_a.hash(&mut hasher_a);
        custom_b.hash(&mut hasher_b);
        assert_eq!(hasher_a.finish(), hasher_b.finish());

        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        unicode_a.hash(&mut hasher_a);
        unicode_b.hash(&mut hasher_b);
        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }
}
