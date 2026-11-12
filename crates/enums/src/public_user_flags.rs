//! Public user flags.
//!
//! These are the flags Discord exposes on
//! user objects; the raw value may also contain private flags, which this type
//! preserves but does not interpret.

/// Wraps the publicly available Discord user flags.
///
/// ```
///     use discord_rust_enums::PublicUserFlags;
///
///     let flags = PublicUserFlags::STAFF | PublicUserFlags::VERIFIED_BOT;
///     assert!(flags.staff());
///     assert!(flags.verified_bot());
///     assert!(!flags.partner());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PublicUserFlags(u64);

impl PublicUserFlags {
    /// No flags.
    pub const NONE: Self = Self(0);

    /// Discord Employee.
    pub const STAFF: Self = Self(1 << 0);
    /// Discord Partner.
    pub const PARTNER: Self = Self(1 << 1);
    /// HypeSquad Events member.
    pub const HYPESQUAD: Self = Self(1 << 2);
    /// Bug Hunter.
    pub const BUG_HUNTER: Self = Self(1 << 3);
    /// HypeSquad Bravery member.
    pub const HYPESQUAD_BRAVERY: Self = Self(1 << 6);
    /// HypeSquad Brilliance member.
    pub const HYPESQUAD_BRILLIANCE: Self = Self(1 << 7);
    /// HypeSquad Balance member.
    pub const HYPESQUAD_BALANCE: Self = Self(1 << 8);
    /// Early Supporter.
    pub const EARLY_SUPPORTER: Self = Self(1 << 9);
    /// Team User.
    pub const TEAM_USER: Self = Self(1 << 10);
    /// System user representing Discord officially.
    pub const SYSTEM: Self = Self(1 << 12);
    /// Bug Hunter Level 2.
    pub const BUG_HUNTER_LEVEL_2: Self = Self(1 << 14);
    /// Verified Bot.
    pub const VERIFIED_BOT: Self = Self(1 << 16);
    /// Early Verified Bot Developer.
    pub const VERIFIED_BOT_DEVELOPER: Self = Self(1 << 17);
    /// Alias of [`PublicUserFlags::VERIFIED_BOT_DEVELOPER`].
    pub const EARLY_VERIFIED_BOT_DEVELOPER: Self = Self::VERIFIED_BOT_DEVELOPER;
    /// Discord Certified Moderator.
    pub const DISCORD_CERTIFIED_MODERATOR: Self = Self(1 << 18);
    /// Bot that only uses HTTP interactions.
    pub const BOT_HTTP_INTERACTIONS: Self = Self(1 << 19);
    /// Flagged as a spammer by Discord.
    pub const SPAMMER: Self = Self(1 << 20);
    /// Active Developer.
    pub const ACTIVE_DEVELOPER: Self = Self(1 << 22);

    /// Every public user flag.
    pub const ALL: Self = Self(
        Self::STAFF.0
            | Self::PARTNER.0
            | Self::HYPESQUAD.0
            | Self::BUG_HUNTER.0
            | Self::HYPESQUAD_BRAVERY.0
            | Self::HYPESQUAD_BRILLIANCE.0
            | Self::HYPESQUAD_BALANCE.0
            | Self::EARLY_SUPPORTER.0
            | Self::TEAM_USER.0
            | Self::SYSTEM.0
            | Self::BUG_HUNTER_LEVEL_2.0
            | Self::VERIFIED_BOT.0
            | Self::VERIFIED_BOT_DEVELOPER.0
            | Self::DISCORD_CERTIFIED_MODERATOR.0
            | Self::BOT_HTTP_INTERACTIONS.0
            | Self::SPAMMER.0
            | Self::ACTIVE_DEVELOPER.0,
    );

    /// Creates an empty flag set.
    #[must_use]
    pub const fn none() -> Self {
        Self::NONE
    }

    /// Creates a set with every public flag enabled.
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

    /// Wraps a raw bitfield, dropping bits not exposed as public flags.
    #[must_use]
    pub const fn from_bits_truncate(bits: u64) -> Self {
        Self(bits & Self::ALL.0)
    }

    /// Returns `true` if no public flag is set.
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

    /// Returns every public flag that is not enabled in `self`.
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0 & Self::ALL.0)
    }
}

impl core::ops::BitOr for PublicUserFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl core::ops::BitOrAssign for PublicUserFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.insert(rhs);
    }
}

