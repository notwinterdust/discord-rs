//! Guild scheduled event models.
//!
//! Scheduled events describe events in a guild, held in a stage instance, a
//! voice channel or an external location.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

use abc::{Asset, HasId, Snowflake};
use data::{Object, Value};
use enums::{GuildScheduledEventEntityType, GuildScheduledEventStatus, PrivacyLevel};

use crate::cast::{self, CastError};
use crate::{Guild, GuildChannel, Member, User};

/// A scheduled event in a guild.
///
/// Equality and hashing are based on the event's ID.
#[derive(Clone, Debug)]
pub struct GuildScheduledEvent {
    /// The event's unique ID.
    pub id: Snowflake,
    /// The ID of the guild the event belongs to.
    pub guild_id: Snowflake,
    /// The ID of the channel the event is held in, if any.
    pub channel_id: Option<Snowflake>,
    /// The event's name.
    pub name: String,
    /// The event's description, if it has one.
    pub description: Option<String>,
    /// The entity the event is held in.
    pub entity_type: GuildScheduledEventEntityType,
    /// The ID of the entity the event is held in, if any.
    pub entity_id: Option<Snowflake>,
    /// When the event starts.
    pub start_time: SystemTime,
    /// When the event ends, if known.
    pub end_time: Option<SystemTime>,
    /// The event's privacy level.
    pub privacy_level: PrivacyLevel,
    /// The event's status.
    pub status: GuildScheduledEventStatus,
    /// The event's cover image, if it has one.
    pub cover_image: Option<Asset>,
    /// The number of users subscribed to the event.
    pub user_count: u64,
    /// The user that created the event, if known.
    pub creator: Option<User>,
    /// The ID of the user that created the event, if known.
    pub creator_id: Option<Snowflake>,
    /// The location of an external event, if any.
    pub location: Option<String>,
}

impl Default for GuildScheduledEvent {
    fn default() -> Self {
        Self {
            id: Snowflake::ZERO,
            guild_id: Snowflake::ZERO,
            channel_id: None,
            name: String::new(),
            description: None,
            entity_type: GuildScheduledEventEntityType::default(),
            entity_id: None,
            start_time: SystemTime::UNIX_EPOCH,
            end_time: None,
            privacy_level: PrivacyLevel::default(),
            status: GuildScheduledEventStatus::default(),
            cover_image: None,
            user_count: 0,
            creator: None,
            creator_id: None,
            location: None,
        }
    }
}

impl GuildScheduledEvent {
    /// Creates a guild scheduled event with its core fields.
    #[must_use]
    pub fn new(
        id: impl Into<Snowflake>,
        guild_id: impl Into<Snowflake>,
        name: impl Into<String>,
        start_time: SystemTime,
    ) -> Self {
        Self {
            id: id.into(),
            guild_id: guild_id.into(),
            name: name.into(),
            start_time,
            ..Self::default()
        }
    }

