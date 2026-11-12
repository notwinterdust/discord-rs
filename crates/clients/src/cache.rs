//! Bounded identifier-keyed caches for client models.
//!
//! [`Cache`] is the low-level building block: a bounded `HashMap` keyed by
//! `u64` ID.  [`ClientCache`] composes per-type [`Cache`]s and updates them
//! when fed [`Event`] values from the gateway.

use std::collections::HashMap;

/// A simple bounded cache keyed by Discord IDs.
#[derive(Clone, Debug)]
pub struct Cache<T> {
    capacity: usize,
    values: HashMap<u64, T>,
}

impl<T> Cache<T> {
    /// Creates a cache with a maximum entry count.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            values: HashMap::new(),
        }
    }

    /// Inserts a value, evicting an arbitrary entry when full.
    pub fn insert(&mut self, id: u64, value: T) {
        if self.capacity == 0 {
            return;
        }
        if self.values.len() >= self.capacity && !self.values.contains_key(&id) {
            if let Some(key) = self.values.keys().next().copied() {
                self.values.remove(&key);
            }
        }
        self.values.insert(id, value);
    }

    /// Returns a cached value.
    #[must_use]
    pub fn get(&self, id: u64) -> Option<&T> {
        self.values.get(&id)
    }

    /// Removes a cached value.
    pub fn remove(&mut self, id: u64) -> Option<T> {
        self.values.remove(&id)
    }

    /// Clears the cache.
    pub fn clear(&mut self) {
        self.values.clear();
    }

    /// Removes all identifiers supplied by an invalidation event.
    pub fn invalidate<I>(&mut self, ids: I)
    where
        I: IntoIterator<Item = u64>,
    {
        for id in ids {
            self.values.remove(&id);
        }
    }

    /// Returns the current number of entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns `true` when the cache holds no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns an iterator over all cached values.
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.values.values()
    }
}

// ---------------------------------------------------------------------------
// Per-guild member sub-cache
// ---------------------------------------------------------------------------

/// Members belonging to a single guild, keyed by user ID.
#[derive(Clone, Debug, Default)]
pub struct MemberCache {
    members: HashMap<u64, models::Member>,
}

impl MemberCache {
    fn insert(&mut self, member: models::Member) {
        self.members.insert(member.user.id.get(), member);
    }

    fn remove(&mut self, user_id: u64) {
        self.members.remove(&user_id);
    }

    /// Returns the cached member for the given user ID.
    #[must_use]
    pub fn get(&self, user_id: u64) -> Option<&models::Member> {
        self.members.get(&user_id)
    }

    /// Returns the number of cached members.
    #[must_use]
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// Returns `true` when no members are cached.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Composite client cache
// ---------------------------------------------------------------------------

/// Configuration for [`ClientCache`] entry limits.
#[derive(Clone, Debug)]
pub struct CacheConfig {
    /// Maximum number of guilds to cache (0 = disabled).
    pub guilds: usize,
    /// Maximum number of channels to cache.
    pub channels: usize,
    /// Maximum number of roles to cache.
    pub roles: usize,
    /// Maximum number of users to cache.
    pub users: usize,
    /// Maximum number of messages to cache.
    pub messages: usize,
    /// Maximum number of presences to cache.
    pub presences: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            guilds: 1_000,
            channels: 10_000,
            roles: 10_000,
            users: 10_000,
            messages: 1_000,
            presences: 10_000,
        }
    }
}

/// Composite cache holding typed per-resource stores updated from gateway events.
///
/// Call [`ClientCache::update`] with every [`models::Event`] received from the
/// gateway to keep the caches consistent.
#[derive(Clone, Debug)]
pub struct ClientCache {
    /// Cached guilds.
    pub guilds: Cache<models::Guild>,
    /// Cached channels (guild channels and threads).
    pub channels: Cache<models::GuildChannel>,
    /// Cached roles, keyed by role ID.
    pub roles: Cache<models::Role>,
    /// Cached users (populated from member/presence events).
    pub users: Cache<models::User>,
    /// Cached messages.
    pub messages: Cache<models::Message>,
    /// Cached presences, keyed by user ID.
    pub presences: Cache<models::PresenceUpdate>,
    /// Per-guild member caches, keyed by guild ID.
    pub members: HashMap<u64, MemberCache>,
}

impl ClientCache {
    /// Creates a new cache with the given configuration.
    #[must_use]
    pub fn new(config: CacheConfig) -> Self {
        Self {
            guilds: Cache::new(config.guilds),
            channels: Cache::new(config.channels),
            roles: Cache::new(config.roles),
            users: Cache::new(config.users),
            messages: Cache::new(config.messages),
            presences: Cache::new(config.presences),
            members: HashMap::new(),
        }
    }

    /// Creates a cache with default capacity limits.
    #[must_use]
    pub fn default_config() -> Self {
        Self::new(CacheConfig::default())
    }

