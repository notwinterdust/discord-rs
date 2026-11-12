//! Soundboard sound models.

use abc::Snowflake;
use data::Value;

use crate::cast::{self, CastError};

/// A soundboard sound item.
#[derive(Clone, Debug, PartialEq)]
pub struct SoundboardSound {
    /// Sound identifier.
    pub sound_id: Snowflake,
    /// Sound name.
    pub name: String,
    /// Sound audio volume from 0.0 to 1.0.
    pub volume: f64,
    /// Emoji ID if custom emoji.
    pub emoji_id: Option<Snowflake>,
    /// Emoji unicode name if standard emoji.
    pub emoji_name: Option<String>,
    /// Guild ID where sound belongs, or None for Discord default sounds.
    pub guild_id: Option<Snowflake>,
    /// Whether the sound is currently available for use.
    pub available: bool,
    /// User who uploaded the sound.
    pub user_id: Option<Snowflake>,
}

impl SoundboardSound {
    /// Returns true if this is a default Discord soundboard sound.
    #[must_use]
    pub fn is_default(&self) -> bool {
        self.guild_id.is_none()
    }

    pub fn from_value(value: &Value, path: &str) -> Result<Self, CastError> {
        let obj = cast::object(value, path)?;
        let sound_id = cast::required_snowflake(obj, path, "sound_id")?;
        let name = cast::required_string(obj, path, "name")?.to_owned();
        let volume = cast::optional_f64(obj, path, "volume")?.unwrap_or(1.0);
        let emoji_id = cast::optional_snowflake(obj, path, "emoji_id")?;
        let emoji_name = cast::optional_string(obj, path, "emoji_name")?.map(ToOwned::to_owned);
        let guild_id = cast::optional_snowflake(obj, path, "guild_id")?;
        let available = cast::optional_bool(obj, path, "available", true)?;
        let user_id = cast::optional_snowflake(obj, path, "user_id")?;

        Ok(Self {
            sound_id,
            name,
            volume,
            emoji_id,
            emoji_name,
            guild_id,
            available,
            user_id,
        })
    }
}
