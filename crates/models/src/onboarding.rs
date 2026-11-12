//! Welcome screen and guild onboarding models.

use abc::Snowflake;
use data::Value;

use crate::cast::{self, CastError};

/// A channel highlighted on a guild welcome screen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WelcomeScreenChannel {
    pub channel_id: Snowflake,
    pub description: String,
    pub emoji_id: Option<Snowflake>,
    pub emoji_name: Option<String>,
}

impl WelcomeScreenChannel {
    pub fn from_value(value: &Value, path: &str) -> Result<Self, CastError> {
        let obj = cast::object(value, path)?;
        Ok(Self {
            channel_id: cast::required_snowflake(obj, path, "channel_id")?,
            description: cast::required_string(obj, path, "description")?.to_owned(),
            emoji_id: cast::optional_snowflake(obj, path, "emoji_id")?,
            emoji_name: cast::optional_string(obj, path, "emoji_name")?.map(ToOwned::to_owned),
        })
    }
}

/// A guild's welcome screen shown to new members.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WelcomeScreen {
    pub description: Option<String>,
    pub welcome_channels: Vec<WelcomeScreenChannel>,
}

impl WelcomeScreen {
    pub fn from_value(value: &Value, path: &str) -> Result<Self, CastError> {
        let obj = cast::object(value, path)?;
        let description = cast::optional_string(obj, path, "description")?.map(ToOwned::to_owned);
        let mut welcome_channels = Vec::new();
        if let Some(ch_vals) = cast::optional_array(obj, path, "welcome_channels")? {
            for (i, ch_val) in ch_vals.iter().enumerate() {
                let ch_path = cast::index_path(&cast::field_path(path, "welcome_channels"), i);
                welcome_channels.push(WelcomeScreenChannel::from_value(ch_val, &ch_path)?);
            }
        }
        Ok(Self {
            description,
            welcome_channels,
        })
    }
}

/// An option in an onboarding prompt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OnboardingPromptOption {
    pub id: Snowflake,
    pub channel_ids: Vec<Snowflake>,
    pub role_ids: Vec<Snowflake>,
    pub emoji_id: Option<Snowflake>,
    pub emoji_name: Option<String>,
    pub title: String,
    pub description: Option<String>,
}

impl OnboardingPromptOption {
    pub fn from_value(value: &Value, path: &str) -> Result<Self, CastError> {
        let obj = cast::object(value, path)?;
        Ok(Self {
            id: cast::required_snowflake(obj, path, "id")?,
            channel_ids: cast::optional_snowflake_array(obj, path, "channel_ids")?.unwrap_or_default(),
            role_ids: cast::optional_snowflake_array(obj, path, "role_ids")?.unwrap_or_default(),
            emoji_id: cast::optional_snowflake(obj, path, "emoji_id")?,
            emoji_name: cast::optional_string(obj, path, "emoji_name")?.map(ToOwned::to_owned),
            title: cast::required_string(obj, path, "title")?.to_owned(),
            description: cast::optional_string(obj, path, "description")?.map(ToOwned::to_owned),
        })
    }
}

/// A prompt in a guild's onboarding flow.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OnboardingPrompt {
    pub id: Snowflake,
    pub prompt_type: u8,
    pub options: Vec<OnboardingPromptOption>,
    pub title: String,
    pub single_select: bool,
    pub required: bool,
    pub in_onboarding: bool,
}

impl OnboardingPrompt {
    pub fn from_value(value: &Value, path: &str) -> Result<Self, CastError> {
        let obj = cast::object(value, path)?;
        let id = cast::required_snowflake(obj, path, "id")?;
        let prompt_type = cast::optional_u8(obj, path, "type")?.unwrap_or(0);
        let title = cast::required_string(obj, path, "title")?.to_owned();
        let single_select = cast::optional_bool(obj, path, "single_select", false)?;
        let required = cast::optional_bool(obj, path, "required", false)?;
        let in_onboarding = cast::optional_bool(obj, path, "in_onboarding", true)?;

        let mut options = Vec::new();
        if let Some(opt_vals) = cast::optional_array(obj, path, "options")? {
            for (i, opt_val) in opt_vals.iter().enumerate() {
                let opt_path = cast::index_path(&cast::field_path(path, "options"), i);
                options.push(OnboardingPromptOption::from_value(opt_val, &opt_path)?);
            }
        }

        Ok(Self {
            id,
            prompt_type,
            options,
            title,
            single_select,
            required,
            in_onboarding,
        })
    }
}

/// Guild onboarding configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Onboarding {
    pub guild_id: Snowflake,
    pub prompts: Vec<OnboardingPrompt>,
    pub default_channel_ids: Vec<Snowflake>,
    pub enabled: bool,
    pub mode: u8,
}

impl Onboarding {
    pub fn from_value(value: &Value, path: &str) -> Result<Self, CastError> {
        let obj = cast::object(value, path)?;
        let guild_id = cast::required_snowflake(obj, path, "guild_id")?;
        let default_channel_ids = cast::optional_snowflake_array(obj, path, "default_channel_ids")?.unwrap_or_default();
        let enabled = cast::required_bool(obj, path, "enabled")?;
        let mode = cast::optional_u8(obj, path, "mode")?.unwrap_or(0);

        let mut prompts = Vec::new();
        if let Some(pr_vals) = cast::optional_array(obj, path, "prompts")? {
            for (i, pr_val) in pr_vals.iter().enumerate() {
                let pr_path = cast::index_path(&cast::field_path(path, "prompts"), i);
                prompts.push(OnboardingPrompt::from_value(pr_val, &pr_path)?);
            }
        }

        Ok(Self {
            guild_id,
            prompts,
            default_channel_ids,
            enabled,
            mode,
        })
    }
}
