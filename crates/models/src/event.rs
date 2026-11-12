//! Typed gateway events.
//!
//! A [`Dispatch`] is the event name, sequence number and data carried by a
//! dispatch payload. [`Event`] converts that data into the model for the named
//! event. Events this library does not model yet are preserved as
//! [`Event::Unknown`] with their name and data intact.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use abc::Snowflake;
use data::Value;
use data::gateway::{GatewayOpcode, GatewayPayload};
use enums::{ActivityType, ChannelType, ReactionType, Status};

use crate::cast::{self, CastError};
use crate::{
    ClientUser, Guild, GuildChannel, GuildScheduledEvent, Interaction, Invite, Member, Message,
    PartialApplication, PartialEmoji, Role, ScheduledEventUser, User,
};

/// The event name, sequence number and data from a dispatch payload.
#[derive(Clone, Debug, PartialEq)]
pub struct Dispatch {
    /// The sequence number, when the gateway supplied one.
    pub sequence: Option<u64>,
    /// The event name, such as `MESSAGE_CREATE`.
    pub event: String,
    /// The event data.
    pub data: Value,
}

/// A dispatch event preserved without typed decoding.
#[derive(Clone, Debug, PartialEq)]
pub struct RawEvent {
    /// The event name.
    pub name: String,
    /// The optional sequence number.
    pub sequence: Option<u64>,
    /// The original event data.
    pub data: Value,
}

/// Common gateway dispatch names.
pub mod names {
    /// Ready event.
    pub const READY: &str = "READY";
    /// Message creation event.
    pub const MESSAGE_CREATE: &str = "MESSAGE_CREATE";
    /// Message update event.
    pub const MESSAGE_UPDATE: &str = "MESSAGE_UPDATE";
    /// Message deletion event.
    pub const MESSAGE_DELETE: &str = "MESSAGE_DELETE";
    /// Guild creation event.
    pub const GUILD_CREATE: &str = "GUILD_CREATE";
}

impl RawEvent {
    /// Creates a raw event.
    #[must_use]
    pub fn new(name: impl Into<String>, sequence: Option<u64>, data: Value) -> Self {
        Self {
            name: name.into(),
            sequence,
            data,
        }
    }
}

impl Dispatch {
    /// Extracts a dispatch from a gateway payload.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when the payload is not a dispatch or has no
    /// event name.
    pub fn from_payload(payload: &GatewayPayload) -> Result<Self, CastError> {
        if payload.op != GatewayOpcode::Dispatch {
            return Err(CastError::invalid_value("payload.op", "a dispatch opcode"));
        }
        let event = payload
            .event
            .clone()
            .ok_or_else(|| CastError::MissingField {
                path: "payload.t".to_owned(),
            })?;
        Ok(Self {
            sequence: payload.sequence,
            event,
            data: payload.data.clone(),
        })
    }

    /// Decodes the dispatch's named event.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when the event data does not match the event's
    /// model. Unknown event names are not an error.
    pub fn event(&self) -> Result<Event, CastError> {
        Event::from_dispatch(self)
    }
}

/// A guild that the client has not fully received yet.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UnavailableGuild {
    /// The guild's ID.
    pub id: Snowflake,
    /// Whether the guild is unavailable.
    pub unavailable: bool,
}

impl UnavailableGuild {
    /// Creates an unavailable guild reference.
    #[must_use]
    pub const fn new(id: Snowflake) -> Self {
        Self {
            id,
            unavailable: true,
        }
    }

    /// Decodes a partial guild from a gateway payload.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when the ID is missing or malformed.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "unavailable_guild")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Ok(Self {
            id: cast::required_snowflake(object, path, "id")?,
            unavailable: cast::optional_bool(object, path, "unavailable", true)?,
        })
    }
}

/// The payload of a `READY` event.
#[derive(Clone, Debug, PartialEq)]
pub struct ReadyEvent {
    /// The gateway protocol version.
    pub version: u16,
    /// The user the client is logged in as.
    pub user: ClientUser,
    /// The guilds the user is in, as partial references.
    pub guilds: Vec<UnavailableGuild>,
    /// The session ID used to resume the connection.
    pub session_id: String,
    /// The URL to resume the session against, when supplied.
    pub resume_gateway_url: Option<String>,
    /// The application associated with the client, when supplied.
    pub application: Option<PartialApplication>,
}

impl ReadyEvent {
    /// Decodes a `READY` payload.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or malformed.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "ready")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;

        let user_path = cast::field_path(path, "user");
        let user =
            ClientUser::from_object_at(cast::required_object(object, path, "user")?, &user_path)?;

        let application_path = cast::field_path(path, "application");
        let application = cast::optional_object(object, path, "application")?
            .map(|application| PartialApplication::from_object_at(application, &application_path))
            .transpose()?;

        Ok(Self {
            version: cast::optional_u16(object, path, "version")?.unwrap_or(10),
            user,
            guilds: cast::optional_array_of(
                object,
                path,
                "guilds",
                UnavailableGuild::from_value_at,
            )?
            .unwrap_or_default(),
            session_id: cast::required_string(object, path, "session_id")?.to_owned(),
            resume_gateway_url: cast::optional_string(object, path, "resume_gateway_url")?
                .map(str::to_owned),
            application,
        })
    }
}

/// The payload of a `GUILD_MEMBER_REMOVE` event.
#[derive(Clone, Debug, PartialEq)]
pub struct GuildMemberRemove {
    /// The guild the member left.
    pub guild_id: Snowflake,
    /// The user that left.
    pub user: User,
}

impl GuildMemberRemove {
    /// Decodes a `GUILD_MEMBER_REMOVE` payload.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or malformed.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "member_remove")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let user_path = cast::field_path(path, "user");
        Ok(Self {
            guild_id: cast::required_snowflake(object, path, "guild_id")?,
            user: User::from_object_at(cast::required_object(object, path, "user")?, &user_path)?,
        })
    }
}

/// The payload of a role create or update event.
#[derive(Clone, Debug, PartialEq)]
pub struct RoleEvent {
    /// The guild the role belongs to.
    pub guild_id: Snowflake,
    /// The role that was created or updated.
    pub role: Role,
}

impl RoleEvent {
    /// Decodes a role create or update payload.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or malformed.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "role_event")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let guild_id = cast::required_snowflake(object, path, "guild_id")?;
        let (role_value, role_path) = cast::required(object, path, "role")?;
        let role = Role::from_value_at(role_value, &role_path, guild_id)?;
        Ok(Self { guild_id, role })
    }
}

/// The payload of a `GUILD_ROLE_DELETE` event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RoleDelete {
    /// The guild the role belonged to.
    pub guild_id: Snowflake,
    /// The deleted role's ID.
    pub role_id: Snowflake,
}

impl RoleDelete {
    /// Decodes a `GUILD_ROLE_DELETE` payload.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or malformed.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "role_delete")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Ok(Self {
            guild_id: cast::required_snowflake(object, path, "guild_id")?,
            role_id: cast::required_snowflake(object, path, "role_id")?,
        })
    }
}

/// The payload of a `THREAD_DELETE` event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ThreadDelete {
    /// The deleted thread's ID.
    pub id: Snowflake,
    /// The guild that owned the thread.
    pub guild_id: Snowflake,
    /// The channel the thread was created in.
    pub parent_id: Snowflake,
    /// The thread's channel type.
    pub kind: ChannelType,
}

impl ThreadDelete {
    /// Decodes a `THREAD_DELETE` payload.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or malformed.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "thread_delete")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Ok(Self {
            id: cast::required_snowflake(object, path, "id")?,
            guild_id: cast::required_snowflake(object, path, "guild_id")?,
            parent_id: cast::required_snowflake(object, path, "parent_id")?,
            kind: cast::required_enum(
                object,
                path,
                "type",
                "a channel type",
                ChannelType::from_value,
            )?,
        })
    }
}

/// The payload of a `MESSAGE_DELETE` event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MessageDelete {
    /// The deleted message's ID.
    pub id: Snowflake,
    /// The channel the message was in.
    pub channel_id: Snowflake,
    /// The guild the message was in, when it was a guild message.
    pub guild_id: Option<Snowflake>,
}

impl MessageDelete {
    /// Decodes a `MESSAGE_DELETE` payload.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or malformed.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "message_delete")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Ok(Self {
            id: cast::required_snowflake(object, path, "id")?,
            channel_id: cast::required_snowflake(object, path, "channel_id")?,
            guild_id: cast::optional_snowflake(object, path, "guild_id")?,
        })
    }
}

/// The payload of a `MESSAGE_DELETE_BULK` event.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MessageDeleteBulk {
    /// The IDs of the deleted messages.
    pub ids: Vec<Snowflake>,
    /// The channel the messages were in.
    pub channel_id: Snowflake,
    /// The guild the messages were in, when they were guild messages.
    pub guild_id: Option<Snowflake>,
}

impl MessageDeleteBulk {
    /// Decodes a `MESSAGE_DELETE_BULK` payload.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or malformed.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "message_delete_bulk")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let (ids_value, ids_path) = cast::required(object, path, "ids")?;
        let ids = cast::array(ids_value, &ids_path)?
            .iter()
            .enumerate()
            .map(|(index, value)| cast::snowflake(value, &cast::index_path(&ids_path, index)))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            ids,
            channel_id: cast::required_snowflake(object, path, "channel_id")?,
            guild_id: cast::optional_snowflake(object, path, "guild_id")?,
        })
    }
}

/// The payload of a reaction add or remove event.
#[derive(Clone, Debug, PartialEq)]
pub struct ReactionEvent {
    /// The user that added or removed the reaction.
    pub user_id: Snowflake,
    /// The channel containing the message.
    pub channel_id: Snowflake,
    /// The message that was reacted to.
    pub message_id: Snowflake,
    /// The guild containing the message, for guild messages.
    pub guild_id: Option<Snowflake>,
    /// The reacting member's guild data, when supplied.
    pub member: Option<Member>,
    /// The emoji that was used.
    pub emoji: PartialEmoji,
    /// Whether the reaction was a super reaction.
    pub kind: ReactionType,
}

impl ReactionEvent {
    /// Decodes a reaction add or remove payload.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or malformed.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "reaction_event")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let guild_id = cast::optional_snowflake(object, path, "guild_id")?;

