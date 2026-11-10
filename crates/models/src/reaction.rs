//! Message reaction model.

use std::fmt;
use std::hash::{Hash, Hasher};

use crate::PartialEmoji;

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
}
