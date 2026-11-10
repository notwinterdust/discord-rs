//! Webhook models.
//!
//! Webhooks allow messages to be posted to a channel without a bot user.
//! Channel follower webhooks additionally carry a partial source guild and
//! channel. When a webhook sends a message, [`WebhookMessage`] records the
//! message together with the thread it was sent in, if any.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::time::SystemTime;

use abc::{Asset, HasId, Snowflake};
use enums::WebhookType;

use crate::{Message, User};

const WEBHOOK_API_BASE: &str = "https://discord.com/api/webhooks";

/// The partial channel data attached to channel follower webhooks.
///
/// Equality and hashing are based on the channel's ID.
#[derive(Clone, Debug, Default)]
pub struct PartialWebhookChannel {
    /// The channel's unique ID.
    pub id: Snowflake,
    /// The channel's name.
    pub name: String,
}

impl PartialWebhookChannel {
    /// Creates a partial webhook channel.
    #[must_use]
    pub fn new(id: impl Into<Snowflake>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }

    /// Returns a string that mentions the channel.
    #[must_use]
    pub fn mention(&self) -> String {
        format!("<#{}>", self.id)
    }
}

impl fmt::Display for PartialWebhookChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl PartialEq for PartialWebhookChannel {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PartialWebhookChannel {}

impl Hash for PartialWebhookChannel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for PartialWebhookChannel {
    fn id(&self) -> Snowflake {
        self.id
    }
}

/// The partial guild data attached to channel follower webhooks.
///
/// Equality and hashing are based on the guild's ID.
#[derive(Clone, Debug, Default)]
pub struct PartialWebhookGuild {
    /// The guild's unique ID.
    pub id: Snowflake,
    /// The guild's name.
    pub name: String,
    /// The guild's icon, if it has one.
    pub icon: Option<Asset>,
}

impl PartialWebhookGuild {
    /// Creates a partial webhook guild.
    #[must_use]
    pub fn new(id: impl Into<Snowflake>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            icon: None,
        }
    }

    /// Sets the guild's icon from its icon hash.
    #[must_use]
    pub fn with_icon_hash(mut self, icon_hash: &str) -> Self {
        self.icon = Some(Asset::from_guild_icon(self.id, icon_hash));
        self
    }

    /// Sets the guild's icon.
    #[must_use]
    pub fn with_icon(mut self, icon: Asset) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Returns the guild's icon URL, if it has one.
    #[must_use]
    pub fn icon_url(&self) -> Option<&str> {
        self.icon.as_ref().map(Asset::url)
    }
}

impl fmt::Display for PartialWebhookGuild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl PartialEq for PartialWebhookGuild {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PartialWebhookGuild {}

impl Hash for PartialWebhookGuild {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for PartialWebhookGuild {
    fn id(&self) -> Snowflake {
        self.id
    }
}

/// A Discord webhook.
///
/// A webhook with no [`Webhook::channel_id`] is partial: it only identifies the
/// webhook and usually carries its token. Equality and hashing are based on the
/// webhook's ID.
#[derive(Clone, Debug, Default)]
pub struct Webhook {
    /// The webhook's unique ID.
    pub id: Snowflake,
    /// The webhook's type.
    pub kind: WebhookType,
    /// The ID of the guild the webhook belongs to, if any.
    pub guild_id: Option<Snowflake>,
    /// The ID of the channel the webhook posts to, if any.
    pub channel_id: Option<Snowflake>,
    /// The webhook's token, used to execute it.
    pub token: Option<String>,
    /// A bot token used to authenticate requests that manage the webhook.
    pub auth_token: Option<String>,
    /// The user that created the webhook, if known.
    pub user: Option<User>,
    /// The webhook's default name.
    pub name: Option<String>,
    /// The webhook's avatar, if it has one.
    pub avatar: Option<Asset>,
    /// The ID of the application that owns the webhook, if any.
    pub application_id: Option<Snowflake>,
    /// The guild followed by a channel follower webhook, if any.
    pub source_guild: Option<PartialWebhookGuild>,
    /// The channel followed by a channel follower webhook, if any.
    pub source_channel: Option<PartialWebhookChannel>,
}

impl Webhook {
    /// Creates an incoming webhook with only its ID set.
    #[must_use]
    pub fn new(id: impl Into<Snowflake>) -> Self {
        Self {
            id: id.into(),
            ..Self::default()
        }
    }

