//! Message reaction model.

use std::fmt;
use std::hash::{Hash, Hasher};

use data::Value;

use crate::PartialEmoji;
use crate::cast::{self, CastError};

/// A reaction to a message.
///
/// Equality and hashing are based on the emoji, matching Discord's view of a
/// reaction as "one emoji per message".
#[derive(Clone, Debug)]
pub struct Reaction {
    /// The emoji that was used.
    pub emoji: PartialEmoji,
    /// The total number of reactions, normal and burst combined.
    pub count: u64,
    /// Whether the current user reacted with this emoji.
    pub me: bool,
    /// Whether the current user used a super reaction.
    pub me_burst: bool,
    /// The number of normal reactions.
    pub normal_count: u64,
    /// The number of super reactions.
    pub burst_count: u64,
}

impl Reaction {
    /// Creates a reaction with a count of one and no current-user state.
    #[must_use]
    pub fn new(emoji: PartialEmoji) -> Self {
        Self {
            emoji,
            count: 1,
            me: false,
            me_burst: false,
            normal_count: 0,
            burst_count: 0,
        }
    }

    /// Sets the total reaction count.
    #[must_use]
    pub fn with_count(mut self, count: u64) -> Self {
        self.count = count;
        self
    }

    /// Sets whether the current user reacted.
    #[must_use]
    pub fn with_me(mut self, me: bool) -> Self {
        self.me = me;
        self
    }

    /// Sets whether the current user used a super reaction.
    #[must_use]
    pub fn with_me_burst(mut self, me_burst: bool) -> Self {
        self.me_burst = me_burst;
        self
    }

    /// Sets the normal and burst reaction counts.
    #[must_use]
    pub fn with_count_details(mut self, normal_count: u64, burst_count: u64) -> Self {
        self.normal_count = normal_count;
        self.burst_count = burst_count;
        self
    }

    /// Decodes a reaction from a Discord reaction object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "reaction")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let emoji_path = cast::field_path(path, "emoji");
        let emoji = PartialEmoji::from_object_at(
            cast::required_object(object, path, "emoji")?,
            &emoji_path,
        )?;

        let count_details_path = cast::field_path(path, "count_details");
        let (normal_count, burst_count) =
            match cast::optional_object(object, path, "count_details")? {
                Some(details) => (
                    cast::required_u64(details, &count_details_path, "normal")?,
                    cast::required_u64(details, &count_details_path, "burst")?,
                ),
                None => (0, 0),
            };

        Ok(Self {
            emoji,
            count: cast::required_u64(object, path, "count")?,
            me: cast::required_bool(object, path, "me")?,
            me_burst: cast::optional_bool(object, path, "me_burst", false)?,
            normal_count,
            burst_count,
        })
    }

    /// Returns whether this reaction uses a custom emoji.
    #[must_use]
    pub const fn is_custom_emoji(&self) -> bool {
        self.emoji.is_custom_emoji()
    }
}

impl fmt::Display for Reaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.emoji.fmt(f)
    }
}

impl PartialEq for Reaction {
    fn eq(&self, other: &Self) -> bool {
        self.emoji == other.emoji
    }
}

impl Eq for Reaction {}

impl Hash for Reaction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.emoji.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use abc::Snowflake;
    use data::json;

    use super::Reaction;
    use crate::PartialEmoji;

    #[test]
    fn builders_and_helpers() {
        let reaction = Reaction::new(PartialEmoji::custom(10_u64, "wave"))
            .with_count(5)
            .with_me(true)
            .with_me_burst(true)
            .with_count_details(3, 2);

        assert_eq!(reaction.count, 5);
        assert!(reaction.me);
        assert!(reaction.me_burst);
        assert_eq!(reaction.normal_count, 3);
        assert_eq!(reaction.burst_count, 2);
        assert!(reaction.is_custom_emoji());
        assert_eq!(reaction.to_string(), "<:wave:10>");

        let unicode = Reaction::new(PartialEmoji::unicode("🎉"));
        assert!(!unicode.is_custom_emoji());
        assert_eq!(unicode.count, 1);
        assert!(!unicode.me);
        assert!(!unicode.me_burst);
        assert_eq!(unicode.to_string(), "🎉");
    }

    #[test]
    fn equality_and_hash_use_emoji() {
        let a = Reaction::new(PartialEmoji::unicode("🎉")).with_count(1);
        let b = Reaction::new(PartialEmoji::unicode("🎉")).with_count(9);
        let c = Reaction::new(PartialEmoji::unicode("🎊"));

        assert_eq!(a, b);
        assert_ne!(a, c);

        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        a.hash(&mut hasher_a);
        b.hash(&mut hasher_b);
        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }

    #[test]
    fn decodes_reaction_objects() {
        let value = json::from_str(
            r#"{
                "emoji": {"id": "10", "name": "wave", "animated": true},
                "count": 5,
                "me": true,
                "me_burst": true,
                "count_details": {"burst": 2, "normal": 3}
            }"#,
        )
        .unwrap();

        let reaction = Reaction::from_value(&value).unwrap();
        assert_eq!(reaction.count, 5);
        assert!(reaction.me);
        assert!(reaction.me_burst);
        assert_eq!(reaction.normal_count, 3);
        assert_eq!(reaction.burst_count, 2);
        assert_eq!(reaction.emoji.id, Some(Snowflake::new(10)));
        assert!(reaction.is_custom_emoji());

        let unicode = Reaction::from_value(
            &json::from_str(r#"{"emoji":{"name":"🎉"},"count":1,"me":false}"#).unwrap(),
        )
        .unwrap();
        assert_eq!(unicode.count, 1);
        assert!(!unicode.me);
        assert!(!unicode.me_burst);
        assert_eq!(unicode.normal_count, 0);
        assert_eq!(unicode.burst_count, 0);
        assert!(!unicode.is_custom_emoji());

        let missing_me =
            Reaction::from_value(&json::from_str(r#"{"emoji":{"name":"🎉"},"count":1}"#).unwrap())
                .unwrap_err();
        assert_eq!(missing_me.to_string(), "missing field `reaction.me`");

        let bad_emoji = Reaction::from_value(
            &json::from_str(r#"{"emoji":"🎉","count":1,"me":false}"#).unwrap(),
        )
        .unwrap_err();
        assert_eq!(
            bad_emoji.to_string(),
            "field `reaction.emoji` must be an object"
        );

        let bad_details = Reaction::from_value(
            &json::from_str(
                r#"{"emoji":{"name":"🎉"},"count":1,"me":false,"count_details":{"normal":1}}"#,
            )
            .unwrap(),
        )
        .unwrap_err();
        assert_eq!(
            bad_details.to_string(),
            "missing field `reaction.count_details.burst`"
        );
    }
}
