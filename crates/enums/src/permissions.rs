//! Discord permission bitfields. 
//! The raw value is a 53-bit flag array. As
//! with [`Intents`](crate::Intents), flags can be inspected and changed
//! individually, and the usual bitwise operators are implemented.

use core::cmp::Ordering;

/// Wraps the Discord permission bitfield.
///
/// ```
///     use enums::Permissions;
///
///     let mut permissions = Permissions::none();
///     permissions.set_send_messages(true);
///     permissions.insert(Permissions::VIEW_CHANNEL);
///
///     assert!(permissions.send_messages());
///     assert!(permissions.view_channel());
///     assert!(permissions.is_superset(Permissions::VIEW_CHANNEL));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Permissions(u64);

impl Permissions {
    /// No permissions.
    pub const NONE: Self = Self(0);

    /// Allows creation of instant invites.
    pub const CREATE_INSTANT_INVITE: Self = Self(1 << 0);
    /// Allows kicking members.
    pub const KICK_MEMBERS: Self = Self(1 << 1);
    /// Allows banning members.
    pub const BAN_MEMBERS: Self = Self(1 << 2);
    /// Allows all permissions and bypasses channel overwrites.
    pub const ADMINISTRATOR: Self = Self(1 << 3);
    /// Allows management and editing of channels.
    pub const MANAGE_CHANNELS: Self = Self(1 << 4);
    /// Allows management and editing of the guild.
    pub const MANAGE_GUILD: Self = Self(1 << 5);
    /// Allows adding reactions to messages.
    pub const ADD_REACTIONS: Self = Self(1 << 6);
    /// Allows viewing the audit log.
    pub const VIEW_AUDIT_LOG: Self = Self(1 << 7);
    /// Allows using priority speaker in a voice channel.
    pub const PRIORITY_SPEAKER: Self = Self(1 << 8);
    /// Allows streaming in a voice channel.
    pub const STREAM: Self = Self(1 << 9);
    /// Allows reading messages in a channel.
    pub const READ_MESSAGES: Self = Self(1 << 10);
    /// Alias of [`Permissions::READ_MESSAGES`].
    pub const VIEW_CHANNEL: Self = Self::READ_MESSAGES;
    /// Allows sending messages.
    pub const SEND_MESSAGES: Self = Self(1 << 11);
    /// Allows sending text-to-speech messages.
    pub const SEND_TTS_MESSAGES: Self = Self(1 << 12);
    /// Allows deleting messages from other users.
    pub const MANAGE_MESSAGES: Self = Self(1 << 13);
    /// Allows embedding links.
    pub const EMBED_LINKS: Self = Self(1 << 14);
    /// Allows attaching files.
    pub const ATTACH_FILES: Self = Self(1 << 15);
    /// Allows reading message history.
    pub const READ_MESSAGE_HISTORY: Self = Self(1 << 16);
    /// Allows mentioning `@everyone` and `@here`.
    pub const MENTION_EVERYONE: Self = Self(1 << 17);
    /// Allows using external emojis.
    pub const EXTERNAL_EMOJIS: Self = Self(1 << 18);
    /// Alias of [`Permissions::EXTERNAL_EMOJIS`].
    pub const USE_EXTERNAL_EMOJIS: Self = Self::EXTERNAL_EMOJIS;
    /// Allows viewing guild insights.
    pub const VIEW_GUILD_INSIGHTS: Self = Self(1 << 19);
    /// Allows connecting to a voice channel.
    pub const CONNECT: Self = Self(1 << 20);
    /// Allows speaking in a voice channel.
    pub const SPEAK: Self = Self(1 << 21);
    /// Allows muting members.
    pub const MUTE_MEMBERS: Self = Self(1 << 22);
    /// Allows deafening members.
    pub const DEAFEN_MEMBERS: Self = Self(1 << 23);
    /// Allows moving members between voice channels.
    pub const MOVE_MEMBERS: Self = Self(1 << 24);
    /// Allows using voice activity detection.
    pub const USE_VOICE_ACTIVATION: Self = Self(1 << 25);
    /// Allows changing one's own nickname.
    pub const CHANGE_NICKNAME: Self = Self(1 << 26);
    /// Allows managing other members' nicknames.
    pub const MANAGE_NICKNAMES: Self = Self(1 << 27);
    /// Allows managing roles.
    pub const MANAGE_ROLES: Self = Self(1 << 28);
    /// Alias of [`Permissions::MANAGE_ROLES`].
    pub const MANAGE_PERMISSIONS: Self = Self::MANAGE_ROLES;
    /// Allows managing webhooks.
    pub const MANAGE_WEBHOOKS: Self = Self(1 << 29);
    /// Allows managing emojis, stickers, and soundboard sounds.
    pub const MANAGE_EXPRESSIONS: Self = Self(1 << 30);
    /// Alias of [`Permissions::MANAGE_EXPRESSIONS`].
    pub const MANAGE_EMOJIS: Self = Self::MANAGE_EXPRESSIONS;
    /// Alias of [`Permissions::MANAGE_EXPRESSIONS`].
    pub const MANAGE_EMOJIS_AND_STICKERS: Self = Self::MANAGE_EXPRESSIONS;
    /// Allows using application commands.
    pub const USE_APPLICATION_COMMANDS: Self = Self(1 << 31);
    /// Allows requesting to speak in stage channels.
    pub const REQUEST_TO_SPEAK: Self = Self(1 << 32);
    /// Allows managing guild events.
    pub const MANAGE_EVENTS: Self = Self(1 << 33);
    /// Allows managing threads.
    pub const MANAGE_THREADS: Self = Self(1 << 34);
    /// Allows creating public threads.
    pub const CREATE_PUBLIC_THREADS: Self = Self(1 << 35);
    /// Allows creating private threads.
    pub const CREATE_PRIVATE_THREADS: Self = Self(1 << 36);
    /// Allows using external stickers.
    pub const EXTERNAL_STICKERS: Self = Self(1 << 37);
    /// Alias of [`Permissions::EXTERNAL_STICKERS`].
    pub const USE_EXTERNAL_STICKERS: Self = Self::EXTERNAL_STICKERS;
    /// Allows sending messages in threads.
    pub const SEND_MESSAGES_IN_THREADS: Self = Self(1 << 38);
    /// Allows launching embedded activities.
    pub const USE_EMBEDDED_ACTIVITIES: Self = Self(1 << 39);
    /// Allows timing out members.
    pub const MODERATE_MEMBERS: Self = Self(1 << 40);
    /// Allows viewing creator monetization analytics.
    pub const VIEW_CREATOR_MONETIZATION_ANALYTICS: Self = Self(1 << 41);
    /// Allows using the soundboard.
    pub const USE_SOUNDBOARD: Self = Self(1 << 42);
    /// Allows creating emojis, stickers, and soundboard sounds.
    pub const CREATE_EXPRESSIONS: Self = Self(1 << 43);
    /// Allows creating guild events.
    pub const CREATE_EVENTS: Self = Self(1 << 44);
    /// Allows using external sounds.
    pub const USE_EXTERNAL_SOUNDS: Self = Self(1 << 45);
    /// Allows sending voice messages.
    pub const SEND_VOICE_MESSAGES: Self = Self(1 << 46);
    /// Allows setting voice channel status.
    pub const SET_VOICE_CHANNEL_STATUS: Self = Self(1 << 48);
    /// Allows sending polls.
    pub const SEND_POLLS: Self = Self(1 << 49);
    /// Alias of [`Permissions::SEND_POLLS`].
    pub const CREATE_POLLS: Self = Self::SEND_POLLS;
    /// Allows using external apps.
    pub const USE_EXTERNAL_APPS: Self = Self(1 << 50);
    /// Allows pinning messages.
    pub const PIN_MESSAGES: Self = Self(1 << 51);
    /// Allows bypassing slowmode.
    pub const BYPASS_SLOWMODE: Self = Self(1 << 52);