        let member_path = cast::field_path(path, "member");
        let member = cast::optional_object(object, path, "member")?
            .map(|member| {
                Member::from_object_at(member, &member_path, guild_id.unwrap_or_default(), &[])
            })
            .transpose()?;

        let emoji_path = cast::field_path(path, "emoji");
        let emoji = PartialEmoji::from_object_at(
            cast::required_object(object, path, "emoji")?,
            &emoji_path,
        )?;

        Ok(Self {
            user_id: cast::required_snowflake(object, path, "user_id")?,
            channel_id: cast::required_snowflake(object, path, "channel_id")?,
            message_id: cast::required_snowflake(object, path, "message_id")?,
            guild_id,
            member,
            emoji,
            kind: cast::optional_enum(
                object,
                path,
                "type",
                "a reaction type",
                ReactionType::from_value,
            )?
            .unwrap_or_default(),
        })
    }
}

/// The payload of a `MESSAGE_REACTION_REMOVE_ALL` event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ReactionRemoveAll {
    /// The channel containing the message.
    pub channel_id: Snowflake,
    /// The message whose reactions were removed.
    pub message_id: Snowflake,
    /// The guild containing the message, for guild messages.
    pub guild_id: Option<Snowflake>,
}

impl ReactionRemoveAll {
    /// Decodes a `MESSAGE_REACTION_REMOVE_ALL` payload.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or malformed.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "reaction_remove_all")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Ok(Self {
            channel_id: cast::required_snowflake(object, path, "channel_id")?,
            message_id: cast::required_snowflake(object, path, "message_id")?,
            guild_id: cast::optional_snowflake(object, path, "guild_id")?,
        })
    }
}

/// The payload of a `MESSAGE_REACTION_REMOVE_EMOJI` event.
#[derive(Clone, Debug, PartialEq)]
pub struct ReactionRemoveEmoji {
    /// The channel containing the message.
    pub channel_id: Snowflake,
    /// The message whose emoji reactions were removed.
    pub message_id: Snowflake,
    /// The guild containing the message, for guild messages.
    pub guild_id: Option<Snowflake>,
    /// The emoji that was removed.
    pub emoji: PartialEmoji,
}

impl ReactionRemoveEmoji {
    /// Decodes a `MESSAGE_REACTION_REMOVE_EMOJI` payload.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or malformed.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "reaction_remove_emoji")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let emoji_path = cast::field_path(path, "emoji");
        let emoji = PartialEmoji::from_object_at(
            cast::required_object(object, path, "emoji")?,
            &emoji_path,
        )?;
        Ok(Self {
            channel_id: cast::required_snowflake(object, path, "channel_id")?,
            message_id: cast::required_snowflake(object, path, "message_id")?,
            guild_id: cast::optional_snowflake(object, path, "guild_id")?,
            emoji,
        })
    }
}

/// The payload of a `WEBHOOKS_UPDATE` event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WebhooksUpdate {
    /// The guild whose webhooks changed.
    pub guild_id: Snowflake,
    /// The channel whose webhooks changed.
    pub channel_id: Snowflake,
}

impl WebhooksUpdate {
    /// Decodes a `WEBHOOKS_UPDATE` payload.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or malformed.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "webhooks_update")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Ok(Self {
            guild_id: cast::required_snowflake(object, path, "guild_id")?,
            channel_id: cast::required_snowflake(object, path, "channel_id")?,
        })
    }
}

/// The payload of a `TYPING_START` event.
#[derive(Clone, Debug, PartialEq)]
pub struct TypingStart {
    /// The channel the user is typing in.
    pub channel_id: Snowflake,
    /// The guild containing the channel, for guild channels.
    pub guild_id: Option<Snowflake>,
    /// The user that started typing.
    pub user_id: Snowflake,
    /// When typing started, as reported by Discord.
    pub timestamp: SystemTime,
    /// The typing member's guild data, when supplied.
    pub member: Option<Member>,
}

impl TypingStart {
    /// Decodes a `TYPING_START` payload.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or malformed.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "typing_start")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let guild_id = cast::optional_snowflake(object, path, "guild_id")?;

        let member_path = cast::field_path(path, "member");
        let member = cast::optional_object(object, path, "member")?
            .map(|member| {
                Member::from_object_at(member, &member_path, guild_id.unwrap_or_default(), &[])
            })
            .transpose()?;

        Ok(Self {
            channel_id: cast::required_snowflake(object, path, "channel_id")?,
            guild_id,
            user_id: cast::required_snowflake(object, path, "user_id")?,
            timestamp: UNIX_EPOCH
                + Duration::from_secs(cast::required_u64(object, path, "timestamp")?),
            member,
        })
    }
}

/// The payload of a `GUILD_BAN_ADD` or `GUILD_BAN_REMOVE` event.
#[derive(Clone, Debug, PartialEq)]
pub struct GuildBanEvent {
    /// The guild the ban was created or removed in.
    pub guild_id: Snowflake,
    /// The banned or unbanned user.
    pub user: User,
}

impl GuildBanEvent {
    /// Decodes a ban event payload.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or malformed.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "guild_ban")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let user_path = cast::field_path(path, "user");
        Ok(Self {
            guild_id: cast::required_snowflake(object, path, "guild_id")?,
            user: User::from_object_at(cast::required_object(object, path, "user")?, &user_path)?,
        })
    }
}

/// A single activity engaged in by a user.
#[derive(Clone, Debug, PartialEq)]
pub struct Activity {
    /// Activity name.
    pub name: String,
    /// Activity type.
    pub kind: ActivityType,
    /// Stream URL if streaming.
    pub url: Option<String>,
}

impl Activity {
    /// Decodes an activity from a value.
    pub fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let kind_raw = cast::optional_u8(object, path, "type")?.unwrap_or(0);
        let kind = ActivityType::from_value(kind_raw).unwrap_or(ActivityType::Playing);
        Ok(Self {
            name: cast::required_string(object, path, "name")?.to_owned(),
            kind,
            url: cast::optional_string(object, path, "url")?.map(str::to_owned),
        })
    }
}

/// The payload of a `PRESENCE_UPDATE` event.
#[derive(Clone, Debug, PartialEq)]
pub struct PresenceUpdate {
    /// The user whose presence was updated.
    pub user_id: Snowflake,
    /// The guild the presence update occurred in.
    pub guild_id: Option<Snowflake>,
    /// The user's status.
    pub status: Status,
    /// The user's active activities.
    pub activities: Vec<Activity>,
}

impl PresenceUpdate {
    /// Decodes a `PRESENCE_UPDATE` payload.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "presence_update")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let user_obj = cast::required_object(object, path, "user")?;
        let user_id = cast::required_snowflake(user_obj, &cast::field_path(path, "user"), "id")?;
        let guild_id = cast::optional_snowflake(object, path, "guild_id")?;
        let status_str = cast::optional_string(object, path, "status")?.unwrap_or("offline");
        let status = Status::from_str(status_str).unwrap_or(Status::Offline);
        let activities = cast::optional_array_of(object, path, "activities", Activity::from_value_at)?
            .unwrap_or_default();

        Ok(Self {
            user_id,
            guild_id,
            status,
            activities,
        })
    }
}

/// The payload of a `GUILD_EMOJIS_UPDATE` event.
#[derive(Clone, Debug, PartialEq)]
pub struct GuildEmojisUpdate {
    /// The guild whose emojis were updated.
    pub guild_id: Snowflake,
    /// The updated list of emojis.
    pub emojis: Vec<PartialEmoji>,
}

impl GuildEmojisUpdate {
    /// Decodes a `GUILD_EMOJIS_UPDATE` payload.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "guild_emojis_update")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let guild_id = cast::required_snowflake(object, path, "guild_id")?;
        let emojis = cast::optional_array_of(object, path, "emojis", PartialEmoji::from_value_at)?
            .unwrap_or_default();
        Ok(Self { guild_id, emojis })
    }
}

/// The payload of a `GUILD_STICKERS_UPDATE` event.
#[derive(Clone, Debug, PartialEq)]
pub struct GuildStickersUpdate {
    /// The guild whose stickers were updated.
    pub guild_id: Snowflake,
    /// The updated list of stickers.
    pub stickers: Vec<crate::Sticker>,
}

impl GuildStickersUpdate {
    /// Decodes a `GUILD_STICKERS_UPDATE` payload.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "guild_stickers_update")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let guild_id = cast::required_snowflake(object, path, "guild_id")?;
        let stickers = cast::optional_array_of(object, path, "stickers", |v, p| {
            crate::Sticker::from_value_at(v, p)
        })?
        .unwrap_or_default();
        Ok(Self { guild_id, stickers })
    }
}

/// The payload of soundboard sound events.
#[derive(Clone, Debug, PartialEq)]
pub struct SoundboardSoundEvent {
    /// The sound's ID.
    pub sound_id: Snowflake,
    /// The guild the sound belongs to.
    pub guild_id: Snowflake,
    /// The name of the sound.
    pub name: Option<String>,
}

impl SoundboardSoundEvent {
    /// Decodes a soundboard sound payload.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "soundboard_sound")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let sound_id = cast::required_snowflake(object, path, "sound_id")?;
        let guild_id = cast::required_snowflake(object, path, "guild_id")?;
        let name = cast::optional_string(object, path, "name")?.map(str::to_owned);
        Ok(Self {
            sound_id,
            guild_id,
            name,
        })
    }
}

/// An Auto Moderation rule payload.
#[derive(Clone, Debug, PartialEq)]
pub struct AutoModRule {
    /// The rule ID.
    pub id: Snowflake,
    /// The guild the rule belongs to.
    pub guild_id: Snowflake,
    /// The rule name.
    pub name: String,
    /// The creator ID.
    pub creator_id: Snowflake,
    /// The event type (e.g., 1 for message send).
    pub event_type: u8,
    /// The trigger type.
    pub trigger_type: u8,
    /// Whether the rule is enabled.
    pub enabled: bool,
}

impl AutoModRule {
    /// Decodes an AutoMod rule from a gateway payload.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "auto_moderation_rule")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Ok(Self {
            id: cast::required_snowflake(object, path, "id")?,
            guild_id: cast::required_snowflake(object, path, "guild_id")?,
            name: cast::required_string(object, path, "name")?.to_owned(),
            creator_id: cast::required_snowflake(object, path, "creator_id")?,
            event_type: cast::required_u16(object, path, "event_type")? as u8,
            trigger_type: cast::required_u16(object, path, "trigger_type")? as u8,
            enabled: cast::optional_bool(object, path, "enabled", true)?,
        })
    }
}

