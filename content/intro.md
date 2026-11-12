# Introduction & Philosophy

Welcome to the **`discord-rust`** documentation!

`discord-rust` is a Discord library built for developers who want **maximum reliability, predictable performance, minimal binary size, and zero unexpected dependencies**.

---

## Core Philosophy

### 1. Pure Standard Library (`std`-only)
Most modern Rust web and API wrappers rely on massive asynchronous runtimes (such as `tokio`, `hyper`, `futures`, etc.). While async is great for high-concurrency I/O servers handling 100,000 idle sockets, a Discord bot typically manages:
- 1 to a few dozen persistent WebSocket connections (shards).
- Occasional outbound HTTP REST requests.
- Synchronous or threaded event dispatching to user handlers.

`discord-rust` is built from the ground up using **pure Rust standard library primitives**:
- Operating system threads via `std::thread`.
- Thread-safe channels via `std::sync::mpsc`.
- Shared memory coordination via `std::sync::Arc`, `std::sync::Mutex`, and `std::sync::atomic`.
- High-performance memory allocators and collections (`std::collections`).

> [!NOTE]
> This means `discord-rust` compiles in **seconds**, has near-zero dependency vulnerabilities, and runs seamlessly on resource-constrained embedded systems, containers, and servers.

---

### 2. Strict Memory Safety (`#![forbid(unsafe_code)]`)
Every single crate in this repository enforces:
```rust
#![forbid(unsafe_code)]
```
There is no `unsafe` pointer manipulation, no unchecked transmutes, and no manual memory fiddling. Memory safety, thread safety, and data race prevention are guaranteed by the Rust compiler.

---

### 3. Pluggable Transport Trait Architecture
Networking in `discord-rust` is decoupled behind simple, ergonomic traits:
- **`GatewayTransport`**: Abstraction for WebSocket frame sending and receiving.
- **`Transport`**: Abstraction for HTTP request execution.

By decoupling transport details from business logic:
- In production, you can connect your preferred WebSocket/TLS transport (such as native-tls or tungstenite).
- In testing, you can use our built-in mock transports to simulate connection drops, resume sessions, or replay gateway frames with 100% determinism.

---

## Workspace Map

`discord-rust` is composed of 14 modular crates, all exposed through the root umbrella crate:

| Crate | Crates.io Name | Description |
|---|---|---|
| **`discord-rust`** | `discord-rust` | Top-level umbrella facade exporting all components |
| **`abc`** | `discord-rust-abc` | Abstract base types: `Snowflake`, `Colour`, and CDN `Asset` helpers |
| **`app`** | `discord-rust-app` | Application commands, slash command options, and UI components |
| **`clients`** | `discord-rust-clients` | HTTP REST client, WebSocket gateway, cache, sharding, and bot runner |
| **`commands`** | `discord-rust-commands` | Command handling, argument parsing, context menus, and cog extensions |
| **`data`** | `discord-rust-data` | Data models, JSON encoding/decoding, and gateway payload primitives |
| **`enums`** | `discord-rust-enums` | Bitflags, permissions, intents, and gateway opcode constants |
| **`event`** | `discord-rust-event` | Gateway event types, event dispatcher, and filter types |
| **`except`** | `discord-rust-except` | Granular error hierarchy (`NotFound`, `Forbidden`, `RateLimitedError`, etc.) |
| **`log`** | `discord-rust-log` | Zero-dependency structured logging framework (`StderrLogger`, `MemoryLogger`) |
| **`models`** | `discord-rust-models` | Strongly-typed Discord domain models (Guilds, Channels, Messages, Users, etc.) |
| **`util`** | `discord-rust-util` | Percent-encoding and query parameter formatting |
| **`version`** | `discord-rust-version` | Discord API version constants and endpoint URL builders |
| **`voice`** | `discord-rust-voice` | Voice gateway connection, RTP packets, and volume source scaling |
| **`webhook`** | `discord-rust-webhook` | Standalone webhook execution and multipart body serialization |

---

## Ready to build?
Head over to the [Step-by-Step Quickstart](#quickstart) to write your very first bot!
