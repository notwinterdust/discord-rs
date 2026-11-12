# discord-rust

[![Crates.io](https://img.shields.io/crates/v/discord-rs.svg)](https://crates.io/crates/discord-rs)
[![Documentation](https://docs.rs/discord-rs/badge.svg)](https://docs.rs/discord-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Safety: Forbid Unsafe](https://img.shields.io/badge/unsafe-forbidden-success.svg)](crates/)

A feature-rich, high-performance Discord API and Gateway library for Rust.

---

## Features

- **Full Gateway Lifecycle**: Connect, Hello, Identify, Resume, Heartbeat, and automatic reconnection.
- **Multi-Shard Architecture**: `ShardManager` with automatic shard counting and `ConcurrentShardRuntime`.
- **LRU In-Memory Cache**: Thread-safe `ClientCache` and `MemberCache` with automatic event invalidation.
- **Application Commands & UI**: Slash commands, typed options, subcommands, buttons, select menus, and modals.
- **Rich Domain Models**: Users, guilds, channels, roles, messages, embeds, presence, automod, stages, soundboard, and monetization.
- **Voice Support**: Voice gateway protocol, RTP packet framing, and volume scaling pipelines.
- **Event Ergonomics**: `EventWaiter` for waiting on future matching gateway events with timeouts.
- **Structured Logging**: Built-in `log` crate with zero third-party dependencies.

---

## Installation

Just add `discord-rust` to your `Cargo.toml`:

```toml
[dependencies]
discord-rust = "0.1.0"
```
---

## Quickstart: Bot Builder

Here is how to set up and run a bot using the high-level `BotBuilder`:

```rust,no_run
use std::time::Duration;
use discord_rust::{BotBuilder, Event, Intents};

fn main() {
    let mut bot = BotBuilder::new()
        .token("YOUR_BOT_TOKEN")
        .intents(Intents::default_intents())
        .on_event(|event| {
            match event {
                Event::MessageCreate(msg) => {
                    println!("Received message: {}", msg.content);
                }
                Event::Ready(ready) => {
                    println!("Connected as {}", ready.user.username);
                }
                _ => {}
            }
        })
        .build()
        .expect("Failed to build bot");

    // Start background shard workers and wait
    let handles = bot.start_shards();
    for handle in handles {
        let _ = handle.join();
    }
}
```

---

## License

This project is licensed under the [MIT License](LICENSE).
