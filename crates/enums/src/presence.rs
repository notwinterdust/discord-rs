//! Discord presence and activity enumerations.

#![forbid(unsafe_code)]

use std::fmt;

/// A user's online status.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    /// The user is online.
    #[default]
    Online,
    /// Do not disturb.
    DoNotDisturb,
    /// Idle / AFK.
    Idle,
    /// Invisible / appears offline.
    Invisible,
    /// Offline.
    Offline,
}

impl Status {
    /// Returns the string name used by Discord.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::DoNotDisturb => "dnd",
            Self::Idle => "idle",
            Self::Invisible => "invisible",
            Self::Offline => "offline",
        }
    }

    /// Parses a Discord status string.
    #[must_use]
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "online" => Some(Self::Online),
            "dnd" => Some(Self::DoNotDisturb),
            "idle" => Some(Self::Idle),
            "invisible" => Some(Self::Invisible),
            "offline" => Some(Self::Offline),
            _ => None,
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The type of activity a user is engaged in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActivityType {
    /// Playing a game.
    #[default]
    Playing,
    /// Streaming on Twitch or YouTube.
    Streaming,
    /// Listening to music (e.g. Spotify).
    Listening,
    /// Watching media.
    Watching,
    /// Custom status.
    Custom,
    /// Competing in an event.
    Competing,
}

impl ActivityType {
    /// Parses an activity type integer from the gateway.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Playing),
            1 => Some(Self::Streaming),
            2 => Some(Self::Listening),
            3 => Some(Self::Watching),
            4 => Some(Self::Custom),
            5 => Some(Self::Competing),
            _ => None,
        }
    }

    /// Returns the raw integer value.
    #[must_use]
    pub const fn value(self) -> u8 {
        match self {
            Self::Playing => 0,
            Self::Streaming => 1,
            Self::Listening => 2,
            Self::Watching => 3,
            Self::Custom => 4,
            Self::Competing => 5,
        }
    }
}

impl fmt::Display for ActivityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Playing => f.write_str("Playing"),
            Self::Streaming => f.write_str("Streaming"),
            Self::Listening => f.write_str("Listening to"),
            Self::Watching => f.write_str("Watching"),
            Self::Custom => f.write_str("Custom"),
            Self::Competing => f.write_str("Competing in"),
        }
    }
}
