//! Guild scheduled event enumerations.

use std::fmt;

/// The entity a guild scheduled event takes place in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum GuildScheduledEventEntityType {
    /// A stage instance.
    #[default]
    StageInstance = 1,
    /// A voice channel.
    Voice = 2,
    /// A location outside of Discord.
    External = 3,
}

impl GuildScheduledEventEntityType {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::StageInstance),
            2 => Some(Self::Voice),
            3 => Some(Self::External),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StageInstance => "stage_instance",
            Self::Voice => "voice",
            Self::External => "external",
        }
    }

    /// Returns `true` if the event is held in a stage instance.
    #[must_use]
    pub const fn is_stage_instance(self) -> bool {
        matches!(self, Self::StageInstance)
    }

    /// Returns `true` if the event is held in a voice channel.
    #[must_use]
    pub const fn is_voice(self) -> bool {
        matches!(self, Self::Voice)
    }

    /// Returns `true` if the event is held outside of Discord.
    #[must_use]
    pub const fn is_external(self) -> bool {
        matches!(self, Self::External)
    }

    /// Returns `true` if the event requires a channel.
    #[must_use]
    pub const fn requires_channel(self) -> bool {
        !self.is_external()
    }
}

impl fmt::Display for GuildScheduledEventEntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The status of a guild scheduled event.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum GuildScheduledEventStatus {
    /// The event has been scheduled but has not started.
    #[default]
    Scheduled = 1,
    /// The event is in progress.
    Active = 2,
    /// The event has completed.
    Completed = 3,
    /// The event was canceled.
    Canceled = 4,
}

impl GuildScheduledEventStatus {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Scheduled),
            2 => Some(Self::Active),
            3 => Some(Self::Completed),
            4 => Some(Self::Canceled),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scheduled => "scheduled",
            Self::Active => "active",
            Self::Completed => "completed",
            Self::Canceled => "canceled",
        }
    }

    /// Returns `true` if the event has not started.
    #[must_use]
    pub const fn is_scheduled(self) -> bool {
        matches!(self, Self::Scheduled)
    }

    /// Returns `true` if the event is in progress.
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns `true` if the event has completed.
    #[must_use]
    pub const fn is_completed(self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Returns `true` if the event was canceled.
    #[must_use]
    pub const fn is_canceled(self) -> bool {
        matches!(self, Self::Canceled)
    }

    /// Returns `true` if the event will not change status again.
    #[must_use]
    pub const fn is_final(self) -> bool {
        matches!(self, Self::Completed | Self::Canceled)
    }
}

impl fmt::Display for GuildScheduledEventStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::{GuildScheduledEventEntityType, GuildScheduledEventStatus};

    #[test]
    fn entity_types_round_trip() {
        let values = [
            (1, GuildScheduledEventEntityType::StageInstance),
            (2, GuildScheduledEventEntityType::Voice),
            (3, GuildScheduledEventEntityType::External),
        ];
        for (value, kind) in values {
            assert_eq!(GuildScheduledEventEntityType::from_value(value), Some(kind));
            assert_eq!(kind.value(), value);
        }
        assert_eq!(GuildScheduledEventEntityType::from_value(0), None);
        assert_eq!(
            GuildScheduledEventEntityType::default(),
            GuildScheduledEventEntityType::StageInstance
        );
        assert!(GuildScheduledEventEntityType::StageInstance.is_stage_instance());
        assert!(GuildScheduledEventEntityType::Voice.is_voice());
        assert!(GuildScheduledEventEntityType::External.is_external());
        assert!(!GuildScheduledEventEntityType::External.requires_channel());
        assert!(GuildScheduledEventEntityType::Voice.requires_channel());
        assert_eq!(
            GuildScheduledEventEntityType::External.to_string(),
            "external"
        );
    }

    #[test]
    fn statuses_round_trip() {
        let values = [
            (1, GuildScheduledEventStatus::Scheduled),
            (2, GuildScheduledEventStatus::Active),
            (3, GuildScheduledEventStatus::Completed),
            (4, GuildScheduledEventStatus::Canceled),
        ];
        for (value, status) in values {
            assert_eq!(GuildScheduledEventStatus::from_value(value), Some(status));
            assert_eq!(status.value(), value);
        }
        assert_eq!(GuildScheduledEventStatus::from_value(0), None);
        assert_eq!(
            GuildScheduledEventStatus::default(),
            GuildScheduledEventStatus::Scheduled
        );
        assert!(GuildScheduledEventStatus::Scheduled.is_scheduled());
        assert!(GuildScheduledEventStatus::Active.is_active());
        assert!(GuildScheduledEventStatus::Completed.is_completed());
        assert!(GuildScheduledEventStatus::Canceled.is_canceled());
        assert!(!GuildScheduledEventStatus::Active.is_final());
        assert!(GuildScheduledEventStatus::Completed.is_final());
        assert!(GuildScheduledEventStatus::Canceled.is_final());
        assert_eq!(GuildScheduledEventStatus::Canceled.to_string(), "canceled");
    }
}