    /// Every permission defined by Discord.
    /// note that reserved bit 47 is not set.
    pub const ALL: Self = Self(0x001F_7FFF_FFFF_FFFF);

    /// Creates an empty permission set.
    #[must_use]
    pub const fn none() -> Self {
        Self::NONE
    }

    /// Creates a set with every known permission enabled.
    #[must_use]
    pub const fn all() -> Self {
        Self::ALL
    }

    /// All channel-specific permissions, with guild-specific ones unset.
    #[must_use]
    pub const fn all_channel() -> Self {
        Self(0x001E_64FD_B3F7_FF51)
    }

    /// All "General" permissions from the official Discord UI.
    #[must_use]
    pub const fn general() -> Self {
        Self(0x0000_0A00_7008_04B0)
    }

    /// All "Membership" permissions from the official Discord UI.
    #[must_use]
    pub const fn membership() -> Self {
        Self(0x0000_0100_0C00_0007)
    }

    /// All "Text" permissions from the official Discord UI.
    #[must_use]
    pub const fn text() -> Self {
        Self(0x001E_407C_8007_F840)
    }

    /// All "Voice" permissions from the official Discord UI.
    #[must_use]
    pub const fn voice() -> Self {
        Self(0x0001_2480_03F0_0300)
    }

    /// All "Stage Channel" permissions from the official Discord UI.
    #[must_use]
    pub const fn stage() -> Self {
        Self(1 << 32)
    }

