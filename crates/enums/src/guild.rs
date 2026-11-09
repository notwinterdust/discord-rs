//! Guild-level enumerations.

use std::fmt;

/// The guild's verification level.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum VerificationLevel {
    /// No verification required.
    #[default]
    None = 0,
    /// Must be registered on Discord for longer than five minutes.
    Low = 1,
    /// Must be a member of the guild for longer than ten minutes.
    Medium = 2,
    /// Must have a verified phone number.
    High = 3,
    /// Must have a verified phone number and be registered for longer than
    /// five minutes.
    Highest = 4,
}

impl VerificationLevel {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::None),
            1 => Some(Self::Low),
            2 => Some(Self::Medium),
            3 => Some(Self::High),
            4 => Some(Self::Highest),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Highest => "highest",
        }
    }
}

impl fmt::Display for VerificationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The guild's default notification setting.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum NotificationLevel {
    /// Notify for every message.
    #[default]
    AllMessages = 0,
    /// Only notify for mentions.
    OnlyMentions = 1,
}

impl NotificationLevel {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::AllMessages),
            1 => Some(Self::OnlyMentions),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllMessages => "all_messages",
            Self::OnlyMentions => "only_mentions",
        }
    }
}

impl fmt::Display for NotificationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The guild's explicit content filter level.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ContentFilter {
    /// No content is filtered.
    #[default]
    Disabled = 0,
    /// Content from members without a role is filtered.
    NoRole = 1,
    /// All content is filtered.
    AllMembers = 2,
}

impl ContentFilter {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Disabled),
            1 => Some(Self::NoRole),
            2 => Some(Self::AllMembers),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::NoRole => "no_role",
            Self::AllMembers => "all_members",
        }
    }
}

impl fmt::Display for ContentFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The guild's multi-factor authentication requirement.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MfaLevel {
    /// MFA is not required.
    #[default]
    Disabled = 0,
    /// Members with moderation permissions must have MFA enabled.
    Require2fa = 1,
}

impl MfaLevel {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Disabled),
            1 => Some(Self::Require2fa),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Require2fa => "require_2fa",
        }
    }
}

impl fmt::Display for MfaLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The guild's NSFW level.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum NsfwLevel {
    /// The guild has no NSFW classification.
    #[default]
    Default = 0,
    /// The guild may contain NSFW content.
    Explicit = 1,
    /// The guild is safe for all ages.
    Safe = 2,
    /// The guild is age-restricted.
    AgeRestricted = 3,
}

impl NsfwLevel {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Default),
            1 => Some(Self::Explicit),
            2 => Some(Self::Safe),
            3 => Some(Self::AgeRestricted),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Explicit => "explicit",
            Self::Safe => "safe",
            Self::AgeRestricted => "age_restricted",
        }
    }
}

impl fmt::Display for NsfwLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::{ContentFilter, MfaLevel, NotificationLevel, NsfwLevel, VerificationLevel};

    #[test]
    fn values_round_trip() {
        for value in 0..=4 {
            let level = VerificationLevel::from_value(value).unwrap();
            assert_eq!(level.value(), value);
        }
        assert_eq!(VerificationLevel::from_value(5), None);
        assert_eq!(VerificationLevel::default(), VerificationLevel::None);

        assert_eq!(
            NotificationLevel::from_value(1),
            Some(NotificationLevel::OnlyMentions)
        );
        assert_eq!(
            ContentFilter::from_value(2),
            Some(ContentFilter::AllMembers)
        );
        assert_eq!(MfaLevel::from_value(1), Some(MfaLevel::Require2fa));
        assert_eq!(NsfwLevel::from_value(3), Some(NsfwLevel::AgeRestricted));

        assert_eq!(MfaLevel::from_value(2), None);
        assert_eq!(NsfwLevel::from_value(4), None);
    }

    #[test]
    fn display_names() {
        assert_eq!(VerificationLevel::Highest.to_string(), "highest");
        assert_eq!(NotificationLevel::AllMessages.to_string(), "all_messages");
        assert_eq!(ContentFilter::NoRole.to_string(), "no_role");
        assert_eq!(MfaLevel::Require2fa.to_string(), "require_2fa");
        assert_eq!(NsfwLevel::AgeRestricted.to_string(), "age_restricted");
    }
}
