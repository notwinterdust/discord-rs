# Changelog

All notable changes to `discord-rust` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-11-12

Initial release of `discord-rust`: a pure Rust, standard-library-only (`std`-only, zero `async`/`tokio`), `#![forbid(unsafe_code)]` Discord API and Gateway library.

### Added

#### Concurrent Runtime
- `ThreadedCoordinator` background worker orchestration with clean atomic shutdown.
- `ThreadedEventDispatcher` non-blocking asynchronous event fan-out with filter matching.
- `ThreadedRequestExecutor` non-blocking REST HTTP request worker queue.
- `ConcurrentShardRuntime` multi-shard worker pool with graceful join and shutdown.

#### Gateway & Events
- WebSocket gateway protocol connection handling (`GatewayConnection`).
- Opcode parsing for `Hello`, `Identify`, `Resume`, `Heartbeat`, `HeartbeatAck`, and `Reconnect`.
- Automatic heartbeat interval scheduling, missed-ACK tracking, and exponential backoff reconnection.
- `EventDispatcher` supporting typed handlers, name-based filtering, and wildcard listeners.

#### Cache Layer
- Thread-safe `ClientCache` with configurable capacity and LRU eviction policy.
- Per-type caching for guilds, channels, users, roles, messages, and emojis.
- `MemberCache` for compound guild-member tracking and cross-reference lookups.
- Event-driven automatic cache population and invalidation.

#### REST API
- `DiscordApi` endpoint builder facade supporting core Discord REST resources:
  - Channels (get, modify, delete, messages, reactions, pins, typing, permissions).
  - Guilds (get, modify, delete, audit logs, voice regions, integrations, prune, vanity).
  - Members, roles, bans, voice states, and invites.
  - Users, user DMs, and bot profile endpoints.
- Bucket-based rate limit tracking with `RateLimit` header extraction and retry delays.

#### Application Commands
- Slash command definitions, subcommand groups, and nested subcommands.
- Strongly-typed `CommandOption` builders (string, integer, boolean, user, channel, role, mentionable, number, attachment).
- User and Message context menu commands.
- Autocomplete interaction payload support.

#### UI Components
- Interactive message components: `ActionRow`, `Button` (with styles), `SelectMenu`, and `TextInput`.
- Component builder ergonomics and structure validation.

#### Webhooks & Interactions
- Standalone `WebhookClient` supporting ID/token authentication and execution.
- Webhook message editing, deletion, and multipart body formatting.
- Interaction response builders (channel message with source, deferred channel message, deferred update, modal response).

#### Voice Support
- Voice gateway connection abstraction and state machine.
- RTP packet serialization, sequence tracking, and SSRC framing.
- Audio source pipeline with `VolumeSource` linear gain scaling.
- Encryption traits for voice packet ciphering.

#### Sharding & Bot Ergonomics
- `ShardManager` with automatic shard count recommendation from Discord Gateway Bot endpoint.
- `BotBuilder` fluent configuration API for token, intents, cache limits, and event hooks.
- `EventWaiter` for synchronously waiting on future matching gateway events with timeouts.
- Cog extension framework for modular command group registration.
- Periodic task loop primitive for background polling and maintenance routines.

#### Domain Models & CDN
- Rich domain models: `User`, `Guild`, `Channel`, `Message`, `Role`, `Emoji`, `Member`, `Attachment`, `Embed`.
- Presence and rich activity tracking (`PresenceUpdate`, `Activity`, `ActivityType`, `StatusType`).
- Auto Moderation rules and execution actions (`AutoModRule`, `TriggerType`, `ActionType`).
- Stage instances and scheduled guild events (`StageInstance`, `PrivacyLevel`).
- Guild onboarding and welcome screens (`WelcomeScreen`, `Onboarding`).
- Guild templates with code generation (`GuildTemplate`).
- Monetization primitives (`Sku`, `Entitlement`).
- Soundboard audio clips (`SoundboardSound`).
- CDN asset URL generation and standard-library `read_to_vec` / `save_to_path` helpers.

#### Architecture, Errors & Structured Logging
- Granular error hierarchy: `NotFound`, `Forbidden`, `Unauthorized`, `RateLimitedError`, `InvalidData`, `GatewayNotFound`, `ClientException`.
- Zero-dependency structured logging framework: `Level`, `Record`, `Logger`, `StderrLogger`, `MemoryLogger`, `NullLogger`, with `emit!`, `debug!`, `info!`, `warn!`, `error!` macros.
- Full gateway and REST integration test suite verifying end-to-end event loops and concurrent execution.
