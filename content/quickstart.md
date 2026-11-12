# Step-by-Step Quickstart

This tutorial will guide you through building and running your very first Discord bot in Rust in **less than 5 minutes**.

---

## Step 1: Create a Discord Application & Bot

1. Open the [Discord Developer Portal](https://discord.com/developers/applications).
2. Click **New Application** in the top-right corner. Give your bot a name (e.g. `RustaceanBot`) and confirm.
3. In the left sidebar, click **Bot**.
4. Click **Reset Token** (or **Copy Token**) to generate your bot token. Keep this token safe and never share it publicly!
5. Scroll down to **Privileged Gateway Intents** and enable:
   - **Message Content Intent** (required if your bot reads standard message text).
   - **Server Members Intent** (if your bot tracks guild members).
6. Invite the bot to your test server:
   - In the left sidebar, go to **OAuth2** $\to$ **URL Generator**.
   - Check `bot` and `applications.commands`.
   - In the permissions list below, check `Send Messages` and `Read Message History`.
   - Copy the generated URL at the bottom and paste it into your browser to invite the bot.

---

## Step 2: Initialize Your Rust Project

Create a new Rust binary project using Cargo:

```bash
cargo new my_discord_bot
cd my_discord_bot
```

Add `discord-rust` to your `Cargo.toml`:

```toml
[dependencies]
discord-rust = "0.1.1"
```

---

## Step 3: Write Your First Ping-Pong Bot

Open `src/main.rs` and replace its contents with the following:

```rust
use discord_rust::{BotBuilder, Event, Intents};

fn main() {
    // Read your token from an environment variable for safety
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Please set the DISCORD_TOKEN environment variable!");

    println!("Starting bot with discord-rust...");

    // 1. Build your bot using the fluent BotBuilder API
    let mut bot = BotBuilder::new()
        .token(token)
        .intents(Intents::default_intents() | Intents::MESSAGE_CONTENT)
        .on_event(|event| {
            // 2. Pattern match on typed Discord Gateway events
            match event {
                Event::Ready(ready) => {
                    println!("Logged in as: {} (ID: {})", ready.user.username, ready.user.id);
                    println!("Connected to {} guilds.", ready.guilds.len());
                }

                Event::MessageCreate(msg) => {
                    // Ignore messages sent by bots to avoid infinite loops
                    if msg.author.bot {
                        return;
                    }

                    println!("[#{}] {}: {}", msg.channel_id, msg.author.username, msg.content);

                    // Check for ping command
                    if msg.content.trim() == "!ping" {
                        println!("Replying pong to message {}", msg.id);
                        // In a real bot, dispatch a REST message response here!
                    }
                }

                _ => {}
            }
        })
        .build()
        .expect("Failed to initialize bot!");

    // 3. Start background shard runners and wait
    println!("Bot initialized successfully. Launching shards...");
    let handles = bot.start_shards();
    for handle in handles {
        let _ = handle.join();
    }
}
```

---

## Step 4: Run the Bot!

Set your bot token and run the project:

```bash
export DISCORD_TOKEN="your_token_here"
cargo run
```

You will see:
```text
Starting bot with discord-rust...
Bot initialized successfully. Launching shards...
Logged in as: RustaceanBot (ID: 1234567890)
Connected to 1 guilds.
```

Go to your Discord server and type `!ping` in any channel where the bot has access — you will immediately see the incoming message event logged to your terminal!

---

## Next Steps
- Learn how to configure hooks and error handlers in [BotBuilder & Lifecycle](#bot-builder).
- Explore the [Gateway & Event Dispatcher](#gateway) to synchronously wait for user input with `EventWaiter`.
- Register rich slash commands with [Slash Commands & Options](#commands).
