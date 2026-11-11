//! Forum tag models.
//!
//! Forum and media channels own a list of [`ForumTag`] values. Threads created
//! in those channels may apply a subset of the parent's tags.

use std::fmt;
use std::hash::{Hash, Hasher};

use abc::{HasId, Snowflake};
use data::Value;

use crate::PartialEmoji;
use crate::cast::{self, CastError};

/// A tag that can be applied to a thread in a forum or media channel.
///
/// Manually constructed tags have an ID of zero until Discord assigns one.
/// Equality and hashing are based on the tag's ID.
#[derive(Clone, Debug, Default)]
pub struct ForumTag {
    /// The tag's unique ID, or zero for a tag that has not been saved.
    pub id: Snowflake,
    /// The tag's name.
    pub name: String,
    /// Whether only moderators may add or remove the tag.
    pub moderated: bool,
    /// The emoji shown with the tag, if any.
    pub emoji: Option<PartialEmoji>,
}

impl ForumTag {
    /// Creates an unsaved tag with the given name.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    /// Sets the tag's ID.
    #[must_use]
    pub fn with_id(mut self, id: impl Into<Snowflake>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets whether only moderators may add or remove the tag.
    #[must_use]
    pub fn with_moderated(mut self, moderated: bool) -> Self {
        self.moderated = moderated;
        self
    }

    /// Sets the tag's emoji.
    #[must_use]
    pub fn with_emoji(mut self, emoji: PartialEmoji) -> Self {
        self.emoji = Some(emoji);
        self
    }

    /// Decodes a forum tag from a Discord forum tag object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "tag")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let emoji_path = cast::field_path(path, "emoji");
        let emoji = cast::optional_object(object, path, "emoji")?
            .map(|emoji| PartialEmoji::from_object_at(emoji, &emoji_path))
            .transpose()?;
        Ok(Self {
            id: cast::required_snowflake(object, path, "id")?,
            name: cast::required_string(object, path, "name")?.to_owned(),
            moderated: cast::optional_bool(object, path, "moderated", false)?,
            emoji,
        })
    }

    /// Returns whether only moderators may add or remove the tag.
    #[must_use]
    pub const fn is_moderated(&self) -> bool {
        self.moderated
    }

    /// Returns whether the tag has been assigned an ID by Discord.
    #[must_use]
    pub const fn is_saved(&self) -> bool {
        !self.id.is_zero()
    }
}

impl fmt::Display for ForumTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl PartialEq for ForumTag {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ForumTag {}

impl Hash for ForumTag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for ForumTag {
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

    use super::ForumTag;
    use crate::PartialEmoji;

    #[test]
    fn builders_and_helpers() {
        let tag = ForumTag::new("Bug")
            .with_id(10_u64)
            .with_moderated(true)
            .with_emoji(PartialEmoji::unicode("🐛"));

        assert_eq!(tag.id, Snowflake::new(10));
        assert_eq!(tag.id(), Snowflake::new(10));
        assert_eq!(tag.name, "Bug");
        assert!(tag.is_moderated());
        assert!(tag.is_saved());
        assert_eq!(
            tag.emoji.as_ref().map(ToString::to_string),
            Some("🐛".to_owned())
        );
        assert_eq!(tag.to_string(), "Bug");

        let unsaved = ForumTag::new("Question");
        assert!(!unsaved.is_saved());
        assert_eq!(unsaved.id, Snowflake::ZERO);
        assert!(!unsaved.is_moderated());
        assert!(unsaved.emoji.is_none());
    }

    #[test]
    fn equality_and_hash_use_id() {
        let a = ForumTag::new("Bug")
            .with_id(1_u64)
            .with_emoji(PartialEmoji::unicode("🐛"));
        let b = ForumTag::new("Defect").with_id(1_u64).with_moderated(true);
        let c = ForumTag::new("Bug").with_id(2_u64);

        assert_eq!(a, b);
        assert_ne!(a, c);

        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        a.hash(&mut hasher_a);
        b.hash(&mut hasher_b);
        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }

    #[test]
    fn decodes_forum_tag_objects() {
        let value = json::from_str(
            r#"{
                "id": "10",
                "name": "Bug",
                "moderated": true,
                "emoji": {"id": "20", "name": "bug", "animated": true}
            }"#,
        )
        .unwrap();
        let tag = ForumTag::from_value(&value).unwrap();
        assert_eq!(tag.id, Snowflake::new(10));
        assert_eq!(tag.name, "Bug");
        assert!(tag.moderated);
        let emoji = tag.emoji.as_ref().unwrap();
        assert_eq!(emoji.id, Some(Snowflake::new(20)));
        assert_eq!(emoji.name.as_deref(), Some("bug"));
        assert!(emoji.animated);

        let minimal = ForumTag::from_value(
            &json::from_str(r#"{"id":"10","name":"Idea","emoji":{"name":"💡"}}"#).unwrap(),
        )
        .unwrap();
        assert!(!minimal.moderated);
        assert_eq!(
            minimal
                .emoji
                .as_ref()
                .and_then(|emoji| emoji.name.as_deref()),
            Some("💡")
        );

        let missing =
            ForumTag::from_value(&json::from_str(r#"{"name":"Bug"}"#).unwrap()).unwrap_err();
        assert_eq!(missing.to_string(), "missing field `tag.id`");

        let bad_emoji =
            ForumTag::from_value(&json::from_str(r#"{"id":"10","name":"Bug","emoji":1}"#).unwrap())
                .unwrap_err();
        assert_eq!(bad_emoji.to_string(), "field `tag.emoji` must be an object");
    }
}
