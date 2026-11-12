# Slash Commands, Options & #[command]

Modern Discord bots interact with users primarily through **Application Commands** (Slash Commands, User Context Menus, and Message Context Menus) and traditional **prefix commands**.

`discord-rust` provides strongly-typed builders for application commands, and the `#[command]` proc macro for effortless prefix command definitions.

---

## Defining a Slash Command

```rust
use discord_rust::app::ApplicationCommand;
use discord_rust::app::CommandOption;

// Build a /ban command with options
let ban_command = ApplicationCommand::chat_input("ban", "Ban a rule-breaking user")
    .with_option(
        CommandOption::user("target", "The user to ban")
            .with_required(true)
    )
    .with_option(
        CommandOption::string("reason", "Reason for the ban")
            .with_required(false)
    )
    .with_option(
        CommandOption::integer("delete_messages_days", "Days of messages to delete (0-7)")
            .with_min_value(0.0)
            .with_max_value(7.0)
    );
```

---

## Supported Option Types

The `CommandOption` builder supports all Discord option variants:

| Type Helper | Description |
|---|---|
| `CommandOption::string(name, desc)` | Standard text string |
| `CommandOption::integer(name, desc)` | Whole integer number |
| `CommandOption::boolean(name, desc)` | True / False boolean |
| `CommandOption::user(name, desc)` | Discord User mention |
| `CommandOption::channel(name, desc)` | Guild Channel mention |
| `CommandOption::role(name, desc)` | Server Role mention |
| `CommandOption::mentionable(name, desc)` | User or Role mention |
| `CommandOption::number(name, desc)` | Floating point number |
| `CommandOption::attachment(name, desc)` | Uploaded file attachment |

---

## Subcommands & Subcommand Groups

You can easily nest subcommands (e.g. `/config set` and `/config view`):

```rust
use discord_rust::app::{ApplicationCommand, CommandOption};

let config_command = ApplicationCommand::chat_input("config", "Manage server settings")
    .with_option(
        CommandOption::subcommand("view", "View current server settings")
    )
    .with_option(
        CommandOption::subcommand("set", "Set a configuration value")
            .with_option(CommandOption::string("key", "The config key").with_required(true))
            .with_option(CommandOption::string("value", "The new value").with_required(true))
    );
```

---

## Context Menu Commands

Context menus appear when right-clicking on a user or message:

```rust
use discord_rust::app::ApplicationCommand;

// User Context Menu: right click a user -> Apps -> "Report User"
let report_user = ApplicationCommand::user("Report User");

// Message Context Menu: right click a message -> Apps -> "Bookmark"
let bookmark_msg = ApplicationCommand::message("Bookmark");
```

---

## Prefix Commands with `#[command]`

For classic prefix-style bots (`!ping`, `!ban @user`), use the `#[command]` attribute macro from `discord-rust-macros`:

```rust
use discord_rust::commands::{command, CommandRegistry, Context};

#[command(description = "Add two numbers", aliases = ["plus", "sum"])]
fn add(ctx: &Context, a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a + b);
}

#[command(description = "Ping the bot", aliases = ["p"])]
fn ping(_ctx: &Context) {
    println!("Pong!");
}

fn main() {
    let mut registry = CommandRegistry::new("!");
    registry.register(add_command()).unwrap();
    registry.register(ping_command()).unwrap();

    // Dispatch incoming message text
    registry.dispatch("!add 15 27");   // → 42
    registry.dispatch("!ping");        // → Pong!
    registry.dispatch("!p");           // → Pong! (alias)
}
```

The macro generates a `{fn_name}_command()` function returning a `CommandDefinition`. Argument types are extracted automatically using `FromStr` — if parsing fails, the handler returns without panicking.

See the [Proc Macros & #[command]](#macros) guide for the full reference.

---

## Autocomplete Interactions

To provide dynamic, real-time choices as the user types:

```rust
let option = CommandOption::string("game", "Choose a game to play")
    .with_autocomplete(true);
```

When the user types, Discord sends an `InteractionType::ApplicationCommandAutocomplete` event, which you can resolve with dynamic choice lists!