/// The payload of an `AUTO_MODERATION_ACTION_EXECUTION` event.
#[derive(Clone, Debug, PartialEq)]
pub struct AutoModActionExecution {
    /// The guild ID.
    pub guild_id: Snowflake,
    /// The rule ID.
    pub rule_id: Snowflake,
    /// The rule trigger type.
    pub rule_trigger_type: u8,
    /// The user ID whose action triggered the rule.
    pub user_id: Snowflake,
    /// The channel ID, if any.
    pub channel_id: Option<Snowflake>,
    /// The message ID, if any.
    pub message_id: Option<Snowflake>,
    /// The alert system message ID, if any.
    pub alert_system_message_id: Option<Snowflake>,
    /// The user content that triggered the rule.
    pub content: Option<String>,
    /// The matched keyword.
    pub matched_keyword: Option<String>,
    /// The matched content.
    pub matched_content: Option<String>,
}

impl AutoModActionExecution {
    /// Decodes an `AUTO_MODERATION_ACTION_EXECUTION` payload.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "auto_moderation_action_execution")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Ok(Self {
            guild_id: cast::required_snowflake(object, path, "guild_id")?,
            rule_id: cast::required_snowflake(object, path, "rule_id")?,
            rule_trigger_type: cast::required_u16(object, path, "rule_trigger_type")? as u8,
            user_id: cast::required_snowflake(object, path, "user_id")?,
            channel_id: cast::optional_snowflake(object, path, "channel_id")?,
            message_id: cast::optional_snowflake(object, path, "message_id")?,
            alert_system_message_id: cast::optional_snowflake(object, path, "alert_system_message_id")?,
            content: cast::optional_string(object, path, "content")?.map(str::to_owned),
            matched_keyword: cast::optional_string(object, path, "matched_keyword")?.map(str::to_owned),
            matched_content: cast::optional_string(object, path, "matched_content")?.map(str::to_owned),
        })
    }
}

/// The payload of `INTEGRATION_CREATE` or `INTEGRATION_UPDATE` event.
#[derive(Clone, Debug, PartialEq)]
pub struct IntegrationEvent {
    /// The integration ID.
    pub id: Snowflake,
    /// The guild ID.
    pub guild_id: Snowflake,
    /// The integration name.
    pub name: String,
    /// The integration type (e.g. "twitch", "youtube", "discord").
    pub kind: String,
    /// Whether the integration is enabled.
    pub enabled: bool,
}

impl IntegrationEvent {
    /// Decodes an integration payload.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "integration")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Ok(Self {
            id: cast::required_snowflake(object, path, "id")?,
            guild_id: cast::required_snowflake(object, path, "guild_id")?,
            name: cast::required_string(object, path, "name")?.to_owned(),
            kind: cast::required_string(object, path, "type")?.to_owned(),
            enabled: cast::optional_bool(object, path, "enabled", true)?,
        })
    }
}

/// The payload of an `INTEGRATION_DELETE` event.
#[derive(Clone, Debug, PartialEq)]
pub struct IntegrationDelete {
    /// The integration ID.
    pub id: Snowflake,
    /// The guild ID.
    pub guild_id: Snowflake,
    /// The application ID, if any.
    pub application_id: Option<Snowflake>,
}

impl IntegrationDelete {
    /// Decodes an `INTEGRATION_DELETE` payload.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "integration_delete")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Ok(Self {
            id: cast::required_snowflake(object, path, "id")?,
            guild_id: cast::required_snowflake(object, path, "guild_id")?,
            application_id: cast::optional_snowflake(object, path, "application_id")?,
        })
    }
}

/// The payload of a `GUILD_INTEGRATIONS_UPDATE` event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GuildIntegrationsUpdate {
    /// The guild whose integrations were updated.
    pub guild_id: Snowflake,
}

impl GuildIntegrationsUpdate {
    /// Decodes a `GUILD_INTEGRATIONS_UPDATE` payload.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "guild_integrations_update")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Ok(Self {
            guild_id: cast::required_snowflake(object, path, "guild_id")?,
        })
    }
}

/// The payload of a `VOICE_STATE_UPDATE` event.
#[derive(Clone, Debug, PartialEq)]
pub struct VoiceStateUpdate {
    /// The guild ID, if any.
    pub guild_id: Option<Snowflake>,
    /// The channel ID the user joined, or None if disconnected.
    pub channel_id: Option<Snowflake>,
    /// The user ID.
    pub user_id: Snowflake,
    /// The member data, if in a guild.
    pub member: Option<Member>,
    /// The session ID.
    pub session_id: String,
    /// Whether this user is deafened by the server.
    pub deaf: bool,
    /// Whether this user is muted by the server.
    pub mute: bool,
    /// Whether this user is locally deafened.
    pub self_deaf: bool,
    /// Whether this user is locally muted.
    pub self_mute: bool,
    /// Whether this user is streaming using "Go Live".
    pub self_stream: bool,
    /// Whether this user's camera is enabled.
    pub self_video: bool,
    /// Whether this user is muted by the current client.
    pub suppress: bool,
}

impl VoiceStateUpdate {
    /// Decodes a `VOICE_STATE_UPDATE` payload.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "voice_state_update")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let guild_id = cast::optional_snowflake(object, path, "guild_id")?;
        let member_path = cast::field_path(path, "member");
        let member = cast::optional_object(object, path, "member")?
            .map(|member| {
                Member::from_object_at(member, &member_path, guild_id.unwrap_or_default(), &[])
            })
            .transpose()?;

        Ok(Self {
            guild_id,
            channel_id: cast::optional_snowflake(object, path, "channel_id")?,
            user_id: cast::required_snowflake(object, path, "user_id")?,
            member,
            session_id: cast::required_string(object, path, "session_id")?.to_owned(),
            deaf: cast::optional_bool(object, path, "deaf", false)?,
            mute: cast::optional_bool(object, path, "mute", false)?,
            self_deaf: cast::optional_bool(object, path, "self_deaf", false)?,
            self_mute: cast::optional_bool(object, path, "self_mute", false)?,
            self_stream: cast::optional_bool(object, path, "self_stream", false)?,
            self_video: cast::optional_bool(object, path, "self_video", false)?,
            suppress: cast::optional_bool(object, path, "suppress", false)?,
        })
    }
}

/// The payload of a `VOICE_SERVER_UPDATE` event.
#[derive(Clone, Debug, PartialEq)]
pub struct VoiceServerUpdate {
    /// Voice connection token.
    pub token: String,
    /// Guild ID the voice server is for.
    pub guild_id: Snowflake,
    /// Voice server host endpoint.
    pub endpoint: Option<String>,
}

impl VoiceServerUpdate {
    /// Decodes a `VOICE_SERVER_UPDATE` payload.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "voice_server_update")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Ok(Self {
            token: cast::required_string(object, path, "token")?.to_owned(),
            guild_id: cast::required_snowflake(object, path, "guild_id")?,
            endpoint: cast::optional_string(object, path, "endpoint")?.map(str::to_owned),
        })
    }
}

/// The payload of a `STAGE_INSTANCE_CREATE`, `UPDATE` or `DELETE` event.
#[derive(Clone, Debug, PartialEq)]
pub struct StageInstanceEvent {
    /// The stage instance ID.
    pub id: Snowflake,
    /// The guild ID.
    pub guild_id: Snowflake,
    /// The channel ID.
    pub channel_id: Snowflake,
    /// The topic of the stage instance.
    pub topic: String,
    /// Privacy level.
    pub privacy_level: u8,
}

impl StageInstanceEvent {
    /// Decodes a stage instance payload.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "stage_instance")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Ok(Self {
            id: cast::required_snowflake(object, path, "id")?,
            guild_id: cast::required_snowflake(object, path, "guild_id")?,
            channel_id: cast::required_snowflake(object, path, "channel_id")?,
            topic: cast::required_string(object, path, "topic")?.to_owned(),
            privacy_level: cast::required_u16(object, path, "privacy_level")? as u8,
        })
    }
}

/// The payload of a `THREAD_LIST_SYNC` event.
#[derive(Clone, Debug, PartialEq)]
pub struct ThreadListSync {
    /// The guild ID.
    pub guild_id: Snowflake,
    /// Parent channel IDs whose threads are being synced, if any.
    pub channel_ids: Option<Vec<Snowflake>>,
    /// Active threads in the given channels.
    pub threads: Vec<GuildChannel>,
    /// Thread members for the client in the synced threads.
    pub members: Vec<crate::ThreadMember>,
}

impl ThreadListSync {
    /// Decodes a `THREAD_LIST_SYNC` payload.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "thread_list_sync")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let guild_id = cast::required_snowflake(object, path, "guild_id")?;

        let channel_ids = cast::optional_array_of(object, path, "channel_ids", |v, p| {
            cast::snowflake(v, p)
        })?;

        let threads = cast::optional_array_of(object, path, "threads", |v, _| channel(v))?
            .unwrap_or_default();

        let members = cast::optional_array_of(object, path, "members", |v, p| {
            let tid = cast::optional_snowflake(cast::object(v, p)?, p, "id")?.unwrap_or_default();
            crate::ThreadMember::from_value_at(v, p, tid)
        })?
        .unwrap_or_default();

        Ok(Self {
            guild_id,
            channel_ids,
            threads,
            members,
        })
    }
}

/// The payload of a `THREAD_MEMBERS_UPDATE` event.
#[derive(Clone, Debug, PartialEq)]
pub struct ThreadMembersUpdate {
    /// The thread ID.
    pub id: Snowflake,
    /// The guild ID.
    pub guild_id: Snowflake,
    /// Approximate member count.
    pub member_count: u32,
    /// Users added to the thread.
    pub added_members: Vec<crate::ThreadMember>,
    /// IDs of users removed from the thread.
    pub removed_member_ids: Vec<Snowflake>,
}

impl ThreadMembersUpdate {
    /// Decodes a `THREAD_MEMBERS_UPDATE` payload.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "thread_members_update")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let id = cast::required_snowflake(object, path, "id")?;
        let guild_id = cast::required_snowflake(object, path, "guild_id")?;
        let member_count = cast::optional_u32(object, path, "member_count")?.unwrap_or(0);