impl core::ops::BitAnd for PublicUserFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl core::ops::BitAndAssign for PublicUserFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl core::ops::BitXor for PublicUserFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl core::ops::BitXorAssign for PublicUserFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.toggle(rhs);
    }
}

impl core::ops::Not for PublicUserFlags {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl From<u64> for PublicUserFlags {
    fn from(bits: u64) -> Self {
        Self::from_bits(bits)
    }
}

impl From<PublicUserFlags> for u64 {
    fn from(flags: PublicUserFlags) -> Self {
        flags.bits()
    }
}

macro_rules! public_user_flag_accessors {
    ($(($constant:ident, $getter:ident, $setter:ident)),* $(,)?) => {
        $(
            /// Returns whether this flag is enabled.
            #[must_use]
            pub const fn $getter(self) -> bool {
                self.contains(Self::$constant)
            }

            /// Enables or disables this flag.
            pub fn $setter(&mut self, value: bool) {
                self.set(Self::$constant, value);
            }
        )*
    };
}

impl PublicUserFlags {
    public_user_flag_accessors! {
        (STAFF, staff, set_staff),
        (PARTNER, partner, set_partner),
        (HYPESQUAD, hypesquad, set_hypesquad),
        (BUG_HUNTER, bug_hunter, set_bug_hunter),
        (HYPESQUAD_BRAVERY, hypesquad_bravery, set_hypesquad_bravery),
        (HYPESQUAD_BRILLIANCE, hypesquad_brilliance, set_hypesquad_brilliance),
        (HYPESQUAD_BALANCE, hypesquad_balance, set_hypesquad_balance),
        (EARLY_SUPPORTER, early_supporter, set_early_supporter),
        (TEAM_USER, team_user, set_team_user),
        (SYSTEM, system, set_system),
        (BUG_HUNTER_LEVEL_2, bug_hunter_level_2, set_bug_hunter_level_2),
        (VERIFIED_BOT, verified_bot, set_verified_bot),
        (VERIFIED_BOT_DEVELOPER, verified_bot_developer, set_verified_bot_developer),
        (
            EARLY_VERIFIED_BOT_DEVELOPER,
            early_verified_bot_developer,
            set_early_verified_bot_developer
        ),
        (
            DISCORD_CERTIFIED_MODERATOR,
            discord_certified_moderator,
            set_discord_certified_moderator
        ),
        (BOT_HTTP_INTERACTIONS, bot_http_interactions, set_bot_http_interactions),
        (SPAMMER, spammer, set_spammer),
        (ACTIVE_DEVELOPER, active_developer, set_active_developer),
    }
}

#[cfg(test)]
mod tests {
    use super::PublicUserFlags;

    #[test]
    fn flag_values() {
        assert_eq!(PublicUserFlags::STAFF.bits(), 1);
        assert_eq!(PublicUserFlags::HYPESQUAD_BRAVERY.bits(), 64);
        assert_eq!(PublicUserFlags::SYSTEM.bits(), 4096);
        assert_eq!(PublicUserFlags::VERIFIED_BOT.bits(), 65536);
        assert_eq!(PublicUserFlags::ACTIVE_DEVELOPER.bits(), 4_194_304);
        assert_eq!(
            PublicUserFlags::EARLY_VERIFIED_BOT_DEVELOPER,
            PublicUserFlags::VERIFIED_BOT_DEVELOPER
        );
    }

    #[test]
    fn accessors_and_operators() {
        let mut flags = PublicUserFlags::none();
        flags.set_staff(true);
        flags.insert(PublicUserFlags::VERIFIED_BOT);

        assert!(flags.staff());
        assert!(flags.verified_bot());
        assert!(!flags.partner());
        assert_eq!(
            flags.bits(),
            PublicUserFlags::STAFF.bits() | PublicUserFlags::VERIFIED_BOT.bits()
        );
        assert_eq!(!PublicUserFlags::none(), PublicUserFlags::all());
        assert_eq!(!PublicUserFlags::all(), PublicUserFlags::none());
    }

    #[test]
    fn raw_round_trip() {
        let flags = PublicUserFlags::PARTNER | PublicUserFlags::EARLY_SUPPORTER;

        assert_eq!(PublicUserFlags::from_bits(flags.bits()), flags);
        assert_eq!(u64::from(flags), flags.bits());
        assert_eq!(
            PublicUserFlags::from_bits_truncate(u64::MAX),
            PublicUserFlags::all()
        );
    }
}
