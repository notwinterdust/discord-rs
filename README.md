# discord-rs

[![Crates.io](https://img.shields.io/crates/v/discord-rs.svg)](https://crates.io/crates/discord-rs)
[![Documentation](https://docs.rs/discord-rs/badge.svg)](https://docs.rs/discord-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Safety: Forbid Unsafe](https://img.shields.io/badge/unsafe-forbidden-success.svg)](crates/)

A feature-rich, high-performance Discord API and Gateway library for Rust.

Built with **pure `std`** (zero `async`/`tokio` runtime dependencies), strict `#![forbid(unsafe_code)]` across all crates, and an extensible transport-trait architecture.

---

## Philosophy & Design Principles

- **Standard Library Only**: Built completely on `std` threads, channels (`std::sync::mpsc`), atomics, and standard I/O. No complex async runtimes or bloated dependency trees.
- **`#![forbid(unsafe_code)]`**: Every single crate in this repository enforces `#![forbid(unsafe_code)]`.
- **Pluggable Transports**: Gateway and REST operations are decoupled behind traits (`GatewayTransport`, `Transport`). You can plug in any WebSocket implementation (e.g. native TLS, tungstenite, mock test fixtures) without changing application logic.
- **Granular Modularity**: 14 focused crates allow applications to pull in only the components they require, or use the top-level `discord-rs` umbrella crate.

---

## Workspace Structure

| Crate | Description |
|---|---|
| [`discord-rs`](.) | Umbrella crate re-exporting all core modules and ergonomic entrypoints |
| [`abc`](crates/abc) | Abstract base components: `Snowflake`, `Colour`, and CDN `Asset` helpers |
| [`app`](crates/app) | Application commands, slash command options, interactions, and UI components |
| [`clients`](crates/clients) | HTTP REST client, WebSocket gateway, cache, sharding, and bot runner |
| [`commands`](crates/commands) | Command handling, argument parsing, context menus, and cog extensions |
| [`data`](crates/data) | Data models, JSON encoding/decoding, and gateway payload primitives |
| [`enums`](crates/enums) | Bitflags, permissions, intents, and gateway opcode constants |
| [`event`](crates/event) | Gateway event types, event dispatcher, and filter types |
| [`except`](crates/except) | Granular error hierarchy (`NotFound`, `Forbidden`, `RateLimitedError`, etc.) |
| [`log`](crates/log) | Zero-dependency structured logging framework (`StderrLogger`, `MemoryLogger`) |
| [`models`](crates/models) | Strongly-typed Discord domain models (Guilds, Channels, Messages, Users, etc.) |
| [`util`](crates/util) | Percent-encoding and query parameter formatting |
| [`version`](crates/version) | Discord API version constants and endpoint URL builders |
| [`voice`](crates/voice) | Voice gateway connection, RTP packets, and volume source scaling |
| [`webhook`](crates/webhook) | Standalone webhook execution and multipart body serialization |

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

Add `discord-rs` to your `Cargo.toml`:

```toml
[dependencies]
discord-rs = "0.1.0"
```

Or select only the specific crates you need:

```toml
[dependencies]
discord-rs-clients = { package = "clients", version = "0.1.0" }
discord-rs-models = { package = "models", version = "0.1.0" }
```

---

## Quickstart: Bot Builder

Here is how to set up and run a bot using the high-level `BotBuilder`:

```rust,no_run
use std::time::Duration;
use discord_rs::{BotBuilder, Event, Intents};

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