        let added_members = cast::optional_array_of(object, path, "added_members", |v, p| {
            crate::ThreadMember::from_value_at(v, p, id)
        })?
        .unwrap_or_default();

        let removed_member_ids = cast::optional_array_of(object, path, "removed_member_ids", |v, p| {
            cast::snowflake(v, p)
        })?
        .unwrap_or_default();

        Ok(Self {
            id,
            guild_id,
            member_count,
            added_members,
            removed_member_ids,
        })
    }
}

/// The payload of a `CHANNEL_PINS_UPDATE` event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChannelPinsUpdate {
    /// The guild ID, if any.
    pub guild_id: Option<Snowflake>,
    /// The channel ID whose pins were updated.
    pub channel_id: Snowflake,
}

impl ChannelPinsUpdate {
    /// Decodes a `CHANNEL_PINS_UPDATE` payload.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "channel_pins_update")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Ok(Self {
            guild_id: cast::optional_snowflake(object, path, "guild_id")?,
            channel_id: cast::required_snowflake(object, path, "channel_id")?,
        })
    }
}

/// The payload of an `ENTITLEMENT_CREATE`, `UPDATE`, or `DELETE` event.
#[derive(Clone, Debug, PartialEq)]
pub struct EntitlementEvent {
    /// The entitlement ID.
    pub id: Snowflake,
    /// The SKU ID.
    pub sku_id: Snowflake,
    /// The application ID.
    pub application_id: Snowflake,
    /// The user ID that owns the entitlement.
    pub user_id: Option<Snowflake>,
    /// The guild ID that owns the entitlement.
    pub guild_id: Option<Snowflake>,
    /// Entitlement type (e.g. 8 for application subscription).
    pub kind: u8,
    /// Whether the entitlement was deleted.
    pub deleted: bool,
}

impl EntitlementEvent {
    /// Decodes an entitlement event payload.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "entitlement")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Ok(Self {
            id: cast::required_snowflake(object, path, "id")?,
            sku_id: cast::required_snowflake(object, path, "sku_id")?,
            application_id: cast::required_snowflake(object, path, "application_id")?,
            user_id: cast::optional_snowflake(object, path, "user_id")?,
            guild_id: cast::optional_snowflake(object, path, "guild_id")?,
            kind: cast::required_u16(object, path, "type")? as u8,
            deleted: cast::optional_bool(object, path, "deleted", false)?,
        })
    }
}

/// The payload of a `SUBSCRIPTION_CREATE`, `UPDATE`, or `DELETE` event.
#[derive(Clone, Debug, PartialEq)]
pub struct SubscriptionEvent {
    /// The subscription ID.
    pub id: Snowflake,
    /// The user ID of the subscriber.
    pub user_id: Snowflake,
    /// List of SKU IDs subscribed to.
    pub sku_ids: Vec<Snowflake>,
    /// Status of the subscription (e.g. 0 for active, 1 for ending, 2 for inactive).
    pub status: u8,
}

impl SubscriptionEvent {
    /// Decodes a subscription event payload.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "subscription")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let sku_ids = cast::optional_array_of(object, path, "sku_ids", |v, p| {
            cast::snowflake(v, p)
        })?
        .unwrap_or_default();

        Ok(Self {
            id: cast::required_snowflake(object, path, "id")?,
            user_id: cast::required_snowflake(object, path, "user_id")?,
            sku_ids,
            status: cast::required_u16(object, path, "status")? as u8,
        })
    }
}

/// A typed gateway event.
///
/// Variants that Discord can send but this library does not model yet are not
/// an error: they arrive as [`Event::Unknown`] with their name and data.
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    /// The client completed the connection handshake.
    Ready(ReadyEvent),
    /// The client resumed an existing session.
    Resumed,
    /// A guild became available.
    GuildCreate(Guild),
    /// A guild was updated.
    GuildUpdate(Guild),
    /// A guild became unavailable or the client was removed from it.
    GuildDelete(UnavailableGuild),
    /// A channel was created.
    ChannelCreate(GuildChannel),
    /// A channel was updated.
    ChannelUpdate(GuildChannel),
    /// A channel was deleted.
    ChannelDelete(GuildChannel),
    /// A thread was created.
    ThreadCreate(GuildChannel),
    /// A thread was updated.
    ThreadUpdate(GuildChannel),
    /// A thread was deleted.
    ThreadDelete(ThreadDelete),
    /// A member joined a guild.
    GuildMemberAdd(Member),
    /// A member left a guild.
    GuildMemberRemove(GuildMemberRemove),
    /// A member was updated.
    GuildMemberUpdate(Member),
    /// A role was created.
    GuildRoleCreate(RoleEvent),
    /// A role was updated.
    GuildRoleUpdate(RoleEvent),
    /// A role was deleted.
    GuildRoleDelete(RoleDelete),
    /// A message was created.
    MessageCreate(Message),
    /// A message was updated.
    MessageUpdate(Message),
    /// A message was deleted.
    MessageDelete(MessageDelete),
    /// Several messages were deleted at once.
    MessageDeleteBulk(MessageDeleteBulk),
    /// A reaction was added to a message.
    MessageReactionAdd(ReactionEvent),
    /// A reaction was removed from a message.
    MessageReactionRemove(ReactionEvent),
    /// All reactions were removed from a message.
    MessageReactionRemoveAll(ReactionRemoveAll),
    /// All reactions with one emoji were removed from a message.
    MessageReactionRemoveEmoji(ReactionRemoveEmoji),
    /// A user was banned from a guild.
    GuildBanAdd(GuildBanEvent),
    /// A user was unbanned from a guild.
    GuildBanRemove(GuildBanEvent),
    /// Guild emojis were updated.
    GuildEmojisUpdate(GuildEmojisUpdate),
    /// Guild stickers were updated.
    GuildStickersUpdate(GuildStickersUpdate),
    /// A soundboard sound was created or updated.
    GuildSoundboardSoundCreate(SoundboardSoundEvent),
    /// A soundboard sound was deleted.
    GuildSoundboardSoundDelete(SoundboardSoundEvent),
    /// An AutoMod rule was created.
    AutoModerationRuleCreate(AutoModRule),
    /// An AutoMod rule was updated.
    AutoModerationRuleUpdate(AutoModRule),
    /// An AutoMod rule was deleted.
    AutoModerationRuleDelete(AutoModRule),
    /// An AutoMod action was executed.
    AutoModerationActionExecution(AutoModActionExecution),
    /// Guild integrations were updated.
    GuildIntegrationsUpdate(GuildIntegrationsUpdate),
    /// An integration was created.
    IntegrationCreate(IntegrationEvent),
    /// An integration was updated.
    IntegrationUpdate(IntegrationEvent),
    /// An integration was deleted.
    IntegrationDelete(IntegrationDelete),
    /// A user's voice state changed.
    VoiceStateUpdate(VoiceStateUpdate),
    /// A guild voice server was updated.
    VoiceServerUpdate(VoiceServerUpdate),
    /// A stage instance was created.
    StageInstanceCreate(StageInstanceEvent),
    /// A stage instance was updated.
    StageInstanceUpdate(StageInstanceEvent),
    /// A stage instance was deleted.
    StageInstanceDelete(StageInstanceEvent),
    /// Active threads in a channel were synced.
    ThreadListSync(ThreadListSync),
    /// A thread member was updated.
    ThreadMemberUpdate(crate::ThreadMember),
    /// Members were added to or removed from a thread.
    ThreadMembersUpdate(ThreadMembersUpdate),
    /// Pinned messages in a channel were updated.
    ChannelPinsUpdate(ChannelPinsUpdate),
    /// An entitlement was created.
    EntitlementCreate(EntitlementEvent),
    /// An entitlement was updated.
    EntitlementUpdate(EntitlementEvent),
    /// An entitlement was deleted.
    EntitlementDelete(EntitlementEvent),
    /// A premium subscription was created.
    SubscriptionCreate(SubscriptionEvent),
    /// A premium subscription was updated.
    SubscriptionUpdate(SubscriptionEvent),
    /// A premium subscription was deleted.
    SubscriptionDelete(SubscriptionEvent),
    /// An audit log entry was created.
    GuildAuditLogEntryCreate(crate::AuditLogEntry),
    /// An interaction was created.
    InteractionCreate(Interaction),
    /// An invite was created.
    InviteCreate(Invite),
    /// An invite was deleted.
    InviteDelete(Invite),
    /// A channel's webhooks changed.
    WebhooksUpdate(WebhooksUpdate),
    /// The client user was updated.
    UserUpdate(User),
    /// A user started typing.
    TypingStart(TypingStart),
    /// A user's presence or status was updated.
    PresenceUpdate(PresenceUpdate),
    /// A scheduled event was created.
    GuildScheduledEventCreate(GuildScheduledEvent),
    /// A scheduled event was updated.
    GuildScheduledEventUpdate(GuildScheduledEvent),
    /// A scheduled event was deleted.
    GuildScheduledEventDelete(GuildScheduledEvent),
    /// A user subscribed to a scheduled event.
    GuildScheduledEventUserAdd(ScheduledEventUser),
    /// A user unsubscribed from a scheduled event.
    GuildScheduledEventUserRemove(ScheduledEventUser),
    /// An event this library does not model yet.
    Unknown {
        /// The event name.
        name: String,
        /// The event data, preserved verbatim.
        data: Value,
    },
}

impl Event {
    /// Decodes the event carried by a gateway payload.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when the payload is not a dispatch, has no event
    /// name, or its data does not match the named event's model.
    pub fn from_payload(payload: &GatewayPayload) -> Result<Self, CastError> {
        Self::from_dispatch(&Dispatch::from_payload(payload)?)
    }

    /// Decodes the event carried by a dispatch.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when the dispatch data does not match the named
    /// event's model.
    pub fn from_dispatch(dispatch: &Dispatch) -> Result<Self, CastError> {
        Self::from_value(&dispatch.event, &dispatch.data)
    }

