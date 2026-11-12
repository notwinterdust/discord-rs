//! Discord presence and activity models.

use abc::Snowflake;
use data::Value;
use enums::{ActivityType, Status};

use crate::cast::{self, CastError};

/// Information about a user's client platforms and status.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ClientStatus {
    /// Status on desktop client.
    pub desktop: Option<Status>,
    /// Status on mobile client.
    pub mobile: Option<Status>,
    /// Status on web client.
    pub web: Option<Status>,
}

impl ClientStatus {
    /// Decodes a client status object.
    pub fn from_value(value: &Value, path: &str) -> Result<Self, CastError> {
        let obj = cast::object(value, path)?;
        Ok(Self {
            desktop: obj.get("desktop").and_then(|v| match v {
                Value::String(s) => Status::from_str(s),
                _ => None,
            }),
            mobile: obj.get("mobile").and_then(|v| match v {
                Value::String(s) => Status::from_str(s),
                _ => None,
            }),
            web: obj.get("web").and_then(|v| match v {
                Value::String(s) => Status::from_str(s),
                _ => None,
            }),
        })
    }
}

/// A user activity (game, stream, spotify, custom status).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Activity {
    /// Activity name.
    pub name: String,
    /// Activity type.
    pub kind: ActivityType,
    /// Stream URL when streaming.
    pub url: Option<String>,
    /// Application ID if attached to a rich presence app.
    pub application_id: Option<Snowflake>,
    /// Details / what the player is doing.
    pub details: Option<String>,
    /// User's current party status.
    pub state: Option<String>,
}

impl Activity {
    /// Creates a playing game activity.
    #[must_use]
    pub fn game(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: ActivityType::Playing,
            url: None,
            application_id: None,
            details: None,
            state: None,
        }
    }

    /// Creates a streaming activity.
    #[must_use]
    pub fn streaming(name: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: ActivityType::Streaming,
            url: Some(url.into()),
            application_id: None,
            details: None,
            state: None,
        }
    }

    /// Creates a listening/Spotify activity.
    #[must_use]
    pub fn listening(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: ActivityType::Listening,
            url: None,
            application_id: None,
            details: None,
            state: None,
        }
    }

    /// Creates a custom status activity.
    #[must_use]
    pub fn custom(state: impl Into<String>) -> Self {
        Self {
            name: "Custom Status".into(),
            kind: ActivityType::Custom,
            url: None,
            application_id: None,
            details: None,
            state: Some(state.into()),
        }
    }

    /// Decodes an activity object.
    pub fn from_value(value: &Value, path: &str) -> Result<Self, CastError> {
        let obj = cast::object(value, path)?;
        let name = cast::required_string(obj, path, "name")?.to_owned();
        let kind_raw = cast::required_u16(obj, path, "type")? as u8;
        let kind = ActivityType::from_value(kind_raw).unwrap_or(ActivityType::Playing);
        let url = cast::optional_string(obj, path, "url")?.map(ToOwned::to_owned);
        let application_id = cast::optional_snowflake(obj, path, "application_id")?;
        let details = cast::optional_string(obj, path, "details")?.map(ToOwned::to_owned);
        let state = cast::optional_string(obj, path, "state")?.map(ToOwned::to_owned);

        Ok(Self {
            name,
            kind,
            url,
            application_id,
            details,
            state,
        })
    }
}

/// A member's full presence state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Presence {
    /// The user ID.
    pub user_id: Snowflake,
    /// The guild ID, if guild-bound.
    pub guild_id: Option<Snowflake>,
    /// Overall online status.
    pub status: Status,
    /// Active activities.
    pub activities: Vec<Activity>,
    /// Per-platform client status.
    pub client_status: ClientStatus,
}

impl Presence {
    /// Decodes a presence payload from the gateway.
    pub fn from_value(value: &Value, path: &str) -> Result<Self, CastError> {
        let obj = cast::object(value, path)?;
        let (user_val, user_path) = cast::required(obj, path, "user")?;
        let user_obj = cast::object(user_val, &user_path)?;
        let user_id = cast::required_snowflake(user_obj, &user_path, "id")?;
        let guild_id = cast::optional_snowflake(obj, path, "guild_id")?;

        let status_str = cast::optional_string(obj, path, "status")?.unwrap_or("offline");
        let status = Status::from_str(status_str).unwrap_or(Status::Offline);

        let mut activities = Vec::new();
        if let Some(act_vals) = cast::optional_array(obj, path, "activities")? {
            for (i, act_val) in act_vals.iter().enumerate() {
                let act_path = cast::index_path(&cast::field_path(path, "activities"), i);
                activities.push(Activity::from_value(act_val, &act_path)?);
            }
        }

        let client_status = if let Some(cs_val) = obj.get("client_status") {
            ClientStatus::from_value(cs_val, &cast::field_path(path, "client_status"))?
        } else {
            ClientStatus::default()
        };

        Ok(Self {
            user_id,
            guild_id,
            status,
            activities,
            client_status,
        })
    }
}
