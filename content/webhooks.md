# Webhooks Execution

Webhooks allow applications to send messages to a Discord channel **without needing a bot account or connecting to the gateway WebSocket**.

They are ideal for CI/CD notification bots, GitHub commit feeds, server monitoring alerts, and logging dispatchers.

---

## Standalone Webhook Client

The [`WebhookClient`](#) is lightweight and operates completely independently of the gateway:

```rust
use discord_rust::webhook::{WebhookClient, WebhookMessage};

// Initialize with a webhook ID and secret token
let webhook = WebhookClient::new(
    123456789012345678_u64,
    "your_webhook_token_here"
);

// Or parse from a full Discord webhook URL
let webhook = WebhookClient::from_url(
    "https://discord.com/api/webhooks/123456789012345678/your_token_here"
).expect("Valid webhook URL");
```

---

## Sending Messages & Embeds

You can override the webhook's default username, avatar, and send rich content:

```rust
use discord_rust::webhook::WebhookMessage;

let message = WebhookMessage::new()
    .with_content("Build #42 passed successfully! 🚀")
    .with_username("CI Notifier")
    .with_avatar_url("https://cdn.discordapp.com/emojis/12345.png")
    .silent(true); // Suppresses notification pings

// Serializes directly to a Discord-compliant JSON payload
let payload_json = webhook.message(message);
```

---

## Editing & Deleting Webhook Messages

```rust
let message_id = 9876543210_u64;

// Endpoint path for modifying an already sent webhook message
let edit_path = webhook.edit_message_path(message_id);
println!("PATCH URL: {}", edit_path);

// Endpoint path for deleting
let delete_path = webhook.delete_message_path(message_id);
println!("DELETE URL: {}", delete_path);
```

---

## Multipart File Uploads

To upload binary log files or image attachments through a webhook, use multipart form assembly:

```rust
use discord_rust::webhook::build_multipart_payload;

let json_payload = r#"{"content": "Here is the error log"}"#;
let file_bytes = b"2026-11-12 20:00:00 [ERROR] Out of memory";

let (content_type, body_bytes) = build_multipart_payload(
    json_payload,
    "crash_log.txt",
    file_bytes
);

println!("Header: Content-Type: {}", content_type);
// Transmit body_bytes with your HTTP transport!
```
