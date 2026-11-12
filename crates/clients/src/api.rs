//! Small endpoint builders for commonly used Discord resources.

use except::HttpError;

use crate::{HttpClient, HttpResponse, Transport};
use data::Value;

/// REST operations for core Discord resources.
#[derive(Clone, Debug)]
pub struct DiscordApi<T> {
    client: HttpClient<T>,
}

impl<T> DiscordApi<T> {
    /// Creates an API facade from an authenticated HTTP client.
    #[must_use]
    pub const fn new(client: HttpClient<T>) -> Self {
        Self { client }
    }

    /// Returns the underlying HTTP client.
    #[must_use]
    pub const fn client(&self) -> &HttpClient<T> {
        &self.client
    }

    /// Returns the underlying HTTP client mutably.
    pub fn client_mut(&mut self) -> &mut HttpClient<T> {
        &mut self.client
    }

    /// Consumes the facade and returns its HTTP client.
    #[must_use]
    pub fn into_client(self) -> HttpClient<T> {
        self.client
    }
}

impl<T: Transport> DiscordApi<T> {
    /// Fetches a user.
    pub fn user(&self, user_id: u64) -> Result<HttpResponse, HttpError> {
        self.client.execute_get(format!("users/{user_id}"))
    }

    /// Fetches a guild.
    pub fn guild(&self, guild_id: u64) -> Result<HttpResponse, HttpError> {
        self.client.execute_get(format!("guilds/{guild_id}"))
    }

    /// Fetches a channel.
    pub fn channel(&self, channel_id: u64) -> Result<HttpResponse, HttpError> {
        self.client.execute_get(format!("channels/{channel_id}"))
    }

    /// Fetches messages from a channel.
    pub fn messages(&self, channel_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("channels/{channel_id}/messages"))
    }

    /// Creates a message using an already encoded JSON body.
    pub fn create_message(
        &self,
        channel_id: u64,
        body: impl Into<Vec<u8>>,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute(
            self.client
                .post(format!("channels/{channel_id}/messages"))
                .with_body(body),
        )
    }

    /// Deletes a message.
    pub fn delete_message(
        &self,
        channel_id: u64,
        message_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_delete(format!("channels/{channel_id}/messages/{message_id}"))
    }

    /// Fetches a guild member.
    pub fn member(&self, guild_id: u64, user_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("guilds/{guild_id}/members/{user_id}"))
    }

    /// Fetches guild roles.
    pub fn roles(&self, guild_id: u64) -> Result<HttpResponse, HttpError> {
        self.client.execute_get(format!("guilds/{guild_id}/roles"))
    }

    /// Adds a reaction to a message.
    pub fn add_reaction(
        &self,
        channel_id: u64,
        message_id: u64,
        emoji: &str,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_put(format!(
            "channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me"
        ))
    }

    /// Creates a guild channel.
    pub fn create_channel(
        &self,
        guild_id: u64,
        body: impl Into<Vec<u8>>,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute(
            self.client
                .post(format!("guilds/{guild_id}/channels"))
                .with_body(body),
        )
    }

    /// Sends an interaction callback.
    pub fn interaction_callback(
        &self,
        interaction_id: u64,
        token: &str,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client
                .post(format!("interactions/{interaction_id}/{token}/callback")),
            body,
        )
    }

    /// Sends a followup interaction message.
    pub fn interaction_followup(
        &self,
        application_id: u64,
        token: &str,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_json(self.client.post(format!("webhooks/{application_id}/{token}")), body)
    }
}
