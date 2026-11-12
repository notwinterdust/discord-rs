//! Message models.
//!
//! This module currently covers core message data: authors, attachments,
//! components, embeds, interactions, polls, reactions, stickers, message
//! references and the message envelope itself.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

use abc::{HasId, Snowflake};
use data::{Object, Value};
use enums::{AttachmentFlags, MessageFlags, MessageReferenceType, MessageType};

use crate::cast::{self, CastError};
use crate::{
    ActionRow, Embed, Member, MessageInteraction, MessageInteractionMetadata, Poll, Reaction,
    StickerItem, User,
};

/// The author of a message.
///
/// Guild messages usually include member data, while direct messages and
/// webhook messages only include the underlying user.
#[derive(Clone, Debug)]
pub enum MessageAuthor {
    /// A plain user, such as a direct message or webhook author.
    User(Box<User>),
    /// A guild member with guild-specific data.
    Member(Box<Member>),
}

impl MessageAuthor {
    /// Wraps a plain user.
    #[must_use]
    pub fn from_user(user: User) -> Self {
        Self::User(Box::new(user))
    }

    /// Wraps a guild member.
    #[must_use]
    pub fn from_member(member: Member) -> Self {
        Self::Member(Box::new(member))
    }

    /// Returns the author's unique ID.
    #[must_use]
    pub fn id(&self) -> Snowflake {
        match self {
            Self::User(user) => user.id,
            Self::Member(member) => member.id(),
        }
    }

    /// Returns the underlying user.
    #[must_use]
    pub fn user(&self) -> &User {
        match self {
            Self::User(user) => user.as_ref(),
            Self::Member(member) => &member.user,
        }
    }

    /// Returns the underlying member, if this author is a member.
    #[must_use]
    pub fn as_member(&self) -> Option<&Member> {
        match self {
            Self::User(_) => None,
            Self::Member(member) => Some(member.as_ref()),
        }
    }

    /// Returns `true` if the author is a guild member.
    #[must_use]
    pub fn is_member(&self) -> bool {
        matches!(self, Self::Member(_))
    }
}

impl Default for MessageAuthor {
    fn default() -> Self {
        Self::User(Box::default())
    }
}

impl From<User> for MessageAuthor {
    fn from(user: User) -> Self {
        Self::from_user(user)
    }
}

impl From<Member> for MessageAuthor {
    fn from(member: Member) -> Self {
        Self::from_member(member)
    }
}

impl HasId for MessageAuthor {
    fn id(&self) -> Snowflake {
        self.id()
    }
}

impl abc::User for MessageAuthor {
    fn name(&self) -> &str {
        match self {
            Self::User(user) => abc::User::name(user.as_ref()),
            Self::Member(member) => abc::User::name(member.as_ref()),
        }
    }

    fn discriminator(&self) -> &str {
        match self {
            Self::User(user) => abc::User::discriminator(user.as_ref()),
            Self::Member(member) => abc::User::discriminator(member.as_ref()),
        }
    }

    fn global_name(&self) -> Option<&str> {
        match self {
            Self::User(user) => abc::User::global_name(user.as_ref()),
            Self::Member(member) => abc::User::global_name(member.as_ref()),
        }
    }

    fn bot(&self) -> bool {
        match self {
            Self::User(user) => abc::User::bot(user.as_ref()),
            Self::Member(member) => abc::User::bot(member.as_ref()),
        }
    }

    fn system(&self) -> bool {
        match self {
            Self::User(user) => abc::User::system(user.as_ref()),
            Self::Member(member) => abc::User::system(member.as_ref()),
        }
    }

    fn avatar(&self) -> Option<&abc::Asset> {
        match self {
            Self::User(user) => abc::User::avatar(user.as_ref()),
            Self::Member(member) => abc::User::avatar(member.as_ref()),
        }
    }

    fn avatar_decoration(&self) -> Option<&abc::Asset> {
        match self {
            Self::User(user) => abc::User::avatar_decoration(user.as_ref()),
            Self::Member(member) => abc::User::avatar_decoration(member.as_ref()),
        }
    }

    fn avatar_decoration_sku_id(&self) -> Option<Snowflake> {
        match self {
            Self::User(user) => abc::User::avatar_decoration_sku_id(user.as_ref()),
            Self::Member(member) => abc::User::avatar_decoration_sku_id(member.as_ref()),
        }
    }

    fn display_name(&self) -> &str {
        match self {
            Self::User(user) => abc::User::display_name(user.as_ref()),
            Self::Member(member) => abc::User::display_name(member.as_ref()),
        }
    }

    fn default_avatar(&self) -> abc::Asset {
        match self {
            Self::User(user) => abc::User::default_avatar(user.as_ref()),
            Self::Member(member) => abc::User::default_avatar(member.as_ref()),
        }
    }

    fn display_avatar(&self) -> abc::Asset {
        match self {
            Self::User(user) => abc::User::display_avatar(user.as_ref()),
            Self::Member(member) => abc::User::display_avatar(member.as_ref()),
        }
    }

    fn mention(&self) -> String {
        match self {
            Self::User(user) => abc::User::mention(user.as_ref()),
            Self::Member(member) => abc::User::mention(member.as_ref()),
        }
    }
}

impl fmt::Display for MessageAuthor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(abc::User::display_name(self))
    }
}

impl PartialEq for MessageAuthor {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for MessageAuthor {}

impl Hash for MessageAuthor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

/// A file attached to a message.
///
/// Equality and hashing are based on the attachment's ID.
#[derive(Clone, Debug)]
pub struct Attachment {
    /// The attachment's unique ID.
    pub id: Snowflake,
    /// The attachment's filename.
    pub filename: String,
    /// The attachment's size in bytes.
    pub size: u64,
    /// The URL of the attachment.
    pub url: String,
    /// A cached proxy URL for the attachment.
    pub proxy_url: String,
    /// The attachment's height, for images and videos.
    pub height: Option<u64>,
    /// The attachment's width, for images and videos.
    pub width: Option<u64>,
    /// The attachment's media type.
    pub content_type: Option<String>,
    /// The attachment's description, for images.
    pub description: Option<String>,
    /// Whether the attachment is ephemeral.
    pub ephemeral: bool,
    /// The duration of a voice message, in seconds.
    pub duration: Option<f64>,
    /// The waveform of a voice message.
    pub waveform: Option<Vec<u8>>,
    /// The normalised title of the attachment.
    pub title: Option<String>,
    /// The attachment's flags.
    pub flags: AttachmentFlags,
}

impl Attachment {
    /// Creates an attachment with the fields Discord always provides.
    #[must_use]
    pub fn new(
        id: impl Into<Snowflake>,
        filename: impl Into<String>,
        size: u64,
        url: impl Into<String>,
        proxy_url: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            filename: filename.into(),
            size,
            url: url.into(),
            proxy_url: proxy_url.into(),
            height: None,
            width: None,
            content_type: None,
            description: None,
            ephemeral: false,
            duration: None,
            waveform: None,
            title: None,
            flags: AttachmentFlags::default(),
        }
    }