    /// Decodes an event from its name and data.
    ///
    /// Unknown names produce [`Event::Unknown`] rather than an error.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when the data does not match a known event's
    /// model.
    pub fn from_value(name: &str, data: &Value) -> Result<Self, CastError> {
        match name {
            "READY" => Ok(Self::Ready(ReadyEvent::from_value(data)?)),
            "RESUMED" => Ok(Self::Resumed),
            "GUILD_CREATE" => Ok(Self::GuildCreate(Guild::from_value(data)?)),
            "GUILD_UPDATE" => Ok(Self::GuildUpdate(Guild::from_value(data)?)),
            "GUILD_DELETE" => Ok(Self::GuildDelete(UnavailableGuild::from_value(data)?)),
            "CHANNEL_CREATE" => Ok(Self::ChannelCreate(channel(data)?)),
            "CHANNEL_UPDATE" => Ok(Self::ChannelUpdate(channel(data)?)),
            "CHANNEL_DELETE" => Ok(Self::ChannelDelete(channel(data)?)),
            "THREAD_CREATE" => Ok(Self::ThreadCreate(channel(data)?)),
            "THREAD_UPDATE" => Ok(Self::ThreadUpdate(channel(data)?)),
            "THREAD_DELETE" => Ok(Self::ThreadDelete(ThreadDelete::from_value(data)?)),
            "GUILD_MEMBER_ADD" => Ok(Self::GuildMemberAdd(member(data)?)),
            "GUILD_MEMBER_REMOVE" => Ok(Self::GuildMemberRemove(GuildMemberRemove::from_value(
                data,
            )?)),
            "GUILD_MEMBER_UPDATE" => Ok(Self::GuildMemberUpdate(member(data)?)),
            "GUILD_ROLE_CREATE" => Ok(Self::GuildRoleCreate(RoleEvent::from_value(data)?)),
            "GUILD_ROLE_UPDATE" => Ok(Self::GuildRoleUpdate(RoleEvent::from_value(data)?)),
            "GUILD_ROLE_DELETE" => Ok(Self::GuildRoleDelete(RoleDelete::from_value(data)?)),
            "MESSAGE_CREATE" => Ok(Self::MessageCreate(Message::from_value(data)?)),
            "MESSAGE_UPDATE" => Ok(Self::MessageUpdate(Message::from_value(data)?)),
            "MESSAGE_DELETE" => Ok(Self::MessageDelete(MessageDelete::from_value(data)?)),
            "MESSAGE_DELETE_BULK" => Ok(Self::MessageDeleteBulk(MessageDeleteBulk::from_value(
                data,
            )?)),
            "MESSAGE_REACTION_ADD" => {
                Ok(Self::MessageReactionAdd(ReactionEvent::from_value(data)?))
            }
            "MESSAGE_REACTION_REMOVE" => Ok(Self::MessageReactionRemove(
                ReactionEvent::from_value(data)?,
            )),
            "MESSAGE_REACTION_REMOVE_ALL" => Ok(Self::MessageReactionRemoveAll(
                ReactionRemoveAll::from_value(data)?,
            )),
            "MESSAGE_REACTION_REMOVE_EMOJI" => Ok(Self::MessageReactionRemoveEmoji(
                ReactionRemoveEmoji::from_value(data)?,
            )),
            "GUILD_BAN_ADD" => Ok(Self::GuildBanAdd(GuildBanEvent::from_value(data)?)),
            "GUILD_BAN_REMOVE" => Ok(Self::GuildBanRemove(GuildBanEvent::from_value(data)?)),
            "GUILD_EMOJIS_UPDATE" => {
                Ok(Self::GuildEmojisUpdate(GuildEmojisUpdate::from_value(data)?))
            }
            "GUILD_STICKERS_UPDATE" => Ok(Self::GuildStickersUpdate(
                GuildStickersUpdate::from_value(data)?,
            )),
            "GUILD_SOUNDBOARD_SOUND_CREATE" | "GUILD_SOUNDBOARD_SOUND_UPDATE" => {
                Ok(Self::GuildSoundboardSoundCreate(
                    SoundboardSoundEvent::from_value(data)?,
                ))
            }
            "GUILD_SOUNDBOARD_SOUND_DELETE" => Ok(Self::GuildSoundboardSoundDelete(
                SoundboardSoundEvent::from_value(data)?,
            )),
            "AUTO_MODERATION_RULE_CREATE" => Ok(Self::AutoModerationRuleCreate(
                AutoModRule::from_value(data)?,
            )),
            "AUTO_MODERATION_RULE_UPDATE" => Ok(Self::AutoModerationRuleUpdate(
                AutoModRule::from_value(data)?,
            )),
            "AUTO_MODERATION_RULE_DELETE" => Ok(Self::AutoModerationRuleDelete(
                AutoModRule::from_value(data)?,
            )),
            "AUTO_MODERATION_ACTION_EXECUTION" => Ok(Self::AutoModerationActionExecution(
                AutoModActionExecution::from_value(data)?,
            )),
            "GUILD_INTEGRATIONS_UPDATE" => Ok(Self::GuildIntegrationsUpdate(
                GuildIntegrationsUpdate::from_value(data)?,
            )),
            "INTEGRATION_CREATE" => Ok(Self::IntegrationCreate(IntegrationEvent::from_value(data)?)),
            "INTEGRATION_UPDATE" => Ok(Self::IntegrationUpdate(IntegrationEvent::from_value(data)?)),
            "INTEGRATION_DELETE" => Ok(Self::IntegrationDelete(IntegrationDelete::from_value(data)?)),
            "VOICE_STATE_UPDATE" => Ok(Self::VoiceStateUpdate(VoiceStateUpdate::from_value(data)?)),
            "VOICE_SERVER_UPDATE" => {
                Ok(Self::VoiceServerUpdate(VoiceServerUpdate::from_value(data)?))
            }
            "STAGE_INSTANCE_CREATE" => Ok(Self::StageInstanceCreate(
                StageInstanceEvent::from_value(data)?,
            )),
            "STAGE_INSTANCE_UPDATE" => Ok(Self::StageInstanceUpdate(
                StageInstanceEvent::from_value(data)?,
            )),
            "STAGE_INSTANCE_DELETE" => Ok(Self::StageInstanceDelete(
                StageInstanceEvent::from_value(data)?,
            )),
            "THREAD_LIST_SYNC" => Ok(Self::ThreadListSync(ThreadListSync::from_value(data)?)),
            "THREAD_MEMBER_UPDATE" => {
                let tid = cast::optional_snowflake(cast::object(data, "thread_member_update")?, "thread_member_update", "id")?.unwrap_or_default();
                Ok(Self::ThreadMemberUpdate(crate::ThreadMember::from_value(data, tid)?))
            }
            "THREAD_MEMBERS_UPDATE" => {
                Ok(Self::ThreadMembersUpdate(ThreadMembersUpdate::from_value(data)?))
            }
            "CHANNEL_PINS_UPDATE" => Ok(Self::ChannelPinsUpdate(ChannelPinsUpdate::from_value(data)?)),
            "ENTITLEMENT_CREATE" => Ok(Self::EntitlementCreate(EntitlementEvent::from_value(data)?)),
            "ENTITLEMENT_UPDATE" => Ok(Self::EntitlementUpdate(EntitlementEvent::from_value(data)?)),
            "ENTITLEMENT_DELETE" => Ok(Self::EntitlementDelete(EntitlementEvent::from_value(data)?)),
            "SUBSCRIPTION_CREATE" => Ok(Self::SubscriptionCreate(SubscriptionEvent::from_value(data)?)),
            "SUBSCRIPTION_UPDATE" => Ok(Self::SubscriptionUpdate(SubscriptionEvent::from_value(data)?)),
            "SUBSCRIPTION_DELETE" => Ok(Self::SubscriptionDelete(SubscriptionEvent::from_value(data)?)),
            "GUILD_AUDIT_LOG_ENTRY_CREATE" => Ok(Self::GuildAuditLogEntryCreate(
                crate::AuditLogEntry::from_value(data, payload_guild_id(data)?)?,
            )),
            "INTERACTION_CREATE" => Ok(Self::InteractionCreate(Interaction::from_value(data)?)),
            "INVITE_CREATE" => Ok(Self::InviteCreate(Invite::from_value(data)?)),
            "INVITE_DELETE" => Ok(Self::InviteDelete(Invite::from_value(data)?)),
            "WEBHOOKS_UPDATE" => Ok(Self::WebhooksUpdate(WebhooksUpdate::from_value(data)?)),
            "USER_UPDATE" => Ok(Self::UserUpdate(User::from_value(data)?)),
            "TYPING_START" => Ok(Self::TypingStart(TypingStart::from_value(data)?)),
            "PRESENCE_UPDATE" => Ok(Self::PresenceUpdate(PresenceUpdate::from_value(data)?)),
            "GUILD_SCHEDULED_EVENT_CREATE" => Ok(Self::GuildScheduledEventCreate(
                GuildScheduledEvent::from_value(data, payload_guild_id(data)?)?,
            )),
            "GUILD_SCHEDULED_EVENT_UPDATE" => Ok(Self::GuildScheduledEventUpdate(
                GuildScheduledEvent::from_value(data, payload_guild_id(data)?)?,
            )),
            "GUILD_SCHEDULED_EVENT_DELETE" => Ok(Self::GuildScheduledEventDelete(
                GuildScheduledEvent::from_value(data, payload_guild_id(data)?)?,
            )),
            "GUILD_SCHEDULED_EVENT_USER_ADD" => Ok(Self::GuildScheduledEventUserAdd(
                ScheduledEventUser::from_value(data, payload_guild_id(data)?)?,
            )),
            "GUILD_SCHEDULED_EVENT_USER_REMOVE" => Ok(Self::GuildScheduledEventUserRemove(
                ScheduledEventUser::from_value(data, payload_guild_id(data)?)?,
            )),
            _ => Ok(Self::Unknown {
                name: name.to_owned(),
                data: data.clone(),
            }),
        }
    }

