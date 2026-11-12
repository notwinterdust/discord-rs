//! Thread enumerations and bitfields.

/// Flags attached to a thread member.
///
/// ```
///     use discord_rust_enums::ThreadMemberFlags;
///
///     let flags = ThreadMemberFlags::IN;
///     assert!(flags.in_thread());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ThreadMemberFlags(u64);

impl ThreadMemberFlags {
    /// No flags.
    pub const NONE: Self = Self(0);

    /// The user has joined the thread.
    pub const IN: Self = Self(1 << 0);
    /// The current user is a moderator of the thread.
    pub const MODERATOR: Self = Self(1 << 1);

    /// Every known thread member flag.
    pub const ALL: Self = Self(Self::IN.0 | Self::MODERATOR.0);

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

    /// Returns whether the user has joined the thread.
    #[must_use]
    pub const fn in_thread(self) -> bool {
        self.contains(Self::IN)
    }

    /// Enables or disables the joined flag.
    pub fn set_in_thread(&mut self, value: bool) {
        self.set(Self::IN, value);
    }

    /// Returns whether the current user is a thread moderator.
    #[must_use]
    pub const fn moderator(self) -> bool {
        self.contains(Self::MODERATOR)
    }

    /// Enables or disables the moderator flag.
    pub fn set_moderator(&mut self, value: bool) {
        self.set(Self::MODERATOR, value);
    }
}

impl core::ops::BitOr for ThreadMemberFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl core::ops::BitOrAssign for ThreadMemberFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.insert(rhs);
    }
}

impl core::ops::BitAnd for ThreadMemberFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl core::ops::BitAndAssign for ThreadMemberFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl core::ops::BitXor for ThreadMemberFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl core::ops::BitXorAssign for ThreadMemberFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.toggle(rhs);
    }
}

impl core::ops::Not for ThreadMemberFlags {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl From<u64> for ThreadMemberFlags {
    fn from(bits: u64) -> Self {
        Self::from_bits(bits)
    }
}

impl From<ThreadMemberFlags> for u64 {
    fn from(flags: ThreadMemberFlags) -> Self {
        flags.bits()
    }
}

#[cfg(test)]
mod tests {
    use super::ThreadMemberFlags;

    #[test]
    fn flag_queries_and_operators() {
        let mut flags = ThreadMemberFlags::IN | ThreadMemberFlags::MODERATOR;

        assert_eq!(flags.bits(), 3);
        assert!(flags.in_thread());
        assert!(flags.moderator());
        assert!(flags.contains(ThreadMemberFlags::IN));
        assert!(flags.intersects(ThreadMemberFlags::MODERATOR));
        assert_eq!(flags & ThreadMemberFlags::IN, ThreadMemberFlags::IN);
        assert_eq!(flags ^ ThreadMemberFlags::IN, ThreadMemberFlags::MODERATOR);
        assert_eq!(!flags, ThreadMemberFlags::none());

        flags.remove(ThreadMemberFlags::MODERATOR);
        assert!(!flags.moderator());
        flags.set(ThreadMemberFlags::MODERATOR, true);
        assert!(flags.moderator());
        flags.toggle(ThreadMemberFlags::IN);
        assert!(!flags.in_thread());

        assert_eq!(ThreadMemberFlags::from(2_u64), ThreadMemberFlags::MODERATOR);
        assert_eq!(u64::from(ThreadMemberFlags::IN), 1);
        assert_eq!(ThreadMemberFlags::from_bits(8).bits(), 8);
        assert_eq!(
            ThreadMemberFlags::from_bits_truncate(8),
            ThreadMemberFlags::NONE
        );
        assert_eq!(ThreadMemberFlags::all().bits(), 3);
        assert!(ThreadMemberFlags::none().is_empty());
    }
}