    /// Sets the attachment's dimensions.
    #[must_use]
    pub fn with_dimensions(mut self, width: u64, height: u64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Sets the attachment's media type.
    #[must_use]
    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    /// Sets the attachment's description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the attachment's ephemeral state.
    #[must_use]
    pub fn with_ephemeral(mut self, ephemeral: bool) -> Self {
        self.ephemeral = ephemeral;
        self
    }

    /// Sets the voice message duration and waveform.
    #[must_use]
    pub fn with_voice_data(mut self, duration: f64, waveform: Vec<u8>) -> Self {
        self.duration = Some(duration);
        self.waveform = Some(waveform);
        self
    }

    /// Sets the attachment's title.
    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the attachment's flags.
    #[must_use]
    pub fn with_flags(mut self, flags: AttachmentFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Decodes an attachment from a Discord attachment object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "attachment")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        let waveform = cast::optional(object, path, "waveform")
            .map(|(value, path)| cast::base64(value, &path))
            .transpose()?;
        Ok(Self {
            id: cast::required_snowflake(object, path, "id")?,
            filename: cast::required_string(object, path, "filename")?.to_owned(),
            size: cast::required_u64(object, path, "size")?,
            url: cast::required_string(object, path, "url")?.to_owned(),
            proxy_url: cast::required_string(object, path, "proxy_url")?.to_owned(),
            height: cast::optional_u64(object, path, "height")?,
            width: cast::optional_u64(object, path, "width")?,
            content_type: cast::optional_string(object, path, "content_type")?.map(str::to_owned),
            description: cast::optional_string(object, path, "description")?.map(str::to_owned),
            ephemeral: cast::optional_bool(object, path, "ephemeral", false)?,
            duration: cast::optional_f64(object, path, "duration_secs")?,
            waveform,
            title: cast::optional_string(object, path, "title")?.map(str::to_owned),
            flags: AttachmentFlags::from_bits(
                cast::optional_u64(object, path, "flags")?.unwrap_or(0),
            ),
        })
    }

    /// Returns whether the attachment is a spoiler.
    #[must_use]
    pub fn is_spoiler(&self) -> bool {
        self.filename.starts_with("SPOILER_") || self.flags.spoiler()
    }

    /// Returns whether the attachment is a voice message.
    #[must_use]
    pub fn is_voice_message(&self) -> bool {
        self.duration.is_some() && self.waveform.is_some()
    }
}

impl fmt::Display for Attachment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.url)
    }
}

impl PartialEq for Attachment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Attachment {}

impl Hash for Attachment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for Attachment {
    fn id(&self) -> Snowflake {
        self.id
    }
}

/// A reference to another message.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MessageReference {
    /// The kind of reference.
    pub kind: MessageReferenceType,
    /// The ID of the referenced message, when known.
    pub message_id: Option<Snowflake>,
    /// The ID of the channel containing the referenced message.
    pub channel_id: Snowflake,
    /// The ID of the guild containing the referenced message, when known.
    pub guild_id: Option<Snowflake>,
    /// Whether fetching should fail if the referenced message no longer exists.
    pub fail_if_not_exists: bool,
}

impl MessageReference {
    /// Creates a reply reference to an existing message.
    #[must_use]
    pub fn new(message_id: impl Into<Snowflake>, channel_id: impl Into<Snowflake>) -> Self {
        Self {
            kind: MessageReferenceType::Reply,
            message_id: Some(message_id.into()),
            channel_id: channel_id.into(),
            guild_id: None,
            fail_if_not_exists: true,
        }
    }

    /// Creates a reference without a known message ID.
    ///
    /// Discord omits the message ID for some system messages.
    #[must_use]
    pub fn without_message(channel_id: impl Into<Snowflake>) -> Self {
        Self {
            kind: MessageReferenceType::Reply,
            message_id: None,
            channel_id: channel_id.into(),
            guild_id: None,
            fail_if_not_exists: true,
        }
    }

    /// Sets the referenced guild ID.
    #[must_use]
    pub fn with_guild_id(mut self, guild_id: impl Into<Snowflake>) -> Self {
        self.guild_id = Some(guild_id.into());
        self
    }

    /// Sets the reference kind.
    #[must_use]
    pub fn with_kind(mut self, kind: MessageReferenceType) -> Self {
        self.kind = kind;
        self
    }

    /// Sets whether fetching should fail if the message no longer exists.
    #[must_use]
    pub fn with_fail_if_not_exists(mut self, fail_if_not_exists: bool) -> Self {
        self.fail_if_not_exists = fail_if_not_exists;
        self
    }

    /// Decodes a message reference from a Discord message reference object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "reference")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Self::from_object_at(object, path)
    }

    pub(crate) fn from_object_at(object: &Object, path: &str) -> Result<Self, CastError> {
        Ok(Self {
            kind: cast::optional_enum(
                object,
                path,
                "type",
                "a message reference type",
                MessageReferenceType::from_value,
            )?
            .unwrap_or_default(),
            message_id: cast::optional_snowflake(object, path, "message_id")?,
            channel_id: cast::required_snowflake(object, path, "channel_id")?,
            guild_id: cast::optional_snowflake(object, path, "guild_id")?,
            fail_if_not_exists: cast::optional_bool(object, path, "fail_if_not_exists", true)?,
        })
    }
}

