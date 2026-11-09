//! Discord gateway intents.
//!
//! Intents are passed to the client when it
//! connects to the gateway and tell Discord which event buckets the bot wants
//! to receive.

/// Wraps the Discord gateway intent bitfield.
///
/// Individual flags can be queried and changed. In Rust this is expressed with 
/// associated constants plus accessor methods
/// example :
/// ```
///     use enums::Intents;
///
///     let mut intents = Intents::default();
///     assert!(intents.guilds());
///     assert!(!intents.members());
///
///     intents.set_members(true);
///     assert!(intents.members());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Intents(u64);

impl Intents {
    /// An empty set of intents.
    pub const NONE: Self = Self(0);
    /// if guild related events are enabled.
    pub const GUILDS: Self = Self(1 << 0);
    /// if guild member related events are enabled.
    pub const MEMBERS: Self = Self(1 << 1);
    /// if guild moderation related events are enabled.
    pub const MODERATION: Self = Self(1 << 2);
    /// if guild emoji, sticker, and soundboard sound events are enabled.
    pub const EXPRESSIONS: Self = Self(1 << 3);
    /// if guild integration related events are enabled.
    pub const INTEGRATIONS: Self = Self(1 << 4);
    /// if guild webhook related events are enabled.
    pub const WEBHOOKS: Self = Self(1 << 5);
    /// if guild invite related events are enabled.
    pub const INVITES: Self = Self(1 << 6);
    /// if guild voice state related events are enabled.
    pub const VOICE_STATES: Self = Self(1 << 7);
    /// if guild presence related events are enabled.
    pub const PRESENCES: Self = Self(1 << 8);
    /// if guild message related events are enabled.
    pub const GUILD_MESSAGES: Self = Self(1 << 9);
    /// if guild message reaction related events are enabled.
    pub const GUILD_REACTIONS: Self = Self(1 << 10);
    /// if guild typing related events are enabled.
    pub const GUILD_TYPING: Self = Self(1 << 11);
    /// if direct message related events are enabled.
    pub const DM_MESSAGES: Self = Self(1 << 12);
    /// if direct message reaction related events are enabled.
    pub const DM_REACTIONS: Self = Self(1 << 13);
    /// if direct message typing related events are enabled.
    pub const DM_TYPING: Self = Self(1 << 14);
    /// if message content, attachments, embeds and components are
    /// included in message events.
    pub const MESSAGE_CONTENT: Self = Self(1 << 15);
    /// if guild scheduled event related events are enabled.
    pub const GUILD_SCHEDULED_EVENTS: Self = Self(1 << 16);
    /// if auto moderation configuration events are enabled.
    pub const AUTO_MODERATION_CONFIGURATION: Self = Self(1 << 20);
    /// if auto moderation execution events are enabled.
    pub const AUTO_MODERATION_EXECUTION: Self = Self(1 << 21);
    /// if guild poll related events are enabled.
    pub const GUILD_POLLS: Self = Self(1 << 24);
    /// if direct message poll related events are enabled.
    pub const DM_POLLS: Self = Self(1 << 25);

    /// alias of [`Intents::MODERATION`].
    pub const BANS: Self = Self::MODERATION;
    /// alias of [`Intents::EXPRESSIONS`].
    pub const EMOJIS: Self = Self::EXPRESSIONS;
    /// alias of [`Intents::EXPRESSIONS`].
    pub const EMOJIS_AND_STICKERS: Self = Self::EXPRESSIONS;

    /// Shortcut for both [`Intents::GUILD_MESSAGES`] and
    /// [`Intents::DM_MESSAGES`].
    pub const MESSAGES: Self = Self(Self::GUILD_MESSAGES.0 | Self::DM_MESSAGES.0);
    /// Shortcut for both [`Intents::GUILD_REACTIONS`] and
    /// [`Intents::DM_REACTIONS`].
    pub const REACTIONS: Self = Self(Self::GUILD_REACTIONS.0 | Self::DM_REACTIONS.0);
    /// Shortcut for both [`Intents::GUILD_TYPING`] and
    /// [`Intents::DM_TYPING`].
    pub const TYPING: Self = Self(Self::GUILD_TYPING.0 | Self::DM_TYPING.0);
    /// Shortcut for both [`Intents::AUTO_MODERATION_CONFIGURATION`] and
    /// [`Intents::AUTO_MODERATION_EXECUTION`].
    pub const AUTO_MODERATION: Self =
        Self(Self::AUTO_MODERATION_CONFIGURATION.0 | Self::AUTO_MODERATION_EXECUTION.0);
    /// Shortcut for both [`Intents::GUILD_POLLS`] and [`Intents::DM_POLLS`].
    pub const POLLS: Self = Self(Self::GUILD_POLLS.0 | Self::DM_POLLS.0);

