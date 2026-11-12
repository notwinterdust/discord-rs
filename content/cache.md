# State Cache & LRU Eviction

Discord bots frequently need access to guild information, member roles, channels, and cached messages without hammering the Discord REST API on every interaction.

`discord-rust` provides an in-memory, thread-safe [`ClientCache`](#) backed by an **LRU (Least Recently Used)** eviction strategy.

---

## Cache Design

The cache is designed with these goals:
1. **Bounded Memory**: Set hard capacity limits for each data type so your bot never exceeds memory limits on large multi-thousand-guild servers.
2. **Thread Safety**: Safely readable and writable across multiple background threads and shard workers.
3. **Event-Driven**: Automatically updates its internal state when Gateway events (`GuildCreate`, `ChannelUpdate`, `MessageCreate`, `MessageDelete`) arrive.

---

## Per-Type Caching

`ClientCache` maintains separate sub-caches for each Discord entity:

```rust
use discord_rust::clients::cache::{ClientCache, CacheConfig};

// Configure limits for your bot's memory profile
let config = CacheConfig::default()
    .with_guild_limit(100)
    .with_channel_limit(1000)
    .with_user_limit(5000)
    .with_message_limit(500);

let cache = ClientCache::new(config);
```

### Supported Entities:
- **Guilds**: Stored by `GuildId` (`Snowflake`).
- **Channels**: Stored by `ChannelId` (`Snowflake`).
- **Members**: Stored by compound `(GuildId, UserId)` key via `MemberCache`.
- **Users**: Stored by `UserId` (`Snowflake`).
- **Roles**: Stored by `RoleId` (`Snowflake`).
- **Messages**: Stored by `MessageId` (`Snowflake`).

---

## Cross-Reference & Member Lookups

Finding a member's roles or permissions is simple:

```rust
use discord_rust::abc::Snowflake;

let guild_id = Snowflake::new(12345);
let user_id = Snowflake::new(67890);

if let Some(member) = cache.member(guild_id, user_id) {
    println!("Member nickname: {:?}", member.nick);
    println!("Member has {} roles", member.roles.len());
}
```

---

## Automatic Invalidation

When a channel or guild is deleted, `ClientCache` automatically purges all related sub-entities:
- When a `ChannelDelete` event is received, all cached messages belonging to that channel are immediately evicted.
- When a `GuildDelete` event is received, all guild channels, roles, and members are cleaned up automatically.