/// A message sent in a channel.
///
/// Equality and hashing are based on the message's ID.
#[derive(Clone, Debug)]
pub struct Message {
    /// The message's unique ID.
    pub id: Snowflake,
    /// The ID of the channel containing the message.
    pub channel_id: Snowflake,
    /// The ID of the guild containing the message, if it is a guild message.
    pub guild_id: Option<Snowflake>,
    /// The message's author.
    pub author: MessageAuthor,
    /// The message's text content.
    pub content: String,
    /// When Discord received the message.
    pub timestamp: SystemTime,
    /// When the message was last edited, if it was edited.
    pub edited_timestamp: Option<SystemTime>,
    /// Whether the message was sent with text-to-speech.
    pub tts: bool,
    /// Whether the message mentions `@everyone` or `@here`.
    pub mention_everyone: bool,
    /// Whether the message is pinned.
    pub pinned: bool,
    /// The message's type.
    pub message_type: MessageType,
    /// The message's flags.
    pub flags: MessageFlags,
    /// The users or members mentioned by the message.
    pub mentions: Vec<MessageAuthor>,
    /// The IDs of roles mentioned by the message.
    pub mention_role_ids: Vec<Snowflake>,
    /// The IDs of channels mentioned by the message.
    pub mention_channel_ids: Vec<Snowflake>,
    /// The message's attachments.
    pub attachments: Vec<Attachment>,
    /// The message's embeds.
    pub embeds: Vec<Embed>,
    /// The message's sticker items.
    pub sticker_items: Vec<StickerItem>,
    /// The message's component rows.
    pub components: Vec<ActionRow>,
    /// The message's poll, if it has one.
    pub poll: Option<Poll>,
    /// The message's reactions.
    pub reactions: Vec<Reaction>,
    /// The message this message references, if any.
    pub reference: Option<MessageReference>,
    /// The ID of the webhook that sent the message, if any.
    pub webhook_id: Option<Snowflake>,
    /// The ID of the application that created the message, if any.
    pub application_id: Option<Snowflake>,
    /// The interaction that this message is a response to, if any.
    pub interaction: Option<MessageInteraction>,
    /// Detailed metadata for the interaction that created this message, if any.
    pub interaction_metadata: Option<MessageInteractionMetadata>,
    /// The approximate position of the message in a thread, if known.
    pub position: Option<i64>,
    /// The nonce used when sending the message, if known.
    pub nonce: Option<String>,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            id: Snowflake::ZERO,
            channel_id: Snowflake::ZERO,
            guild_id: None,
            author: MessageAuthor::default(),
            content: String::new(),
            timestamp: SystemTime::UNIX_EPOCH,
            edited_timestamp: None,
            tts: false,
            mention_everyone: false,
            pinned: false,
            message_type: MessageType::default(),
            flags: MessageFlags::default(),
            mentions: Vec::new(),
            mention_role_ids: Vec::new(),
            mention_channel_ids: Vec::new(),
            attachments: Vec::new(),
            embeds: Vec::new(),
            sticker_items: Vec::new(),
            components: Vec::new(),
            poll: None,
            reactions: Vec::new(),
            reference: None,
            webhook_id: None,
            application_id: None,
            interaction: None,
            interaction_metadata: None,
            position: None,
            nonce: None,
        }
    }
}

impl Message {
    /// Creates a message with the fields Discord always provides.
    ///
    /// The timestamp defaults to the ID's creation time. Use
    /// [`Message::with_timestamp`] when the payload provides a different value.
    #[must_use]
    pub fn new(
        id: impl Into<Snowflake>,
        channel_id: impl Into<Snowflake>,
        author: impl Into<MessageAuthor>,
    ) -> Self {
        let id = id.into();
        Self {
            id,
            channel_id: channel_id.into(),
            author: author.into(),
            timestamp: id.created_at(),
            ..Self::default()
        }
    }

    /// Decodes a message from a Discord message object.
    ///
    /// Guild messages with a `member` object retain the guild-specific member
    /// data; role IDs resolve to placeholder roles because message payloads do
    /// not include the guild's full role list.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        let path = "message";
        let object = cast::object(value, path)?;
        let id = cast::required_snowflake(object, path, "id")?;
        let channel_id = cast::required_snowflake(object, path, "channel_id")?;
        let guild_id = cast::optional_snowflake(object, path, "guild_id")?;

        let author_path = cast::field_path(path, "author");
        let author_object = cast::required_object(object, path, "author")?;
        let user = User::from_object_at(author_object, &author_path)?;
        let author = if let Some(member) = cast::optional_object(object, path, "member")? {
            let member_path = cast::field_path(path, "member");
            MessageAuthor::from_member(Member::from_object_with_user_at(
                member,
                &member_path,
                guild_id.unwrap_or_default(),
                &[],
                user,
            )?)
        } else {
            MessageAuthor::from_user(user)
        };

        let mentions = cast::optional_array_of(object, path, "mentions", |value, path| {
            Ok(MessageAuthor::from_user(User::from_value_at(value, path)?))
        })?
        .unwrap_or_default();

        let mention_channels =
            cast::optional_array_of(object, path, "mention_channels", |value, path| {
                let object = cast::object(value, path)?;
                cast::required_snowflake(object, path, "id")
            })?
            .unwrap_or_default();

        let poll_path = cast::field_path(path, "poll");
        let poll = cast::optional_object(object, path, "poll")?
            .map(|poll| Poll::from_object_at(poll, &poll_path))
            .transpose()?;

        let reference_path = cast::field_path(path, "reference");
        let reference = cast::optional_object(object, path, "reference")?
            .map(|reference| MessageReference::from_object_at(reference, &reference_path))
            .transpose()?;

        let interaction_path = cast::field_path(path, "interaction");
        let interaction = cast::optional_object(object, path, "interaction")?
            .map(|interaction| MessageInteraction::from_object_at(interaction, &interaction_path))
            .transpose()?;

        let metadata_path = cast::field_path(path, "interaction_metadata");
        let interaction_metadata = cast::optional_object(object, path, "interaction_metadata")?
            .map(|metadata| MessageInteractionMetadata::from_object_at(metadata, &metadata_path))
            .transpose()?;

        let nonce = cast::optional(object, path, "nonce")
            .map(|(value, path)| match value.as_str() {
                Some(text) => Ok(text.to_owned()),
                None if value.as_number().is_some() => Ok(value.to_string()),
                None => Err(CastError::invalid_value(&path, "a string or number")),
            })
            .transpose()?;

