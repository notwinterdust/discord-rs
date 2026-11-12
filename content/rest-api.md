# REST API Client & Rate Limiting

The [`DiscordApi`](#) facade provides typed, clean wrappers for all Discord REST API endpoints.

All requests are authenticated and automatically observe Discord's per-route and global rate limits.

---

## Basic REST Usage

```rust
use discord_rust::clients::api::DiscordApi;
use discord_rust::clients::client::HttpClient;
use discord_rust::clients::http::TcpTransport; // Or any custom Transport

// 1. Initialize an authenticated HTTP client
let transport = TcpTransport::new();
let http = HttpClient::new("Bot YOUR_TOKEN", transport);
let api = DiscordApi::new(http);

// 2. Fetch a channel
let channel_id = 123456789012345678_u64;
let response = api.channel(channel_id).expect("Fetch channel");
println!("Status: {}", response.status);

// 3. Send a message to a channel
let body = r#"{"content": "Hello from discord-rust!"}"#;
let msg_resp = api.create_message(channel_id, body.as_bytes()).expect("Send message");
println!("Message sent with response status: {}", msg_resp.status);
```

---

## Supported Endpoint Groups

`DiscordApi` provides complete coverage of Discord's core resources:

### Channels
- `channel(channel_id)`: Fetch channel metadata.
- `modify_channel(channel_id, body)`: Edit channel settings, topic, or bitrate.
- `delete_channel(channel_id)`: Delete a channel.
- `create_message(channel_id, body)`: Post a message.
- `edit_message(channel_id, message_id, body)`: Edit an existing message.
- `delete_message(channel_id, message_id)`: Delete a message.
- `bulk_delete_messages(channel_id, body)`: Bulk purge messages.
- `create_reaction(channel_id, message_id, emoji)`: Add reaction.
- `delete_own_reaction(...)` / `delete_user_reaction(...)`: Manage reactions.
- `trigger_typing_indicator(channel_id)`: Show typing indicator.

### Guilds
- `guild(guild_id)`: Fetch guild information.
- `guild_preview(guild_id)`: Fetch public preview.
- `create_guild(body)` / `modify_guild(guild_id, body)`: Manage servers.
- `guild_channels(guild_id)`: List all channels in a guild.
- `guild_roles(guild_id)`: List server roles.
- `guild_bans(guild_id)`: List server bans.
- `guild_audit_logs(guild_id)`: Query audit log entries with filters.

### Users
- `user(user_id)`: Fetch a user profile (use `"@me"` for the bot itself).
- `current_user_guilds()`: List guilds the bot belongs to.
- `create_dm(body)`: Open a DM channel with a user.

---

## Bucket-Based Rate Limiting

Discord enforces strict rate limits per endpoint bucket. `discord-rust` automatically parses response headers:
- `X-RateLimit-Limit`: Maximum requests per window.
- `X-RateLimit-Remaining`: Remaining requests.
- `X-RateLimit-Reset-After`: Seconds until the current bucket resets.
- `X-RateLimit-Scope`: `user`, `global`, or `shared`.

When a `429 Too Many Requests` is encountered, the client returns [`except::RateLimitedError`](#) with the exact retry duration:

```rust
use discord_rust::except::RateLimitedError;

match api.create_message(channel_id, body.as_bytes()) {
    Ok(resp) => println!("Message sent!"),
    Err(e) => {
        eprintln!("Failed to send: {}", e);
    }
}
```
