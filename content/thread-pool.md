# Thread Pool & Concurrent Dispatch

`discord-rust` ships a built-in bounded worker thread pool used internally for event dispatch, and exposed directly for your own background work.

No async runtime is required — all concurrency is built on `std::thread` and `std::sync` primitives.

---

## Why a Bounded Pool?

Spawning a new OS thread per event is fast to write but dangerous at scale. A busy bot on a large guild can receive thousands of gateway events per second. Unbounded spawning:

- Exhausts OS thread limits (typically 1,000 – 32,000 depending on the system).
- Creates runaway memory usage from per-thread stacks.
- Causes priority inversion and cache thrashing.

A **bounded pool** creates a fixed number of worker threads at startup and queues incoming tasks into a shared `VecDeque`. Workers sleep on a `Condvar` when idle and wake only when a task is ready.

---

## Creating a Thread Pool

```rust
use discord_rust::util::ThreadPool;

// Pool sized to your CPU logical core count
let pool = ThreadPool::new(4);

pool.execute(|| {
    println!("Running on a worker thread!");
});
```

---

## Builder API

For fine-grained control, use the builder:

```rust
use discord_rust::util::ThreadPool;

let pool = ThreadPool::builder()
    .num_threads(8)
    .name_prefix("my-bot-worker")
    .stack_size(2 * 1024 * 1024)   // 2 MiB per worker
    .build()
    .expect("pool spawn failed");
```

| Builder Method | Description |
|---|---|
| `.num_threads(n)` | Fixed number of worker threads (defaults to `available_parallelism()`) |
| `.name_prefix(s)` | Thread name prefix shown in profilers / `top` |
| `.stack_size(bytes)` | Per-thread OS stack size |

---

## Introspection

```rust
println!("Workers: {}", pool.thread_count());
println!("Queued:  {}", pool.queued_jobs());
println!("Active:  {}", pool.active_workers());
println!("Stopped: {}", pool.is_shutdown());
```

---

## Graceful Shutdown

```rust
pool.shutdown(); // signals all workers to drain and exit
// Drop also calls shutdown automatically
```

Workers finish their current job before exiting. No task is silently dropped — the queue drains cleanly before threads join.

---

## PooledEventDispatcher

The `PooledEventDispatcher` wires a `ThreadPool` directly into the gateway event pipeline so each matching handler fires in parallel:

```rust
use discord_rust::clients::events::PooledEventDispatcher;

let dispatcher = PooledEventDispatcher::builder()
    .workers(4)
    .on_name("MESSAGE_CREATE", |event| {
        // Runs on a pool worker, not the gateway reader thread
        println!("Got message event: {:?}", event);
    })
    .on_name("GUILD_CREATE", |event| {
        println!("Joined a guild: {:?}", event);
    })
    .build();

// Attach to your gateway runtime's EventDispatcher
dispatcher.attach_to(&mut event_dispatcher);
```

Unlike `ThreadedEventDispatcher` (single background thread, sequential), `PooledEventDispatcher` fans out every matching handler invocation across the pool — multiple `MESSAGE_CREATE` handlers can run simultaneously on separate workers.

---

## How Many Threads?

| Scenario | Recommendation |
|---|---|
| Single-guild / small bot | 2–4 workers |
| Medium bot (< 100 guilds) | 4–8 workers |
| Large bot (100+ guilds, sharded) | `available_parallelism()` × 2 |
| CPU-heavy handlers (image processing, etc.) | Separate dedicated pool per task type |

A rule of thumb: `available_parallelism()` returns the number of **logical CPU cores**. For I/O-bound handler work (REST calls, database writes), you can safely go 2× or more above this. For CPU-bound work, stay at or below core count.

---

## Thread Safety

All public types in `ThreadPool` and `PooledEventDispatcher` are `Send + Sync`. Handlers registered with `PooledEventDispatcher` must be `Fn(&Event) + Send + Sync + 'static`.
