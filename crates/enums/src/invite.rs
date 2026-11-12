//! Invite enumerations and bitfields.

use std::fmt;

/// The type of an invite.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum InviteType {
    /// An invite to a guild.
    #[default]
    Guild = 0,
    /// An invite to a group direct message.
    GroupDm = 1,
    /// A friend invite.
    Friend = 2,
}

impl InviteType {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Guild),
            1 => Some(Self::GroupDm),
            2 => Some(Self::Friend),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Guild => "guild",
            Self::GroupDm => "group_dm",
            Self::Friend => "friend",
        }
    }

    /// Returns `true` if this is a guild invite.
    #[must_use]
    pub const fn is_guild(self) -> bool {
        matches!(self, Self::Guild)
    }

    /// Returns `true` if this is a group direct message invite.
    #[must_use]
    pub const fn is_group_dm(self) -> bool {
        matches!(self, Self::GroupDm)
    }

    /// Returns `true` if this is a friend invite.
    #[must_use]
    pub const fn is_friend(self) -> bool {
        matches!(self, Self::Friend)
    }
}

impl fmt::Display for InviteType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The target of a voice channel invite.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum InviteTarget {
    /// No specific target.
    #[default]
    Unknown = 0,
    /// A user's stream.
    Stream = 1,
    /// An embedded application.
    EmbeddedApplication = 2,
}

impl InviteTarget {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Unknown),
            1 => Some(Self::Stream),
            2 => Some(Self::EmbeddedApplication),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Stream => "stream",
            Self::EmbeddedApplication => "embedded_application",
        }
    }

    /// Returns `true` if the invite targets a stream.
    #[must_use]
    pub const fn is_stream(self) -> bool {
        matches!(self, Self::Stream)
    }

    /// Returns `true` if the invite targets an embedded application.
    #[must_use]
    pub const fn is_embedded_application(self) -> bool {
        matches!(self, Self::EmbeddedApplication)
    }
}

impl fmt::Display for InviteTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Wraps the Discord invite flags.
///
/// ```
///     use discord_rust_enums::InviteFlags;
///
///     let flags = InviteFlags::GUEST;
///     assert!(flags.guest());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct InviteFlags(u64);

impl InviteFlags {
    /// No flags.
    pub const NONE: Self = Self(0);

    /// The invite is a guest invite for a voice channel.
    pub const GUEST: Self = Self(1 << 0);

    /// Every known invite flag.
    pub const ALL: Self = Self(Self::GUEST.0);

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

    /// Returns whether this is a guest invite.
    #[must_use]
    pub const fn guest(self) -> bool {
        self.contains(Self::GUEST)
    }

    /// Enables or disables the guest flag.
    pub fn set_guest(&mut self, value: bool) {
        self.set(Self::GUEST, value);
    }
}

impl core::ops::BitOr for InviteFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl core::ops::BitOrAssign for InviteFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.insert(rhs);
    }
}

impl core::ops::BitAnd for InviteFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl core::ops::BitAndAssign for InviteFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl core::ops::BitXor for InviteFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl core::ops::BitXorAssign for InviteFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.toggle(rhs);
    }
}

impl core::ops::Not for InviteFlags {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl From<u64> for InviteFlags {
    fn from(bits: u64) -> Self {
        Self::from_bits(bits)
    }
}

impl From<InviteFlags> for u64 {
    fn from(flags: InviteFlags) -> Self {
        flags.bits()
    }
}

#[cfg(test)]
mod tests {
    use super::{InviteFlags, InviteTarget, InviteType};

    #[test]
    fn invite_types_round_trip() {
        let types = [
            (0, InviteType::Guild),
            (1, InviteType::GroupDm),
            (2, InviteType::Friend),
        ];
        for (value, kind) in types {
            assert_eq!(InviteType::from_value(value), Some(kind));
            assert_eq!(kind.value(), value);
        }
        assert_eq!(InviteType::from_value(3), None);
        assert_eq!(InviteType::default(), InviteType::Guild);
        assert!(InviteType::Guild.is_guild());
        assert!(InviteType::GroupDm.is_group_dm());
        assert!(InviteType::Friend.is_friend());
        assert_eq!(InviteType::GroupDm.to_string(), "group_dm");

        let targets = [
            (0, InviteTarget::Unknown),
            (1, InviteTarget::Stream),
            (2, InviteTarget::EmbeddedApplication),
        ];
        for (value, target) in targets {
            assert_eq!(InviteTarget::from_value(value), Some(target));
            assert_eq!(target.value(), value);
        }
        assert_eq!(InviteTarget::from_value(3), None);
        assert_eq!(InviteTarget::default(), InviteTarget::Unknown);
        assert!(InviteTarget::Stream.is_stream());
        assert!(InviteTarget::EmbeddedApplication.is_embedded_application());
        assert_eq!(
            InviteTarget::EmbeddedApplication.to_string(),
            "embedded_application"
        );
    }

    #[test]
    fn invite_flags_and_operators() {
        let mut flags = InviteFlags::GUEST;
        assert!(flags.guest());
        assert_eq!(flags.bits(), 1);
        assert!(flags.contains(InviteFlags::GUEST));
        assert!(flags.intersects(InviteFlags::ALL));
        assert_eq!(flags | InviteFlags::none(), flags);
        assert_eq!(flags & InviteFlags::GUEST, InviteFlags::GUEST);
        assert_eq!(flags ^ InviteFlags::GUEST, InviteFlags::none());
        assert_eq!(!flags, InviteFlags::none());

        flags.set_guest(false);
        assert!(!flags.guest());
        flags.set_guest(true);
        assert!(flags.guest());
        flags.toggle(InviteFlags::GUEST);
        assert!(flags.is_empty());

        assert_eq!(InviteFlags::from(1_u64), InviteFlags::GUEST);
        assert_eq!(u64::from(InviteFlags::GUEST), 1);
        assert_eq!(InviteFlags::from_bits(8).bits(), 8);
        assert_eq!(InviteFlags::from_bits_truncate(8), InviteFlags::NONE);
        assert_eq!(InviteFlags::all().bits(), 1);
    }
}
