//! Guild template model.

use abc::Snowflake;
use data::Value;

use crate::cast::{self, CastError};
use crate::user::User;

/// A guild template snapshot for creating new guilds.
#[derive(Clone, Debug, PartialEq)]
pub struct GuildTemplate {
    /// Template unique code.
    pub code: String,
    /// Template name.
    pub name: String,
    /// Description.
    pub description: Option<String>,
    /// Number of times used.
    pub usage_count: u32,
    /// Creator ID.
    pub creator_id: Snowflake,
    /// Creator user object.
    pub creator: User,
    /// Source guild ID.
    pub source_guild_id: Snowflake,
    /// Serialized guild snapshot.
    pub serialized_source_guild: Value,
}

impl GuildTemplate {
    pub fn from_value(value: &Value, path: &str) -> Result<Self, CastError> {
        let obj = cast::object(value, path)?;
        let code = cast::required_string(obj, path, "code")?.to_owned();
        let name = cast::required_string(obj, path, "name")?.to_owned();
        let description = cast::optional_string(obj, path, "description")?.map(ToOwned::to_owned);
        let usage_count = cast::optional_u32(obj, path, "usage_count")?.unwrap_or(0);
        let creator_id = cast::required_snowflake(obj, path, "creator_id")?;

        let (creator_val, _) = cast::required(obj, path, "creator")?;
        let creator = User::from_value(creator_val)?;

        let source_guild_id = cast::required_snowflake(obj, path, "source_guild_id")?;
        let serialized_source_guild = obj
            .get("serialized_source_guild")
            .cloned()
            .unwrap_or(Value::Null);

        Ok(Self {
            code,
            name,
            description,
            usage_count,
            creator_id,
            creator,
            source_guild_id,
            serialized_source_guild,
        })
    }
}
