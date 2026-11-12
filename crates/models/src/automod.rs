//! Auto Moderation models.

use abc::Snowflake;
use data::Value;

use crate::cast::{self, CastError};

/// Metadata associated with an AutoMod trigger.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AutoModTriggerMetadata {
    /// Substrings that will trigger the rule.
    pub keyword_filter: Vec<String>,
    /// Regular expressions that will trigger the rule.
    pub regex_patterns: Vec<String>,
    /// Discord preset keyword lists.
    pub presets: Vec<u8>,
    /// Substrings that are exempt from triggering.
    pub allow_list: Vec<String>,
    /// Total mentions limit (for mention spam).
    pub mention_total_limit: Option<u32>,
}

impl AutoModTriggerMetadata {
    pub fn from_value(value: &Value, path: &str) -> Result<Self, CastError> {
        let obj = cast::object(value, path)?;
        let keyword_filter = cast::optional_string_array(obj, path, "keyword_filter")?.unwrap_or_default();
        let regex_patterns = cast::optional_string_array(obj, path, "regex_patterns")?.unwrap_or_default();
        let allow_list = cast::optional_string_array(obj, path, "allow_list")?.unwrap_or_default();
        let mention_total_limit = cast::optional_u32(obj, path, "mention_total_limit")?;

        let mut presets = Vec::new();
        if let Some(arr) = cast::optional_array(obj, path, "presets")? {
            for v in arr {
                if let Value::Number(n) = v {
                    if let Some(i) = n.as_i64() {
                        presets.push(i as u8);
                    }
                }
            }
        }

        Ok(Self {
            keyword_filter,
            regex_patterns,
            presets,
            allow_list,
            mention_total_limit,
        })
    }
}

/// Metadata for an AutoMod action.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AutoModActionMetadata {
    /// Channel to send alert to.
    pub channel_id: Option<Snowflake>,
    /// Timeout duration in seconds.
    pub duration_seconds: Option<u32>,
    /// Custom message displayed to user on block.
    pub custom_message: Option<String>,
}

impl AutoModActionMetadata {
    pub fn from_value(value: &Value, path: &str) -> Result<Self, CastError> {
        let obj = cast::object(value, path)?;
        Ok(Self {
            channel_id: cast::optional_snowflake(obj, path, "channel_id")?,
            duration_seconds: cast::optional_u32(obj, path, "duration_seconds")?,
            custom_message: cast::optional_string(obj, path, "custom_message")?.map(ToOwned::to_owned),
        })
    }
}

/// An action executed when an AutoMod rule triggers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AutoModAction {
    /// Type of action (1: BlockMessage, 2: SendAlertMessage, 3: Timeout).
    pub action_type: u8,
    /// Action configuration.
    pub metadata: Option<AutoModActionMetadata>,
}

impl AutoModAction {
    pub fn from_value(value: &Value, path: &str) -> Result<Self, CastError> {
        let obj = cast::object(value, path)?;
        let action_type = cast::required_u16(obj, path, "type")? as u8;
        let metadata = if let Some(m_val) = obj.get("metadata") {
            Some(AutoModActionMetadata::from_value(m_val, &cast::field_path(path, "metadata"))?)
        } else {
            None
        };
        Ok(Self { action_type, metadata })
    }
}

/// A configured Auto Moderation rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AutoModRule {
    /// Rule ID.
    pub id: Snowflake,
    /// Guild ID.
    pub guild_id: Snowflake,
    /// Rule name.
    pub name: String,
    /// User who created the rule.
    pub creator_id: Snowflake,
    /// Event type (1: MessageSend).
    pub event_type: u8,
    /// Trigger type (1: Keyword, 2: HarmfulLink, 3: Spam, 4: KeywordPreset, 5: MentionSpam).
    pub trigger_type: u8,
    /// Trigger metadata.
    pub trigger_metadata: AutoModTriggerMetadata,
    /// Actions taken on trigger.
    pub actions: Vec<AutoModAction>,
    /// Whether the rule is currently active.
    pub enabled: bool,
    /// Role IDs exempt from rule.
    pub exempt_roles: Vec<Snowflake>,
    /// Channel IDs exempt from rule.
    pub exempt_channels: Vec<Snowflake>,
}

impl AutoModRule {
    pub fn from_value(value: &Value, path: &str) -> Result<Self, CastError> {
        let obj = cast::object(value, path)?;
        let id = cast::required_snowflake(obj, path, "id")?;
        let guild_id = cast::required_snowflake(obj, path, "guild_id")?;
        let name = cast::required_string(obj, path, "name")?.to_owned();
        let creator_id = cast::required_snowflake(obj, path, "creator_id")?;
        let event_type = cast::required_u16(obj, path, "event_type")? as u8;
        let trigger_type = cast::required_u16(obj, path, "trigger_type")? as u8;

        let trigger_metadata = if let Some(tm_val) = obj.get("trigger_metadata") {
            AutoModTriggerMetadata::from_value(tm_val, &cast::field_path(path, "trigger_metadata"))?
        } else {
            AutoModTriggerMetadata::default()
        };

        let mut actions = Vec::new();
        if let Some(act_vals) = cast::optional_array(obj, path, "actions")? {
            for (i, act_val) in act_vals.iter().enumerate() {
                let act_path = cast::index_path(&cast::field_path(path, "actions"), i);
                actions.push(AutoModAction::from_value(act_val, &act_path)?);
            }
        }

        let enabled = cast::required_bool(obj, path, "enabled")?;
        let exempt_roles = cast::optional_snowflake_array(obj, path, "exempt_roles")?.unwrap_or_default();
        let exempt_channels = cast::optional_snowflake_array(obj, path, "exempt_channels")?.unwrap_or_default();

        Ok(Self {
            id,
            guild_id,
            name,
            creator_id,
            event_type,
            trigger_type,
            trigger_metadata,
            actions,
            enabled,
            exempt_roles,
            exempt_channels,
        })
    }
}
