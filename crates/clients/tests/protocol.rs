use clients::{Cache, ShardManager};

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
