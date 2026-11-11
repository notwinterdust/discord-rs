//! Thread models.
//!
//! Thread channels are represented by [`GuildChannel`](crate::GuildChannel)
//! with a [`ThreadInfo`] attached. This module provides the thread-specific
//! state, metadata and member models.

use std::hash::{Hash, Hasher};
use std::time::SystemTime;

use abc::{HasId, Snowflake};
use data::{Object, Value};
use enums::ThreadMemberFlags;

use crate::cast::{self, CastError};

/// The metadata describing a thread's archive state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThreadMetadata {
    /// Whether the thread is archived.
    pub archived: bool,
    /// The duration in minutes before the thread is automatically archived.
    pub auto_archive_duration: u64,
    /// When the thread's archive state was last changed.
    pub archive_timestamp: SystemTime,
    /// The user that archived the thread, if known.
    pub archiver_id: Option<Snowflake>,
    /// Whether the thread is locked.
    pub locked: bool,
    /// Whether non-moderators can invite other non-moderators.
    pub invitable: bool,
    /// When the thread was created, if Discord provided a timestamp.
    pub create_timestamp: Option<SystemTime>,
}

impl Default for ThreadMetadata {
    fn default() -> Self {
        Self {
            archived: false,
            auto_archive_duration: 1440,
            archive_timestamp: SystemTime::UNIX_EPOCH,
            archiver_id: None,
            locked: false,
            invitable: true,
            create_timestamp: None,
        }
    }
}

impl ThreadMetadata {
    /// The one-hour auto-archive duration, in minutes.
    pub const ONE_HOUR: u64 = 60;
    /// The one-day auto-archive duration, in minutes.
    pub const ONE_DAY: u64 = 1440;
    /// The three-day auto-archive duration, in minutes.
    pub const THREE_DAYS: u64 = 4320;
    /// The one-week auto-archive duration, in minutes.
    pub const ONE_WEEK: u64 = 10_080;

    /// Creates metadata with an auto-archive duration and archive timestamp.
    #[must_use]
    pub fn new(auto_archive_duration: u64, archive_timestamp: SystemTime) -> Self {
        Self {
            auto_archive_duration,
            archive_timestamp,
            ..Self::default()
        }
    }

    /// Decodes thread metadata from a Discord thread metadata object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "thread_metadata")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Self::from_object_at(object, path)
    }

    pub(crate) fn from_object_at(object: &Object, path: &str) -> Result<Self, CastError> {
        Ok(Self {
            archived: cast::optional_bool(object, path, "archived", false)?,
            auto_archive_duration: cast::optional_u64(object, path, "auto_archive_duration")?
                .unwrap_or(Self::ONE_DAY),
            archive_timestamp: cast::required_timestamp(object, path, "archive_timestamp")?,
            archiver_id: cast::optional_snowflake(object, path, "archiver_id")?,
            locked: cast::optional_bool(object, path, "locked", false)?,
            invitable: cast::optional_bool(object, path, "invitable", true)?,
            create_timestamp: cast::optional_timestamp(object, path, "create_timestamp")?,
        })
    }

    /// Sets whether the thread is archived.
    #[must_use]
    pub fn with_archived(mut self, archived: bool) -> Self {
        self.archived = archived;
        self
    }

    /// Sets the auto-archive duration in minutes.
    #[must_use]
    pub fn with_auto_archive_duration(mut self, minutes: u64) -> Self {
        self.auto_archive_duration = minutes;
        self
    }

    /// Sets the user that archived the thread.
    #[must_use]
    pub fn with_archiver_id(mut self, archiver_id: impl Into<Snowflake>) -> Self {
        self.archiver_id = Some(archiver_id.into());
        self
    }

    /// Sets whether the thread is locked.
    #[must_use]
    pub fn with_locked(mut self, locked: bool) -> Self {
        self.locked = locked;
        self
    }

    /// Sets whether non-moderators can invite other non-moderators.
    #[must_use]
    pub fn with_invitable(mut self, invitable: bool) -> Self {
        self.invitable = invitable;
        self
    }

    /// Sets when the thread was created.
    #[must_use]
    pub fn with_create_timestamp(mut self, timestamp: SystemTime) -> Self {
        self.create_timestamp = Some(timestamp);
        self
    }

    /// Returns whether the thread is archived.
    #[must_use]
    pub const fn is_archived(&self) -> bool {
        self.archived
    }

    /// Returns whether the thread is locked.
    #[must_use]
    pub const fn is_locked(&self) -> bool {
        self.locked
    }

    /// Returns whether non-moderators can invite other non-moderators.
    #[must_use]
    pub const fn is_invitable(&self) -> bool {
        self.invitable
    }

    /// Returns when the thread was created, if known.
    #[must_use]
    pub const fn created_at(&self) -> Option<SystemTime> {
        self.create_timestamp
    }
}