    /// Decodes a scheduled event from a Discord scheduled event object.
    ///
    /// The payload's `guild_id` is used when present, otherwise the supplied
    /// `guild_id` is used.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value, guild_id: impl Into<Snowflake>) -> Result<Self, CastError> {
        Self::from_value_at(value, "event", guild_id.into())
    }

    pub(crate) fn from_value_at(
        value: &Value,
        path: &str,
        guild_id: Snowflake,
    ) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Self::from_object_at(object, path, guild_id)
    }

    pub(crate) fn from_object_at(
        object: &Object,
        path: &str,
        guild_id: Snowflake,
    ) -> Result<Self, CastError> {
        let id = cast::required_snowflake(object, path, "id")?;
        let guild_id = cast::optional_snowflake(object, path, "guild_id")?.unwrap_or(guild_id);

        let metadata_path = cast::field_path(path, "entity_metadata");
        let location = match cast::optional_object(object, path, "entity_metadata")? {
            Some(metadata) => {
                cast::optional_string(metadata, &metadata_path, "location")?.map(str::to_owned)
            }
            None => None,
        };

        let creator_path = cast::field_path(path, "creator");
        let creator = cast::optional_object(object, path, "creator")?
            .map(|creator| User::from_object_at(creator, &creator_path))
            .transpose()?;

        Ok(Self {
            id,
            guild_id,
            channel_id: cast::optional_snowflake(object, path, "channel_id")?,
            name: cast::required_string(object, path, "name")?.to_owned(),
            description: cast::optional_string(object, path, "description")?.map(str::to_owned),
            entity_type: cast::required_enum(
                object,
                path,
                "entity_type",
                "a scheduled event entity type",
                GuildScheduledEventEntityType::from_value,
            )?,
            entity_id: cast::optional_snowflake(object, path, "entity_id")?,
            start_time: cast::required_timestamp(object, path, "scheduled_start_time")?,
            end_time: cast::optional_timestamp(object, path, "scheduled_end_time")?,
            privacy_level: cast::required_enum(
                object,
                path,
                "privacy_level",
                "a privacy level",
                PrivacyLevel::from_value,
            )?,
            status: cast::required_enum(
                object,
                path,
                "status",
                "a scheduled event status",
                GuildScheduledEventStatus::from_value,
            )?,
            cover_image: cast::optional_string(object, path, "image")?
                .map(|hash| Asset::from_scheduled_event_cover_image(id, hash)),
            user_count: cast::optional_u64(object, path, "user_count")?.unwrap_or(0),
            creator,
            creator_id: cast::optional_snowflake(object, path, "creator_id")?,
            location,
        })
    }

    /// Sets the channel the event is held in.
    #[must_use]
    pub fn with_channel_id(mut self, channel_id: impl Into<Snowflake>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    /// Sets the event's description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the entity the event is held in.
    #[must_use]
    pub fn with_entity_type(mut self, entity_type: GuildScheduledEventEntityType) -> Self {
        self.entity_type = entity_type;
        self
    }

    /// Sets the ID of the entity the event is held in.
    #[must_use]
    pub fn with_entity_id(mut self, entity_id: impl Into<Snowflake>) -> Self {
        self.entity_id = Some(entity_id.into());
        self
    }

    /// Sets when the event ends.
    #[must_use]
    pub fn with_end_time(mut self, end_time: SystemTime) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// Sets the event's privacy level.
    #[must_use]
    pub fn with_privacy_level(mut self, privacy_level: PrivacyLevel) -> Self {
        self.privacy_level = privacy_level;
        self
    }

    /// Sets the event's status.
    #[must_use]
    pub fn with_status(mut self, status: GuildScheduledEventStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the event's cover image from its image hash.
    #[must_use]
    pub fn with_cover_image_hash(mut self, image_hash: &str) -> Self {
        self.cover_image = Some(Asset::from_scheduled_event_cover_image(self.id, image_hash));
        self
    }

    /// Sets the event's cover image.
    #[must_use]
    pub fn with_cover_image(mut self, cover_image: Asset) -> Self {
        self.cover_image = Some(cover_image);
        self
    }

    /// Sets the number of users subscribed to the event.
    #[must_use]
    pub fn with_user_count(mut self, user_count: u64) -> Self {
        self.user_count = user_count;
        self
    }

    /// Sets the user that created the event.
    #[must_use]
    pub fn with_creator(mut self, creator: User) -> Self {
        self.creator_id = Some(creator.id);
        self.creator = Some(creator);
        self
    }

    /// Sets the ID of the user that created the event.
    #[must_use]
    pub fn with_creator_id(mut self, creator_id: impl Into<Snowflake>) -> Self {
        self.creator_id = Some(creator_id.into());
        self
    }

    /// Sets the location of an external event.
    #[must_use]
    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    /// Returns the event's creation time.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.id.created_at()
    }

    /// Returns the URL that opens the event in Discord.
    #[must_use]
    pub fn url(&self) -> String {
        format!("https://discord.com/events/{}/{}", self.guild_id, self.id)
    }

    /// Returns the cover image URL, if the event has one.
    #[must_use]
    pub fn cover_image_url(&self) -> Option<&str> {
        self.cover_image.as_ref().map(Asset::url)
    }

    /// Returns the channel the event is held in, if it is cached.
    #[must_use]
    pub fn channel<'a>(&self, guild: &'a Guild) -> Option<&'a GuildChannel> {
        self.channel_id
            .and_then(|channel_id| guild.get_channel(channel_id))
    }

    /// Returns `true` if the event has not started.
    #[must_use]
    pub const fn is_scheduled(&self) -> bool {
        self.status.is_scheduled()
    }

    /// Returns `true` if the event is in progress.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.status.is_active()
    }

    /// Returns `true` if the event has completed.
    #[must_use]
    pub const fn is_completed(&self) -> bool {
        self.status.is_completed()
    }

    /// Returns `true` if the event was canceled.
    #[must_use]
    pub const fn is_canceled(&self) -> bool {
        self.status.is_canceled()
    }

    /// Returns `true` if the event is held in a stage instance.
    #[must_use]
    pub const fn is_stage_instance(&self) -> bool {
        self.entity_type.is_stage_instance()
    }

    /// Returns `true` if the event is held in a voice channel.
    #[must_use]
    pub const fn is_voice(&self) -> bool {
        self.entity_type.is_voice()
    }

    /// Returns `true` if the event is held outside of Discord.
    #[must_use]
    pub const fn is_external(&self) -> bool {
        self.entity_type.is_external()
    }
}