    /// Every intent defined by this crate.
    /// We only sets known flags, not
    /// reserved or undefined bits.
    pub const ALL: Self = Self(
        Self::GUILDS.0
            | Self::MEMBERS.0
            | Self::MODERATION.0
            | Self::EXPRESSIONS.0
            | Self::INTEGRATIONS.0
            | Self::WEBHOOKS.0
            | Self::INVITES.0
            | Self::VOICE_STATES.0
            | Self::PRESENCES.0
            | Self::GUILD_MESSAGES.0
            | Self::GUILD_REACTIONS.0
            | Self::GUILD_TYPING.0
            | Self::DM_MESSAGES.0
            | Self::DM_REACTIONS.0
            | Self::DM_TYPING.0
            | Self::MESSAGE_CONTENT.0
            | Self::GUILD_SCHEDULED_EVENTS.0
            | Self::AUTO_MODERATION_CONFIGURATION.0
            | Self::AUTO_MODERATION_EXECUTION.0
            | Self::GUILD_POLLS.0
            | Self::DM_POLLS.0,
    );

    /// Everything except [`Intents::PRESENCES`], [`Intents::MEMBERS`], and
    /// [`Intents::MESSAGE_CONTENT`].
    pub const DEFAULT: Self =
        Self(Self::ALL.0 & !(Self::PRESENCES.0 | Self::MEMBERS.0 | Self::MESSAGE_CONTENT.0));

    /// Creates an empty set of intents.
    #[must_use]
    pub const fn none() -> Self {
        Self::NONE
    }

    /// Creates a set with every known intent enabled.
    #[must_use]
    pub const fn all() -> Self {
        Self::ALL
    }

    /// Returns the raw bitfield.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Wraps a raw bitfield without validating it.
    #[must_use]
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Wraps a raw bitfield, dropping bits not defined by this crate.
    #[must_use]
    pub const fn from_bits_truncate(bits: u64) -> Self {
        Self(bits & Self::ALL.0)
    }

    /// Returns `true` if no intents are enabled.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if all flags in `other` are enabled in `self`.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Returns `true` if at least one flag in `other` is enabled in `self`.
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }

    /// Enables the flags in `other`.
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    /// Disables the flags in `other`.
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }

    /// Toggles the flags in `other`.
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.0;
    }

    /// Enables or disables the flags in `other`.
    pub fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    /// Returns the union of both sets.
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Returns the intersection of both sets.
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Returns the flags in `self` that are not in `other`.
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// Returns the flags enabled in exactly one of the two sets.
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    /// Returns every defined flag that is not enabled in `self`.
    /// Only flags known to this crate are returned without leaking reserved bits
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0 & Self::ALL.0)
    }
}