    /// Returns the event's gateway name.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Ready(_) => "READY",
            Self::Resumed => "RESUMED",
            Self::GuildCreate(_) => "GUILD_CREATE",
            Self::GuildUpdate(_) => "GUILD_UPDATE",
            Self::GuildDelete(_) => "GUILD_DELETE",
            Self::ChannelCreate(_) => "CHANNEL_CREATE",
            Self::ChannelUpdate(_) => "CHANNEL_UPDATE",
            Self::ChannelDelete(_) => "CHANNEL_DELETE",
            Self::ThreadCreate(_) => "THREAD_CREATE",
            Self::ThreadUpdate(_) => "THREAD_UPDATE",
            Self::ThreadDelete(_) => "THREAD_DELETE",
            Self::GuildMemberAdd(_) => "GUILD_MEMBER_ADD",
            Self::GuildMemberRemove(_) => "GUILD_MEMBER_REMOVE",
            Self::GuildMemberUpdate(_) => "GUILD_MEMBER_UPDATE",
            Self::GuildRoleCreate(_) => "GUILD_ROLE_CREATE",
            Self::GuildRoleUpdate(_) => "GUILD_ROLE_UPDATE",
            Self::GuildRoleDelete(_) => "GUILD_ROLE_DELETE",
            Self::MessageCreate(_) => "MESSAGE_CREATE",
            Self::MessageUpdate(_) => "MESSAGE_UPDATE",
            Self::MessageDelete(_) => "MESSAGE_DELETE",
            Self::MessageDeleteBulk(_) => "MESSAGE_DELETE_BULK",
            Self::MessageReactionAdd(_) => "MESSAGE_REACTION_ADD",
            Self::MessageReactionRemove(_) => "MESSAGE_REACTION_REMOVE",
            Self::MessageReactionRemoveAll(_) => "MESSAGE_REACTION_REMOVE_ALL",
            Self::MessageReactionRemoveEmoji(_) => "MESSAGE_REACTION_REMOVE_EMOJI",
            Self::GuildBanAdd(_) => "GUILD_BAN_ADD",
            Self::GuildBanRemove(_) => "GUILD_BAN_REMOVE",
            Self::GuildEmojisUpdate(_) => "GUILD_EMOJIS_UPDATE",
            Self::GuildStickersUpdate(_) => "GUILD_STICKERS_UPDATE",
            Self::GuildSoundboardSoundCreate(_) => "GUILD_SOUNDBOARD_SOUND_CREATE",
            Self::GuildSoundboardSoundDelete(_) => "GUILD_SOUNDBOARD_SOUND_DELETE",
            Self::AutoModerationRuleCreate(_) => "AUTO_MODERATION_RULE_CREATE",
            Self::AutoModerationRuleUpdate(_) => "AUTO_MODERATION_RULE_UPDATE",
            Self::AutoModerationRuleDelete(_) => "AUTO_MODERATION_RULE_DELETE",
            Self::AutoModerationActionExecution(_) => "AUTO_MODERATION_ACTION_EXECUTION",
            Self::GuildIntegrationsUpdate(_) => "GUILD_INTEGRATIONS_UPDATE",
            Self::IntegrationCreate(_) => "INTEGRATION_CREATE",
            Self::IntegrationUpdate(_) => "INTEGRATION_UPDATE",
            Self::IntegrationDelete(_) => "INTEGRATION_DELETE",
            Self::VoiceStateUpdate(_) => "VOICE_STATE_UPDATE",
            Self::VoiceServerUpdate(_) => "VOICE_SERVER_UPDATE",
            Self::StageInstanceCreate(_) => "STAGE_INSTANCE_CREATE",
            Self::StageInstanceUpdate(_) => "STAGE_INSTANCE_UPDATE",
            Self::StageInstanceDelete(_) => "STAGE_INSTANCE_DELETE",
            Self::ThreadListSync(_) => "THREAD_LIST_SYNC",
            Self::ThreadMemberUpdate(_) => "THREAD_MEMBER_UPDATE",
            Self::ThreadMembersUpdate(_) => "THREAD_MEMBERS_UPDATE",
            Self::ChannelPinsUpdate(_) => "CHANNEL_PINS_UPDATE",
            Self::EntitlementCreate(_) => "ENTITLEMENT_CREATE",
            Self::EntitlementUpdate(_) => "ENTITLEMENT_UPDATE",
            Self::EntitlementDelete(_) => "ENTITLEMENT_DELETE",
            Self::SubscriptionCreate(_) => "SUBSCRIPTION_CREATE",
            Self::SubscriptionUpdate(_) => "SUBSCRIPTION_UPDATE",
            Self::SubscriptionDelete(_) => "SUBSCRIPTION_DELETE",
            Self::GuildAuditLogEntryCreate(_) => "GUILD_AUDIT_LOG_ENTRY_CREATE",
            Self::InteractionCreate(_) => "INTERACTION_CREATE",
            Self::InviteCreate(_) => "INVITE_CREATE",
            Self::InviteDelete(_) => "INVITE_DELETE",
            Self::WebhooksUpdate(_) => "WEBHOOKS_UPDATE",
            Self::UserUpdate(_) => "USER_UPDATE",
            Self::TypingStart(_) => "TYPING_START",
            Self::PresenceUpdate(_) => "PRESENCE_UPDATE",
            Self::GuildScheduledEventCreate(_) => "GUILD_SCHEDULED_EVENT_CREATE",
            Self::GuildScheduledEventUpdate(_) => "GUILD_SCHEDULED_EVENT_UPDATE",
            Self::GuildScheduledEventDelete(_) => "GUILD_SCHEDULED_EVENT_DELETE",
            Self::GuildScheduledEventUserAdd(_) => "GUILD_SCHEDULED_EVENT_USER_ADD",
            Self::GuildScheduledEventUserRemove(_) => "GUILD_SCHEDULED_EVENT_USER_REMOVE",
            Self::Unknown { name, .. } => name,
        }
    }

    /// Returns `true` if the event name is not modelled yet.
    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown { .. })
    }
}

fn channel(value: &Value) -> Result<GuildChannel, CastError> {
    GuildChannel::from_value(value, Snowflake::ZERO)
}

fn member(value: &Value) -> Result<Member, CastError> {
    Member::from_value(value, payload_guild_id(value)?, &[])
}