impl fmt::Display for GuildScheduledEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl PartialEq for GuildScheduledEvent {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for GuildScheduledEvent {}

impl Hash for GuildScheduledEvent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for GuildScheduledEvent {
    fn id(&self) -> Snowflake {
        self.id
    }
}

/// A user subscribed to a guild scheduled event.
///
/// Equality and hashing are based on the event and user IDs.
#[derive(Clone, Debug)]
pub struct ScheduledEventUser {
    /// The ID of the event the user is subscribed to.
    pub guild_scheduled_event_id: Snowflake,
    /// The subscribed user.
    pub user: User,
    /// The subscribed user's member data, when available.
    pub member: Option<Member>,
}

impl ScheduledEventUser {
    /// Creates a scheduled event subscriber.
    #[must_use]
    pub fn new(guild_scheduled_event_id: impl Into<Snowflake>, user: User) -> Self {
        Self {
            guild_scheduled_event_id: guild_scheduled_event_id.into(),
            user,
            member: None,
        }
    }

    /// Sets the subscriber's member data.
    #[must_use]
    pub fn with_member(mut self, member: Member) -> Self {
        self.member = Some(member);
        self
    }

    /// Returns the subscriber's user ID.
    #[must_use]
    pub const fn user_id(&self) -> Snowflake {
        self.user.id
    }

    /// Decodes a scheduled event subscriber from a Discord gateway object.
    ///
    /// `guild_id` is used for the subscriber's member data, when present.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or malformed.
    pub fn from_value(value: &Value, guild_id: impl Into<Snowflake>) -> Result<Self, CastError> {
        Self::from_value_at(value, "scheduled_event_user", guild_id.into())
    }

    pub(crate) fn from_value_at(
        value: &Value,
        path: &str,
        guild_id: Snowflake,
    ) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;

        let user_path = cast::field_path(path, "user");
        let user = User::from_object_at(cast::required_object(object, path, "user")?, &user_path)?;

        let member_path = cast::field_path(path, "member");
        let member = cast::optional_object(object, path, "member")?
            .map(|member| Member::from_object_at(member, &member_path, guild_id, &[]))
            .transpose()?;

        Ok(Self {
            guild_scheduled_event_id: cast::required_snowflake(
                object,
                path,
                "guild_scheduled_event_id",
            )?,
            user,
            member,
        })
    }
}

impl fmt::Display for ScheduledEventUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.user, f)
    }
}

impl PartialEq for ScheduledEventUser {
    fn eq(&self, other: &Self) -> bool {
        self.guild_scheduled_event_id == other.guild_scheduled_event_id
            && self.user.id == other.user.id
    }
}

impl Eq for ScheduledEventUser {}

