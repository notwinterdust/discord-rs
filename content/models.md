# Domain Models, Snowflakes & CDN Assets

`discord-rust` provides strongly-typed Rust representations for every Discord entity.

All models implement:
- Memory-safe decoding from JSON.
- `Clone`, `Debug`, and `PartialEq`.
- Strict validation of integer boundaries, timestamp epochs, and bitflags.

---

## The Snowflake Newtype

Discord uses 64-bit integers called **Snowflakes** for all entity IDs (users, guilds, channels, messages, roles).

The [`Snowflake`](#) type guarantees type safety and extracts metadata directly from the bits:

```rust
use discord_rust::abc::Snowflake;

let id = Snowflake::new(175928847299117063);

// 1. Extract the exact timestamp when this ID was created
let created_at = id.timestamp();
println!("Entity was created at: {:?}", created_at);

// 2. Extract worker ID and process ID
println!("Worker ID: {}", id.worker_id());
println!("Process ID: {}", id.process_id());
println!("Sequence: {}", id.increment());
```

---

## CDN Asset Helpers

Discord CDN resources (user avatars, server icons, banners, emojis, soundboard sounds) are represented by the [`Asset`](#) type.

It handles dynamic format selection, size resizing, animated WebP/GIF checking, and asset reading/saving:

```rust
use discord_rust::abc::Asset;

let avatar = Asset::new(
    "https://cdn.discordapp.com/avatars/12345/abcdef.png",
    "abcdef",
    false // animated
);

// Get a resized 512x512 JPEG URL
let resized_url = avatar.url_with_size_and_format(512, "jpeg").unwrap();
println!("CDN URL: {}", resized_url);

// Save the asset directly to disk using your HTTP client
avatar.save("/tmp/avatar.png", |url| {
    // Execute HTTP GET and return Vec<u8>
    Ok::<_, &'static str>(vec![/* image bytes */])
}).expect("Saved image");
```

---

## Rich Domain Entities

### User & Guild Member
```rust
use discord_rust::models::{User, Member};

// Fully parsed user
println!("Username: {}", user.username);
println!("Bot Account: {}", user.bot);
println!("MFA Enabled: {}", user.mfa_enabled);

// Guild-specific member data
println!("Joined at: {:?}", member.joined_at);
println!("Server Deafened: {}", member.deaf);
println!("Server Muted: {}", member.mute);
```

### Rich Embeds
```rust
use discord_rust::models::embed::{Embed, EmbedAuthor, EmbedFooter, EmbedField};
use discord_rust::abc::Colour;

let embed = Embed::new()
    .with_title("Server Status")
    .with_description("All systems operational.")
    .with_colour(Colour::green())
    .with_author(EmbedAuthor::new("Status Bot"))
    .add_field(EmbedField::new("Uptime", "99.98%").with_inline(true))
    .add_field(EmbedField::new("Ping", "18ms").with_inline(true))
    .with_footer(EmbedFooter::new("Last checked: 1 minute ago"));
```

### Auto Moderation Rules
```rust
use discord_rust::models::AutoModRule;

println!("Rule Name: {}", rule.name);
println!("Trigger Type: {:?}", rule.trigger_type);
println!("Actions Count: {}", rule.actions.len());
```
