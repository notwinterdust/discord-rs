# Gateway & Event Dispatcher

The Discord Gateway is a WebSocket connection used by bots to receive real-time events from Discord (messages, member joins, typing events, presence updates) and send heartbeat signals.

---

## Gateway Lifecycle

`discord-rust` implements the complete Discord Gateway Protocol (v10):
1. **Connect**: Establishes a WebSocket connection to `wss://gateway.discord.gg`.
2. **Hello (Opcode 10)**: Discord sends the required `heartbeat_interval` in milliseconds.
3. **Identify (Opcode 2)**: The bot sends its authentication token, shard info, and intents.
4. **Ready Event**: Discord transmits initial user info, session ID, and available guilds.
5. **Heartbeat Loop (Opcode 1)**: Automatic background timers transmit heartbeats and expect **Heartbeat ACK (Opcode 11)**.
6. **Resume (Opcode 6)**: If a disconnect occurs, the bot automatically reconnects to `resume_gateway_url` and restores missing sequence numbers without dropping state!

---

## EventDispatcher & Filters

You can register targeted event handlers that only fire for specific event names:

```rust
use discord_rust::clients::events::ThreadedEventDispatcher;
use discord_rust::models::Event;

let dispatcher = ThreadedEventDispatcher::builder()
    // Only listen for TYPING_START events
    .on_name("TYPING_START", |event| {
        if let Event::TypingStart(typing) = event {
            println!("User {} is typing in channel {}", typing.user_id, typing.channel_id);
        }
    })
    // Only listen for MESSAGE_CREATE events
    .on_name("MESSAGE_CREATE", |event| {
        if let Event::MessageCreate(msg) = event {
            println!("New message: {}", msg.content);
        }
    })
    // Catch-all listener
    .on_any(|event| {
        // Trace logging
    })
    .build();
```

---

## Waiting on Future Events (`EventWaiter`)

Have you ever wanted to ask a user a question, and **wait for their response with a timeout**, just like `client.wait_for(...)` in Python?

`discord-rust` includes a dedicated [`EventWaiter`](#) that lets you synchronously block a worker thread until a matching event arrives or a timeout elapses:

```rust
use std::time::Duration;
use discord_rust::clients::EventWaiter;
use discord_rust::models::Event;

let waiter = EventWaiter::new();

// Wait up to 30 seconds for a user to type "!confirm"
let response = waiter.wait_for(|event| {
    if let Event::MessageCreate(msg) = event {
        msg.content.trim() == "!confirm"
    } else {
        false
    }
}, Duration::from_secs(30));

match response {
    Ok(Event::MessageCreate(msg)) => {
        println!("User confirmed: {}", msg.author.username);
    }
    Err(_) => {
        println!("Confirmation timed out!");
    }
    _ => {}
}
```

> [!TIP]
> `EventWaiter` uses `std::sync::mpsc::channel` internally. It does not spin-lock your CPU and wakes up immediately when the matching event is dispatched.
