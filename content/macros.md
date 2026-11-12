# Proc Macros & #[command]

`discord-rust-macros` provides procedural attribute macros that eliminate boilerplate when defining prefix commands.

---

## The `#[command]` Attribute

Annotate any plain Rust function with `#[command]` to automatically generate a `CommandDefinition` constructor for it. The macro inspects the function signature and emits typed argument extraction code.

```rust
use discord_rust::commands::{command, CommandRegistry, Context};

#[command(description = "Add two numbers together", aliases = ["plus", "sum"])]
fn add(ctx: &Context, a: i32, b: i32) {
    let result = a + b;
    println!("{} + {} = {}", a, b, result);
    // In a real bot: send result to ctx.channel_id via REST
}

fn main() {
    let mut registry = CommandRegistry::new("!");
    registry.register(add_command()).unwrap();

    // Dispatches "!add 10 32" → calls add(ctx, 10, 32)
    registry.dispatch("!add 10 32");
    // Dispatches "!plus 100 50" → calls add(ctx, 100, 50) via alias
    registry.dispatch("!plus 100 50");
}
```

The macro generates a free function `add_command()` that returns a `CommandDefinition` containing:
- The command name (`"add"`)
- All aliases (`["plus", "sum"]`)
- The optional description string
- A boxed handler closure that extracts typed args from `ctx` and delegates to your function

---

## Supported Attribute Parameters

| Parameter | Type | Description |
|---|---|---|
| `description = "..."` | `&str` | Human-readable description for help output |
| `aliases = ["...", "..."]` | array of `&str` | Alternative command names |

```rust
#[command(description = "Ping the bot", aliases = ["p", "latency"])]
fn ping(ctx: &Context) {
    println!("Pong!");
}
```

---

## Typed Argument Extraction

The macro generates argument extraction using `ctx.arg::<T>(index)` for each non-`Context` parameter. If an argument is missing or fails to parse, the handler returns silently without panicking.

```rust
#[command(description = "Set a slowmode delay in seconds")]
fn slowmode(ctx: &Context, channel_id: u64, delay: u32) {
    println!("Setting slowmode {} on channel {}", delay, channel_id);
}
```

Supported argument types: anything implementing `std::str::FromStr` — `i32`, `u64`, `f64`, `String`, `bool`, and your own types.

---

## Without the Macro

If you prefer explicit control, you can build `CommandDefinition` by hand:

```rust
use discord_rust::commands::{CommandDefinition, CommandRegistry, Context};

let mut registry = CommandRegistry::new("!");

registry.register(
    CommandDefinition::new(
        "ping",
        ["p"],
        |_ctx: &Context| println!("Pong!"),
    )
    .with_description("Ping the bot"),
).unwrap();
```

---

## Combining with CommandGroups

`#[command]`-generated definitions compose naturally with `CommandGroup` for subcommand hierarchies:

```rust
use discord_rust::commands::{command, CommandGroup, CommandRegistry, Context};

#[command(description = "Show server config")]
fn config_view(ctx: &Context) { println!("Showing config..."); }

#[command(description = "Set a config value")]
fn config_set(ctx: &Context, key: String, value: String) {
    println!("Setting {} = {}", key, value);
}

let mut group = CommandGroup::new("config");
group.add_subcommand_definition(config_view_command());
group.add_subcommand_definition(config_set_command());

let mut registry = CommandRegistry::new("!");
registry.add_group(group).unwrap();
// "!config view" and "!config set key value" both work
```