    /// Creates a partial webhook from its ID and token.
    #[must_use]
    pub fn partial(id: impl Into<Snowflake>, token: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            token: Some(token.into()),
            ..Self::default()
        }
    }

    /// Parses a partial webhook from a webhook URL.
    ///
    /// The URL must contain a `webhooks` path segment followed by the webhook
    /// ID and token. Returns `None` if the URL does not contain a valid
    /// ID/token pair.
    #[must_use]
    pub fn from_url(url: &str) -> Option<Self> {
        let mut parts = url.trim_end_matches('/').rsplit('/');
        let token = parts.next()?;
        let id = parts.next()?;
        let marker = parts.next()?;

        if marker != "webhooks" || token.is_empty() {
            return None;
        }

        Some(Self::partial(id.parse::<u64>().ok()?, token))
    }

    /// Sets the webhook's type.
    #[must_use]
    pub fn with_type(mut self, kind: WebhookType) -> Self {
        self.kind = kind;
        self
    }

    /// Sets the guild the webhook belongs to.
    #[must_use]
    pub fn with_guild_id(mut self, guild_id: impl Into<Snowflake>) -> Self {
        self.guild_id = Some(guild_id.into());
        self
    }

    /// Sets the channel the webhook posts to.
    #[must_use]
    pub fn with_channel_id(mut self, channel_id: impl Into<Snowflake>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    /// Sets the webhook's execution token.
    #[must_use]
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Sets the bot token used to manage the webhook.
    #[must_use]
    pub fn with_auth_token(mut self, auth_token: impl Into<String>) -> Self {
        self.auth_token = Some(auth_token.into());
        self
    }

    /// Sets the user that created the webhook.
    #[must_use]
    pub fn with_user(mut self, user: User) -> Self {
        self.user = Some(user);
        self
    }

    /// Sets the webhook's default name.
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the webhook's avatar.
    #[must_use]
    pub fn with_avatar(mut self, avatar: Asset) -> Self {
        self.avatar = Some(avatar);
        self
    }

    /// Sets the webhook's avatar from its avatar hash.
    #[must_use]
    pub fn with_avatar_hash(mut self, avatar_hash: &str) -> Self {
        self.avatar = Some(Asset::from_avatar(self.id, avatar_hash));
        self
    }

    /// Sets the application that owns the webhook.
    #[must_use]
    pub fn with_application_id(mut self, application_id: impl Into<Snowflake>) -> Self {
        self.application_id = Some(application_id.into());
        self
    }

    /// Sets the guild followed by a channel follower webhook.
    #[must_use]
    pub fn with_source_guild(mut self, source_guild: PartialWebhookGuild) -> Self {
        self.source_guild = Some(source_guild);
        self
    }

    /// Sets the channel followed by a channel follower webhook.
    #[must_use]
    pub fn with_source_channel(mut self, source_channel: PartialWebhookChannel) -> Self {
        self.source_channel = Some(source_channel);
        self
    }

    /// Returns `true` if this is an incoming webhook.
    #[must_use]
    pub const fn is_incoming(&self) -> bool {
        self.kind.is_incoming()
    }

    /// Returns `true` if this is a channel follower webhook.
    #[must_use]
    pub const fn is_channel_follower(&self) -> bool {
        self.kind.is_channel_follower()
    }

    /// Returns `true` if this is an application webhook.
    #[must_use]
    pub const fn is_application(&self) -> bool {
        self.kind.is_application()
    }

    /// Returns `true` if the webhook is partial, i.e. has no known channel.
    #[must_use]
    pub const fn is_partial(&self) -> bool {
        self.channel_id.is_none()
    }

    /// Returns `true` if the webhook has a bot token for authenticated
    /// requests.
    #[must_use]
    pub const fn is_authenticated(&self) -> bool {
        self.auth_token.is_some()
    }

    /// Returns the webhook's creation time.
    #[must_use]
    pub fn created_at(&self) -> SystemTime {
        self.id.created_at()
    }

    /// Returns the webhook's execution URL, if it has a token.
    #[must_use]
    pub fn url(&self) -> Option<String> {
        self.token
            .as_ref()
            .map(|token| format!("{WEBHOOK_API_BASE}/{}/{token}", self.id))
    }

    /// Returns the webhook's default avatar.
    #[must_use]
    pub fn default_avatar(&self) -> Asset {
        Asset::from_default_avatar(((self.id.get() >> 22) % 6) as u8)
    }

    /// Returns the webhook's avatar, falling back to its default avatar.
    #[must_use]
    pub fn display_avatar(&self) -> Asset {
        self.avatar.clone().unwrap_or_else(|| self.default_avatar())
    }
}

impl PartialEq for Webhook {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Webhook {}

impl Hash for Webhook {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HasId for Webhook {
    fn id(&self) -> Snowflake {
        self.id
    }
}

/// A message sent by a webhook.
///
/// The wrapper records the thread the message was sent in, which webhook
/// message edits and deletes need in addition to the message ID.
#[derive(Clone, Debug)]
pub struct WebhookMessage {
    /// The underlying message.
    pub message: Message,
    /// The ID of the thread the message was sent in, if any.
    pub thread_id: Option<Snowflake>,
}

impl WebhookMessage {
    /// Creates a webhook message from a message.
    #[must_use]
    pub const fn new(message: Message) -> Self {
        Self {
            message,
            thread_id: None,
        }
    }