impl Default for Intents {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl core::ops::BitOr for Intents {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl core::ops::BitOrAssign for Intents {
    fn bitor_assign(&mut self, rhs: Self) {
        self.insert(rhs);
    }
}

impl core::ops::BitAnd for Intents {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl core::ops::BitAndAssign for Intents {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl core::ops::BitXor for Intents {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl core::ops::BitXorAssign for Intents {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.toggle(rhs);
    }
}

impl core::ops::Not for Intents {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl From<u64> for Intents {
    fn from(bits: u64) -> Self {
        Self::from_bits(bits)
    }
}

impl From<Intents> for u64 {
    fn from(intents: Intents) -> Self {
        intents.bits()
    }
}

macro_rules! intent_accessors {
    ($(($constant:ident, $getter:ident, $setter:ident)),* $(,)?) => {
        $(
            /// Returns whether this intent is enabled.
            #[must_use]
            pub const fn $getter(self) -> bool {
                self.contains(Self::$constant)
            }

            /// Enables or disables this intent.
            pub fn $setter(&mut self, value: bool) {
                self.set(Self::$constant, value);
            }
        )*
    };
}

impl Intents {
    intent_accessors! {
        (GUILDS, guilds, set_guilds),
        (MEMBERS, members, set_members),
        (MODERATION, moderation, set_moderation),
        (BANS, bans, set_bans),
        (EXPRESSIONS, expressions, set_expressions),
        (EMOJIS, emojis, set_emojis),
        (EMOJIS_AND_STICKERS, emojis_and_stickers, set_emojis_and_stickers),
        (INTEGRATIONS, integrations, set_integrations),
        (WEBHOOKS, webhooks, set_webhooks),
        (INVITES, invites, set_invites),
        (VOICE_STATES, voice_states, set_voice_states),
        (PRESENCES, presences, set_presences),
        (MESSAGES, messages, set_messages),
        (GUILD_MESSAGES, guild_messages, set_guild_messages),
        (DM_MESSAGES, dm_messages, set_dm_messages),
        (REACTIONS, reactions, set_reactions),
        (GUILD_REACTIONS, guild_reactions, set_guild_reactions),
        (DM_REACTIONS, dm_reactions, set_dm_reactions),
        (TYPING, typing, set_typing),
        (GUILD_TYPING, guild_typing, set_guild_typing),
        (DM_TYPING, dm_typing, set_dm_typing),
        (MESSAGE_CONTENT, message_content, set_message_content),
        (GUILD_SCHEDULED_EVENTS, guild_scheduled_events, set_guild_scheduled_events),
        (AUTO_MODERATION, auto_moderation, set_auto_moderation),
        (
            AUTO_MODERATION_CONFIGURATION,
            auto_moderation_configuration,
            set_auto_moderation_configuration
        ),
        (
            AUTO_MODERATION_EXECUTION,
            auto_moderation_execution,
            set_auto_moderation_execution
        ),
        (POLLS, polls, set_polls),
        (GUILD_POLLS, guild_polls, set_guild_polls),
        (DM_POLLS, dm_polls, set_dm_polls),
    }
}

#[cfg(test)]
mod tests {
    use super::Intents;

    #[test]
    fn default_matches_discord_py() {
        let default = Intents::default();

        assert!(default.guilds());
        assert!(default.guild_messages());
        assert!(default.dm_messages());
        assert!(default.guild_scheduled_events());
        assert!(default.auto_moderation());
        assert!(default.polls());
        assert!(!default.presences());
        assert!(!default.members());
        assert!(!default.message_content());
    }

    #[test]
    fn all_enables_every_defined_flag() {
        let all = Intents::all();

        assert!(all.guilds());
        assert!(all.members());
        assert!(all.presences());
        assert!(all.message_content());
        assert!(all.guild_polls());
        assert!(all.dm_polls());
        assert_eq!(all.complement(), Intents::none());
    }

    #[test]
    fn group_accessors_toggle_both_bits() {
        let mut intents = Intents::none();
        intents.set_messages(true);

        assert!(intents.guild_messages());
        assert!(intents.dm_messages());

        intents.set_messages(false);
        assert!(!intents.guild_messages());
        assert!(!intents.dm_messages());
    }

    #[test]
    fn group_accessor_requires_every_bit() {
        let mut intents = Intents::none();
        intents.set_guild_messages(true);

        assert!(!intents.messages());

        intents.set_dm_messages(true);
        assert!(intents.messages());
    }

    #[test]
    fn operators_and_bits_round_trip() {
        let intents = Intents::GUILDS | Intents::GUILD_MESSAGES;

        assert_eq!(intents.bits(), (1 << 0) | (1 << 9));
        assert!(intents.contains(Intents::GUILDS));
        assert!(!intents.contains(Intents::MEMBERS));
        assert_eq!(Intents::from_bits(intents.bits()), intents);
        assert_eq!(u64::from(intents), intents.bits());
    }

    #[test]
    fn complement_only_inverts_defined_flags() {
        let complement = !Intents::none();

        assert!(complement.guilds());
        assert!(complement.message_content());
        assert!(!complement.contains(Intents::from_bits(1 << 17)));
        assert!(!complement.contains(Intents::from_bits(1 << 23)));
        assert_eq!(complement, Intents::all());
        assert_eq!(!Intents::all(), Intents::none());
    }
}