    /// All permissions for stage moderators.
    #[must_use]
    pub const fn stage_moderator() -> Self {
        Self(0x0000_0000_0140_0010)
    }

    /// All permissions that require two-factor authentication.
    #[must_use]
    pub const fn elevated() -> Self {
        Self(0x0000_0104_7000_203E)
    }

    /// All "Apps" permissions from the official Discord UI.
    #[must_use]
    pub const fn apps() -> Self {
        Self(0x0004_0080_8000_0000)
    }

    /// All "Events" permissions from the official Discord UI.
    #[must_use]
    pub const fn events() -> Self {
        Self(0x0000_1002_0000_0000)
    }

    /// All "Advanced" permissions from the official Discord UI.
    #[must_use]
    pub const fn advanced() -> Self {
        Self::ADMINISTRATOR
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

    /// Returns `true` if no permissions are enabled.
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
    /// Only flags known to this crate are returned without leaking reserved bits.
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0 & Self::ALL.0)
    }

    /// Returns `true` if `self` has the same or fewer permissions than
    /// `other`.
    #[must_use]
    pub const fn is_subset(self, other: Self) -> bool {
        (self.0 & other.0) == self.0
    }

    /// Returns `true` if `self` has the same or more permissions than
    /// `other`.
    #[must_use]
    pub const fn is_superset(self, other: Self) -> bool {
        (self.0 | other.0) == self.0
    }

    /// Returns `true` if `self` is a strict subset of `other`.
    #[must_use]
    pub const fn is_strict_subset(self, other: Self) -> bool {
        self.is_subset(other) && self.0 != other.0
    }

    /// Returns `true` if `self` is a strict superset of `other`.
    #[must_use]
    pub const fn is_strict_superset(self, other: Self) -> bool {
        self.is_superset(other) && self.0 != other.0
    }
}

impl Default for Permissions {
    fn default() -> Self {
        Self::NONE
    }
}

impl PartialOrd for Permissions {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.is_subset(*other), self.is_superset(*other)) {
            (true, true) => Some(Ordering::Equal),
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            (false, false) => None,
        }
    }
}

impl core::ops::BitOr for Permissions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl core::ops::BitOrAssign for Permissions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.insert(rhs);
    }
}

