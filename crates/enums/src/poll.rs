//! Poll enumerations.

use std::fmt;

/// The layout used to display a poll.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PollLayoutType {
    /// Discord's default poll layout.
    #[default]
    Default = 1,
}

impl PollLayoutType {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Default),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
        }
    }
}

impl fmt::Display for PollLayoutType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::PollLayoutType;

    #[test]
    fn poll_layout_type_round_trips() {
        assert_eq!(PollLayoutType::from_value(1), Some(PollLayoutType::Default));
        assert_eq!(PollLayoutType::from_value(0), None);
        assert_eq!(PollLayoutType::from_value(2), None);
        assert_eq!(PollLayoutType::default(), PollLayoutType::Default);
        assert_eq!(PollLayoutType::Default.value(), 1);
        assert_eq!(PollLayoutType::Default.to_string(), "default");
    }
}