/// A member of a thread.
///
/// Equality and hashing are based on the thread and user IDs.
#[derive(Clone, Debug)]
pub struct ThreadMember {
    /// The member's user ID.
    pub id: Snowflake,
    /// The ID of the thread the member belongs to.
    pub thread_id: Snowflake,
    /// When the member joined the thread.
    pub joined_at: SystemTime,
    /// The member's thread flags.
    pub flags: ThreadMemberFlags,
}

impl Default for ThreadMember {
    fn default() -> Self {
        Self {
            id: Snowflake::ZERO,
            thread_id: Snowflake::ZERO,
            joined_at: SystemTime::UNIX_EPOCH,
            flags: ThreadMemberFlags::default(),
        }
    }
}

impl ThreadMember {
    /// Creates a thread member.
    #[must_use]
    pub fn new(
        id: impl Into<Snowflake>,
        thread_id: impl Into<Snowflake>,
        joined_at: SystemTime,
    ) -> Self {
        Self {
            id: id.into(),
            thread_id: thread_id.into(),
            joined_at,
            flags: ThreadMemberFlags::default(),
        }
    }

    /// Decodes a thread member from a Discord thread member object.
    ///
    /// Thread member payloads often omit the thread ID when they are nested in
    /// a thread channel, so `thread_id` is used as the fallback.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value, thread_id: impl Into<Snowflake>) -> Result<Self, CastError> {
        Self::from_value_at(value, "thread_member", thread_id.into())
    }

    pub(crate) fn from_value_at(
        value: &Value,
        path: &str,
        thread_id: Snowflake,
    ) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Self::from_object_at(object, path, thread_id)
    }

    pub(crate) fn from_object_at(
        object: &Object,
        path: &str,
        thread_id: Snowflake,
    ) -> Result<Self, CastError> {
        Ok(Self {
            id: cast::required_snowflake(object, path, "user_id")?,
            thread_id: cast::optional_snowflake(object, path, "id")?.unwrap_or(thread_id),
            joined_at: cast::optional_timestamp(object, path, "join_timestamp")?
                .unwrap_or(SystemTime::UNIX_EPOCH),
            flags: ThreadMemberFlags::from_bits(
                cast::optional_u64(object, path, "flags")?.unwrap_or(0),
            ),
        })
    }

    /// Sets the member's thread flags.
    #[must_use]
    pub fn with_flags(mut self, flags: ThreadMemberFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Returns whether the member has joined the thread.
    #[must_use]
    pub const fn is_in_thread(&self) -> bool {
        self.flags.in_thread()
    }

    /// Returns whether the member is a thread moderator.
    #[must_use]
    pub const fn is_moderator(&self) -> bool {
        self.flags.moderator()
    }
}

impl PartialEq for ThreadMember {
    fn eq(&self, other: &Self) -> bool {
        self.thread_id == other.thread_id && self.id == other.id
    }
}

impl Eq for ThreadMember {}

impl Hash for ThreadMember {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.thread_id.hash(state);
        self.id.hash(state);
    }
}

impl HasId for ThreadMember {
    fn id(&self) -> Snowflake {
        self.id
    }
}