impl core::ops::BitAnd for Permissions {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl core::ops::BitAndAssign for Permissions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl core::ops::BitXor for Permissions {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl core::ops::BitXorAssign for Permissions {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.toggle(rhs);
    }
}

impl core::ops::Not for Permissions {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl From<u64> for Permissions {
    fn from(bits: u64) -> Self {
        Self::from_bits(bits)
    }
}

impl From<Permissions> for u64 {
    fn from(permissions: Permissions) -> Self {
        permissions.bits()
    }
}

macro_rules! permission_accessors {
    ($(($constant:ident, $getter:ident, $setter:ident)),* $(,)?) => {
        $(
            /// Returns whether this permission is enabled.
            #[must_use]
            pub const fn $getter(self) -> bool {
                self.contains(Self::$constant)
            }

            /// Enables or disables this permission.
            pub fn $setter(&mut self, value: bool) {
                self.set(Self::$constant, value);
            }
        )*
    };
}

impl Permissions {
    permission_accessors! {
        (CREATE_INSTANT_INVITE, create_instant_invite, set_create_instant_invite),
        (KICK_MEMBERS, kick_members, set_kick_members),
        (BAN_MEMBERS, ban_members, set_ban_members),
        (ADMINISTRATOR, administrator, set_administrator),
        (MANAGE_CHANNELS, manage_channels, set_manage_channels),
        (MANAGE_GUILD, manage_guild, set_manage_guild),
        (ADD_REACTIONS, add_reactions, set_add_reactions),
        (VIEW_AUDIT_LOG, view_audit_log, set_view_audit_log),
        (PRIORITY_SPEAKER, priority_speaker, set_priority_speaker),
        (STREAM, stream, set_stream),
        (READ_MESSAGES, read_messages, set_read_messages),
        (VIEW_CHANNEL, view_channel, set_view_channel),
        (SEND_MESSAGES, send_messages, set_send_messages),
        (SEND_TTS_MESSAGES, send_tts_messages, set_send_tts_messages),
        (MANAGE_MESSAGES, manage_messages, set_manage_messages),
        (EMBED_LINKS, embed_links, set_embed_links),
        (ATTACH_FILES, attach_files, set_attach_files),
        (READ_MESSAGE_HISTORY, read_message_history, set_read_message_history),
        (MENTION_EVERYONE, mention_everyone, set_mention_everyone),
        (EXTERNAL_EMOJIS, external_emojis, set_external_emojis),
        (USE_EXTERNAL_EMOJIS, use_external_emojis, set_use_external_emojis),
        (VIEW_GUILD_INSIGHTS, view_guild_insights, set_view_guild_insights),
        (CONNECT, connect, set_connect),
        (SPEAK, speak, set_speak),
        (MUTE_MEMBERS, mute_members, set_mute_members),
        (DEAFEN_MEMBERS, deafen_members, set_deafen_members),
        (MOVE_MEMBERS, move_members, set_move_members),
        (USE_VOICE_ACTIVATION, use_voice_activation, set_use_voice_activation),
        (CHANGE_NICKNAME, change_nickname, set_change_nickname),
        (MANAGE_NICKNAMES, manage_nicknames, set_manage_nicknames),
        (MANAGE_ROLES, manage_roles, set_manage_roles),
        (MANAGE_PERMISSIONS, manage_permissions, set_manage_permissions),
        (MANAGE_WEBHOOKS, manage_webhooks, set_manage_webhooks),
        (MANAGE_EXPRESSIONS, manage_expressions, set_manage_expressions),
        (MANAGE_EMOJIS, manage_emojis, set_manage_emojis),
        (
            MANAGE_EMOJIS_AND_STICKERS,
            manage_emojis_and_stickers,
            set_manage_emojis_and_stickers
        ),
        (USE_APPLICATION_COMMANDS, use_application_commands, set_use_application_commands),
        (REQUEST_TO_SPEAK, request_to_speak, set_request_to_speak),
        (MANAGE_EVENTS, manage_events, set_manage_events),
        (MANAGE_THREADS, manage_threads, set_manage_threads),
        (CREATE_PUBLIC_THREADS, create_public_threads, set_create_public_threads),
        (CREATE_PRIVATE_THREADS, create_private_threads, set_create_private_threads),
        (EXTERNAL_STICKERS, external_stickers, set_external_stickers),
        (USE_EXTERNAL_STICKERS, use_external_stickers, set_use_external_stickers),
        (
            SEND_MESSAGES_IN_THREADS,
            send_messages_in_threads,
            set_send_messages_in_threads
        ),
        (
            USE_EMBEDDED_ACTIVITIES,
            use_embedded_activities,
            set_use_embedded_activities
        ),
        (MODERATE_MEMBERS, moderate_members, set_moderate_members),
        (
            VIEW_CREATOR_MONETIZATION_ANALYTICS,
            view_creator_monetization_analytics,
            set_view_creator_monetization_analytics
        ),
        (USE_SOUNDBOARD, use_soundboard, set_use_soundboard),
        (CREATE_EXPRESSIONS, create_expressions, set_create_expressions),
        (CREATE_EVENTS, create_events, set_create_events),
        (USE_EXTERNAL_SOUNDS, use_external_sounds, set_use_external_sounds),
        (SEND_VOICE_MESSAGES, send_voice_messages, set_send_voice_messages),
        (
            SET_VOICE_CHANNEL_STATUS,
            set_voice_channel_status,
            set_set_voice_channel_status
        ),
        (SEND_POLLS, send_polls, set_send_polls),
        (CREATE_POLLS, create_polls, set_create_polls),
        (USE_EXTERNAL_APPS, use_external_apps, set_use_external_apps),
        (PIN_MESSAGES, pin_messages, set_pin_messages),
        (BYPASS_SLOWMODE, bypass_slowmode, set_bypass_slowmode),
    }
}

#[cfg(test)]
mod tests {
    use super::Permissions;

