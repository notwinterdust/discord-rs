# Definition of crates and their utility

In this markdown, you will find the full list of the crates and their
use, so that you can differentiate which one does what.

Every crate is part of the root Cargo workspace and uses only the Rust
standard library.

## crates/abc

`abc` is an abbreviation for Abstract Base Class.

Rust has no multiple inheritance, so shared behaviour is expressed through
value types and traits.

- `Snowflake` — Discord's 64-bit identifier newtype, with creation-time and
  component helpers.
- `HasId` — implemented by every model that exposes an `id`.
- `Asset` — a CDN asset, with URL builders, resizing and format conversion.
- `Colour`/`Color` — a 24-bit RGB colour, including the named Discord colours
  and string parsing.
- `User` — the shared user behaviour implemented by the concrete user models.

## crates/app

Application command and application-info models.

## crates/clients

Discord clients, gateway connection lifecycle, sharding and reconnection.

## crates/data

Raw gateway event payloads and the data containers that are decoded before
being turned into typed models.

## crates/enums

Discord enumerations and bitfields.

- `Intents` — gateway intent flags.
- `Permissions` — permission flags.
- `PublicUserFlags` — public user flags.
- `MemberFlags` — guild member flags.
- `RoleFlags` — guild role flags.

## crates/event

Typed Discord events dispatched to client callbacks.

## crates/except

Discord exception and error types such as `DiscordException`,
`HTTPException` and `Forbidden`.

## crates/log

Logging setup and helpers.

## crates/models

Discord API models such as `User`, `Guild`, `Channel`, `Message`, `Member`,
`Role` and `Emoji`.

- `User` — the first concrete model. It implements `abc::User` and `HasId`,
  and compares and hashes by ID.
- `ClientUser` — the logged-in account, adding verification, MFA and locale
  fields on top of `User`.
- `Role` — guild roles, including tags, colours, hierarchy comparisons and
  role icons.
- `Member` — guild members, with nicknames, guild avatars, role hierarchy and
  permission helpers. Roles are stored as resolved snapshots until the
  `Guild` model provides a live cache.

## crates/util

Shared utilities such as escaping helpers and OAuth URL generation.

## crates/version

Library version and supported Discord API version information.

## crates/voice

Voice client, voice protocol and audio sources.

## crates/webhook

Webhook and `SyncWebhook` support.
