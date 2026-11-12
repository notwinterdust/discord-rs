use clients::{Cache, ShardManager};
use discord_rust_clients as clients;

#[test]
fn shard_manager_assigns_all_shards() {
    let manager = ShardManager::new(4, 2).unwrap();
    assert_eq!(manager.shards().len(), 4);
    assert_eq!(manager.identify_delay(manager.shards()[0]).as_secs(), 0);
    assert_eq!(manager.identify_delay(manager.shards()[1]).as_secs(), 5);
}

#[test]
fn cache_evicts_when_full() {
    let mut cache = Cache::new(1);
    cache.insert(1, "one");
    cache.insert(2, "two");
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.get(2), Some(&"two"));
}

#[test]
fn threaded_gateway_lifecycle_round_trip() {
    use std::collections::VecDeque;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::thread;
    use std::time::Duration;

    use clients::{
        GatewayConfig, GatewayConnection, GatewayFrame, GatewayTransport, GatewayTransportError,
        ThreadedCoordinator, ThreadedEventDispatcher,
    };
    use data::Value;
    use data::gateway::{GatewayOpcode, GatewayPayload, Hello};
    use enums::Intents;

    struct ScriptedTransport {
        frames: VecDeque<GatewayFrame>,
        sent_heartbeats: AtomicUsize,
    }

    impl GatewayTransport for ScriptedTransport {
        fn connect(&mut self, _url: &str) -> Result<(), GatewayTransportError> {
            Ok(())
        }

        fn recv(
            &mut self,
            _timeout: Option<Duration>,
        ) -> Result<GatewayFrame, GatewayTransportError> {
            if let Some(frame) = self.frames.pop_front() {
                Ok(frame)
            } else {
                thread::sleep(Duration::from_millis(5));
                Ok(GatewayFrame::Timeout)
            }
        }

        fn send(&mut self, text: &str) -> Result<(), GatewayTransportError> {
            if text.contains(r#""op":1"#) {
                self.sent_heartbeats.fetch_add(1, Ordering::SeqCst);
            }
            Ok(())
        }

        fn close(&mut self) {}
    }

    let mut frames = VecDeque::new();
    let hello = GatewayPayload::new(
        GatewayOpcode::Hello,
        Hello {
            heartbeat_interval: 10,
        }
        .to_value(),
    )
    .to_json();
    frames.push_back(GatewayFrame::Text(hello));

    let mut typing_data = data::Object::new();
    typing_data.insert("channel_id", Value::String("123".to_string()));
    typing_data.insert("user_id", Value::String("456".to_string()));
    typing_data.insert("timestamp", Value::from(1700000000_u64));

    let dispatch = GatewayPayload::new(GatewayOpcode::Dispatch, Value::Object(typing_data))
        .with_sequence(1)
        .with_event("TYPING_START")
        .to_json();
    frames.push_back(GatewayFrame::Text(dispatch));

    let ack = GatewayPayload::new(GatewayOpcode::HeartbeatAck, Value::Null).to_json();
    frames.push_back(GatewayFrame::Text(ack));

    let transport = Arc::new(std::sync::Mutex::new(ScriptedTransport {
        frames,
        sent_heartbeats: AtomicUsize::new(0),
    }));
    let transport_clone = Arc::clone(&transport);

    struct SharedTransport(Arc<std::sync::Mutex<ScriptedTransport>>);

    impl GatewayTransport for SharedTransport {
        fn connect(&mut self, url: &str) -> Result<(), GatewayTransportError> {
            self.0.lock().unwrap().connect(url)
        }

        fn recv(
            &mut self,
            timeout: Option<Duration>,
        ) -> Result<GatewayFrame, GatewayTransportError> {
            self.0.lock().unwrap().recv(timeout)
        }

        fn send(&mut self, text: &str) -> Result<(), GatewayTransportError> {
            self.0.lock().unwrap().send(text)
        }

        fn close(&mut self) {
            self.0.lock().unwrap().close();
        }
    }

    let event_count = Arc::new(AtomicUsize::new(0));
    let ec = Arc::clone(&event_count);

    let dispatcher = ThreadedEventDispatcher::builder()
        .on_name("TYPING_START", move |_| {
            ec.fetch_add(1, Ordering::SeqCst);
        })
        .build();

    let config = GatewayConfig::new("test-token", Intents::none());

    let mut coordinator = ThreadedCoordinator::spawn(move |flag: Arc<AtomicBool>| {
        let mut connection = GatewayConnection::new(config, SharedTransport(transport_clone));
        connection.connect()?;
        while !flag.load(Ordering::SeqCst) {
            match connection.poll() {
                Ok(Some(update)) => {
                    if let Some(event) = update.event {
                        let _ = dispatcher.dispatch(event);
                    }
                }
                Ok(None) => {
                    thread::sleep(Duration::from_millis(5));
                }
                Err(err) => {
                    eprintln!("Poll error in test: {:?}", err);
                    return Err(err);
                }
            }
        }
        Ok(())
    });

    let deadline = std::time::Instant::now() + Duration::from_millis(500);
    while event_count.load(Ordering::SeqCst) < 1 && std::time::Instant::now() < deadline {
        thread::sleep(Duration::from_millis(5));
    }
    assert!(coordinator.shutdown().is_ok());
    assert_eq!(event_count.load(Ordering::SeqCst), 1);
}

#[test]
fn concurrent_shard_orchestration_and_startup_buckets() {
    use clients::{ConcurrentShardRuntime, ShardManager};

    let manager = ShardManager::from_gateway_recommendation(2, 1).unwrap();
    assert_eq!(manager.shards().len(), 2);
    // Bucket verification
    assert_eq!(manager.identify_delay(manager.shards()[0]).as_secs(), 0);
    assert_eq!(manager.identify_delay(manager.shards()[1]).as_secs(), 0);

    let runtime = ConcurrentShardRuntime::new(manager);

    let handles = runtime.spawn(move |shard, tx| {
        let _ = tx.send((shard, format!("SHARD_{}_READY", shard.id)));
    });

    assert_eq!(handles.len(), 2);

    let mut ready_shards = Vec::new();
    for _ in 0..2 {
        if let Ok((shard, msg)) = runtime
            .event_rx
            .recv_timeout(std::time::Duration::from_secs(1))
        {
            ready_shards.push((shard.id, msg));
        }
    }

    assert_eq!(ready_shards.len(), 2);
    for handle in handles {
        assert!(handle.join().is_ok());
    }
}
