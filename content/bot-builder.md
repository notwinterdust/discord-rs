# BotBuilder & Lifecycle

The [`BotBuilder`](#) struct is the primary ergonomic entry point for configuring and booting a bot in `discord-rust`.

It manages:
- Authentication credentials (Bot Token).
- Gateway Intents.
- Sharding topology (single shard or auto-sharded worker pool).
- In-memory state cache limits.
- Event listener callbacks, execution hooks, and global error handlers.

---

## Basic Configuration

```rust
use discord_rust::{BotBuilder, Intents};

let bot = BotBuilder::new()
    .token("Bot YOUR_TOKEN_HERE")
    .intents(Intents::default_intents() | Intents::GUILD_MEMBERS)
    .max_messages(500)   // LRU cache limit for channel messages
    .build()
    .expect("Valid bot configuration");
```

---

## Configuring Execution Hooks

You can attach pre-invoke, post-invoke, and error hooks directly on the builder:

```rust
use discord_rust::{BotBuilder, Event, Intents};

let mut bot = BotBuilder::new()
    .token("Bot TOKEN")
    .intents(Intents::default_intents())
    // Runs before every event is processed
    .before_invoke(|event| {
        if let Event::MessageCreate(msg) = event {
            println!(">> Pre-processing message: {}", msg.id);
        }
    })
    // Runs after every event is processed
    .after_invoke(|event| {
        // Log telemetry or update metrics
    })
    // Catches uncaught runtime handler errors
    .on_error(|event, error_message| {
        eprintln!("Error processing event {:?}: {}", event, error_message);
    })
    .on_event(|event| {
        // Main event dispatcher
    })
    .build()
    .unwrap();
```

---

## Lifecycle & Shard Management

Once built, the bot can start its gateway connections:

```rust
// Spawns gateway worker threads across background OS threads
let handles = bot.start_shards();

println!("Running {} shard worker(s)...", handles.len());

// Wait for all shard threads to finish (or until shutdown signal)
for handle in handles {
    if let Err(e) = handle.join() {
        eprintln!("Shard thread terminated with error: {:?}", e);
    }
}
```

> [!TIP]
> Each shard runs independently on its own OS thread. A network hiccup or socket drop on Shard 0 will **never** block or freeze events on Shard 1!
