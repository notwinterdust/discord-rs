//! Stage instance model.

use abc::Snowflake;
use data::Value;

use crate::cast::{self, CastError};

/// Privacy level of a stage instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StagePrivacyLevel {
    /// Visible publicly (deprecated by Discord, but kept in API).
    Public = 1,
    /// Visible only to guild members.
    GuildOnly = 2,
}

impl StagePrivacyLevel {
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Public),
            2 => Some(Self::GuildOnly),
            _ => None,
        }
    }
}

/// A live Stage Instance in a stage channel.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StageInstance {
    /// Unique stage instance ID.
    pub id: Snowflake,
    /// The guild ID.
    pub guild_id: Snowflake,
    /// The stage channel ID.
    pub channel_id: Snowflake,
    /// The topic of the stage instance (1-120 characters).
    pub topic: String,
    /// Privacy level.
    pub privacy_level: StagePrivacyLevel,
    /// Associated scheduled event ID.
    pub guild_scheduled_event_id: Option<Snowflake>,
}

impl StageInstance {
    pub fn from_value(value: &Value, path: &str) -> Result<Self, CastError> {
        let obj = cast::object(value, path)?;
        let id = cast::required_snowflake(obj, path, "id")?;
        let guild_id = cast::required_snowflake(obj, path, "guild_id")?;
        let channel_id = cast::required_snowflake(obj, path, "channel_id")?;
        let topic = cast::required_string(obj, path, "topic")?.to_owned();
        let privacy_raw = cast::required_u16(obj, path, "privacy_level")? as u8;
        let privacy_level = StagePrivacyLevel::from_value(privacy_raw)
            .unwrap_or(StagePrivacyLevel::GuildOnly);
        let guild_scheduled_event_id = cast::optional_snowflake(obj, path, "guild_scheduled_event_id")?;

        Ok(Self {
            id,
            guild_id,
            channel_id,
            topic,
            privacy_level,
            guild_scheduled_event_id,
        })
    }
}
