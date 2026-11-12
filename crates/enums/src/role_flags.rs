//! Guild role flags.
//!
/// Wraps the Discord guild role flags.
///
/// ```
///     use discord_rust_enums::RoleFlags;
///
///     let flags = RoleFlags::IN_PROMPT;
///     assert!(flags.in_prompt());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RoleFlags(u64);

impl RoleFlags {
    /// No flags.
    pub const NONE: Self = Self(0);

    /// The role can be selected by members in an onboarding prompt.
    pub const IN_PROMPT: Self = Self(1 << 0);

    /// Every role flag.
    pub const ALL: Self = Self(Self::IN_PROMPT.0);

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

    /// Returns whether the role can be selected in an onboarding prompt.
    #[must_use]
    pub const fn in_prompt(self) -> bool {
        self.contains(Self::IN_PROMPT)
    }

    /// Enables or disables onboarding prompt selection.
    pub fn set_in_prompt(&mut self, value: bool) {
        self.set(Self::IN_PROMPT, value);
    }
}

impl core::ops::BitOr for RoleFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl core::ops::BitOrAssign for RoleFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.insert(rhs);
    }
}

impl core::ops::BitAnd for RoleFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl core::ops::BitAndAssign for RoleFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl core::ops::BitXor for RoleFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl core::ops::BitXorAssign for RoleFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.toggle(rhs);
    }
}

impl core::ops::Not for RoleFlags {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl From<u64> for RoleFlags {
    fn from(bits: u64) -> Self {
        Self::from_bits(bits)
    }
}

impl From<RoleFlags> for u64 {
    fn from(flags: RoleFlags) -> Self {
        flags.bits()
    }
}

#[cfg(test)]
mod tests {
    use super::RoleFlags;

    #[test]
    fn flag_values() {
        assert_eq!(RoleFlags::IN_PROMPT.bits(), 1);
        assert_eq!(RoleFlags::all().bits(), 1);
    }

    #[test]
    fn accessors_and_operators() {
        let mut flags = RoleFlags::none();
        assert!(!flags.in_prompt());

        flags.set_in_prompt(true);
        assert!(flags.in_prompt());
        assert_eq!(flags, RoleFlags::IN_PROMPT);

        flags.set_in_prompt(false);
        assert!(flags.is_empty());
        assert_eq!(!RoleFlags::none(), RoleFlags::all());
    }
}