fn payload_guild_id(value: &Value) -> Result<Snowflake, CastError> {
    let path = "payload.d";
    let object = cast::object(value, path)?;
    Ok(cast::optional_snowflake(object, path, "guild_id")?.unwrap_or(Snowflake::ZERO))
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, UNIX_EPOCH};

    use abc::Snowflake;
    use data::Value;
    use data::gateway::GatewayOpcode;
    use data::json;
    use enums::{ChannelType, ReactionType};

    use super::{Dispatch, Event, ReadyEvent, ThreadDelete};
    use crate::{PartialApplication, PartialEmoji};

    fn value(raw: &str) -> Value {
        json::from_str(raw).unwrap()
    }

    fn event(name: &str, raw: &str) -> Event {
        Event::from_value(name, &value(raw)).unwrap()
    }

    #[test]
    fn extracts_dispatches_and_rejects_other_payloads() {
        let payload =
            data::gateway::GatewayPayload::from_json(r#"{"op":0,"s":3,"t":"RESUMED","d":{}}"#)
                .unwrap();
        let dispatch = Dispatch::from_payload(&payload).unwrap();
        assert_eq!(dispatch.sequence, Some(3));
        assert_eq!(dispatch.event, "RESUMED");
        assert_eq!(dispatch.event().unwrap(), Event::Resumed);
        assert_eq!(Event::from_payload(&payload).unwrap(), Event::Resumed);

        let heartbeat = data::gateway::GatewayPayload::new(GatewayOpcode::Heartbeat, Value::Null);
        assert_eq!(
            Dispatch::from_payload(&heartbeat).unwrap_err().to_string(),
            "field `payload.op` must be a dispatch opcode"
        );

        let unnamed = data::gateway::GatewayPayload::new(GatewayOpcode::Dispatch, Value::Null);
        assert_eq!(
            Dispatch::from_payload(&unnamed).unwrap_err().to_string(),
            "missing field `payload.t`"
        );
    }

    #[test]
    fn decodes_ready_payloads() {
        let payload = value(
            r#"{
                "version": 10,
                "user": {
                    "id": "1",
                    "username": "winter",
                    "verified": true,
                    "mfa_enabled": false,
                    "locale": "en-US"
                },
                "guilds": [
                    {"id": "2", "unavailable": true},
                    {"id": "3", "unavailable": false}
                ],
                "session_id": "session",
                "resume_gateway_url": "wss://resume.example",
                "application": {"id": "4", "name": "Bot", "description": "does things"}
            }"#,
        );

        let ready = ReadyEvent::from_value(&payload).unwrap();
        assert_eq!(ready.version, 10);
        assert_eq!(ready.user.id(), Snowflake::new(1));
        assert!(ready.user.verified);
        assert_eq!(ready.user.locale.as_deref(), Some("en-US"));
        assert_eq!(ready.guilds.len(), 2);
        assert!(ready.guilds[0].unavailable);
        assert!(!ready.guilds[1].unavailable);
        assert_eq!(ready.session_id, "session");
        assert_eq!(
            ready.resume_gateway_url.as_deref(),
            Some("wss://resume.example")
        );
        assert_eq!(
            ready.application,
            Some(PartialApplication::new(4_u64, "Bot").with_description("does things"))
        );

        assert!(matches!(
            Event::from_value("READY", &payload).unwrap(),
            Event::Ready(_)
        ));

        let missing_session =
            ReadyEvent::from_value(&value(r#"{"user":{"id":"1","username":"w"}}"#)).unwrap_err();
        assert_eq!(
            missing_session.to_string(),
            "missing field `ready.session_id`"
        );
    }

    #[test]
    fn decodes_guild_channel_and_thread_events() {
        let guild = event("GUILD_CREATE", r#"{"id":"2","name":"Guild"}"#);
        match guild {
            Event::GuildCreate(guild) => {
                assert_eq!(guild.id, Snowflake::new(2));
                assert_eq!(guild.name, "Guild");
            }
            other => panic!("expected a guild create event, got {other:?}"),
        }

        let updated = event("GUILD_UPDATE", r#"{"id":"2","name":"Renamed"}"#);
        assert!(matches!(updated, Event::GuildUpdate(guild) if guild.name == "Renamed"));

        let deleted = event("GUILD_DELETE", r#"{"id":"2","unavailable":true}"#);
        assert!(matches!(
            deleted,
            Event::GuildDelete(guild) if guild.id == Snowflake::new(2) && guild.unavailable
        ));

        let channel = event(
            "CHANNEL_CREATE",
            r#"{"id":"4","guild_id":"2","name":"general","type":0}"#,
        );
        match channel {
            Event::ChannelCreate(channel) => {
                assert_eq!(channel.id, Snowflake::new(4));
                assert_eq!(channel.guild_id, Snowflake::new(2));
                assert_eq!(channel.channel_type, ChannelType::Text);
            }
            other => panic!("expected a channel create event, got {other:?}"),
        }

        let updated = event(
            "CHANNEL_UPDATE",
            r#"{"id":"4","guild_id":"2","name":"renamed","type":0}"#,
        );
        assert!(matches!(updated, Event::ChannelUpdate(channel) if channel.name == "renamed"));

        let deleted = event(
            "CHANNEL_DELETE",
            r#"{"id":"4","guild_id":"2","name":"general","type":0}"#,
        );
        assert!(matches!(
            deleted,
            Event::ChannelDelete(channel) if channel.id == Snowflake::new(4)
        ));

        let created = event(
            "THREAD_CREATE",
            r#"{"id":"5","guild_id":"2","parent_id":"4","name":"thread","type":11}"#,
        );
        match created {
            Event::ThreadCreate(thread) => {
                assert_eq!(thread.channel_type, ChannelType::PublicThread);
                assert_eq!(
                    thread.thread.as_ref().map(|thread| thread.parent_id),
                    Some(Snowflake::new(4))
                );
            }
            other => panic!("expected a thread create event, got {other:?}"),
        }

        assert!(matches!(
            event(
                "THREAD_UPDATE",
                r#"{"id":"5","guild_id":"2","parent_id":"4","name":"renamed","type":11}"#
            ),
            Event::ThreadUpdate(thread) if thread.name == "renamed"
        ));

        let deleted = event(
            "THREAD_DELETE",
            r#"{"id":"5","guild_id":"2","parent_id":"4","type":11}"#,
        );
        assert_eq!(
            deleted,
            Event::ThreadDelete(ThreadDelete {
                id: Snowflake::new(5),
                guild_id: Snowflake::new(2),
                parent_id: Snowflake::new(4),
                kind: ChannelType::PublicThread,
            })
        );

        let missing_type = Event::from_value(
            "THREAD_DELETE",
            &value(r#"{"id":"5","guild_id":"2","parent_id":"4"}"#),
        )
        .unwrap_err();
        assert_eq!(
            missing_type.to_string(),
            "missing field `thread_delete.type`"
        );
    }

    #[test]
    fn decodes_member_and_role_events() {
        let added = event(
            "GUILD_MEMBER_ADD",
            r#"{
                "guild_id": "2",
                "user": {"id": "6", "username": "winter"},
                "roles": [],
                "joined_at": "2021-08-01T12:34:56Z"
            }"#,
        );
        match added {
            Event::GuildMemberAdd(member) => {
                assert_eq!(member.user.id, Snowflake::new(6));
                assert_eq!(member.guild_id, Snowflake::new(2));
            }
            other => panic!("expected a member add event, got {other:?}"),
        }

        let updated = event(
            "GUILD_MEMBER_UPDATE",
            r#"{"guild_id":"2","user":{"id":"6","username":"winter"},"roles":[]}"#,
        );
        assert!(matches!(
            updated,
            Event::GuildMemberUpdate(member) if member.user.name == "winter"
        ));

        let removed = event(
            "GUILD_MEMBER_REMOVE",
            r#"{"guild_id":"2","user":{"id":"6","username":"winter"}}"#,
        );
        match removed {
            Event::GuildMemberRemove(removed) => {
                assert_eq!(removed.guild_id, Snowflake::new(2));
                assert_eq!(removed.user.id, Snowflake::new(6));
            }
            other => panic!("expected a member remove event, got {other:?}"),
        }

        let created = event(
            "GUILD_ROLE_CREATE",
            r#"{"guild_id":"2","role":{"id":"7","name":"Role","color":255}}"#,
        );
        match created {
            Event::GuildRoleCreate(event) => {
                assert_eq!(event.guild_id, Snowflake::new(2));
                assert_eq!(event.role.id, Snowflake::new(7));
                assert_eq!(event.role.guild_id, Snowflake::new(2));
            }
            other => panic!("expected a role create event, got {other:?}"),
        }

        assert!(matches!(
            event(
                "GUILD_ROLE_UPDATE",
                r#"{"guild_id":"2","role":{"id":"7","name":"Renamed"}}"#
            ),
            Event::GuildRoleUpdate(event) if event.role.name == "Renamed"
        ));

        let deleted = event("GUILD_ROLE_DELETE", r#"{"guild_id":"2","role_id":"7"}"#);
        assert!(matches!(
            deleted,
            Event::GuildRoleDelete(event)
                if event.guild_id == Snowflake::new(2) && event.role_id == Snowflake::new(7)
        ));
    }

    #[test]
    fn decodes_message_and_reaction_events() {
        let raw_message = r#"{
            "id": "10",
            "channel_id": "4",
            "guild_id": "2",
            "author": {"id": "6", "username": "winter"},
            "content": "hello",
            "timestamp": "2021-08-01T12:34:56Z"
        }"#;

        match event("MESSAGE_CREATE", raw_message) {
            Event::MessageCreate(message) => {
                assert_eq!(message.id, Snowflake::new(10));
                assert_eq!(message.content, "hello");
                assert_eq!(message.guild_id, Some(Snowflake::new(2)));
            }
            other => panic!("expected a message create event, got {other:?}"),
        }

        assert!(matches!(
            event("MESSAGE_UPDATE", raw_message),
            Event::MessageUpdate(message) if message.channel_id == Snowflake::new(4)
        ));

        let deleted = event(
            "MESSAGE_DELETE",
            r#"{"id":"10","channel_id":"4","guild_id":"2"}"#,
        );
        assert!(matches!(
            deleted,
            Event::MessageDelete(event)
                if event.id == Snowflake::new(10) && event.guild_id == Some(Snowflake::new(2))
        ));

        let bulk = event(
            "MESSAGE_DELETE_BULK",
            r#"{"ids":["10","11"],"channel_id":"4","guild_id":"2"}"#,
        );
        match bulk {
            Event::MessageDeleteBulk(event) => {
                assert_eq!(event.ids, vec![Snowflake::new(10), Snowflake::new(11)]);
                assert_eq!(event.channel_id, Snowflake::new(4));
            }
            other => panic!("expected a bulk delete event, got {other:?}"),
        }

        let reaction = event(
            "MESSAGE_REACTION_ADD",
            r#"{
                "user_id": "6",
                "channel_id": "4",
                "message_id": "10",
                "guild_id": "2",
                "emoji": {"id": "8", "name": "wave"},
                "type": 1,
                "member": {
                    "user": {"id": "6", "username": "winter"},
                    "roles": [],
                    "joined_at": "2021-08-01T12:34:56Z"
                }
            }"#,
        );
        match reaction {
            Event::MessageReactionAdd(reaction) => {
                assert_eq!(reaction.user_id, Snowflake::new(6));
                assert_eq!(reaction.message_id, Snowflake::new(10));
                assert_eq!(reaction.kind, ReactionType::Burst);
                assert_eq!(reaction.emoji, PartialEmoji::custom(8_u64, "wave"));
                assert_eq!(
                    reaction.member.as_ref().map(|member| member.user.id),
                    Some(Snowflake::new(6))
                );
            }
            other => panic!("expected a reaction add event, got {other:?}"),
        }

        assert!(matches!(
            event(
                "MESSAGE_REACTION_REMOVE",
                r#"{
                    "user_id":"6",
                    "channel_id":"4",
                    "message_id":"10",
                    "emoji":{"name":"wave"}
                }"#
            ),
            Event::MessageReactionRemove(reaction)
                if reaction.kind == ReactionType::Normal && reaction.guild_id.is_none()
        ));

        let remove_all = event(
            "MESSAGE_REACTION_REMOVE_ALL",
            r#"{"channel_id":"4","message_id":"10","guild_id":"2"}"#,
        );
        assert!(matches!(
            remove_all,
            Event::MessageReactionRemoveAll(event)
                if event.message_id == Snowflake::new(10) && event.guild_id == Some(Snowflake::new(2))
        ));

        let remove_emoji = event(
            "MESSAGE_REACTION_REMOVE_EMOJI",
            r#"{
                "channel_id":"4",
                "message_id":"10",
                "guild_id":"2",
                "emoji":{"id":"8","name":"wave"}
            }"#,
        );
        assert!(matches!(
            remove_emoji,
            Event::MessageReactionRemoveEmoji(event)
                if event.emoji == PartialEmoji::custom(8_u64, "wave")
        ));

        let malformed = Event::from_value("MESSAGE_DELETE", &value(r#"{"id":"10"}"#)).unwrap_err();
        assert_eq!(
            malformed.to_string(),
            "missing field `message_delete.channel_id`"
        );
    }

    #[test]
    fn decodes_interaction_invite_webhook_and_user_events() {
        let interaction = event(
            "INTERACTION_CREATE",
            r#"{
                "id": "20",
                "application_id": "21",
                "type": 3,
                "token": "token",
                "data": {"custom_id": "button", "component_type": 2}
            }"#,
        );
        match interaction {
            Event::InteractionCreate(interaction) => {
                assert_eq!(interaction.id, Snowflake::new(20));
                assert!(
                    interaction
                        .data
                        .as_ref()
                        .and_then(|data| data.message_component())
                        .is_some()
                );
            }
            other => panic!("expected an interaction create event, got {other:?}"),
        }

        let invite = event(
            "INVITE_CREATE",
            r#"{
                "code": "abc",
                "type": 0,
                "guild": {"id": "2", "name": "Guild"},
                "channel": {"id": "4", "name": "general", "type": 0}
            }"#,
        );
        match invite {
            Event::InviteCreate(invite) => {
                assert_eq!(invite.code, "abc");
                assert_eq!(
                    invite.guild.as_ref().map(|guild| guild.id()),
                    Some(Snowflake::new(2))
                );
            }
            other => panic!("expected an invite create event, got {other:?}"),
        }

        let deleted = event("INVITE_DELETE", r#"{"code":"abc"}"#);
        assert!(matches!(deleted, Event::InviteDelete(invite) if invite.code == "abc"));

        let webhooks = event("WEBHOOKS_UPDATE", r#"{"guild_id":"2","channel_id":"4"}"#);
        assert!(matches!(
            webhooks,
            Event::WebhooksUpdate(event)
                if event.guild_id == Snowflake::new(2) && event.channel_id == Snowflake::new(4)
        ));

        let user = event("USER_UPDATE", r#"{"id":"6","username":"renamed"}"#);
        assert!(matches!(user, Event::UserUpdate(user) if user.name == "renamed"));
    }

    #[test]
    fn decodes_typing_and_scheduled_event_events() {
        let typing = event(
            "TYPING_START",
            r#"{
                "channel_id": "4",
                "guild_id": "2",
                "user_id": "6",
                "timestamp": 1627821296,
                "member": {
                    "user": {"id": "6", "username": "winter"},
                    "roles": []
                }
            }"#,
        );
        match typing {
            Event::TypingStart(typing) => {
                assert_eq!(typing.channel_id, Snowflake::new(4));
                assert_eq!(typing.user_id, Snowflake::new(6));
                assert_eq!(
                    typing.timestamp,
                    UNIX_EPOCH + Duration::from_secs(1_627_821_296)
                );
                assert_eq!(
                    typing.member.as_ref().map(|member| member.user.id),
                    Some(Snowflake::new(6))
                );
            }
            other => panic!("expected a typing start event, got {other:?}"),
        }

        let raw_event = r#"{
            "id": "30",
            "guild_id": "2",
            "name": "Event",
            "entity_type": 2,
            "scheduled_start_time": "2021-08-01T12:34:56Z",
            "privacy_level": 2,
            "status": 1
        }"#;

        match event("GUILD_SCHEDULED_EVENT_CREATE", raw_event) {
            Event::GuildScheduledEventCreate(event) => {
                assert_eq!(event.id, Snowflake::new(30));
                assert_eq!(event.guild_id, Snowflake::new(2));
                assert_eq!(event.name, "Event");
            }
            other => panic!("expected a scheduled event create event, got {other:?}"),
        }

        assert!(matches!(
            event(
                "GUILD_SCHEDULED_EVENT_UPDATE",
                r#"{
                    "id":"30",
                    "guild_id":"2",
                    "name":"Renamed",
                    "entity_type":2,
                    "scheduled_start_time":"2021-08-01T12:34:56Z",
                    "privacy_level":2,
                    "status":1
                }"#
            ),
            Event::GuildScheduledEventUpdate(event) if event.name == "Renamed"
        ));

        assert!(matches!(
            event("GUILD_SCHEDULED_EVENT_DELETE", raw_event),
            Event::GuildScheduledEventDelete(event) if event.id == Snowflake::new(30)
        ));

        let subscriber = event(
            "GUILD_SCHEDULED_EVENT_USER_ADD",
            r#"{
                "guild_id": "2",
                "guild_scheduled_event_id": "30",
                "user": {"id": "6", "username": "winter"}
            }"#,
        );
        match subscriber {
            Event::GuildScheduledEventUserAdd(subscriber) => {
                assert_eq!(subscriber.guild_scheduled_event_id, Snowflake::new(30));
                assert_eq!(subscriber.user.id, Snowflake::new(6));
            }
            other => panic!("expected a scheduled event subscriber event, got {other:?}"),
        }

        assert!(matches!(
            event(
                "GUILD_SCHEDULED_EVENT_USER_REMOVE",
                r#"{
                    "guild_id":"2",
                    "guild_scheduled_event_id":"30",
                    "user":{"id":"6","username":"winter"}
                }"#
            ),
            Event::GuildScheduledEventUserRemove(subscriber)
                if subscriber.user.id == Snowflake::new(6)
        ));
    }

    #[test]
    fn decodes_moderation_events() {
        let ban_raw = r#"{
            "guild_id": "100",
            "user": {
                "id": "200",
                "username": "banned_user"
            }
        }"#;

        match event("GUILD_BAN_ADD", ban_raw) {
            Event::GuildBanAdd(ban) => {
                assert_eq!(ban.guild_id, Snowflake::new(100));
                assert_eq!(ban.user.id, Snowflake::new(200));
                assert_eq!(ban.user.name, "banned_user");
            }
            other => panic!("expected GuildBanAdd, got {other:?}"),
        }

        match event("GUILD_BAN_REMOVE", ban_raw) {
            Event::GuildBanRemove(ban) => {
                assert_eq!(ban.guild_id, Snowflake::new(100));
                assert_eq!(ban.user.id, Snowflake::new(200));
            }
            other => panic!("expected GuildBanRemove, got {other:?}"),
        }

        let audit_log_raw = r#"{
            "id": "500",
            "guild_id": "100",
            "action_type": 1,
            "user_id": "200"
        }"#;

        match event("GUILD_AUDIT_LOG_ENTRY_CREATE", audit_log_raw) {
            Event::GuildAuditLogEntryCreate(entry) => {
                assert_eq!(entry.id, Snowflake::new(500));
                assert_eq!(entry.user_id, Some(Snowflake::new(200)));
            }
            other => panic!("expected GuildAuditLogEntryCreate, got {other:?}"),
        }
    }

    #[test]
    fn decodes_presence_update_event() {
        let presence_raw = r#"{
            "user": {
                "id": "300"
            },
            "guild_id": "100",
            "status": "dnd",
            "activities": [
                {
                    "name": "Rust compiler",
                    "type": 0
                }
            ]
        }"#;

        match event("PRESENCE_UPDATE", presence_raw) {
            Event::PresenceUpdate(presence) => {
                assert_eq!(presence.user_id, Snowflake::new(300));
                assert_eq!(presence.guild_id, Some(Snowflake::new(100)));
                assert_eq!(presence.status, enums::Status::DoNotDisturb);
                assert_eq!(presence.activities.len(), 1);
                assert_eq!(presence.activities[0].name, "Rust compiler");
                assert_eq!(presence.activities[0].kind, enums::ActivityType::Playing);
            }
            other => panic!("expected PresenceUpdate, got {other:?}"),
        }
    }

    #[test]
    fn decodes_guild_asset_events() {
        let emojis_raw = r#"{
            "guild_id": "100",
            "emojis": [
                {
                    "id": "10",
                    "name": "custom_emoji"
                }
            ]
        }"#;

        match event("GUILD_EMOJIS_UPDATE", emojis_raw) {
            Event::GuildEmojisUpdate(update) => {
                assert_eq!(update.guild_id, Snowflake::new(100));
                assert_eq!(update.emojis.len(), 1);
                assert_eq!(update.emojis[0].id, Some(Snowflake::new(10)));
                assert_eq!(update.emojis[0].name.as_deref(), Some("custom_emoji"));
            }
            other => panic!("expected GuildEmojisUpdate, got {other:?}"),
        }

        let stickers_raw = r#"{
            "guild_id": "100",
            "stickers": [
                {
                    "id": "20",
                    "name": "custom_sticker",
                    "format_type": 1,
                    "type": 2
                }
            ]
        }"#;

        match event("GUILD_STICKERS_UPDATE", stickers_raw) {
            Event::GuildStickersUpdate(update) => {
                assert_eq!(update.guild_id, Snowflake::new(100));
                assert_eq!(update.stickers.len(), 1);
                assert_eq!(update.stickers[0].id, Snowflake::new(20));
                assert_eq!(update.stickers[0].name, "custom_sticker");
            }
            other => panic!("expected GuildStickersUpdate, got {other:?}"),
        }

        let sound_raw = r#"{
            "sound_id": "50",
            "guild_id": "100",
            "name": "airhorn"
        }"#;

        match event("GUILD_SOUNDBOARD_SOUND_CREATE", sound_raw) {
            Event::GuildSoundboardSoundCreate(sound) => {
                assert_eq!(sound.sound_id, Snowflake::new(50));
                assert_eq!(sound.guild_id, Snowflake::new(100));
                assert_eq!(sound.name.as_deref(), Some("airhorn"));
            }
            other => panic!("expected GuildSoundboardSoundCreate, got {other:?}"),
        }

        match event("GUILD_SOUNDBOARD_SOUND_DELETE", sound_raw) {
            Event::GuildSoundboardSoundDelete(sound) => {
                assert_eq!(sound.sound_id, Snowflake::new(50));
                assert_eq!(sound.guild_id, Snowflake::new(100));
            }
            other => panic!("expected GuildSoundboardSoundDelete, got {other:?}"),
        }
    }

    #[test]
    fn decodes_automod_events() {
        let rule_raw = r#"{
            "id": "100",
            "guild_id": "200",
            "name": "Block spam",
            "creator_id": "300",
            "event_type": 1,
            "trigger_type": 1,
            "enabled": true
        }"#;

        match event("AUTO_MODERATION_RULE_CREATE", rule_raw) {
            Event::AutoModerationRuleCreate(rule) => {
                assert_eq!(rule.id, Snowflake::new(100));
                assert_eq!(rule.guild_id, Snowflake::new(200));
                assert_eq!(rule.name, "Block spam");
                assert_eq!(rule.event_type, 1);
                assert!(rule.enabled);
            }
            other => panic!("expected AutoModerationRuleCreate, got {other:?}"),
        }

        match event("AUTO_MODERATION_RULE_DELETE", rule_raw) {
            Event::AutoModerationRuleDelete(rule) => {
                assert_eq!(rule.id, Snowflake::new(100));
            }
            other => panic!("expected AutoModerationRuleDelete, got {other:?}"),
        }

        let exec_raw = r#"{
            "guild_id": "200",
            "rule_id": "100",
            "rule_trigger_type": 1,
            "user_id": "300",
            "content": "bad message"
        }"#;

        match event("AUTO_MODERATION_ACTION_EXECUTION", exec_raw) {
            Event::AutoModerationActionExecution(exec) => {
                assert_eq!(exec.guild_id, Snowflake::new(200));
                assert_eq!(exec.rule_id, Snowflake::new(100));
                assert_eq!(exec.content.as_deref(), Some("bad message"));
            }
            other => panic!("expected AutoModerationActionExecution, got {other:?}"),
        }
    }

    #[test]
    fn preserves_unknown_events_and_name_and_data() {
        let raw = value(r#"{"id":"1","future":true}"#);
        match Event::from_value("FUTURE_EVENT", &raw).unwrap() {
            Event::Unknown { name, data } => {
                assert_eq!(name, "FUTURE_EVENT");
                assert_eq!(data, raw);
            }
            other => panic!("expected an unknown event, got {other:?}"),
        }

        let unknown = Event::from_value("FUTURE_EVENT", &raw).unwrap();
        assert_eq!(unknown.name(), "FUTURE_EVENT");
        assert!(unknown.is_unknown());
        assert_eq!(Event::Resumed.name(), "RESUMED");
        assert!(!Event::Resumed.is_unknown());
    }
}