    /// Sets the thread the message was sent in.
    #[must_use]
    pub fn with_thread_id(mut self, thread_id: impl Into<Snowflake>) -> Self {
        self.thread_id = Some(thread_id.into());
        self
    }

    /// Returns the underlying message.
    #[must_use]
    pub const fn as_message(&self) -> &Message {
        &self.message
    }

    /// Consumes the wrapper and returns the underlying message.
    #[must_use]
    pub fn into_message(self) -> Message {
        self.message
    }

    /// Returns the ID of the webhook that sent the message, if known.
    #[must_use]
    pub const fn webhook_id(&self) -> Option<Snowflake> {
        self.message.webhook_id
    }
}

impl From<Message> for WebhookMessage {
    fn from(message: Message) -> Self {
        Self::new(message)
    }
}

impl Deref for WebhookMessage {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

impl PartialEq for WebhookMessage {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message
    }
}

impl Eq for WebhookMessage {}

impl Hash for WebhookMessage {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.message.hash(state);
    }
}

impl HasId for WebhookMessage {
    fn id(&self) -> Snowflake {
        self.message.id
    }
}

#[cfg(test)]
mod tests {
    use abc::{Asset, HasId, Snowflake};
    use enums::WebhookType;

    use super::{PartialWebhookChannel, PartialWebhookGuild, Webhook, WebhookMessage};
    use crate::{Message, User};

    #[test]
    fn partial_source_models() {
        let channel = PartialWebhookChannel::new(10_u64, "announcements");
        assert_eq!(channel.id, Snowflake::new(10));
        assert_eq!(channel.id(), Snowflake::new(10));
        assert_eq!(channel.name, "announcements");
        assert_eq!(channel.mention(), "<#10>");
        assert_eq!(channel.to_string(), "announcements");
        assert_eq!(channel, PartialWebhookChannel::new(10_u64, "other"));

        let guild = PartialWebhookGuild::new(20_u64, "Source").with_icon_hash("icon_hash");
        assert_eq!(guild.id, Snowflake::new(20));
        assert_eq!(guild.id(), Snowflake::new(20));
        assert_eq!(guild.name, "Source");
        assert_eq!(
            guild.icon,
            Some(Asset::from_guild_icon(Snowflake::new(20), "icon_hash"))
        );
        assert_eq!(
            guild.icon_url(),
            Some(Asset::from_guild_icon(Snowflake::new(20), "icon_hash").url())
        );
        assert_eq!(guild.to_string(), "Source");

        let no_icon = PartialWebhookGuild::new(21_u64, "Other");
        assert!(no_icon.icon.is_none());
        assert_eq!(no_icon.icon_url(), None);
    }

