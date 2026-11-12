# State Cache, Edit Tracking & Eviction

Discord bots frequently need access to guild information, member roles, channels, and cached messages without hammering the Discord REST API on every interaction.

`discord-rust` provides an in-memory, bounded [`ClientCache`](#) with automatic eviction and a built-in **message edit tracker** for correlating command messages with bot responses.

---

## Cache Design

The cache is designed with these goals:
1. **Bounded Memory**: Set hard capacity limits for each data type so your bot never exceeds memory limits on large multi-thousand-guild servers.
2. **Event-Driven**: Automatically updates its internal state when Gateway events (`GuildCreate`, `ChannelUpdate`, `MessageCreate`, `MessageDelete`) arrive.
3. **Edit Awareness**: Track which bot response corresponds to which user command, so edits can update the right message.

---

## Per-Type Caching

`ClientCache` maintains separate sub-caches for each Discord entity:

```rust
use discord_rust::clients::cache::{ClientCache, CacheConfig};

let config = CacheConfig {
    guilds:       1_000,
    channels:     10_000,
    roles:        10_000,
    users:        10_000,
    messages:     1_000,
    presences:    10_000,
    message_edits: 1_000,  // edit tracker capacity
};

let mut cache = ClientCache::new(config);
```

### Supported Entities:
- **Guilds**: Stored by `GuildId` (`Snowflake`).
- **Channels**: Stored by `ChannelId` (`Snowflake`).
- **Members**: Stored by compound `(GuildId, UserId)` key via `MemberCache`.
- **Users**: Stored by `UserId` (`Snowflake`).
- **Roles**: Stored by `RoleId` (`Snowflake`).
- **Messages**: Stored by `MessageId` (`Snowflake`).

---

## Updating the Cache

Feed gateway events directly into the cache:

```rust
// Returns true if the event mutated any cache entry
let mutated = cache.update(&event);
```

For message edits, use `update_tracked` to capture the before/after state:

```rust
let (mutated, edit_info) = cache.update_tracked(&event);

if let Some((previous, updated)) = edit_info {
    if let Some(old) = previous {
        println!("Message edited: '{}' → '{}'", old.content, updated.content);
    }
}
```

---

## Message Edit Tracker

`MessageEditTracker` maintains a bidirectional mapping between user command messages and the bot's response messages. This lets you update the bot's reply when a user edits their command.

```rust
use discord_rust::clients::MessageEditTracker;

let mut tracker = MessageEditTracker::new(1_000);

// When the bot responds to a user message
let user_msg_id = 111_000_000_000_000_000_u64;
let bot_reply_id = 222_000_000_000_000_000_u64;
tracker.register_response(user_msg_id, bot_reply_id);

// Look up the bot's reply given the user's original message ID
if let Some(reply_id) = tracker.get_response(user_msg_id) {
    println!("Should edit message {}", reply_id);
}

// Look up the original command given a bot response ID
if let Some(cmd_id) = tracker.get_user_message(bot_reply_id) {
    println!("User originally sent message {}", cmd_id);
}
```

The tracker is embedded in `ClientCache` as `cache.edit_tracker` and automatically cleans up entries when messages are deleted via `MessageDelete` and `MessageDeleteBulk` events.

---

## Cross-Reference & Member Lookups

Finding a member's roles or permissions is simple:

```rust
use discord_rust::abc::Snowflake;

let guild_id = Snowflake::new(12345);
let user_id  = Snowflake::new(67890);

if let Some(member) = cache.member(guild_id.get(), user_id.get()) {
    println!("Member nickname: {:?}", member.nick);
    println!("Member has {} roles", member.roles.len());
}
```

---

## Automatic Invalidation

When a channel or guild is deleted, `ClientCache` automatically purges all related sub-entities:
- When a `ChannelDelete` event is received, all cached messages belonging to that channel are immediately evicted.
- When a `GuildDelete` event is received, all guild channels, roles, and members are cleaned up automatically.
- When a `MessageDelete` or `MessageDeleteBulk` event is received, `edit_tracker` entries for those messages are also removed.