    /// Updates the cache from a gateway event.
    ///
    /// Returns `true` when the event caused at least one cache mutation.
    pub fn update(&mut self, event: &models::Event) -> bool {
        match event {
            // ---- Guilds ------------------------------------------------
            models::Event::GuildCreate(guild) => {
                self.cache_guild(guild.clone());
                true
            }
            models::Event::GuildUpdate(guild) => {
                self.cache_guild(guild.clone());
                true
            }
            models::Event::GuildDelete(unavailable) => {
                self.guilds.remove(unavailable.id.get());
                self.members.remove(&unavailable.id.get());
                true
            }

            // ---- Channels ----------------------------------------------
            models::Event::ChannelCreate(ch)
            | models::Event::ChannelUpdate(ch)
            | models::Event::ThreadCreate(ch)
            | models::Event::ThreadUpdate(ch) => {
                self.channels.insert(ch.id.get(), ch.clone());
                true
            }
            models::Event::ChannelDelete(ch) => {
                self.channels.remove(ch.id.get());
                true
            }
            models::Event::ThreadDelete(td) => {
                self.channels.remove(td.id.get());
                true
            }

            // ---- Members -----------------------------------------------
            models::Event::GuildMemberAdd(member) | models::Event::GuildMemberUpdate(member) => {
                let guild_id = member.guild_id.get();
                self.members
                    .entry(guild_id)
                    .or_default()
                    .insert(member.clone());
                self.users.insert(member.user.id.get(), member.user.clone());
                true
            }
            models::Event::GuildMemberRemove(remove) => {
                let guild_id = remove.guild_id.get();
                if let Some(mc) = self.members.get_mut(&guild_id) {
                    mc.remove(remove.user.id.get());
                }
                true
            }
            models::Event::GuildMembersChunk(chunk) => {
                let gc = self.members.entry(chunk.guild_id.get()).or_default();
                for member in &chunk.members {
                    gc.insert(member.clone());
                    self.users.insert(member.user.id.get(), member.user.clone());
                }
                true
            }

            // ---- Roles -------------------------------------------------
            models::Event::GuildRoleCreate(ev) | models::Event::GuildRoleUpdate(ev) => {
                self.roles.insert(ev.role.id.get(), ev.role.clone());
                true
            }
            models::Event::GuildRoleDelete(ev) => {
                self.roles.remove(ev.role_id.get());
                true
            }

            // ---- Messages ----------------------------------------------
            models::Event::MessageCreate(msg) | models::Event::MessageUpdate(msg) => {
                self.messages.insert(msg.id.get(), msg.clone());
                true
            }
            models::Event::MessageDelete(del) => {
                self.messages.remove(del.id.get());
                true
            }
            models::Event::MessageDeleteBulk(bulk) => {
                for id in &bulk.ids {
                    self.messages.remove(id.get());
                }
                true
            }

            // ---- Presences ---------------------------------------------
            models::Event::PresenceUpdate(presence) => {
                self.presences
                    .insert(presence.user_id.get(), presence.clone());
                true
            }

            // ---- User update -------------------------------------------
            models::Event::UserUpdate(user) => {
                self.users.insert(user.id.get(), user.clone());
                true
            }

            // All other events do not mutate the cache.
            _ => false,
        }
    }

    /// Looks up a member in the per-guild cache.
    #[must_use]
    pub fn member(&self, guild_id: u64, user_id: u64) -> Option<&models::Member> {
        self.members.get(&guild_id)?.get(user_id)
    }

    /// Returns all cached members for a guild.
    #[must_use]
    pub fn guild_members(&self, guild_id: u64) -> Option<&MemberCache> {
        self.members.get(&guild_id)
    }

    // ---- private helpers -----------------------------------------------

    fn cache_guild(&mut self, guild: models::Guild) {
        // Seed per-guild caches from the GUILD_CREATE payload.
        for ch in guild.channels.iter().chain(guild.threads()) {
            self.channels.insert(ch.id.get(), ch.clone());
        }
        for role in &guild.roles {
            self.roles.insert(role.id.get(), role.clone());
        }
        let mc = self.members.entry(guild.id.get()).or_default();
        for member in &guild.members {
            mc.insert(member.clone());
            self.users.insert(member.user.id.get(), member.user.clone());
        }
        self.guilds.insert(guild.id.get(), guild);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cache() -> ClientCache {
        ClientCache::new(CacheConfig {
            guilds: 10,
            channels: 100,
            roles: 100,
            users: 100,
            messages: 50,
            presences: 100,
        })
    }

    #[test]
    fn cache_insert_get_remove() {
        let mut c: Cache<String> = Cache::new(3);
        c.insert(1, "a".into());
        c.insert(2, "b".into());
        assert_eq!(c.get(1), Some(&"a".to_owned()));
        assert_eq!(c.len(), 2);
        c.remove(1);
        assert_eq!(c.get(1), None);
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn cache_capacity_evicts_on_overflow() {
        let mut c: Cache<u32> = Cache::new(2);
        c.insert(1, 1);
        c.insert(2, 2);
        c.insert(3, 3); // should evict one of 1 or 2
        assert_eq!(c.len(), 2);
        assert!(c.get(3).is_some());
    }

    #[test]
    fn cache_zero_capacity_accepts_nothing() {
        let mut c: Cache<u32> = Cache::new(0);
        c.insert(1, 1);
        assert!(c.is_empty());
    }

    #[test]
    fn cache_invalidate_removes_entries() {
        let mut c: Cache<u32> = Cache::new(10);
        c.insert(1, 1);
        c.insert(2, 2);
        c.insert(3, 3);
        c.invalidate([1_u64, 3]);
        assert_eq!(c.len(), 1);
        assert!(c.get(2).is_some());
        assert!(c.get(1).is_none());
    }
}
