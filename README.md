# discord-rust

[![Documentation](https://img.shields.io/badge/docs-discord--rust.netlify.app-5865F2.svg)](https://discord-rust.netlify.app/)
[![Crates.io](https://img.shields.io/crates/v/discord-rust.svg)](https://crates.io/crates/discord-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A feature-rich, high-performance Discord API and Gateway library for Rust.

---

## Features

- **Full Gateway Lifecycle**: Connect, Hello, Identify, Resume, Heartbeat, and automatic reconnection.
- **Multi-Shard Architecture**: `ShardManager` with automatic shard counting and `ConcurrentShardRuntime`.
- **LRU In-Memory Cache & Edit Tracking**: Thread-safe `ClientCache` and `MessageEditTracker` with automatic event invalidation.
- **Bounded Worker Thread Pool**: Dedicated `ThreadPool` with `PooledEventDispatcher` for concurrent event fan-out across CPU cores.
- **Application Commands & Proc Macros**: Slash commands, typed options, subcommands, and `#[command]` attribute macro for prefix commands.
- **UI Components & Paginator**: Buttons, select menus, modals, and interactive multi-page `Paginator`.
- **Voice & FFmpeg Streaming**: Voice gateway protocol, RTP packet framing, `FfmpegSource` child process audio streamer, and volume scaling.
- **Rich Domain Models**: Users, guilds, channels, roles, messages, embeds, presence, automod, stages, soundboard, and monetization.
- **Event Ergonomics**: `EventWaiter` for waiting on future matching gateway events with timeouts.
- **Structured Logging**: Built-in structured logging framework with zero third-party runtime dependencies.

---

## Documentation & Wiki

Explore the full interactive documentation, wiki guides, and complete architecture manuals at:
**[https://discord-rust.netlify.app/](https://discord-rust.netlify.app/)**

---

## Installation

Add `discord-rust` to your `Cargo.toml`:

```toml
[dependencies]
discord-rust = "0.1.1"
```

---

## Quickstart: Bot Builder

For the complete 5-minute interactive tutorial and guide, see the **[Step-by-Step Quickstart on the Docs Site](https://discord-rust.netlify.app/#quickstart)**.

Here is a quick overview of setting up and running a bot with `BotBuilder`:

```rust,no_run
use discord_rust::{BotBuilder, Event, Intents};

fn main() {
    let mut bot = BotBuilder::new()
        .token("YOUR_BOT_TOKEN")
        .intents(Intents::default_intents() | Intents::MESSAGE_CONTENT)
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

Learn more in the **[BotBuilder & Lifecycle Guide](https://discord-rust.netlify.app/#bot-builder)** or visit the **[Live Documentation](https://discord-rust.netlify.app/)**.

---

## License

This project is licensed under the [MIT License](LICENSE).
