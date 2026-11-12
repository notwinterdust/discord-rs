//! Small endpoint builders for commonly used Discord resources.
//!
//! [`DiscordApi`] wraps an authenticated [`HttpClient`] and provides typed
//! helpers for every Discord REST endpoint group.  All methods return a raw
//! [`HttpResponse`]; callers may decode the body with [`models`] or handle it
//! themselves.

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
    // =========================================================================
    // Users / Current User
    // =========================================================================

    /// Fetches a user by ID string — pass `"@me"` for the current bot user.
    pub fn user(&self, user_id: &str) -> Result<HttpResponse, HttpError> {
        self.client.execute_get(format!("users/{user_id}"))
    }

    /// Fetches the current bot user's guild list.
    pub fn current_user_guilds(&self) -> Result<HttpResponse, HttpError> {
        self.client.execute_get("users/@me/guilds")
    }

    /// Fetches the current user's connections.
    pub fn current_user_connections(&self) -> Result<HttpResponse, HttpError> {
        self.client.execute_get("users/@me/connections")
    }

    /// Modifies the current bot user.
    pub fn modify_current_user(&self, body: &Value) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_json(self.client.patch("users/@me"), body)
    }

    /// Opens or returns an existing DM channel with the given user.
    pub fn create_dm(&self, body: &Value) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_json(self.client.post("users/@me/channels"), body)
    }

    // =========================================================================
    // Guilds
    // =========================================================================

    /// Fetches a guild.
    pub fn guild(&self, guild_id: u64) -> Result<HttpResponse, HttpError> {
        self.client.execute_get(format!("guilds/{guild_id}"))
    }

    /// Fetches guild preview (works without being a member).
    pub fn guild_preview(&self, guild_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("guilds/{guild_id}/preview"))
    }

    /// Creates a guild.
    pub fn create_guild(&self, body: &Value) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(self.client.post("guilds"), body)
    }

    /// Modifies a guild.
    pub fn modify_guild(&self, guild_id: u64, body: &Value) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_json(self.client.patch(format!("guilds/{guild_id}")), body)
    }

    /// Deletes a guild (bot must be owner).
    pub fn delete_guild(&self, guild_id: u64) -> Result<HttpResponse, HttpError> {
        self.client.execute_delete(format!("guilds/{guild_id}"))
    }

    // =========================================================================
    // Channels
    // =========================================================================

    /// Fetches a channel.
    pub fn channel(&self, channel_id: u64) -> Result<HttpResponse, HttpError> {
        self.client.execute_get(format!("channels/{channel_id}"))
    }

    /// Modifies a channel or thread.
    pub fn modify_channel(
        &self,
        channel_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_json(self.client.patch(format!("channels/{channel_id}")), body)
    }

    /// Deletes a channel or closes a thread.
    pub fn delete_channel(&self, channel_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_delete(format!("channels/{channel_id}"))
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

    /// Fetches the channels for a guild.
    pub fn guild_channels(&self, guild_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("guilds/{guild_id}/channels"))
    }

    /// Modifies guild channel positions.
    pub fn modify_guild_channel_positions(
        &self,
        guild_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client.patch(format!("guilds/{guild_id}/channels")),
            body,
        )
    }

    // =========================================================================
    // Threads
    // =========================================================================

    /// Starts a thread from an existing message.
    pub fn start_thread_from_message(
        &self,
        channel_id: u64,
        message_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client
                .post(format!("channels/{channel_id}/messages/{message_id}/threads")),
            body,
        )
    }

    /// Starts a standalone thread (forum / announcement / no message).
    pub fn start_thread(
        &self,
        channel_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client.post(format!("channels/{channel_id}/threads")),
            body,
        )
    }

    /// Adds the current user to a thread.
    pub fn join_thread(&self, channel_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_put(format!("channels/{channel_id}/thread-members/@me"))
    }

    /// Removes the current user from a thread.
    pub fn leave_thread(&self, channel_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_delete(format!("channels/{channel_id}/thread-members/@me"))
    }

    /// Adds a member to a thread.
    pub fn add_thread_member(
        &self,
        channel_id: u64,
        user_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_put(format!("channels/{channel_id}/thread-members/{user_id}"))
    }

    /// Removes a member from a thread.
    pub fn remove_thread_member(
        &self,
        channel_id: u64,
        user_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_delete(format!("channels/{channel_id}/thread-members/{user_id}"))
    }

    /// Lists members of a thread.
    pub fn list_thread_members(&self, channel_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("channels/{channel_id}/thread-members"))
    }

    /// Lists public archived threads in a channel.
    pub fn list_public_archived_threads(
        &self,
        channel_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("channels/{channel_id}/threads/archived/public"))
    }

    /// Lists private archived threads in a channel.
    pub fn list_private_archived_threads(
        &self,
        channel_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("channels/{channel_id}/threads/archived/private"))
    }

    // =========================================================================
    // Messages
    // =========================================================================

    /// Fetches messages from a channel.
    pub fn messages(&self, channel_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("channels/{channel_id}/messages"))
    }

    /// Fetches a single message.
    pub fn message(&self, channel_id: u64, message_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("channels/{channel_id}/messages/{message_id}"))
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

    /// Edits a message.
    pub fn edit_message(
        &self,
        channel_id: u64,
        message_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client
                .patch(format!("channels/{channel_id}/messages/{message_id}")),
            body,
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

    /// Bulk-deletes up to 100 messages.
    pub fn bulk_delete_messages(
        &self,
        channel_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client
                .post(format!("channels/{channel_id}/messages/bulk-delete")),
            body,
        )
    }

    /// Pins a message.
    pub fn pin_message(
        &self,
        channel_id: u64,
        message_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_put(format!("channels/{channel_id}/pins/{message_id}"))
    }

    /// Unpins a message.
    pub fn unpin_message(
        &self,
        channel_id: u64,
        message_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_delete(format!("channels/{channel_id}/pins/{message_id}"))
    }

    /// Lists pinned messages.
    pub fn pinned_messages(&self, channel_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("channels/{channel_id}/pins"))
    }

    // =========================================================================
    // Reactions
    // =========================================================================

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

    /// Removes the bot's own reaction.
    pub fn remove_own_reaction(
        &self,
        channel_id: u64,
        message_id: u64,
        emoji: &str,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_delete(format!(
            "channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me"
        ))
    }

    /// Removes another user's reaction.
    pub fn remove_user_reaction(
        &self,
        channel_id: u64,
        message_id: u64,
        emoji: &str,
        user_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_delete(format!(
            "channels/{channel_id}/messages/{message_id}/reactions/{emoji}/{user_id}"
        ))
    }

    /// Lists users who reacted with an emoji.
    pub fn reaction_users(
        &self,
        channel_id: u64,
        message_id: u64,
        emoji: &str,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_get(format!(
            "channels/{channel_id}/messages/{message_id}/reactions/{emoji}"
        ))
    }

    /// Deletes all reactions on a message.
    pub fn delete_all_reactions(
        &self,
        channel_id: u64,
        message_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_delete(format!(
            "channels/{channel_id}/messages/{message_id}/reactions"
        ))
    }

    /// Deletes all reactions for one emoji on a message.
    pub fn delete_all_reactions_for_emoji(
        &self,
        channel_id: u64,
        message_id: u64,
        emoji: &str,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_delete(format!(
            "channels/{channel_id}/messages/{message_id}/reactions/{emoji}"
        ))
    }

    // =========================================================================
    // Members
    // =========================================================================

    /// Fetches a guild member.
    pub fn member(&self, guild_id: u64, user_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("guilds/{guild_id}/members/{user_id}"))
    }

    /// Lists guild members.
    pub fn list_members(&self, guild_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("guilds/{guild_id}/members"))
    }

    /// Searches guild members by username/nickname.
    pub fn search_members(
        &self,
        guild_id: u64,
        query: &str,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_get(format!(
            "guilds/{guild_id}/members/search?query={query}"
        ))
    }

    /// Modifies a guild member.
    pub fn modify_member(
        &self,
        guild_id: u64,
        user_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client
                .patch(format!("guilds/{guild_id}/members/{user_id}")),
            body,
        )
    }

    /// Kicks a guild member.
    pub fn kick_member(&self, guild_id: u64, user_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_delete(format!("guilds/{guild_id}/members/{user_id}"))
    }

    // =========================================================================
    // Bans
    // =========================================================================

    /// Fetches the ban list for a guild.
    pub fn bans(&self, guild_id: u64) -> Result<HttpResponse, HttpError> {
        self.client.execute_get(format!("guilds/{guild_id}/bans"))
    }

    /// Fetches a specific ban entry.
    pub fn ban(&self, guild_id: u64, user_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("guilds/{guild_id}/bans/{user_id}"))
    }

    /// Bans a user from a guild.
    pub fn ban_member(
        &self,
        guild_id: u64,
        user_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client
                .put(format!("guilds/{guild_id}/bans/{user_id}")),
            body,
        )
    }

    /// Unbans a user from a guild.
    pub fn unban_member(&self, guild_id: u64, user_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_delete(format!("guilds/{guild_id}/bans/{user_id}"))
    }

    // =========================================================================
    // Roles
    // =========================================================================

    /// Fetches guild roles.
    pub fn roles(&self, guild_id: u64) -> Result<HttpResponse, HttpError> {
        self.client.execute_get(format!("guilds/{guild_id}/roles"))
    }

    /// Creates a guild role.
    pub fn create_role(&self, guild_id: u64, body: &Value) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_json(self.client.post(format!("guilds/{guild_id}/roles")), body)
    }

    /// Modifies a guild role.
    pub fn modify_role(
        &self,
        guild_id: u64,
        role_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client
                .patch(format!("guilds/{guild_id}/roles/{role_id}")),
            body,
        )
    }

    /// Deletes a guild role.
    pub fn delete_role(&self, guild_id: u64, role_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_delete(format!("guilds/{guild_id}/roles/{role_id}"))
    }

    /// Adds a role to a member.
    pub fn add_member_role(
        &self,
        guild_id: u64,
        user_id: u64,
        role_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_put(format!(
            "guilds/{guild_id}/members/{user_id}/roles/{role_id}"
        ))
    }

    /// Removes a role from a member.
    pub fn remove_member_role(
        &self,
        guild_id: u64,
        user_id: u64,
        role_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_delete(format!(
            "guilds/{guild_id}/members/{user_id}/roles/{role_id}"
        ))
    }

    // =========================================================================
    // Emojis & Stickers
    // =========================================================================

    /// Lists custom emojis for a guild.
    pub fn guild_emojis(&self, guild_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("guilds/{guild_id}/emojis"))
    }

    /// Fetches a specific guild emoji.
    pub fn guild_emoji(&self, guild_id: u64, emoji_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("guilds/{guild_id}/emojis/{emoji_id}"))
    }

    /// Creates a guild emoji.
    pub fn create_guild_emoji(
        &self,
        guild_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client.post(format!("guilds/{guild_id}/emojis")),
            body,
        )
    }

    /// Modifies a guild emoji.
    pub fn modify_guild_emoji(
        &self,
        guild_id: u64,
        emoji_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client
                .patch(format!("guilds/{guild_id}/emojis/{emoji_id}")),
            body,
        )
    }

    /// Deletes a guild emoji.
    pub fn delete_guild_emoji(
        &self,
        guild_id: u64,
        emoji_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_delete(format!("guilds/{guild_id}/emojis/{emoji_id}"))
    }

    /// Lists stickers for a guild.
    pub fn guild_stickers(&self, guild_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("guilds/{guild_id}/stickers"))
    }

    /// Fetches a specific guild sticker.
    pub fn guild_sticker(
        &self,
        guild_id: u64,
        sticker_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("guilds/{guild_id}/stickers/{sticker_id}"))
    }

    /// Modifies a guild sticker.
    pub fn modify_guild_sticker(
        &self,
        guild_id: u64,
        sticker_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client
                .patch(format!("guilds/{guild_id}/stickers/{sticker_id}")),
            body,
        )
    }

    /// Deletes a guild sticker.
    pub fn delete_guild_sticker(
        &self,
        guild_id: u64,
        sticker_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_delete(format!("guilds/{guild_id}/stickers/{sticker_id}"))
    }

    // =========================================================================
    // Scheduled Events
    // =========================================================================

    /// Lists scheduled events for a guild.
    pub fn guild_scheduled_events(&self, guild_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("guilds/{guild_id}/scheduled-events"))
    }

    /// Creates a scheduled event.
    pub fn create_scheduled_event(
        &self,
        guild_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client
                .post(format!("guilds/{guild_id}/scheduled-events")),
            body,
        )
    }

    /// Fetches a scheduled event.
    pub fn scheduled_event(
        &self,
        guild_id: u64,
        event_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("guilds/{guild_id}/scheduled-events/{event_id}"))
    }

    /// Modifies a scheduled event.
    pub fn modify_scheduled_event(
        &self,
        guild_id: u64,
        event_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client
                .patch(format!("guilds/{guild_id}/scheduled-events/{event_id}")),
            body,
        )
    }

    /// Deletes a scheduled event.
    pub fn delete_scheduled_event(
        &self,
        guild_id: u64,
        event_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_delete(format!("guilds/{guild_id}/scheduled-events/{event_id}"))
    }

    // =========================================================================
    // Stage Instances
    // =========================================================================

    /// Creates a stage instance.
    pub fn create_stage_instance(&self, body: &Value) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_json(self.client.post("stage-instances"), body)
    }

    /// Fetches the stage instance for a channel.
    pub fn stage_instance(&self, channel_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("stage-instances/{channel_id}"))
    }

    /// Modifies a stage instance.
    pub fn modify_stage_instance(
        &self,
        channel_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client.patch(format!("stage-instances/{channel_id}")),
            body,
        )
    }

    /// Deletes a stage instance.
    pub fn delete_stage_instance(&self, channel_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_delete(format!("stage-instances/{channel_id}"))
    }

    // =========================================================================
    // AutoMod
    // =========================================================================

    /// Lists AutoMod rules for a guild.
    pub fn automod_rules(&self, guild_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("guilds/{guild_id}/auto-moderation/rules"))
    }

    /// Fetches an AutoMod rule.
    pub fn automod_rule(&self, guild_id: u64, rule_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("guilds/{guild_id}/auto-moderation/rules/{rule_id}"))
    }

    /// Creates an AutoMod rule.
    pub fn create_automod_rule(
        &self,
        guild_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client
                .post(format!("guilds/{guild_id}/auto-moderation/rules")),
            body,
        )
    }

    /// Modifies an AutoMod rule.
    pub fn modify_automod_rule(
        &self,
        guild_id: u64,
        rule_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client
                .patch(format!("guilds/{guild_id}/auto-moderation/rules/{rule_id}")),
            body,
        )
    }

    /// Deletes an AutoMod rule.
    pub fn delete_automod_rule(
        &self,
        guild_id: u64,
        rule_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_delete(format!("guilds/{guild_id}/auto-moderation/rules/{rule_id}"))
    }

    // =========================================================================
    // Audit Log
    // =========================================================================

    /// Fetches the audit log for a guild.
    pub fn audit_log(&self, guild_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("guilds/{guild_id}/audit-logs"))
    }

    // =========================================================================
    // Webhooks
    // =========================================================================

    /// Fetches a webhook.
    pub fn webhook(&self, webhook_id: u64) -> Result<HttpResponse, HttpError> {
        self.client.execute_get(format!("webhooks/{webhook_id}"))
    }

    /// Fetches a webhook using its token (no auth required).
    pub fn webhook_with_token(
        &self,
        webhook_id: u64,
        token: &str,
    ) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("webhooks/{webhook_id}/{token}"))
    }

    /// Lists webhooks for a channel.
    pub fn channel_webhooks(&self, channel_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("channels/{channel_id}/webhooks"))
    }

    /// Lists webhooks for a guild.
    pub fn guild_webhooks(&self, guild_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("guilds/{guild_id}/webhooks"))
    }

    /// Creates a webhook.
    pub fn create_webhook(
        &self,
        channel_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client.post(format!("channels/{channel_id}/webhooks")),
            body,
        )
    }

    /// Modifies a webhook.
    pub fn modify_webhook(
        &self,
        webhook_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client.patch(format!("webhooks/{webhook_id}")),
            body,
        )
    }

    /// Deletes a webhook.
    pub fn delete_webhook(&self, webhook_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_delete(format!("webhooks/{webhook_id}"))
    }

    /// Executes (sends) a webhook message.
    pub fn execute_webhook(
        &self,
        webhook_id: u64,
        token: &str,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client.post(format!("webhooks/{webhook_id}/{token}")),
            body,
        )
    }

    // =========================================================================
    // Application Commands
    // =========================================================================

    /// Lists global application commands.
    pub fn global_commands(&self, application_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("applications/{application_id}/commands"))
    }

    /// Creates a global application command.
    pub fn create_global_command(
        &self,
        application_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client
                .post(format!("applications/{application_id}/commands")),
            body,
        )
    }

    /// Overwrites all global application commands.
    pub fn overwrite_global_commands(
        &self,
        application_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client
                .put(format!("applications/{application_id}/commands")),
            body,
        )
    }

    /// Fetches a global application command.
    pub fn global_command(
        &self,
        application_id: u64,
        command_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("applications/{application_id}/commands/{command_id}"))
    }

    /// Edits a global application command.
    pub fn edit_global_command(
        &self,
        application_id: u64,
        command_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client.patch(format!(
                "applications/{application_id}/commands/{command_id}"
            )),
            body,
        )
    }

    /// Deletes a global application command.
    pub fn delete_global_command(
        &self,
        application_id: u64,
        command_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_delete(format!(
            "applications/{application_id}/commands/{command_id}"
        ))
    }

    /// Lists guild application commands.
    pub fn guild_commands(
        &self,
        application_id: u64,
        guild_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_get(format!(
            "applications/{application_id}/guilds/{guild_id}/commands"
        ))
    }

    /// Replaces application commands for a guild.
    pub fn overwrite_guild_commands(
        &self,
        application_id: u64,
        guild_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client.post(format!(
                "applications/{application_id}/guilds/{guild_id}/commands"
            )),
            body,
        )
    }

    /// Fetches guild application command permissions.
    pub fn guild_command_permissions(
        &self,
        application_id: u64,
        guild_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_get(format!(
            "applications/{application_id}/guilds/{guild_id}/commands/permissions"
        ))
    }

    // =========================================================================
    // Interactions
    // =========================================================================

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
        self.client.execute_json(
            self.client.post(format!("webhooks/{application_id}/{token}")),
            body,
        )
    }

    /// Fetches the original interaction response.
    pub fn get_original_interaction_response(
        &self,
        application_id: u64,
        token: &str,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_get(format!(
            "webhooks/{application_id}/{token}/messages/@original"
        ))
    }

    /// Edits the original interaction response.
    pub fn edit_original_interaction_response(
        &self,
        application_id: u64,
        token: &str,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client.patch(format!(
                "webhooks/{application_id}/{token}/messages/@original"
            )),
            body,
        )
    }

    /// Deletes the original interaction response.
    pub fn delete_original_interaction_response(
        &self,
        application_id: u64,
        token: &str,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_delete(format!(
            "webhooks/{application_id}/{token}/messages/@original"
        ))
    }

    // =========================================================================
    // Polls
    // =========================================================================

    /// Fetches the users who voted for a specific answer in a poll.
    pub fn poll_answer_voters(
        &self,
        channel_id: u64,
        message_id: u64,
        answer_id: u32,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_get(format!(
            "channels/{channel_id}/polls/{message_id}/answers/{answer_id}"
        ))
    }

    /// Ends (expires) a poll early.
    pub fn end_poll(&self, channel_id: u64, message_id: u64) -> Result<HttpResponse, HttpError> {
        self.client.execute(
            self.client
                .post(format!("channels/{channel_id}/polls/{message_id}/expire")),
        )
    }

    // =========================================================================
    // Invites
    // =========================================================================

    /// Fetches an invite by its code.
    pub fn invite(&self, code: &str) -> Result<HttpResponse, HttpError> {
        self.client.execute_get(format!("invites/{code}"))
    }

    /// Deletes an invite.
    pub fn delete_invite(&self, code: &str) -> Result<HttpResponse, HttpError> {
        self.client.execute_delete(format!("invites/{code}"))
    }

    /// Lists invites for a guild.
    pub fn guild_invites(&self, guild_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("guilds/{guild_id}/invites"))
    }

    /// Lists invites for a channel.
    pub fn channel_invites(&self, channel_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("channels/{channel_id}/invites"))
    }

    /// Creates an invite for a channel.
    pub fn create_invite(
        &self,
        channel_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client.post(format!("channels/{channel_id}/invites")),
            body,
        )
    }

    // =========================================================================
    // Integrations
    // =========================================================================

    /// Lists guild integrations.
    pub fn guild_integrations(&self, guild_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("guilds/{guild_id}/integrations"))
    }

    /// Deletes a guild integration.
    pub fn delete_guild_integration(
        &self,
        guild_id: u64,
        integration_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_delete(format!(
            "guilds/{guild_id}/integrations/{integration_id}"
        ))
    }

    // =========================================================================
    // SKUs / Entitlements (Monetisation)
    // =========================================================================

    /// Lists SKUs for an application.
    pub fn skus(&self, application_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("applications/{application_id}/skus"))
    }

    /// Lists entitlements for an application.
    pub fn entitlements(&self, application_id: u64) -> Result<HttpResponse, HttpError> {
        self.client
            .execute_get(format!("applications/{application_id}/entitlements"))
    }

    /// Creates a test entitlement.
    pub fn create_test_entitlement(
        &self,
        application_id: u64,
        body: &Value,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_json(
            self.client
                .post(format!("applications/{application_id}/entitlements")),
            body,
        )
    }

    /// Deletes a test entitlement.
    pub fn delete_test_entitlement(
        &self,
        application_id: u64,
        entitlement_id: u64,
    ) -> Result<HttpResponse, HttpError> {
        self.client.execute_delete(format!(
            "applications/{application_id}/entitlements/{entitlement_id}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::http::{HttpRequest, HttpMethod};

    #[derive(Clone, Default)]
    struct MockTransport {
        requests: Rc<RefCell<Vec<HttpRequest>>>,
    }

    impl Transport for MockTransport {
        fn execute(&self, request: &HttpRequest) -> Result<HttpResponse, HttpError> {
            self.requests.borrow_mut().push(request.clone());
            Ok(HttpResponse::new(200, Vec::new(), Vec::new()))
        }
    }

    fn make_api() -> (DiscordApi<MockTransport>, MockTransport) {
        let transport = MockTransport::default();
        let client = HttpClient::new("secret", transport.clone());
        let api = DiscordApi::new(client);
        (api, transport)
    }

    #[test]
    fn api_dispatches_expected_routes_and_methods() {
        let (api, transport) = make_api();

        api.user("@me").unwrap();
        api.current_user_guilds().unwrap();
        api.guild(123).unwrap();
        api.channel(456).unwrap();
        api.messages(456).unwrap();
        api.message(456, 789).unwrap();
        api.member(123, 1).unwrap();
        api.roles(123).unwrap();
        api.add_reaction(456, 789, "👍").unwrap();
        api.remove_own_reaction(456, 789, "👍").unwrap();
        api.bans(123).unwrap();
        api.guild_emojis(123).unwrap();
        api.guild_scheduled_events(123).unwrap();
        api.automod_rules(123).unwrap();
        api.audit_log(123).unwrap();
        api.webhook(999).unwrap();
        api.global_commands(100).unwrap();
        api.guild_commands(100, 123).unwrap();
        api.invite("discord-rs").unwrap();
        api.guild_integrations(123).unwrap();
        api.skus(100).unwrap();
        api.entitlements(100).unwrap();
        api.end_poll(456, 789).unwrap();

        let reqs = transport.requests.borrow();
        assert_eq!(reqs.len(), 23);
        assert_eq!(reqs[0].path, "users/@me");
        assert_eq!(reqs[0].method, HttpMethod::Get);
        assert_eq!(reqs[1].path, "users/@me/guilds");
        assert_eq!(reqs[2].path, "guilds/123");
        assert_eq!(reqs[3].path, "channels/456");
        assert_eq!(reqs[4].path, "channels/456/messages");
        assert_eq!(reqs[5].path, "channels/456/messages/789");
        assert_eq!(reqs[6].path, "guilds/123/members/1");
        assert_eq!(reqs[7].path, "guilds/123/roles");
        assert_eq!(reqs[8].path, "channels/456/messages/789/reactions/👍/@me");
        assert_eq!(reqs[8].method, HttpMethod::Put);
        assert_eq!(reqs[9].path, "channels/456/messages/789/reactions/👍/@me");
        assert_eq!(reqs[9].method, HttpMethod::Delete);
        assert_eq!(reqs[22].path, "channels/456/polls/789/expire");
        assert_eq!(reqs[22].method, HttpMethod::Post);
    }
}