    #[test]
    fn webhook_builders_and_helpers() {
        let user = User::new(1_u64, "hook-user");
        let source_guild = PartialWebhookGuild::new(20_u64, "Source");
        let source_channel = PartialWebhookChannel::new(30_u64, "source-channel");
        let avatar = Asset::from_avatar(Snowflake::new(40), "avatar_hash");
        let webhook = Webhook::new(40_u64)
            .with_type(WebhookType::ChannelFollower)
            .with_guild_id(50_u64)
            .with_channel_id(60_u64)
            .with_token("token")
            .with_auth_token("bot-token")
            .with_user(user.clone())
            .with_name("Hook")
            .with_avatar(avatar.clone())
            .with_application_id(70_u64)
            .with_source_guild(source_guild.clone())
            .with_source_channel(source_channel.clone());

        assert_eq!(webhook.id, Snowflake::new(40));
        assert_eq!(webhook.id(), Snowflake::new(40));
        assert_eq!(webhook.kind, WebhookType::ChannelFollower);
        assert!(webhook.is_channel_follower());
        assert!(!webhook.is_incoming());
        assert!(!webhook.is_application());
        assert_eq!(webhook.guild_id, Some(Snowflake::new(50)));
        assert_eq!(webhook.channel_id, Some(Snowflake::new(60)));
        assert!(!webhook.is_partial());
        assert!(webhook.is_authenticated());
        assert_eq!(webhook.user, Some(user));
        assert_eq!(webhook.name.as_deref(), Some("Hook"));
        assert_eq!(webhook.application_id, Some(Snowflake::new(70)));
        assert_eq!(webhook.source_guild, Some(source_guild));
        assert_eq!(webhook.source_channel, Some(source_channel));
        assert_eq!(webhook.created_at(), Snowflake::new(40).created_at());
        assert_eq!(
            webhook.url().as_deref(),
            Some("https://discord.com/api/webhooks/40/token")
        );
        assert_eq!(webhook.display_avatar(), avatar);

        let partial = Webhook::partial(41_u64, "partial-token");
        assert!(partial.is_partial());
        assert!(partial.is_incoming());
        assert!(!partial.is_authenticated());
        assert_eq!(
            partial.url().as_deref(),
            Some("https://discord.com/api/webhooks/41/partial-token")
        );
        assert_eq!(partial.display_avatar(), partial.default_avatar());

        let no_token = Webhook::new((2_u64 << 22) + 1);
        assert!(no_token.url().is_none());
        assert_eq!(no_token.default_avatar(), Asset::from_default_avatar(2));

        assert_eq!(Webhook::new(40_u64), webhook);
        assert_ne!(Webhook::new(41_u64), webhook);
    }

    #[test]
    fn webhook_urls_are_parsed() {
        let webhook = Webhook::from_url("https://discord.com/api/webhooks/123456789/token-abc")
            .expect("valid webhook URL");
        assert_eq!(webhook.id, Snowflake::new(123_456_789));
        assert_eq!(webhook.token.as_deref(), Some("token-abc"));
        assert!(webhook.is_partial());
        assert!(webhook.is_incoming());

        let with_trailing_slash =
            Webhook::from_url("https://discord.com/api/webhooks/9/token/").unwrap();
        assert_eq!(with_trailing_slash.id, Snowflake::new(9));
        assert_eq!(with_trailing_slash.token.as_deref(), Some("token"));

        assert!(Webhook::from_url("https://discord.com/api/webhooks/not-an-id/token").is_none());
        assert!(Webhook::from_url("https://discord.com/api/channels/1/2").is_none());
        assert!(Webhook::from_url("https://discord.com/api/webhooks/1/").is_none());
        assert!(Webhook::from_url("token").is_none());
    }

    #[test]
    fn webhook_message_wraps_message() {
        let message = Message::new(1_u64, 2_u64, User::new(3_u64, "author"))
            .with_webhook_id(4_u64)
            .with_content("hello");
        let webhook_message = WebhookMessage::new(message.clone()).with_thread_id(5_u64);

        assert_eq!(webhook_message.id, Snowflake::new(1));
        assert_eq!(webhook_message.id(), Snowflake::new(1));
        assert_eq!(webhook_message.content, "hello");
        assert_eq!(webhook_message.thread_id, Some(Snowflake::new(5)));
        assert_eq!(webhook_message.webhook_id(), Some(Snowflake::new(4)));
        assert_eq!(webhook_message.as_message(), &message);

        let from_message: WebhookMessage = message.clone().into();
        assert_eq!(from_message, WebhookMessage::new(message.clone()));
        assert_eq!(from_message.into_message(), message);
    }
}