impl Hash for ScheduledEventUser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.guild_scheduled_event_id.hash(state);
        self.user.id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use abc::{Asset, HasId, Snowflake};
    use data::json;
    use enums::{
        ChannelType, GuildScheduledEventEntityType, GuildScheduledEventStatus, PrivacyLevel,
    };

    use super::{GuildScheduledEvent, ScheduledEventUser};
    use crate::{Guild, GuildChannel, Member, User};

    #[test]
    fn builders_and_helpers() {
        let start = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000);
        let end = SystemTime::UNIX_EPOCH + Duration::from_secs(2_000);
        let creator = User::new(1_u64, "creator");
        let event = GuildScheduledEvent::new(10_u64, 20_u64, "Event", start)
            .with_channel_id(30_u64)
            .with_description("An event")
            .with_entity_type(GuildScheduledEventEntityType::Voice)
            .with_entity_id(30_u64)
            .with_end_time(end)
            .with_privacy_level(PrivacyLevel::GuildOnly)
            .with_status(GuildScheduledEventStatus::Active)
            .with_cover_image_hash("cover")
            .with_user_count(7)
            .with_creator(creator.clone())
            .with_location("Somewhere");

        assert_eq!(event.id, Snowflake::new(10));
        assert_eq!(event.id(), Snowflake::new(10));
        assert_eq!(event.guild_id, Snowflake::new(20));
        assert_eq!(event.channel_id, Some(Snowflake::new(30)));
        assert_eq!(event.name, "Event");
        assert_eq!(event.description.as_deref(), Some("An event"));
        assert_eq!(event.entity_type, GuildScheduledEventEntityType::Voice);
        assert_eq!(event.entity_id, Some(Snowflake::new(30)));
        assert_eq!(event.start_time, start);
        assert_eq!(event.end_time, Some(end));
        assert_eq!(event.privacy_level, PrivacyLevel::GuildOnly);
        assert_eq!(event.status, GuildScheduledEventStatus::Active);
        assert_eq!(event.user_count, 7);
        assert_eq!(event.creator, Some(creator));
        assert_eq!(event.creator_id, Some(Snowflake::new(1)));
        assert_eq!(event.location.as_deref(), Some("Somewhere"));
        assert_eq!(
            event.cover_image,
            Some(Asset::from_scheduled_event_cover_image(
                Snowflake::new(10),
                "cover"
            ))
        );
        assert_eq!(
            event.cover_image_url(),
            Some(Asset::from_scheduled_event_cover_image(Snowflake::new(10), "cover").url())
        );
        assert_eq!(event.created_at(), Snowflake::new(10).created_at());
        assert_eq!(event.url(), "https://discord.com/events/20/10");
        assert_eq!(event.to_string(), "Event");
        assert!(event.is_active());
        assert!(!event.is_scheduled());
        assert!(!event.is_completed());
        assert!(!event.is_canceled());
        assert!(event.is_voice());
        assert!(!event.is_stage_instance());
        assert!(!event.is_external());
    }

    #[test]
    fn entity_and_status_helpers_and_channel_lookup() {
        let start = SystemTime::UNIX_EPOCH;
        let stage = GuildScheduledEvent::new(1_u64, 100_u64, "Stage", start)
            .with_entity_type(GuildScheduledEventEntityType::StageInstance)
            .with_status(GuildScheduledEventStatus::Scheduled);
        let external = GuildScheduledEvent::new(2_u64, 100_u64, "External", start)
            .with_entity_type(GuildScheduledEventEntityType::External)
            .with_status(GuildScheduledEventStatus::Canceled);

        assert!(stage.is_stage_instance());
        assert!(stage.is_scheduled());
        assert!(external.is_external());
        assert!(external.is_canceled());
        assert!(!external.is_voice());

        let channel = GuildChannel::new(30_u64, 100_u64, "voice", ChannelType::Voice);
        let guild = Guild::new(100_u64, "Guild").with_channels(vec![channel]);
        let event = stage.with_channel_id(30_u64);
        assert_eq!(
            event.channel(&guild).map(|channel| channel.id),
            Some(Snowflake::new(30))
        );
        assert!(external.channel(&guild).is_none());

        assert_eq!(
            GuildScheduledEvent::default().start_time,
            SystemTime::UNIX_EPOCH
        );
        assert_eq!(
            GuildScheduledEvent::default().privacy_level,
            PrivacyLevel::GuildOnly
        );
    }

    #[test]
    fn subscribers_use_event_and_user_ids_for_equality() {
        let user = User::new(1_u64, "subscriber");
        let member = Member::new(user.clone(), 100_u64);
        let first = ScheduledEventUser::new(10_u64, user.clone());
        let same = ScheduledEventUser::new(10_u64, User::new(1_u64, "renamed"));
        let other_event = ScheduledEventUser::new(11_u64, user.clone());
        let other_user = ScheduledEventUser::new(10_u64, User::new(2_u64, "other"));
        let with_member = first.clone().with_member(member);

        assert_eq!(first.guild_scheduled_event_id, Snowflake::new(10));
        assert_eq!(first.user_id(), Snowflake::new(1));
        assert_eq!(first, same);
        assert_ne!(first, other_event);
        assert_ne!(first, other_user);
        assert_eq!(first, with_member);
        assert_eq!(first.to_string(), user.to_string());
        assert!(with_member.member.is_some());
    }

    #[test]
    fn decodes_scheduled_event_objects() {
        let value = json::from_str(
            r#"{
                "id": "10",
                "guild_id": "100",
                "channel_id": "30",
                "name": "Launch",
                "description": "We are live",
                "entity_type": 2,
                "entity_id": "30",
                "scheduled_start_time": "2021-08-01T12:34:56Z",
                "scheduled_end_time": "2021-08-02T00:00:00Z",
                "privacy_level": 2,
                "status": 2,
                "image": "coverhash",
                "user_count": 5,
                "creator": {"id": "1", "username": "winter"},
                "creator_id": "1",
                "entity_metadata": {"location": "Stage"}
            }"#,
        )
        .unwrap();

        let event = GuildScheduledEvent::from_value(&value, 999_u64).unwrap();
        assert_eq!(event.guild_id, Snowflake::new(100));
        assert_eq!(event.channel_id, Some(Snowflake::new(30)));
        assert_eq!(event.name, "Launch");
        assert_eq!(event.description.as_deref(), Some("We are live"));
        assert_eq!(event.entity_type, GuildScheduledEventEntityType::Voice);
        assert_eq!(event.entity_id, Some(Snowflake::new(30)));
        assert_eq!(
            event.start_time,
            UNIX_EPOCH + Duration::new(1_627_821_296, 0)
        );
        assert_eq!(
            event.end_time,
            Some(UNIX_EPOCH + Duration::new(1_627_862_400, 0))
        );
        assert_eq!(event.privacy_level, PrivacyLevel::GuildOnly);
        assert_eq!(event.status, GuildScheduledEventStatus::Active);
        assert_eq!(event.user_count, 5);
        assert_eq!(event.location.as_deref(), Some("Stage"));
        assert_eq!(
            event.creator.as_ref().map(|creator| creator.id),
            Some(Snowflake::new(1))
        );
        assert_eq!(event.creator_id, Some(Snowflake::new(1)));
        assert!(
            event
                .cover_image
                .as_ref()
                .unwrap()
                .url()
                .contains("/guild-events/10/coverhash.png")
        );

        let minimal = GuildScheduledEvent::from_value(
            &json::from_str(
                r#"{
                    "id": "11",
                    "name": "External",
                    "entity_type": 3,
                    "scheduled_start_time": "2021-08-01T12:34:56Z",
                    "privacy_level": 2,
                    "status": 1
                }"#,
            )
            .unwrap(),
            100_u64,
        )
        .unwrap();
        assert_eq!(minimal.guild_id, Snowflake::new(100));
        assert_eq!(minimal.channel_id, None);
        assert_eq!(minimal.end_time, None);
        assert_eq!(minimal.cover_image, None);
        assert_eq!(minimal.user_count, 0);
        assert_eq!(minimal.creator, None);
        assert_eq!(minimal.location, None);

        let bad_entity = GuildScheduledEvent::from_value(
            &json::from_str(
                r#"{
                    "id": "11",
                    "name": "External",
                    "entity_type": 9,
                    "scheduled_start_time": "2021-08-01T12:34:56Z",
                    "privacy_level": 2,
                    "status": 1
                }"#,
            )
            .unwrap(),
            100_u64,
        )
        .unwrap_err();
        assert_eq!(
            bad_entity.to_string(),
            "field `event.entity_type` must be a scheduled event entity type"
        );

        let bad_time = GuildScheduledEvent::from_value(
            &json::from_str(
                r#"{
                    "id": "11",
                    "name": "External",
                    "entity_type": 3,
                    "scheduled_start_time": "tomorrow",
                    "privacy_level": 2,
                    "status": 1
                }"#,
            )
            .unwrap(),
            100_u64,
        )
        .unwrap_err();
        assert_eq!(
            bad_time.to_string(),
            "field `event.scheduled_start_time` must be a valid ISO 8601 timestamp"
        );

        let bad_creator = GuildScheduledEvent::from_value(
            &json::from_str(
                r#"{
                    "id": "11",
                    "name": "External",
                    "entity_type": 3,
                    "scheduled_start_time": "2021-08-01T12:34:56Z",
                    "privacy_level": 2,
                    "status": 1,
                    "creator": {"id": "nope", "username": "winter"}
                }"#,
            )
            .unwrap(),
            100_u64,
        )
        .unwrap_err();
        assert_eq!(
            bad_creator.to_string(),
            "field `event.creator.id` must be a snowflake"
        );
    }
}