    #[test]
    fn none_and_all_match_discord_py() {
        assert_eq!(Permissions::none(), Permissions::default());
        assert_eq!(Permissions::none().bits(), 0);
        assert_eq!(Permissions::all().bits(), 0x001F_7FFF_FFFF_FFFF);
        assert_eq!(!Permissions::none(), Permissions::all());
        assert_eq!(!Permissions::all(), Permissions::none());
    }

    #[test]
    fn factories_match_discord_py_masks() {
        assert_eq!(Permissions::all_channel().bits(), 0x001E_64FD_B3F7_FF51);
        assert_eq!(Permissions::general().bits(), 0x0000_0A00_7008_04B0);
        assert_eq!(Permissions::membership().bits(), 0x0000_0100_0C00_0007);
        assert_eq!(Permissions::text().bits(), 0x001E_407C_8007_F840);
        assert_eq!(Permissions::voice().bits(), 0x0001_2480_03F0_0300);
        assert_eq!(Permissions::stage(), Permissions::REQUEST_TO_SPEAK);
        assert_eq!(Permissions::stage_moderator().bits(), 0x0000_0000_0140_0010);
        assert_eq!(Permissions::elevated().bits(), 0x0000_0104_7000_203E);
        assert_eq!(Permissions::apps().bits(), 0x0004_0080_8000_0000);
        assert_eq!(Permissions::events().bits(), 0x0000_1002_0000_0000);
        assert_eq!(Permissions::advanced(), Permissions::ADMINISTRATOR);
    }

    #[test]
    fn aliases_share_the_same_bit() {
        assert_eq!(Permissions::VIEW_CHANNEL, Permissions::READ_MESSAGES);
        assert_eq!(
            Permissions::USE_EXTERNAL_EMOJIS,
            Permissions::EXTERNAL_EMOJIS
        );
        assert_eq!(Permissions::MANAGE_PERMISSIONS, Permissions::MANAGE_ROLES);
        assert_eq!(
            Permissions::MANAGE_EMOJIS_AND_STICKERS,
            Permissions::MANAGE_EXPRESSIONS
        );
        assert_eq!(
            Permissions::USE_EXTERNAL_STICKERS,
            Permissions::EXTERNAL_STICKERS
        );
        assert_eq!(Permissions::CREATE_POLLS, Permissions::SEND_POLLS);
    }

    #[test]
    fn accessors_are_two_way() {
        let mut permissions = Permissions::none();

        assert!(!permissions.send_messages());
        permissions.set_send_messages(true);
        assert!(permissions.send_messages());
        permissions.set_send_messages(false);
        assert!(!permissions.send_messages());

        permissions.insert(Permissions::VIEW_CHANNEL);
        assert!(permissions.read_messages());
        assert!(permissions.view_channel());
    }

    #[test]
    fn subset_and_superset_ordering() {
        let view = Permissions::VIEW_CHANNEL;
        let both = view | Permissions::SEND_MESSAGES;

        assert!(view.is_subset(both));
        assert!(view.is_strict_subset(both));
        assert!(both.is_superset(view));
        assert!(both.is_strict_superset(view));
        assert!(!both.is_subset(view));
        assert_eq!(view.partial_cmp(&view), Some(core::cmp::Ordering::Equal));
        assert_eq!(view.partial_cmp(&both), Some(core::cmp::Ordering::Less));
        assert_eq!(both.partial_cmp(&view), Some(core::cmp::Ordering::Greater));

        let unrelated = Permissions::BAN_MEMBERS | Permissions::KICK_MEMBERS;
        assert_eq!(view.partial_cmp(&unrelated), None);
    }

    #[test]
    fn raw_round_trip() {
        let permissions = Permissions::SEND_MESSAGES | Permissions::ATTACH_FILES;

        assert_eq!(Permissions::from_bits(permissions.bits()), permissions);
        assert_eq!(Permissions::from(permissions.bits()), permissions);
        assert_eq!(u64::from(permissions), permissions.bits());
        assert_eq!(
            Permissions::from_bits_truncate(u64::MAX),
            Permissions::all()
        );
    }
}
