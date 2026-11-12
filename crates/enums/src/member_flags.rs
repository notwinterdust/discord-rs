//! Guild member flags.
/// Wraps the Discord guild member flags.
///
/// ```
///     use discord_rust_enums::MemberFlags;
///
///     let flags = MemberFlags::DID_REJOIN | MemberFlags::GUEST;
///     assert!(flags.did_rejoin());
///     assert!(flags.guest());
///     assert!(!flags.completed_onboarding());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct MemberFlags(u64);

impl MemberFlags {
    /// No flags.
    pub const NONE: Self = Self(0);

    /// The member left and rejoined the guild.
    pub const DID_REJOIN: Self = Self(1 << 0);
    /// The member completed onboarding.
    pub const COMPLETED_ONBOARDING: Self = Self(1 << 1);
    /// The member can bypass the guild verification requirements.
    pub const BYPASSES_VERIFICATION: Self = Self(1 << 2);
    /// The member started onboarding.
    pub const STARTED_ONBOARDING: Self = Self(1 << 3);
    /// The member is a guest and can only access the voice channel they were
    /// invited to.
    pub const GUEST: Self = Self(1 << 4);
    /// The member started Server Guide new member actions.
    pub const STARTED_HOME_ACTIONS: Self = Self(1 << 5);
    /// The member completed Server Guide new member actions.
    pub const COMPLETED_HOME_ACTIONS: Self = Self(1 << 6);
    /// The member's username, nickname or global name was blocked by AutoMod.
    pub const AUTOMOD_QUARANTINED_USERNAME: Self = Self(1 << 7);
    /// The member dismissed the DM settings upsell.
    pub const DM_SETTINGS_UPSELL_ACKNOWLEDGED: Self = Self(1 << 9);
    /// The member's guild tag was blocked by AutoMod.
    pub const AUTOMOD_QUARANTINED_GUILD_TAG: Self = Self(1 << 10);

    /// Every flag
    pub const ALL: Self = Self(
        Self::DID_REJOIN.0
            | Self::COMPLETED_ONBOARDING.0
            | Self::BYPASSES_VERIFICATION.0
            | Self::STARTED_ONBOARDING.0
            | Self::GUEST.0
            | Self::STARTED_HOME_ACTIONS.0
            | Self::COMPLETED_HOME_ACTIONS.0
            | Self::AUTOMOD_QUARANTINED_USERNAME.0
            | Self::DM_SETTINGS_UPSELL_ACKNOWLEDGED.0
            | Self::AUTOMOD_QUARANTINED_GUILD_TAG.0,
    );

    /// Creates an empty flag set.
    #[must_use]
    pub const fn none() -> Self {
        Self::NONE
    }

    /// Creates a set with every known flag enabled.
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

    /// Returns `true` if no flag is set.
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

    /// Returns every known flag that is not enabled in `self`.
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0 & Self::ALL.0)
    }
}

impl core::ops::BitOr for MemberFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl core::ops::BitOrAssign for MemberFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.insert(rhs);
    }
}

impl core::ops::BitAnd for MemberFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl core::ops::BitAndAssign for MemberFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl core::ops::BitXor for MemberFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl core::ops::BitXorAssign for MemberFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.toggle(rhs);
    }
}

impl core::ops::Not for MemberFlags {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl From<u64> for MemberFlags {
    fn from(bits: u64) -> Self {
        Self::from_bits(bits)
    }
}

impl From<MemberFlags> for u64 {
    fn from(flags: MemberFlags) -> Self {
        flags.bits()
    }
}

macro_rules! member_flag_accessors {
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

impl MemberFlags {
    member_flag_accessors! {
        (DID_REJOIN, did_rejoin, set_did_rejoin),
        (COMPLETED_ONBOARDING, completed_onboarding, set_completed_onboarding),
        (
            BYPASSES_VERIFICATION,
            bypasses_verification,
            set_bypasses_verification
        ),
        (STARTED_ONBOARDING, started_onboarding, set_started_onboarding),
        (GUEST, guest, set_guest),
        (STARTED_HOME_ACTIONS, started_home_actions, set_started_home_actions),
        (
            COMPLETED_HOME_ACTIONS,
            completed_home_actions,
            set_completed_home_actions
        ),
        (
            AUTOMOD_QUARANTINED_USERNAME,
            automod_quarantined_username,
            set_automod_quarantined_username
        ),
        (
            DM_SETTINGS_UPSELL_ACKNOWLEDGED,
            dm_settings_upsell_acknowledged,
            set_dm_settings_upsell_acknowledged
        ),
        (
            AUTOMOD_QUARANTINED_GUILD_TAG,
            automod_quarantined_guild_tag,
            set_automod_quarantined_guild_tag
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::MemberFlags;

    #[test]
    fn flag_values() {
        assert_eq!(MemberFlags::DID_REJOIN.bits(), 1);
        assert_eq!(MemberFlags::GUEST.bits(), 16);
        assert_eq!(MemberFlags::AUTOMOD_QUARANTINED_USERNAME.bits(), 128);
        assert_eq!(MemberFlags::DM_SETTINGS_UPSELL_ACKNOWLEDGED.bits(), 512);
        assert_eq!(MemberFlags::AUTOMOD_QUARANTINED_GUILD_TAG.bits(), 1024);
    }

    #[test]
    fn accessors_and_operators() {
        let mut flags = MemberFlags::none();
        flags.set_did_rejoin(true);
        flags.insert(MemberFlags::GUEST);

        assert!(flags.did_rejoin());
        assert!(flags.guest());
        assert!(!flags.started_onboarding());
        assert_eq!(
            flags.bits(),
            MemberFlags::DID_REJOIN.bits() | MemberFlags::GUEST.bits()
        );
        assert_eq!(!MemberFlags::none(), MemberFlags::all());
        assert_eq!(!MemberFlags::all(), MemberFlags::none());
    }

    #[test]
    fn raw_round_trip() {
        let flags = MemberFlags::COMPLETED_ONBOARDING | MemberFlags::STARTED_HOME_ACTIONS;

        assert_eq!(MemberFlags::from_bits(flags.bits()), flags);
        assert_eq!(u64::from(flags), flags.bits());
        assert_eq!(
            MemberFlags::from_bits_truncate(u64::MAX),
            MemberFlags::all()
        );
    }
}