/// The thread-specific state attached to a thread channel.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ThreadInfo {
    /// The ID of the thread's parent channel.
    pub parent_id: Snowflake,
    /// The ID of the user that created the thread.
    pub owner_id: Snowflake,
    /// The thread's archive metadata.
    pub metadata: ThreadMetadata,
    /// The approximate number of messages in the thread.
    pub message_count: u64,
    /// The approximate number of members in the thread.
    pub member_count: u64,
    /// The total number of messages sent in the thread.
    pub total_message_sent: u64,
    /// The IDs of forum tags applied to the thread.
    pub applied_tags: Vec<Snowflake>,
    /// The cached thread members.
    pub members: Vec<ThreadMember>,
    /// The current user's thread member, if they have joined.
    pub me: Option<ThreadMember>,
    /// When the last message in the thread was pinned, if any.
    pub last_pin_timestamp: Option<SystemTime>,
    /// Whether the thread was newly created.
    pub newly_created: bool,
}

impl ThreadInfo {
    /// Creates thread state for a parent channel and owner.
    #[must_use]
    pub fn new(parent_id: impl Into<Snowflake>, owner_id: impl Into<Snowflake>) -> Self {
        Self {
            parent_id: parent_id.into(),
            owner_id: owner_id.into(),
            ..Self::default()
        }
    }

    /// Decodes thread state from a Discord thread channel object.
    ///
    /// `thread_id` is used for nested thread members that omit their thread ID,
    /// and `parent_id` is used when the payload does not repeat the parent.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a field has an unexpected type.
    pub fn from_value(
        value: &Value,
        thread_id: impl Into<Snowflake>,
        parent_id: impl Into<Snowflake>,
    ) -> Result<Self, CastError> {
        Self::from_value_at(value, "thread", thread_id.into(), parent_id.into())
    }

    pub(crate) fn from_value_at(
        value: &Value,
        path: &str,
        thread_id: Snowflake,
        parent_id: Snowflake,
    ) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;

        let metadata_path = cast::field_path(path, "thread_metadata");
        let metadata = match cast::optional_object(object, path, "thread_metadata")? {
            Some(metadata) => ThreadMetadata::from_object_at(metadata, &metadata_path)?,
            None => ThreadMetadata::default(),
        };

        let members_path = cast::field_path(path, "members");
        let mut members = Vec::new();
        if let Some(values) = cast::optional_array(object, path, "members")? {
            members.reserve(values.len());
            for (index, value) in values.iter().enumerate() {
                members.push(ThreadMember::from_value_at(
                    value,
                    &cast::index_path(&members_path, index),
                    thread_id,
                )?);
            }
        }

        let thread_member_path = cast::field_path(path, "thread_member");
        let legacy_member_path = cast::field_path(path, "member");
        let me = if let Some(member) = cast::optional_object(object, path, "thread_member")? {
            Some(ThreadMember::from_object_at(
                member,
                &thread_member_path,
                thread_id,
            )?)
        } else if let Some(member) = cast::optional_object(object, path, "member")? {
            Some(ThreadMember::from_object_at(
                member,
                &legacy_member_path,
                thread_id,
            )?)
        } else {
            None
        };

