//! Discord snowflake identifiers.
//!
//! | 63-22 ; milliseconds since the Discord epoch | 21-17 ; internal worker ID   | 16-12 ; internal process ID  | 11-0  ; increment  |

use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Milliseconds between the Unix epoch and the Discord epoch
pub const DISCORD_EPOCH_MS: u64 = 1_420_070_400_000;

/// A Discord snowflake.
///
/// This is the Rust equivalent of the `id` attribute exposed by
/// `discord.abc.Snowflake`. Keeping it as a newtype, instead of a bare `u64`)
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Snowflake(u64);

impl Snowflake {
    /// The zero snowflake.
    pub const ZERO: Self = Self(0);

    /// Creates a snowflake from its raw representation.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw representation.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns `true` if the raw representation is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns the creation time as milliseconds since the Unix epoch.
    #[must_use]
    pub const fn timestamp_ms(self) -> u64 {
        (self.0 >> 22) + DISCORD_EPOCH_MS
    }

    /// Returns the creation time as a [`SystemTime`].
    #[must_use]
    pub fn created_at(self) -> SystemTime {
        UNIX_EPOCH + Duration::from_millis(self.timestamp_ms())
    }

    /// Returns the 5-bit internal worker ID.
    #[must_use]
    pub const fn worker_id(self) -> u8 {
        ((self.0 >> 17) & 0x1F) as u8
    }

    /// Returns the 5-bit internal process ID.
    #[must_use]
    pub const fn process_id(self) -> u8 {
        ((self.0 >> 12) & 0x1F) as u8
    }

    /// Returns the 12-bit increment.
    #[must_use]
    pub const fn increment(self) -> u16 {
        (self.0 & 0xFFF) as u16
    }

    /// Builds the snowflake for a timestamp in milliseconds since the Unix
    /// epoch, with the lower 22 bits set to zero.
    /// Returns `None` if `unix_ms` is before the Discord epoch. This is the
    /// low end of the range 
    #[must_use]
    pub const fn from_timestamp_ms(unix_ms: u64) -> Option<Self> {
        match unix_ms.checked_sub(DISCORD_EPOCH_MS) {
            Some(delta) => Some(Self(delta << 22)),
            None => None,
        }
    }

    /// Builds the snowflake for a timestamp in milliseconds since the Unix
    /// epoch, with the lower 22 bits set to all ones.
    /// Returns `None` if `unix_ms` is before the Discord epoch. This is the
    /// high end of the range 
    #[must_use]
    pub const fn from_timestamp_ms_high(unix_ms: u64) -> Option<Self> {
        match unix_ms.checked_sub(DISCORD_EPOCH_MS) {
            Some(delta) => Some(Self((delta << 22) | 0x3F_FFFF)),
            None => None,
        }
    }
}

impl fmt::Display for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Debug for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Snowflake({})", self.0)
    }
}

impl From<u64> for Snowflake {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Snowflake> for u64 {
    fn from(value: Snowflake) -> Self {
        value.0
    }
}

impl FromStr for Snowflake {
    type Err = ParseIntError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.parse::<u64>().map(Self)
    }
}

/// A type that exposes a Discord snowflake.
/// Implement it on every model that has an `id`.
pub trait HasId {
    /// Returns the object's unique ID.
    fn id(&self) -> Snowflake;
}

impl HasId for Snowflake {
    fn id(&self) -> Snowflake {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_discord_epoch_boundaries() {
        assert_eq!(Snowflake::new(0).timestamp_ms(), DISCORD_EPOCH_MS);
        assert_eq!(
            Snowflake::from_timestamp_ms(DISCORD_EPOCH_MS),
            Some(Snowflake::ZERO)
        );
        assert_eq!(Snowflake::from_timestamp_ms(DISCORD_EPOCH_MS - 1), None);
    }

    #[test]
    fn extracts_snowflake_parts() {
        let unix_ms = 1_577_836_800_000;
        let delta = unix_ms - DISCORD_EPOCH_MS;
        let raw = (delta << 22) | (1 << 17) | (2 << 12) | 3;
        let snowflake = Snowflake::new(raw);

        assert_eq!(snowflake.timestamp_ms(), unix_ms);
        assert_eq!(snowflake.worker_id(), 1);
        assert_eq!(snowflake.process_id(), 2);
        assert_eq!(snowflake.increment(), 3);
        assert_eq!(snowflake.to_string(), raw.to_string());
        assert_eq!(raw.to_string().parse::<Snowflake>(), Ok(snowflake));
    }

    #[test]
    fn high_snowflake_has_all_low_bits_set() {
        let some_ms = DISCORD_EPOCH_MS + 42;
        let high = Snowflake::from_timestamp_ms_high(some_ms).unwrap();

        assert_eq!(high.timestamp_ms(), some_ms);
        assert_eq!(high.increment(), 0xFFF);
        assert_eq!(high.worker_id(), 0x1F);
        assert_eq!(high.process_id(), 0x1F);
    }

    #[test]
    fn has_id_for_snowflake() {
        let snowflake = Snowflake::new(123);
        assert_eq!(snowflake.id(), snowflake);
    }
}
