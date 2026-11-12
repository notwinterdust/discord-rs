# Logging & Error Handling

Robust Discord bots need granular error differentiation and low-overhead, zero-dependency structured logging.

`discord-rust` provides both out of the box.

---

## Granular Error Hierarchy

The [`except`](#) crate defines a fine-grained, typed error hierarchy instead of opaque error strings:

```rust
use discord_rust::except::{
    NotFound, Forbidden, Unauthorized, RateLimitedError, InvalidData, GatewayNotFound
};
```

### Matching on Typed Exceptions

```rust
match result {
    Ok(data) => println!("Success!"),
    Err(err) => {
        if let Some(rate_limit) = err.downcast_ref::<RateLimitedError>() {
            println!("Rate limited! Retry after {} seconds (Global: {})",
                rate_limit.retry_after, rate_limit.global);
        } else if let Some(not_found) = err.downcast_ref::<NotFound>() {
            println!("Requested resource was not found: {}", not_found);
        } else if let Some(forbidden) = err.downcast_ref::<Forbidden>() {
            println!("Missing permissions: {}", forbidden);
        } else {
            eprintln!("Unexpected error: {}", err);
        }
    }
}
```

---

## Zero-Dependency Structured Logging

The built-in [`log`](#) crate allows you to emit structured diagnostic messages without pulling in third-party logging frameworks.

### Log Levels
- `Level::Error` (Highest severity)
- `Level::Warn`
- `Level::Info` (Default)
- `Level::Debug`
- `Level::Trace`

### Built-in Loggers
1. **`StderrLogger`**: Formats logs with timestamps and levels directly to standard error (`eprintln!`).
2. **`MemoryLogger`**: Thread-safe in-memory vector capturing logs for unit tests or telemetry buffers.
3. **`NullLogger`**: Zero-cost no-op logger that discards all messages.

```rust
use discord_rust::log::{Level, StderrLogger, MemoryLogger};

// 1. Console logging with StderrLogger
let logger = StderrLogger::new(Level::Debug);

// 2. In-memory test buffer with MemoryLogger
let mem_logger = MemoryLogger::new(Level::Trace);
```

### Logging Macros

Use the provided logging macros:

```rust
use discord_rust::log::{debug, info, warn, error};

info!("Bot initialized with shard count {}", 4);
debug!("Received gateway heartbeat ACK in {}ms", 12);
warn!("Rate limit approaching bucket limit: {}/{}", remaining, total);
error!("Gateway connection dropped: connection reset by peer");
```
