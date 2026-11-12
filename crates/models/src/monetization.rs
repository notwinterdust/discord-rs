//! Entitlement and SKU monetization models.

use abc::Snowflake;
use data::Value;

use crate::cast::{self, CastError};

/// An SKU representing a monetizable product or subscription in an application.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sku {
    pub id: Snowflake,
    pub sku_type: u8,
    pub application_id: Snowflake,
    pub name: String,
    pub slug: String,
    pub flags: u64,
}

impl Sku {
    pub fn from_value(value: &Value, path: &str) -> Result<Self, CastError> {
        let obj = cast::object(value, path)?;
        Ok(Self {
            id: cast::required_snowflake(obj, path, "id")?,
            sku_type: cast::required_u16(obj, path, "type")? as u8,
            application_id: cast::required_snowflake(obj, path, "application_id")?,
            name: cast::required_string(obj, path, "name")?.to_owned(),
            slug: cast::required_string(obj, path, "slug")?.to_owned(),
            flags: cast::optional_u64(obj, path, "flags")?.unwrap_or(0),
        })
    }
}

/// An entitlement granting a user or guild access to an SKU.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entitlement {
    pub id: Snowflake,
    pub sku_id: Snowflake,
    pub application_id: Snowflake,
    pub user_id: Option<Snowflake>,
    pub guild_id: Option<Snowflake>,
    pub entitlement_type: u8,
    pub deleted: bool,
    pub consumed: bool,
}

impl Entitlement {
    pub fn from_value(value: &Value, path: &str) -> Result<Self, CastError> {
        let obj = cast::object(value, path)?;
        Ok(Self {
            id: cast::required_snowflake(obj, path, "id")?,
            sku_id: cast::required_snowflake(obj, path, "sku_id")?,
            application_id: cast::required_snowflake(obj, path, "application_id")?,
            user_id: cast::optional_snowflake(obj, path, "user_id")?,
            guild_id: cast::optional_snowflake(obj, path, "guild_id")?,
            entitlement_type: cast::optional_u8(obj, path, "type")?.unwrap_or(1),
            deleted: cast::optional_bool(obj, path, "deleted", false)?,
            consumed: cast::optional_bool(obj, path, "consumed", false)?,
        })
    }
}