        Ok(Self {
            id,
            channel_id,
            guild_id,
            author,
            content: cast::optional_string(object, path, "content")?
                .unwrap_or_default()
                .to_owned(),
            timestamp: cast::required_timestamp(object, path, "timestamp")?,
            edited_timestamp: cast::optional_timestamp(object, path, "edited_timestamp")?,
            tts: cast::optional_bool(object, path, "tts", false)?,
            mention_everyone: cast::optional_bool(object, path, "mention_everyone", false)?,
            pinned: cast::optional_bool(object, path, "pinned", false)?,
            message_type: cast::optional_enum(
                object,
                path,
                "type",
                "a message type",
                MessageType::from_value,
            )?
            .unwrap_or_default(),
            flags: MessageFlags::from_bits(cast::optional_u64(object, path, "flags")?.unwrap_or(0)),
            mentions,
            mention_role_ids: cast::optional_snowflake_array(object, path, "mention_roles")?
                .unwrap_or_default(),
            mention_channel_ids: mention_channels,
            attachments: cast::optional_array_of(
                object,
                path,
                "attachments",
                Attachment::from_value_at,
            )?
            .unwrap_or_default(),
            embeds: cast::optional_array_of(object, path, "embeds", Embed::from_value_at)?
                .unwrap_or_default(),
            sticker_items: cast::optional_array_of(
                object,
                path,
                "sticker_items",
                StickerItem::from_value_at,
            )?
            .unwrap_or_default(),
            components: cast::optional_array_of(
                object,
                path,
                "components",
                ActionRow::from_value_at,
            )?
            .unwrap_or_default(),
            poll,
            reactions: cast::optional_array_of(object, path, "reactions", Reaction::from_value_at)?
                .unwrap_or_default(),
            reference,
            webhook_id: cast::optional_snowflake(object, path, "webhook_id")?,
            application_id: cast::optional_snowflake(object, path, "application_id")?,
            interaction,
            interaction_metadata,
            position: cast::optional_i64(object, path, "position")?,
            nonce,
        })
    }

    /// Sets the ID of the guild containing the message.
    #[must_use]
    pub fn with_guild_id(mut self, guild_id: impl Into<Snowflake>) -> Self {
        self.guild_id = Some(guild_id.into());
        self
    }

    /// Sets the message's text content.
    #[must_use]
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    /// Sets the message timestamp.
    #[must_use]
    pub fn with_timestamp(mut self, timestamp: SystemTime) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Sets when the message was last edited.
    #[must_use]
    pub fn with_edited_timestamp(mut self, timestamp: SystemTime) -> Self {
        self.edited_timestamp = Some(timestamp);
        self
    }

    /// Sets the text-to-speech state.
    #[must_use]
    pub fn with_tts(mut self, tts: bool) -> Self {
        self.tts = tts;
        self
    }

    /// Sets whether the message mentions everyone.
    #[must_use]
    pub fn with_mention_everyone(mut self, mention_everyone: bool) -> Self {
        self.mention_everyone = mention_everyone;
        self
    }

    /// Sets the pinned state.
    #[must_use]
    pub fn with_pinned(mut self, pinned: bool) -> Self {
        self.pinned = pinned;
        self
    }

    /// Sets the message type.
    #[must_use]
    pub fn with_message_type(mut self, message_type: MessageType) -> Self {
        self.message_type = message_type;
        self
    }

    /// Sets the message flags.
    #[must_use]
    pub fn with_flags(mut self, flags: MessageFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Sets the mentioned users or members.
    #[must_use]
    pub fn with_mentions(mut self, mentions: Vec<MessageAuthor>) -> Self {
        self.mentions = mentions;
        self
    }

    /// Sets the mentioned role IDs.
    #[must_use]
    pub fn with_mention_role_ids(mut self, role_ids: Vec<Snowflake>) -> Self {
        self.mention_role_ids = role_ids;
        self
    }

    /// Sets the mentioned channel IDs.
    #[must_use]
    pub fn with_mention_channel_ids(mut self, channel_ids: Vec<Snowflake>) -> Self {
        self.mention_channel_ids = channel_ids;
        self
    }

    /// Sets the message's attachments.
    #[must_use]
    pub fn with_attachments(mut self, attachments: Vec<Attachment>) -> Self {
        self.attachments = attachments;
        self
    }

    /// Sets the message's embeds.
    #[must_use]
    pub fn with_embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.embeds = embeds;
        self
    }

    /// Sets the message's sticker items.
    #[must_use]
    pub fn with_sticker_items(mut self, sticker_items: Vec<StickerItem>) -> Self {
        self.sticker_items = sticker_items;
        self
    }

    /// Sets the message's component rows.
    #[must_use]
    pub fn with_components(mut self, components: Vec<ActionRow>) -> Self {
        self.components = components;
        self
    }

    /// Sets the message's poll.
    #[must_use]
    pub fn with_poll(mut self, poll: Poll) -> Self {
        self.poll = Some(poll);
        self
    }

    /// Sets the message's reactions.
    #[must_use]
    pub fn with_reactions(mut self, reactions: Vec<Reaction>) -> Self {
        self.reactions = reactions;
        self
    }

    /// Returns the reaction for an emoji, if one exists.
    #[must_use]
    pub fn reaction(&self, emoji: &crate::PartialEmoji) -> Option<&Reaction> {
        self.reactions
            .iter()
            .find(|reaction| &reaction.emoji == emoji)
    }

    /// Sets the message reference.
    #[must_use]
    pub fn with_reference(mut self, reference: MessageReference) -> Self {
        self.reference = Some(reference);
        self
    }

    /// Sets the webhook ID.
    #[must_use]
    pub fn with_webhook_id(mut self, webhook_id: impl Into<Snowflake>) -> Self {
        self.webhook_id = Some(webhook_id.into());
        self
    }

    /// Sets the application ID.
    #[must_use]
    pub fn with_application_id(mut self, application_id: impl Into<Snowflake>) -> Self {
        self.application_id = Some(application_id.into());
        self
    }

    /// Sets the interaction that this message is a response to.
    #[must_use]
    pub fn with_interaction(mut self, interaction: MessageInteraction) -> Self {
        self.interaction = Some(interaction);
        self
    }

    /// Sets detailed interaction metadata for this message.
    #[must_use]
    pub fn with_interaction_metadata(mut self, metadata: MessageInteractionMetadata) -> Self {
        self.interaction_metadata = Some(metadata);
        self
    }

    /// Sets the message's approximate position in a thread.
    #[must_use]
    pub fn with_position(mut self, position: i64) -> Self {
        self.position = Some(position);
        self
    }

    /// Sets the nonce used when sending the message.
    #[must_use]
    pub fn with_nonce(mut self, nonce: impl Into<String>) -> Self {
        self.nonce = Some(nonce.into());
        self
    }

    /// Returns the message's type.
    #[must_use]
    pub const fn message_type(&self) -> MessageType {
        self.message_type
    }

    /// Returns the message's author ID.
    #[must_use]
    pub fn author_id(&self) -> Snowflake {
        self.author.id()
    }

    /// Returns the message's creation time based on its ID.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.id.created_at()
    }

    /// Returns when the message was last edited, if it was edited.
    #[must_use]
    pub const fn edited_at(&self) -> Option<SystemTime> {
        self.edited_timestamp
    }

    /// Returns whether the message was generated by Discord.
    #[must_use]
    pub const fn is_system(&self) -> bool {
        self.message_type.is_system()
    }

    /// Returns whether the message can be deleted by a client.
    #[must_use]
    pub const fn is_deletable(&self) -> bool {
        self.message_type.is_deletable()
    }

    /// Returns whether the message is pinned.
    #[must_use]
    pub const fn is_pinned(&self) -> bool {
        self.pinned
    }

    /// Returns whether the message was sent by a webhook.
    #[must_use]
    pub const fn is_webhook(&self) -> bool {
        self.webhook_id.is_some()
    }

    /// Returns whether the message was created by an application.
    #[must_use]
    pub const fn is_application(&self) -> bool {
        self.application_id.is_some()
    }

    /// Returns a URL that opens the message in Discord.
    #[must_use]
    pub fn jump_url(&self) -> String {
        let guild_id = self
            .guild_id
            .map_or_else(|| "@me".to_owned(), |id| id.to_string());
        format!(
            "https://discord.com/channels/{}/{}/{}",
            guild_id, self.channel_id, self.id
        )
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Message {}

impl Hash for Message {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for Message {
    fn id(&self) -> Snowflake {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use abc::User as _;
    use abc::{HasId, Snowflake};
    use data::json;
    use enums::{
        AttachmentFlags, ButtonStyle, InteractionType, MessageFlags, MessageReferenceType,
        MessageType,
    };

    use super::{Attachment, Message, MessageAuthor, MessageReference};
    use crate::{
        ActionRow, Button, Embed, Member, MessageInteraction, MessageInteractionMetadata,
        PartialEmoji, Poll, PollAnswer, Reaction, StickerItem, User,
    };

    #[test]
    fn attachment_builders_and_helpers() {
        let attachment = Attachment::new(
            Snowflake::new(1),
            "SPOILER_image.png",
            2048,
            "https://cdn.example/image.png",
            "https://proxy.example/image.png",
        )
        .with_dimensions(640, 480)
        .with_content_type("image/png")
        .with_description("An image")
        .with_ephemeral(true)
        .with_title("Image")
        .with_flags(AttachmentFlags::SPOILER);

        assert_eq!(attachment.id, Snowflake::new(1));
        assert_eq!(attachment.id(), Snowflake::new(1));
        assert_eq!(attachment.filename, "SPOILER_image.png");
        assert_eq!(attachment.size, 2048);
        assert_eq!(attachment.width, Some(640));
        assert_eq!(attachment.height, Some(480));
        assert_eq!(attachment.content_type.as_deref(), Some("image/png"));
        assert_eq!(attachment.description.as_deref(), Some("An image"));
        assert!(attachment.ephemeral);
        assert_eq!(attachment.title.as_deref(), Some("Image"));
        assert!(attachment.is_spoiler());
        assert!(!attachment.is_voice_message());
        assert_eq!(attachment.to_string(), "https://cdn.example/image.png");

        let voice = Attachment::new(2_u64, "voice.ogg", 10, "url", "proxy")
            .with_voice_data(1.5, vec![1, 2, 3]);
        assert!(voice.is_voice_message());
        assert_eq!(voice.duration, Some(1.5));
        assert_eq!(voice.waveform, Some(vec![1, 2, 3]));

        let spoiler_by_flag = Attachment::new(3_u64, "image.png", 10, "url", "proxy")
            .with_flags(AttachmentFlags::SPOILER);
        assert!(spoiler_by_flag.is_spoiler());
    }

    #[test]
    fn attachment_equality_uses_id() {
        let a = Attachment::new(1_u64, "a.png", 1, "url-a", "proxy-a");
        let b = Attachment::new(1_u64, "b.png", 2, "url-b", "proxy-b");
        let c = Attachment::new(2_u64, "a.png", 1, "url-a", "proxy-a");

        assert_eq!(a, b);
        assert_ne!(a, c);

        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        a.hash(&mut hasher_a);
        b.hash(&mut hasher_b);
        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }

    #[test]
    fn message_author_delegates_user_and_member() {
        let user = User::new(1_u64, "winter").with_global_name("Winter");
        let author = MessageAuthor::from(user.clone());

        assert_eq!(author.id(), Snowflake::new(1));
        assert_eq!(author.id(), Snowflake::new(1));
        assert_eq!(abc::User::name(&author), "winter");
        assert_eq!(abc::User::display_name(&author), "Winter");
        assert_eq!(abc::User::mention(&author), "<@1>");
        assert!(!author.is_member());
        assert!(author.as_member().is_none());
        assert_eq!(author.user(), &user);
        assert_eq!(author.to_string(), "Winter");

        let member = Member::new(user, 100_u64).with_nick("Snow");
        let author = MessageAuthor::from_member(member);
        assert_eq!(author.id(), Snowflake::new(1));
        assert_eq!(abc::User::name(&author), "winter");
        assert_eq!(abc::User::display_name(&author), "Snow");
        assert!(author.is_member());
        assert_eq!(
            author.as_member().map(|member| member.nick.as_deref()),
            Some(Some("Snow"))
        );
        assert_eq!(author.to_string(), "Snow");
    }

    #[test]
    fn message_author_equality_uses_id() {
        let user = User::new(1_u64, "winter");
        let member = Member::new(user.clone(), 100_u64).with_nick("Snow");
        let author_user = MessageAuthor::from(user);
        let author_member = MessageAuthor::from(member);

        assert_eq!(author_user, author_member);
        assert_ne!(author_user, MessageAuthor::from(User::new(2_u64, "other")));
    }

    #[test]
    fn message_reference_builders() {
        let reference = MessageReference::new(10_u64, 20_u64)
            .with_guild_id(30_u64)
            .with_kind(MessageReferenceType::Forward)
            .with_fail_if_not_exists(false);

        assert_eq!(reference.message_id, Some(Snowflake::new(10)));
        assert_eq!(reference.channel_id, Snowflake::new(20));
        assert_eq!(reference.guild_id, Some(Snowflake::new(30)));
        assert_eq!(reference.kind, MessageReferenceType::Forward);
        assert!(!reference.fail_if_not_exists);

        let without_message = MessageReference::without_message(20_u64);
        assert_eq!(without_message.message_id, None);
        assert_eq!(without_message.channel_id, Snowflake::new(20));
        assert_eq!(without_message.kind, MessageReferenceType::Reply);
        assert!(without_message.fail_if_not_exists);
    }

    #[test]
    fn message_builders_and_helpers() {
        let author = User::new(1_u64, "winter");
        let attachment = Attachment::new(5_u64, "file.txt", 10, "url", "proxy");
        let mention = User::new(2_u64, "mentioned");
        let reference = MessageReference::new(9_u64, 10_u64);
        let timestamp = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);

        let message = Message::new(10_u64, 20_u64, author)
            .with_guild_id(100_u64)
            .with_content("Hello, world!")
            .with_timestamp(timestamp)
            .with_edited_timestamp(timestamp + Duration::from_secs(60))
            .with_tts(true)
            .with_mention_everyone(true)
            .with_pinned(true)
            .with_message_type(MessageType::Reply)
            .with_flags(MessageFlags::EPHEMERAL | MessageFlags::HAS_THREAD)
            .with_mentions(vec![MessageAuthor::from(mention)])
            .with_mention_role_ids(vec![Snowflake::new(30)])
            .with_mention_channel_ids(vec![Snowflake::new(40)])
            .with_attachments(vec![attachment])
            .with_reference(reference)
            .with_webhook_id(50_u64)
            .with_application_id(60_u64)
            .with_position(7)
            .with_nonce("nonce");

        assert_eq!(message.id, Snowflake::new(10));
        assert_eq!(message.id(), Snowflake::new(10));
        assert_eq!(message.channel_id, Snowflake::new(20));
        assert_eq!(message.guild_id, Some(Snowflake::new(100)));
        assert_eq!(message.author_id(), Snowflake::new(1));
        assert_eq!(message.content, "Hello, world!");
        assert_eq!(message.timestamp, timestamp);
        assert_eq!(
            message.edited_at(),
            Some(timestamp + Duration::from_secs(60))
        );
        assert!(message.tts);
        assert!(message.mention_everyone);
        assert!(message.is_pinned());
        assert_eq!(message.message_type(), MessageType::Reply);
        assert!(!message.is_system());
        assert!(message.is_deletable());
        assert!(message.flags.ephemeral());
        assert!(message.flags.has_thread());
        assert_eq!(message.mentions.len(), 1);
        assert_eq!(message.mention_role_ids, vec![Snowflake::new(30)]);
        assert_eq!(message.mention_channel_ids, vec![Snowflake::new(40)]);
        assert_eq!(message.attachments.len(), 1);
        assert!(message.reference.is_some());
        assert!(message.is_webhook());
        assert!(message.is_application());
        assert_eq!(message.position, Some(7));
        assert_eq!(message.nonce.as_deref(), Some("nonce"));
        assert_eq!(message.created_at(), Snowflake::new(10).created_at());
        assert_eq!(message.jump_url(), "https://discord.com/channels/100/20/10");
    }

    #[test]
    fn message_reactions_are_queryable() {
        let emoji = PartialEmoji::unicode("🎉");
        let reaction = Reaction::new(emoji.clone()).with_count(3);
        let message =
            Message::new(10_u64, 20_u64, User::new(1_u64, "winter")).with_reactions(vec![reaction]);

        assert_eq!(message.reactions.len(), 1);
        assert_eq!(
            message.reaction(&emoji).map(|reaction| reaction.count),
            Some(3)
        );
        assert!(message.reaction(&PartialEmoji::unicode("🎊")).is_none());
    }

    #[test]
    fn message_embeds_are_stored() {
        let embed = Embed::new()
            .with_title("Title")
            .with_description("Description");
        let message =
            Message::new(10_u64, 20_u64, User::new(1_u64, "winter")).with_embeds(vec![embed]);

        assert_eq!(message.embeds.len(), 1);
        assert_eq!(message.embeds[0].title.as_deref(), Some("Title"));
    }

    #[test]
    fn message_sticker_items_are_stored() {
        let item = StickerItem::new(30_u64, "wave");
        let message =
            Message::new(10_u64, 20_u64, User::new(1_u64, "winter")).with_sticker_items(vec![item]);

        assert_eq!(message.sticker_items.len(), 1);
        assert_eq!(message.sticker_items[0].name, "wave");
    }

    #[test]
    fn message_components_are_stored() {
        let row = ActionRow::new(vec![
            Button::new(ButtonStyle::Primary)
                .with_custom_id("confirm")
                .into(),
        ]);
        let message =
            Message::new(10_u64, 20_u64, User::new(1_u64, "winter")).with_components(vec![row]);

        assert_eq!(message.components.len(), 1);
        assert_eq!(message.components[0].buttons().count(), 1);
    }

    #[test]
    fn message_poll_is_stored() {
        let poll = Poll::new("Favourite?").add_answer(PollAnswer::new(1, "Yes"));
        let message = Message::new(10_u64, 20_u64, User::new(1_u64, "winter")).with_poll(poll);

        assert_eq!(
            message.poll.as_ref().and_then(Poll::question_text),
            Some("Favourite?")
        );
        assert_eq!(
            message.poll.as_ref().map(|poll| poll.answers().len()),
            Some(1)
        );
    }

    #[test]
    fn message_interactions_are_stored() {
        let user = User::new(1_u64, "winter");
        let interaction = MessageInteraction::new(
            30_u64,
            InteractionType::ApplicationCommand,
            "greet",
            user.clone(),
        );
        let metadata =
            MessageInteractionMetadata::new(31_u64, InteractionType::ApplicationCommand, user);
        let message = Message::new(10_u64, 20_u64, User::new(2_u64, "author"))
            .with_interaction(interaction)
            .with_interaction_metadata(metadata);

        assert_eq!(
            message.interaction.as_ref().map(|value| value.id),
            Some(Snowflake::new(30))
        );
        assert_eq!(
            message.interaction_metadata.as_ref().map(|value| value.id),
            Some(Snowflake::new(31))
        );
    }

    #[test]
    fn system_messages_and_direct_message_urls() {
        let message = Message::new(10_u64, 20_u64, User::new(1_u64, "winter"))
            .with_message_type(MessageType::PinsAdd);

        assert!(message.is_system());
        assert_eq!(message.jump_url(), "https://discord.com/channels/@me/20/10");

        let uneditable = Message::new(10_u64, 20_u64, User::new(1_u64, "winter"))
            .with_message_type(MessageType::ThreadStarterMessage);
        assert!(!uneditable.is_deletable());
        assert!(!uneditable.is_system());
    }

    #[test]
    fn message_equality_uses_id() {
        let author = User::new(1_u64, "winter");
        let a = Message::new(10_u64, 20_u64, author.clone()).with_content("First");
        let b = Message::new(10_u64, 20_u64, author).with_content("Second");
        let c = Message::new(11_u64, 20_u64, User::new(1_u64, "winter"));

        assert_eq!(a, b);
        assert_ne!(a, c);

        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        a.hash(&mut hasher_a);
        b.hash(&mut hasher_b);
        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }

    #[test]
    fn decodes_attachments_and_references() {
        let attachment = Attachment::from_value(
            &json::from_str(
                r#"{
                    "id": "1",
                    "filename": "SPOILER_voice.ogg",
                    "size": 2048,
                    "url": "https://cdn.example/voice.ogg",
                    "proxy_url": "https://proxy.example/voice.ogg",
                    "height": 100,
                    "width": 200,
                    "content_type": "audio/ogg",
                    "description": "A voice message",
                    "ephemeral": true,
                    "duration_secs": 2.5,
                    "waveform": "AQID",
                    "title": "Voice",
                    "flags": 8
                }"#,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(attachment.id, Snowflake::new(1));
        assert_eq!(attachment.filename, "SPOILER_voice.ogg");
        assert_eq!(attachment.size, 2048);
        assert_eq!(attachment.height, Some(100));
        assert_eq!(attachment.width, Some(200));
        assert_eq!(attachment.content_type.as_deref(), Some("audio/ogg"));
        assert!(attachment.ephemeral);
        assert_eq!(attachment.duration, Some(2.5));
        assert_eq!(attachment.waveform, Some(vec![1, 2, 3]));
        assert_eq!(attachment.title.as_deref(), Some("Voice"));
        assert_eq!(attachment.flags, AttachmentFlags::SPOILER);
        assert!(attachment.is_spoiler());
        assert!(attachment.is_voice_message());

        let bad_waveform = Attachment::from_value(
            &json::from_str(
                r#"{"id":"1","filename":"a","size":1,"url":"u","proxy_url":"p","waveform":"!"}"#,
            )
            .unwrap(),
        )
        .unwrap_err();
        assert_eq!(
            bad_waveform.to_string(),
            "field `attachment.waveform` must be a base64 string"
        );

        let empty_waveform = Attachment::from_value(
            &json::from_str(
                r#"{"id":"1","filename":"a","size":1,"url":"u","proxy_url":"p","waveform":""}"#,
            )
            .unwrap(),
        )
        .unwrap_err();
        assert_eq!(
            empty_waveform.to_string(),
            "field `attachment.waveform` must be a base64 string"
        );

        let reference = MessageReference::from_value(
            &json::from_str(
                r#"{"type":1,"message_id":"10","channel_id":"20","guild_id":"100","fail_if_not_exists":false}"#,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(reference.kind, MessageReferenceType::Forward);
        assert_eq!(reference.message_id, Some(Snowflake::new(10)));
        assert_eq!(reference.channel_id, Snowflake::new(20));
        assert_eq!(reference.guild_id, Some(Snowflake::new(100)));
        assert!(!reference.fail_if_not_exists);

        let minimal =
            MessageReference::from_value(&json::from_str(r#"{"channel_id":"20"}"#).unwrap())
                .unwrap();
        assert_eq!(minimal.kind, MessageReferenceType::Reply);
        assert_eq!(minimal.message_id, None);
        assert!(minimal.fail_if_not_exists);

        let missing_channel =
            MessageReference::from_value(&json::from_str(r#"{"message_id":"10"}"#).unwrap())
                .unwrap_err();
        assert_eq!(
            missing_channel.to_string(),
            "missing field `reference.channel_id`"
        );
    }

    #[test]
    fn decodes_a_full_message_payload() {
        let value = json::from_str(
            r#"{
                "id": "10",
                "channel_id": "20",
                "guild_id": "100",
                "author": {"id": "1", "username": "winter", "global_name": "Winter"},
                "member": {
                    "nick": "Frost",
                    "roles": ["10"],
                    "joined_at": "2021-08-01T12:34:56Z"
                },
                "content": "Hello <@2>",
                "timestamp": "2021-08-01T12:34:56Z",
                "edited_timestamp": "2021-08-02T00:00:00Z",
                "tts": true,
                "mention_everyone": true,
                "pinned": true,
                "type": 19,
                "flags": 64,
                "mentions": [{"id": "2", "username": "member"}],
                "mention_roles": ["10"],
                "mention_channels": [
                    {"id": "30", "guild_id": "100", "type": 0, "name": "general"}
                ],
                "attachments": [
                    {
                        "id": "3",
                        "filename": "image.png",
                        "size": 1024,
                        "url": "https://cdn.example/image.png",
                        "proxy_url": "https://proxy.example/image.png"
                    }
                ],
                "embeds": [{"title": "Embed"}],
                "sticker_items": [
                    {"id": "4", "name": "wave", "format_type": 1}
                ],
                "components": [
                    {
                        "type": 1,
                        "components": [
                            {"type": 2, "style": 1, "custom_id": "confirm", "label": "Confirm"}
                        ]
                    }
                ],
                "poll": {
                    "question": {"text": "Favourite?"},
                    "answers": [{"answer_id": 1, "poll_media": {"text": "A"}}]
                },
                "reactions": [
                    {
                        "emoji": {"name": "🎉"},
                        "count": 2,
                        "me": true,
                        "count_details": {"normal": 1, "burst": 1}
                    }
                ],
                "reference": {
                    "type": 0,
                    "message_id": "9",
                    "channel_id": "20",
                    "guild_id": "100"
                },
                "webhook_id": "40",
                "application_id": "41",
                "interaction": {
                    "id": "50",
                    "type": 2,
                    "name": "command",
                    "user": {"id": "1", "username": "winter"}
                },
                "interaction_metadata": {
                    "id": "50",
                    "type": 2,
                    "user": {"id": "1", "username": "winter"},
                    "authorizing_integration_owners": {"0": "100", "1": "1"},
                    "original_response_message_id": "51",
                    "interacted_message_id": "52",
                    "target_user": {"id": "2", "username": "target"},
                    "target_message_id": "53"
                },
                "position": 3,
                "nonce": 12345
            }"#,
        )
        .unwrap();

        let message = Message::from_value(&value).unwrap();

        assert_eq!(message.id, Snowflake::new(10));
        assert_eq!(message.channel_id, Snowflake::new(20));
        assert_eq!(message.guild_id, Some(Snowflake::new(100)));
        assert!(message.author.is_member());
        assert_eq!(message.author.id(), Snowflake::new(1));
        assert_eq!(message.author.display_name(), "Frost");
        let member = message.author.as_member().unwrap();
        assert_eq!(member.guild_id, Snowflake::new(100));
        assert_eq!(member.roles.len(), 1);
        assert_eq!(member.roles[0].id, Snowflake::new(10));
        assert_eq!(member.roles[0].name, "");

        assert_eq!(message.content, "Hello <@2>");
        assert_eq!(
            message.timestamp,
            UNIX_EPOCH + Duration::new(1_627_821_296, 0)
        );
        assert_eq!(
            message.edited_timestamp,
            Some(UNIX_EPOCH + Duration::new(1_627_862_400, 0))
        );
        assert!(message.tts);
        assert!(message.mention_everyone);
        assert!(message.pinned);
        assert_eq!(message.message_type, MessageType::Reply);
        assert_eq!(message.flags, MessageFlags::EPHEMERAL);
        assert_eq!(message.mentions.len(), 1);
        assert_eq!(message.mentions[0].id(), Snowflake::new(2));
        assert_eq!(message.mention_role_ids, [Snowflake::new(10)]);
        assert_eq!(message.mention_channel_ids, [Snowflake::new(30)]);

        assert_eq!(message.attachments.len(), 1);
        assert_eq!(message.attachments[0].filename, "image.png");
        assert_eq!(message.embeds.len(), 1);
        assert_eq!(message.embeds[0].title.as_deref(), Some("Embed"));
        assert_eq!(message.sticker_items.len(), 1);
        assert_eq!(message.sticker_items[0].id, Snowflake::new(4));
        assert_eq!(message.components.len(), 1);
        assert_eq!(message.components[0].children.len(), 1);

        let poll = message.poll.as_ref().unwrap();
        assert_eq!(poll.question_text(), Some("Favourite?"));

        assert_eq!(message.reactions.len(), 1);
        assert_eq!(message.reactions[0].count, 2);
        assert!(message.reactions[0].me);

        let reference = message.reference.as_ref().unwrap();
        assert_eq!(reference.kind, MessageReferenceType::Reply);
        assert_eq!(reference.message_id, Some(Snowflake::new(9)));
        assert_eq!(message.webhook_id, Some(Snowflake::new(40)));
        assert_eq!(message.application_id, Some(Snowflake::new(41)));

        let interaction = message.interaction.as_ref().unwrap();
        assert_eq!(interaction.id, Snowflake::new(50));
        assert_eq!(interaction.kind, InteractionType::ApplicationCommand);
        assert_eq!(interaction.name, "command");

        let metadata = message.interaction_metadata.as_ref().unwrap();
        assert_eq!(metadata.id, Snowflake::new(50));
        assert_eq!(metadata.authorizing_integration_owners.len(), 2);
        assert_eq!(
            metadata.original_response_message_id,
            Some(Snowflake::new(51))
        );
        assert_eq!(metadata.interacted_message_id, Some(Snowflake::new(52)));
        assert_eq!(
            metadata.target_user.as_ref().map(|user| user.id),
            Some(Snowflake::new(2))
        );
        assert_eq!(metadata.target_message_id, Some(Snowflake::new(53)));

        assert_eq!(message.position, Some(3));
        assert_eq!(message.nonce.as_deref(), Some("12345"));
    }

    #[test]
    fn decodes_minimal_messages_and_reports_errors() {
        let minimal = Message::from_value(
            &json::from_str(
                r#"{
                    "id": "10",
                    "channel_id": "20",
                    "author": {"id": "1", "username": "winter"},
                    "timestamp": "2021-08-01T12:34:56Z"
                }"#,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(minimal.guild_id, None);
        assert!(!minimal.author.is_member());
        assert_eq!(minimal.content, "");
        assert_eq!(minimal.message_type, MessageType::Default);
        assert!(minimal.flags.is_empty());
        assert!(minimal.attachments.is_empty());
        assert!(minimal.embeds.is_empty());
        assert!(minimal.poll.is_none());
        assert!(minimal.reference.is_none());
        assert!(minimal.interaction.is_none());
        assert_eq!(minimal.position, None);
        assert_eq!(minimal.nonce, None);

        let string_nonce = Message::from_value(
            &json::from_str(
                r#"{"id":"10","channel_id":"20","author":{"id":"1","username":"winter"},"timestamp":"2021-08-01T12:34:56Z","nonce":"abc"}"#,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(string_nonce.nonce.as_deref(), Some("abc"));

        let missing_author = Message::from_value(
            &json::from_str(r#"{"id":"10","channel_id":"20","timestamp":"2021-08-01T12:34:56Z"}"#)
                .unwrap(),
        )
        .unwrap_err();
        assert_eq!(missing_author.to_string(), "missing field `message.author`");

        let bad_author = Message::from_value(
            &json::from_str(
                r#"{"id":"10","channel_id":"20","author":"winter","timestamp":"2021-08-01T12:34:56Z"}"#,
            )
            .unwrap(),
        )
        .unwrap_err();
        assert_eq!(
            bad_author.to_string(),
            "field `message.author` must be an object"
        );

        let bad_timestamp = Message::from_value(
            &json::from_str(
                r#"{"id":"10","channel_id":"20","author":{"id":"1","username":"winter"},"timestamp":"today"}"#,
            )
            .unwrap(),
        )
        .unwrap_err();
        assert_eq!(
            bad_timestamp.to_string(),
            "field `message.timestamp` must be a valid ISO 8601 timestamp"
        );

        let bad_nonce = Message::from_value(
            &json::from_str(
                r#"{"id":"10","channel_id":"20","author":{"id":"1","username":"winter"},"timestamp":"2021-08-01T12:34:56Z","nonce":true}"#,
            )
            .unwrap(),
        )
        .unwrap_err();
        assert_eq!(
            bad_nonce.to_string(),
            "field `message.nonce` must be a string or number"
        );

        let bad_attachment = Message::from_value(
            &json::from_str(
                r#"{"id":"10","channel_id":"20","author":{"id":"1","username":"winter"},"timestamp":"2021-08-01T12:34:56Z","attachments":[{"id":"3"}]}"#,
            )
            .unwrap(),
        )
        .unwrap_err();
        assert_eq!(
            bad_attachment.to_string(),
            "missing field `message.attachments[0].filename`"
        );

        let bad_channel_mention = Message::from_value(
            &json::from_str(
                r#"{"id":"10","channel_id":"20","author":{"id":"1","username":"winter"},"timestamp":"2021-08-01T12:34:56Z","mention_channels":[{"guild_id":"100"}]}"#,
            )
            .unwrap(),
        )
        .unwrap_err();
        assert_eq!(
            bad_channel_mention.to_string(),
            "missing field `message.mention_channels[0].id`"
        );
    }
}
