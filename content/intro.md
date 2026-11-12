# Introduction & Philosophy

Welcome to the **`discord-rust`** documentation!

`discord-rust` is a Discord library built for developers who want **maximum reliability, predictable performance, and clean ergonomics**.

---

## Core Architecture

### 1. High-Performance Multi-Threaded Concurrency
A Discord bot manages:
- Persistent WebSocket connections (shards).
- Outbound HTTP REST requests.
- Parallel event dispatching to user handlers.

`discord-rust` provides dedicated worker architectures:
- Independent background threads for gateway connections and heartbeats.
- A **bounded `ThreadPool`** with configurable worker count for parallel event dispatch.
- Thread-safe channels for event routing.
- Worker queues for asynchronous HTTP request execution.
- Fast compile times and minimal resource overhead.

---

### 2. Type-Safe Architecture & Domain Modeling
Every Discord entity is represented as a strongly-typed Rust data structure:
- Type-safe `Snowflake` identifiers with built-in creation timestamp extraction.
- Explicit bitflags for permissions, intents, and message flags.
- Comprehensive enum matching on all Gateway events.
- Validated builders for Application Commands and UI components.

---

### 3. Pluggable Transport Trait Architecture
Networking in `discord-rust` is decoupled behind simple, ergonomic traits:
- **`GatewayTransport`**: Abstraction for WebSocket frame sending and receiving.
- **`Transport`**: Abstraction for HTTP request execution.

By decoupling transport details from business logic:
- In production, you can connect your preferred WebSocket/TLS transport.
- In testing, you can use built-in mock transports to simulate connection drops, resume sessions, or replay gateway frames with 100% determinism.

---

## Workspace Map

`discord-rust` is composed of **15 modular crates**, all exposed through the root umbrella crate:

| Crate | Crates.io Name | Description |
|---|---|---|
| **`discord-rust`** | `discord-rust` | Top-level umbrella facade exporting all components |
| **`abc`** | `discord-rust-abc` | Base types: `Snowflake`, `Colour`, and CDN `Asset` helpers |
| **`app`** | `discord-rust-app` | Application commands, slash command options, UI components, and `Paginator` |
| **`clients`** | `discord-rust-clients` | HTTP REST client, WebSocket gateway, cache, sharding, and bot runner |
| **`commands`** | `discord-rust-commands` | Command handling, argument parsing, context menus, and cog extensions |
| **`data`** | `discord-rust-data` | Data models, JSON encoding/decoding, and gateway payload primitives |
| **`enums`** | `discord-rust-enums` | Bitflags, permissions, intents, and gateway opcode constants |
| **`event`** | `discord-rust-event` | Gateway event types, event dispatcher, and filter types |
| **`except`** | `discord-rust-except` | Granular error hierarchy (`NotFound`, `Forbidden`, `RateLimitedError`, etc.) |
| **`log`** | `discord-rust-log` | Structured logging framework (`StderrLogger`, `MemoryLogger`) |
| **`macros`** | `discord-rust-macros` | Procedural `#[command]` attribute macro for zero-boilerplate command definitions |
| **`models`** | `discord-rust-models` | Strongly-typed Discord domain models (Guilds, Channels, Messages, Users, etc.) |
| **`util`** | `discord-rust-util` | Percent-encoding, query parameter formatting, and bounded `ThreadPool` |
| **`version`** | `discord-rust-version` | Discord API version constants and endpoint URL builders |
| **`voice`** | `discord-rust-voice` | Voice gateway, RTP packets, `FfmpegSource` audio streaming, and volume scaling |
| **`webhook`** | `discord-rust-webhook` | Standalone webhook execution and multipart body serialization |

---

## Ready to build?
Head over to the [Step-by-Step Quickstart](#quickstart) to write your very first bot!