        Ok(Self {
            parent_id: cast::optional_snowflake(object, path, "parent_id")?.unwrap_or(parent_id),
            owner_id: cast::optional_snowflake(object, path, "owner_id")?.unwrap_or_default(),
            metadata,
            message_count: cast::optional_u64(object, path, "message_count")?.unwrap_or(0),
            member_count: cast::optional_u64(object, path, "member_count")?.unwrap_or(0),
            total_message_sent: cast::optional_u64(object, path, "total_message_sent")?
                .unwrap_or(0),
            applied_tags: cast::optional_snowflake_array(object, path, "applied_tags")?
                .unwrap_or_default(),
            members,
            me,
            last_pin_timestamp: cast::optional_timestamp(object, path, "last_pin_timestamp")?,
            newly_created: cast::optional_bool(object, path, "newly_created", false)?,
        })
    }

    /// Sets the thread's archive metadata.
    #[must_use]
    pub fn with_metadata(mut self, metadata: ThreadMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Sets the approximate message and member counts.
    #[must_use]
    pub fn with_counts(mut self, message_count: u64, member_count: u64) -> Self {
        self.message_count = message_count;
        self.member_count = member_count;
        self
    }

    /// Sets the approximate message count.
    #[must_use]
    pub fn with_message_count(mut self, message_count: u64) -> Self {
        self.message_count = message_count;
        self
    }

    /// Sets the approximate member count.
    #[must_use]
    pub fn with_member_count(mut self, member_count: u64) -> Self {
        self.member_count = member_count;
        self
    }

    /// Sets the total number of messages sent in the thread.
    #[must_use]
    pub fn with_total_message_sent(mut self, total_message_sent: u64) -> Self {
        self.total_message_sent = total_message_sent;
        self
    }

    /// Sets the forum tags applied to the thread.
    #[must_use]
    pub fn with_applied_tags(mut self, tags: Vec<Snowflake>) -> Self {
        self.applied_tags = tags;
        self
    }

    /// Appends a forum tag.
    #[must_use]
    pub fn add_applied_tag(mut self, tag_id: impl Into<Snowflake>) -> Self {
        self.applied_tags.push(tag_id.into());
        self
    }

    /// Replaces the cached thread members.
    #[must_use]
    pub fn with_members(mut self, members: Vec<ThreadMember>) -> Self {
        self.members = members;
        self
    }

    /// Adds or replaces a cached thread member.
    pub fn add_member(&mut self, member: ThreadMember) {
        if let Some(existing) = self
            .members
            .iter_mut()
            .find(|existing| existing.id == member.id)
        {
            *existing = member;
        } else {
            self.members.push(member);
        }
    }

    /// Removes a cached thread member, returning it if it existed.
    pub fn remove_member(&mut self, user_id: impl Into<Snowflake>) -> Option<ThreadMember> {
        let user_id = user_id.into();
        let index = self
            .members
            .iter()
            .position(|member| member.id == user_id)?;
        Some(self.members.remove(index))
    }

    /// Returns the cached member with the given user ID.
    #[must_use]
    pub fn member(&self, user_id: impl Into<Snowflake>) -> Option<&ThreadMember> {
        let user_id = user_id.into();
        self.members.iter().find(|member| member.id == user_id)
    }

    /// Sets the current user's thread member.
    #[must_use]
    pub fn with_me(mut self, member: ThreadMember) -> Self {
        self.me = Some(member);
        self
    }

    /// Sets when the last message was pinned.
    #[must_use]
    pub fn with_last_pin_timestamp(mut self, timestamp: SystemTime) -> Self {
        self.last_pin_timestamp = Some(timestamp);
        self
    }

    /// Sets whether the thread was newly created.
    #[must_use]
    pub fn with_newly_created(mut self, newly_created: bool) -> Self {
        self.newly_created = newly_created;
        self
    }

    /// Returns the cached thread members.
    #[must_use]
    pub fn members(&self) -> &[ThreadMember] {
        &self.members
    }

    /// Returns whether the thread is archived.
    #[must_use]
    pub const fn is_archived(&self) -> bool {
        self.metadata.archived
    }

    /// Returns whether the thread is locked.
    #[must_use]
    pub const fn is_locked(&self) -> bool {
        self.metadata.locked
    }

    /// Returns whether non-moderators can invite other non-moderators.
    #[must_use]
    pub const fn is_invitable(&self) -> bool {
        self.metadata.invitable
    }

    /// Returns when the thread was created, if known.
    #[must_use]
    pub const fn created_at(&self) -> Option<SystemTime> {
        self.metadata.create_timestamp
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use abc::{HasId, Snowflake};
    use data::json;
    use enums::ThreadMemberFlags;

    use super::{ThreadInfo, ThreadMember, ThreadMetadata};

    #[test]
    fn metadata_builders_and_defaults() {
        let archive_timestamp = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000);
        let created = SystemTime::UNIX_EPOCH + Duration::from_secs(900);
        let metadata = ThreadMetadata::new(ThreadMetadata::ONE_WEEK, archive_timestamp)
            .with_archived(true)
            .with_archiver_id(10_u64)
            .with_locked(true)
            .with_invitable(false)
            .with_create_timestamp(created);

        assert!(metadata.is_archived());
        assert!(metadata.is_locked());
        assert!(!metadata.is_invitable());
        assert_eq!(metadata.auto_archive_duration, 10_080);
        assert_eq!(metadata.archive_timestamp, archive_timestamp);
        assert_eq!(metadata.archiver_id, Some(Snowflake::new(10)));
        assert_eq!(metadata.created_at(), Some(created));

        assert_eq!(ThreadMetadata::default().auto_archive_duration, 1440);
        assert!(ThreadMetadata::default().is_invitable());
        assert_eq!(ThreadMetadata::default().created_at(), None);
        assert_eq!(ThreadMetadata::ONE_HOUR, 60);
        assert_eq!(ThreadMetadata::ONE_DAY, 1440);
        assert_eq!(ThreadMetadata::THREE_DAYS, 4320);
    }

    #[test]
    fn thread_member_flags_and_equality() {
        let joined_at = SystemTime::UNIX_EPOCH + Duration::from_secs(500);
        let member = ThreadMember::new(1_u64, 2_u64, joined_at)
            .with_flags(ThreadMemberFlags::IN | ThreadMemberFlags::MODERATOR);

        assert_eq!(member.id, Snowflake::new(1));
        assert_eq!(member.id(), Snowflake::new(1));
        assert_eq!(member.thread_id, Snowflake::new(2));
        assert_eq!(member.joined_at, joined_at);
        assert!(member.is_in_thread());
        assert!(member.is_moderator());

        let same = ThreadMember::new(1_u64, 2_u64, SystemTime::UNIX_EPOCH);
        assert_eq!(member, same);

        let other_thread = ThreadMember::new(1_u64, 3_u64, joined_at);
        assert_ne!(member, other_thread);

        let other_user = ThreadMember::new(2_u64, 2_u64, joined_at);
        assert_ne!(member, other_user);
        assert_eq!(ThreadMember::default().flags, ThreadMemberFlags::none());
    }

    #[test]
    fn thread_info_builders_and_members() {
        let archive_timestamp = SystemTime::UNIX_EPOCH + Duration::from_secs(2_000);
        let pinned = SystemTime::UNIX_EPOCH + Duration::from_secs(1_500);
        let member = ThreadMember::new(1_u64, 10_u64, archive_timestamp);
        let replacement = ThreadMember::new(1_u64, 10_u64, pinned);
        let other = ThreadMember::new(2_u64, 10_u64, archive_timestamp);

        let mut info = ThreadInfo::new(20_u64, 30_u64)
            .with_metadata(
                ThreadMetadata::new(ThreadMetadata::ONE_DAY, archive_timestamp)
                    .with_archived(true)
                    .with_locked(true)
                    .with_create_timestamp(archive_timestamp),
            )
            .with_counts(5, 2)
            .with_total_message_sent(8)
            .with_applied_tags(vec![Snowflake::new(40)])
            .add_applied_tag(41_u64)
            .with_members(vec![member.clone()])
            .with_me(member.clone())
            .with_last_pin_timestamp(pinned)
            .with_newly_created(true);

        info.add_member(replacement.clone());
        info.add_member(other.clone());

        assert_eq!(info.parent_id, Snowflake::new(20));
        assert_eq!(info.owner_id, Snowflake::new(30));
        assert!(info.is_archived());
        assert!(info.is_locked());
        assert!(info.is_invitable());
        assert_eq!(info.message_count, 5);
        assert_eq!(info.member_count, 2);
        assert_eq!(info.total_message_sent, 8);
        assert_eq!(
            info.applied_tags,
            vec![Snowflake::new(40), Snowflake::new(41)]
        );
        assert_eq!(info.members(), &[replacement.clone(), other]);
        assert_eq!(
            info.member(1_u64).map(|member| member.joined_at),
            Some(pinned)
        );
        assert_eq!(
            info.member(2_u64).map(|member| member.id),
            Some(Snowflake::new(2))
        );
        assert!(info.member(3_u64).is_none());
        assert_eq!(
            info.me.as_ref().map(|member| member.id),
            Some(Snowflake::new(1))
        );
        assert_eq!(info.last_pin_timestamp, Some(pinned));
        assert!(info.newly_created);
        assert_eq!(info.created_at(), Some(archive_timestamp));

        assert_eq!(info.remove_member(1_u64), Some(replacement));
        assert!(info.remove_member(99_u64).is_none());
        assert_eq!(info.members().len(), 1);
        assert_eq!(ThreadInfo::default().parent_id, Snowflake::ZERO);
    }

    #[test]
    fn decodes_thread_metadata_and_members() {
        let metadata = ThreadMetadata::from_value(
            &json::from_str(
                r#"{
                    "archived": true,
                    "auto_archive_duration": 60,
                    "archive_timestamp": "2021-08-01T12:34:56Z",
                    "archiver_id": "1",
                    "locked": true,
                    "invitable": false,
                    "create_timestamp": "2021-07-01T00:00:00Z"
                }"#,
            )
            .unwrap(),
        )
        .unwrap();
        assert!(metadata.archived);
        assert!(metadata.locked);
        assert!(!metadata.invitable);
        assert_eq!(metadata.auto_archive_duration, 60);
        assert_eq!(
            metadata.archive_timestamp,
            UNIX_EPOCH + Duration::new(1_627_821_296, 0)
        );
        assert_eq!(metadata.archiver_id, Some(Snowflake::new(1)));
        assert_eq!(
            metadata.create_timestamp,
            Some(UNIX_EPOCH + Duration::new(1_625_097_600, 0))
        );

        let member = ThreadMember::from_value(
            &json::from_str(r#"{"user_id":"2","join_timestamp":"2021-08-02T00:00:00Z","flags":1}"#)
                .unwrap(),
            11_u64,
        )
        .unwrap();
        assert_eq!(member.id, Snowflake::new(2));
        assert_eq!(member.thread_id, Snowflake::new(11));
        assert_eq!(
            member.joined_at,
            UNIX_EPOCH + Duration::new(1_627_862_400, 0)
        );
        assert!(member.is_in_thread());

        let minimal = ThreadMember::from_value(
            &json::from_str(r#"{"id":"11","user_id":"3"}"#).unwrap(),
            99_u64,
        )
        .unwrap();
        assert_eq!(minimal.thread_id, Snowflake::new(11));
        assert_eq!(minimal.joined_at, UNIX_EPOCH);
        assert!(!minimal.is_in_thread());
    }

    #[test]
    fn decodes_thread_info_from_a_channel_object() {
        let info = ThreadInfo::from_value(
            &json::from_str(
                r#"{
                    "id": "11",
                    "parent_id": "10",
                    "owner_id": "2",
                    "message_count": 3,
                    "member_count": 1,
                    "total_message_sent": 4,
                    "applied_tags": ["7"],
                    "member": {
                        "id": "11",
                        "user_id": "3",
                        "join_timestamp": "2021-08-03T00:00:00Z"
                    },
                    "newly_created": true
                }"#,
            )
            .unwrap(),
            11_u64,
            99_u64,
        )
        .unwrap();

        assert_eq!(info.parent_id, Snowflake::new(10));
        assert_eq!(info.owner_id, Snowflake::new(2));
        assert_eq!(info.message_count, 3);
        assert_eq!(info.member_count, 1);
        assert_eq!(info.total_message_sent, 4);
        assert_eq!(info.applied_tags, [Snowflake::new(7)]);
        assert_eq!(info.me.as_ref().map(|me| me.id), Some(Snowflake::new(3)));
        assert!(info.newly_created);

        let malformed = json::from_str(r#"{"owner_id":"nope"}"#).unwrap();
        let error = ThreadInfo::from_value(&malformed, 11_u64, 10_u64).unwrap_err();
        assert_eq!(
            error.to_string(),
            "field `thread.owner_id` must be a snowflake"
        );
    }
}
