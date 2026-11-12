//! Message-level enumerations and bitfields.

use std::fmt;

/// The type of a message.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MessageType {
    /// A regular user message.
    #[default]
    Default = 0,
    /// A user was added to a group or thread.
    RecipientAdd = 1,
    /// A user was removed from a group or thread.
    RecipientRemove = 2,
    /// A call was started or updated.
    Call = 3,
    /// A channel's name changed.
    ChannelNameChange = 4,
    /// A channel's icon changed.
    ChannelIconChange = 5,
    /// A message was pinned.
    PinsAdd = 6,
    /// A new member joined the guild.
    NewMember = 7,
    /// A member boosted the guild.
    PremiumGuildSubscription = 8,
    /// A member reached guild boost tier 1.
    PremiumGuildTier1 = 9,
    /// A member reached guild boost tier 2.
    PremiumGuildTier2 = 10,
    /// A member reached guild boost tier 3.
    PremiumGuildTier3 = 11,
    /// A channel followed another channel.
    ChannelFollowAdd = 12,
    /// A member started streaming.
    GuildStream = 13,
    /// The guild was disqualified from discovery.
    GuildDiscoveryDisqualified = 14,
    /// The guild was requalified for discovery.
    GuildDiscoveryRequalified = 15,
    /// The initial discovery grace-period warning.
    GuildDiscoveryGracePeriodInitialWarning = 16,
    /// The final discovery grace-period warning.
    GuildDiscoveryGracePeriodFinalWarning = 17,
    /// A thread was created.
    ThreadCreated = 18,
    /// A reply to another message.
    Reply = 19,
    /// A chat input command was used.
    ChatInputCommand = 20,
    /// The starter message of a thread.
    ThreadStarterMessage = 21,
    /// A reminder for a guild invite.
    GuildInviteReminder = 22,
    /// A context menu command was used.
    ContextMenuCommand = 23,
    /// An auto moderation action was taken.
    AutoModerationAction = 24,
    /// A role subscription was purchased or renewed.
    RoleSubscriptionPurchase = 25,
    /// An interaction premium upsell was shown.
    InteractionPremiumUpsell = 26,
    /// A stage instance started.
    StageStart = 27,
    /// A stage instance ended.
    StageEnd = 28,
    /// A user became a stage speaker.
    StageSpeaker = 29,
    /// A user raised their hand in a stage.
    StageRaiseHand = 30,
    /// A stage topic changed.
    StageTopic = 31,
    /// A guild application premium subscription was purchased.
    GuildApplicationPremiumSubscription = 32,
    /// Guild incident alert mode was enabled.
    GuildIncidentAlertModeEnabled = 36,
    /// Guild incident alert mode was disabled.
    GuildIncidentAlertModeDisabled = 37,
    /// A raid was reported.
    GuildIncidentReportRaid = 38,
    /// A false alarm was reported.
    GuildIncidentReportFalseAlarm = 39,
    /// A purchase notification.
    PurchaseNotification = 44,
    /// A poll result message.
    PollResult = 46,
    /// An emoji was added to the guild.
    EmojiAdded = 63,
}

impl MessageType {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Default),
            1 => Some(Self::RecipientAdd),
            2 => Some(Self::RecipientRemove),
            3 => Some(Self::Call),
            4 => Some(Self::ChannelNameChange),
            5 => Some(Self::ChannelIconChange),
            6 => Some(Self::PinsAdd),
            7 => Some(Self::NewMember),
            8 => Some(Self::PremiumGuildSubscription),
            9 => Some(Self::PremiumGuildTier1),
            10 => Some(Self::PremiumGuildTier2),
            11 => Some(Self::PremiumGuildTier3),
            12 => Some(Self::ChannelFollowAdd),
            13 => Some(Self::GuildStream),
            14 => Some(Self::GuildDiscoveryDisqualified),
            15 => Some(Self::GuildDiscoveryRequalified),
            16 => Some(Self::GuildDiscoveryGracePeriodInitialWarning),
            17 => Some(Self::GuildDiscoveryGracePeriodFinalWarning),
            18 => Some(Self::ThreadCreated),
            19 => Some(Self::Reply),
            20 => Some(Self::ChatInputCommand),
            21 => Some(Self::ThreadStarterMessage),
            22 => Some(Self::GuildInviteReminder),
            23 => Some(Self::ContextMenuCommand),
            24 => Some(Self::AutoModerationAction),
            25 => Some(Self::RoleSubscriptionPurchase),
            26 => Some(Self::InteractionPremiumUpsell),
            27 => Some(Self::StageStart),
            28 => Some(Self::StageEnd),
            29 => Some(Self::StageSpeaker),
            30 => Some(Self::StageRaiseHand),
            31 => Some(Self::StageTopic),
            32 => Some(Self::GuildApplicationPremiumSubscription),
            36 => Some(Self::GuildIncidentAlertModeEnabled),
            37 => Some(Self::GuildIncidentAlertModeDisabled),
            38 => Some(Self::GuildIncidentReportRaid),
            39 => Some(Self::GuildIncidentReportFalseAlarm),
            44 => Some(Self::PurchaseNotification),
            46 => Some(Self::PollResult),
            63 => Some(Self::EmojiAdded),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::RecipientAdd => "recipient_add",
            Self::RecipientRemove => "recipient_remove",
            Self::Call => "call",
            Self::ChannelNameChange => "channel_name_change",
            Self::ChannelIconChange => "channel_icon_change",
            Self::PinsAdd => "pins_add",
            Self::NewMember => "new_member",
            Self::PremiumGuildSubscription => "premium_guild_subscription",
            Self::PremiumGuildTier1 => "premium_guild_tier_1",
            Self::PremiumGuildTier2 => "premium_guild_tier_2",
            Self::PremiumGuildTier3 => "premium_guild_tier_3",
            Self::ChannelFollowAdd => "channel_follow_add",
            Self::GuildStream => "guild_stream",
            Self::GuildDiscoveryDisqualified => "guild_discovery_disqualified",
            Self::GuildDiscoveryRequalified => "guild_discovery_requalified",
            Self::GuildDiscoveryGracePeriodInitialWarning => {
                "guild_discovery_grace_period_initial_warning"
            }
            Self::GuildDiscoveryGracePeriodFinalWarning => {
                "guild_discovery_grace_period_final_warning"
            }
            Self::ThreadCreated => "thread_created",
            Self::Reply => "reply",
            Self::ChatInputCommand => "chat_input_command",
            Self::ThreadStarterMessage => "thread_starter_message",
            Self::GuildInviteReminder => "guild_invite_reminder",
            Self::ContextMenuCommand => "context_menu_command",
            Self::AutoModerationAction => "auto_moderation_action",
            Self::RoleSubscriptionPurchase => "role_subscription_purchase",
            Self::InteractionPremiumUpsell => "interaction_premium_upsell",
            Self::StageStart => "stage_start",
            Self::StageEnd => "stage_end",
            Self::StageSpeaker => "stage_speaker",
            Self::StageRaiseHand => "stage_raise_hand",
            Self::StageTopic => "stage_topic",
            Self::GuildApplicationPremiumSubscription => "guild_application_premium_subscription",
            Self::GuildIncidentAlertModeEnabled => "guild_incident_alert_mode_enabled",
            Self::GuildIncidentAlertModeDisabled => "guild_incident_alert_mode_disabled",
            Self::GuildIncidentReportRaid => "guild_incident_report_raid",
            Self::GuildIncidentReportFalseAlarm => "guild_incident_report_false_alarm",
            Self::PurchaseNotification => "purchase_notification",
            Self::PollResult => "poll_result",
            Self::EmojiAdded => "emoji_added",
        }
    }

    /// Returns `true` if the message is generated by Discord.
    #[must_use]
    pub const fn is_system(self) -> bool {
        !matches!(
            self,
            Self::Default
                | Self::Reply
                | Self::ChatInputCommand
                | Self::ContextMenuCommand
                | Self::ThreadStarterMessage
                | Self::PollResult
        )
    }

    /// Returns `true` if a client may delete this kind of message.
    #[must_use]
    pub const fn is_deletable(self) -> bool {
        !matches!(
            self,
            Self::RecipientAdd
                | Self::RecipientRemove
                | Self::Call
                | Self::ChannelNameChange
                | Self::ChannelIconChange
                | Self::ThreadStarterMessage
        )
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The type of a message reference.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MessageReferenceType {
    /// A reply to another message.
    #[default]
    Reply = 0,
    /// A forwarded message.
    Forward = 1,
}

impl MessageReferenceType {
    /// Alias of [`MessageReferenceType::Reply`].
    pub const DEFAULT: Self = Self::Reply;

    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Reply),
            1 => Some(Self::Forward),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reply => "reply",
            Self::Forward => "forward",
        }
    }
}

impl fmt::Display for MessageReferenceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The type of a reaction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ReactionType {
    /// A normal reaction.
    #[default]
    Normal = 0,
    /// A super reaction.
    Burst = 1,
}

impl ReactionType {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Normal),
            1 => Some(Self::Burst),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Burst => "burst",
        }
    }
}

impl fmt::Display for ReactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Wraps the Discord message flag bitfield.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct MessageFlags(u64);

impl MessageFlags {
    /// No flags.
    pub const NONE: Self = Self(0);

    /// The message is the original crossposted message.
    pub const CROSSPOSTED: Self = Self(1 << 0);
    /// The message was crossposted from another channel.
    pub const IS_CROSSPOSTED: Self = Self(1 << 1);
    /// The message's embeds are suppressed.
    pub const SUPPRESS_EMBEDS: Self = Self(1 << 2);
    /// The source message for a crosspost was deleted.
    pub const SOURCE_MESSAGE_DELETED: Self = Self(1 << 3);
    /// The message is an urgent message from Discord Trust and Safety.
    pub const URGENT: Self = Self(1 << 4);
    /// The message is associated with a thread.
    pub const HAS_THREAD: Self = Self(1 << 5);
    /// The message is ephemeral.
    pub const EPHEMERAL: Self = Self(1 << 6);
    /// The message is an interaction response that is still loading.
    pub const LOADING: Self = Self(1 << 7);
    /// Some roles could not be mentioned in a thread.
    pub const FAILED_TO_MENTION_SOME_ROLES_IN_THREAD: Self = Self(1 << 8);
    /// The message does not trigger push or desktop notifications.
    pub const SUPPRESS_NOTIFICATIONS: Self = Self(1 << 12);
    /// Alias of [`MessageFlags::SUPPRESS_NOTIFICATIONS`].
    pub const SILENT: Self = Self::SUPPRESS_NOTIFICATIONS;
    /// The message is a voice message.
    pub const VOICE: Self = Self(1 << 13);
    /// The message was forwarded.
    pub const FORWARDED: Self = Self(1 << 14);
    /// The message uses Discord's v2 components.
    pub const COMPONENTS_V2: Self = Self(1 << 15);

    /// Every known message flag.
    pub const ALL: Self = Self(
        Self::CROSSPOSTED.0
            | Self::IS_CROSSPOSTED.0
            | Self::SUPPRESS_EMBEDS.0
            | Self::SOURCE_MESSAGE_DELETED.0
            | Self::URGENT.0
            | Self::HAS_THREAD.0
            | Self::EPHEMERAL.0
            | Self::LOADING.0
            | Self::FAILED_TO_MENTION_SOME_ROLES_IN_THREAD.0
            | Self::SUPPRESS_NOTIFICATIONS.0
            | Self::VOICE.0
            | Self::FORWARDED.0
            | Self::COMPONENTS_V2.0,
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

    /// Returns whether the message is the original crossposted message.
    #[must_use]
    pub const fn crossposted(self) -> bool {
        self.contains(Self::CROSSPOSTED)
    }

    /// Returns whether the message was crossposted from another channel.
    #[must_use]
    pub const fn is_crossposted(self) -> bool {
        self.contains(Self::IS_CROSSPOSTED)
    }

    /// Returns whether the message's embeds are suppressed.
    #[must_use]
    pub const fn suppress_embeds(self) -> bool {
        self.contains(Self::SUPPRESS_EMBEDS)
    }

    /// Returns whether the source message of a crosspost was deleted.
    #[must_use]
    pub const fn source_message_deleted(self) -> bool {
        self.contains(Self::SOURCE_MESSAGE_DELETED)
    }

    /// Returns whether the message is urgent.
    #[must_use]
    pub const fn urgent(self) -> bool {
        self.contains(Self::URGENT)
    }

    /// Returns whether the message is associated with a thread.
    #[must_use]
    pub const fn has_thread(self) -> bool {
        self.contains(Self::HAS_THREAD)
    }

    /// Returns whether the message is ephemeral.
    #[must_use]
    pub const fn ephemeral(self) -> bool {
        self.contains(Self::EPHEMERAL)
    }

    /// Returns whether the message is still loading.
    #[must_use]
    pub const fn loading(self) -> bool {
        self.contains(Self::LOADING)
    }

    /// Returns whether some roles could not be mentioned in a thread.
    #[must_use]
    pub const fn failed_to_mention_some_roles_in_thread(self) -> bool {
        self.contains(Self::FAILED_TO_MENTION_SOME_ROLES_IN_THREAD)
    }

    /// Returns whether the message suppresses notifications.
    #[must_use]
    pub const fn suppress_notifications(self) -> bool {
        self.contains(Self::SUPPRESS_NOTIFICATIONS)
    }

    /// Alias of [`MessageFlags::suppress_notifications`].
    #[must_use]
    pub const fn silent(self) -> bool {
        self.suppress_notifications()
    }

    /// Returns whether the message is a voice message.
    #[must_use]
    pub const fn voice(self) -> bool {
        self.contains(Self::VOICE)
    }

    /// Returns whether the message was forwarded.
    #[must_use]
    pub const fn forwarded(self) -> bool {
        self.contains(Self::FORWARDED)
    }

    /// Returns whether the message uses Discord's v2 components.
    #[must_use]
    pub const fn components_v2(self) -> bool {
        self.contains(Self::COMPONENTS_V2)
    }
}

impl core::ops::BitOr for MessageFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl core::ops::BitOrAssign for MessageFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.insert(rhs);
    }
}

impl core::ops::BitAnd for MessageFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl core::ops::BitAndAssign for MessageFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl core::ops::BitXor for MessageFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl core::ops::BitXorAssign for MessageFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.toggle(rhs);
    }
}

impl core::ops::Not for MessageFlags {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl From<u64> for MessageFlags {
    fn from(bits: u64) -> Self {
        Self::from_bits(bits)
    }
}

impl From<MessageFlags> for u64 {
    fn from(flags: MessageFlags) -> Self {
        flags.bits()
    }
}

/// Wraps the Discord attachment flag bitfield.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct AttachmentFlags(u64);

impl AttachmentFlags {
    /// No flags.
    pub const NONE: Self = Self(0);

    /// The attachment is a clip.
    pub const CLIP: Self = Self(1 << 0);
    /// The attachment is a thumbnail.
    pub const THUMBNAIL: Self = Self(1 << 1);
    /// The attachment is a remix.
    pub const REMIX: Self = Self(1 << 2);
    /// The attachment is a spoiler.
    pub const SPOILER: Self = Self(1 << 3);
    /// The attachment contains explicit media.
    pub const CONTAINS_EXPLICIT_MEDIA: Self = Self(1 << 4);
    /// The attachment is animated.
    pub const ANIMATED: Self = Self(1 << 5);

    /// Every known attachment flag.
    pub const ALL: Self = Self(
        Self::CLIP.0
            | Self::THUMBNAIL.0
            | Self::REMIX.0
            | Self::SPOILER.0
            | Self::CONTAINS_EXPLICIT_MEDIA.0
            | Self::ANIMATED.0,
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

    /// Returns whether the attachment is a clip.
    #[must_use]
    pub const fn clip(self) -> bool {
        self.contains(Self::CLIP)
    }

    /// Returns whether the attachment is a thumbnail.
    #[must_use]
    pub const fn thumbnail(self) -> bool {
        self.contains(Self::THUMBNAIL)
    }

    /// Returns whether the attachment is a remix.
    #[must_use]
    pub const fn remix(self) -> bool {
        self.contains(Self::REMIX)
    }

    /// Returns whether the attachment is a spoiler.
    #[must_use]
    pub const fn spoiler(self) -> bool {
        self.contains(Self::SPOILER)
    }

    /// Returns whether the attachment contains explicit media.
    #[must_use]
    pub const fn contains_explicit_media(self) -> bool {
        self.contains(Self::CONTAINS_EXPLICIT_MEDIA)
    }

    /// Returns whether the attachment is animated.
    #[must_use]
    pub const fn animated(self) -> bool {
        self.contains(Self::ANIMATED)
    }
}

impl core::ops::BitOr for AttachmentFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl core::ops::BitOrAssign for AttachmentFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.insert(rhs);
    }
}

impl core::ops::BitAnd for AttachmentFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl core::ops::BitAndAssign for AttachmentFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl core::ops::BitXor for AttachmentFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl core::ops::BitXorAssign for AttachmentFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.toggle(rhs);
    }
}

impl core::ops::Not for AttachmentFlags {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl From<u64> for AttachmentFlags {
    fn from(bits: u64) -> Self {
        Self::from_bits(bits)
    }
}

impl From<AttachmentFlags> for u64 {
    fn from(flags: AttachmentFlags) -> Self {
        flags.bits()
    }
}

#[cfg(test)]
mod tests {
    use super::{AttachmentFlags, MessageFlags, MessageReferenceType, MessageType, ReactionType};

    #[test]
    fn message_type_round_trips() {
        let values = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 36, 37, 38, 39, 44, 46, 63,
        ];

        for value in values {
            let message_type = MessageType::from_value(value).unwrap();
            assert_eq!(message_type.value(), value);
        }

        assert_eq!(MessageType::from_value(33), None);
        assert_eq!(MessageType::from_value(64), None);
        assert_eq!(MessageType::default(), MessageType::Default);
    }

    #[test]
    fn message_type_classification() {
        assert!(!MessageType::Default.is_system());
        assert!(!MessageType::Reply.is_system());
        assert!(!MessageType::ThreadStarterMessage.is_system());
        assert!(MessageType::PinsAdd.is_system());
        assert!(MessageType::EmojiAdded.is_system());

        assert!(MessageType::Default.is_deletable());
        assert!(!MessageType::RecipientAdd.is_deletable());
        assert!(!MessageType::ThreadStarterMessage.is_deletable());
    }

    #[test]
    fn message_reference_type() {
        assert_eq!(
            MessageReferenceType::from_value(0),
            Some(MessageReferenceType::Reply)
        );
        assert_eq!(
            MessageReferenceType::from_value(1),
            Some(MessageReferenceType::Forward)
        );
        assert_eq!(MessageReferenceType::from_value(2), None);
        assert_eq!(MessageReferenceType::DEFAULT, MessageReferenceType::Reply);
        assert_eq!(MessageReferenceType::Forward.to_string(), "forward");
    }

    #[test]
    fn reaction_type() {
        assert_eq!(ReactionType::from_value(0), Some(ReactionType::Normal));
        assert_eq!(ReactionType::from_value(1), Some(ReactionType::Burst));
        assert_eq!(ReactionType::from_value(2), None);
        assert_eq!(ReactionType::Normal.to_string(), "normal");
        assert_eq!(ReactionType::Burst.to_string(), "burst");
    }

    #[test]
    fn message_flag_values() {
        assert_eq!(MessageFlags::CROSSPOSTED.bits(), 1);
        assert_eq!(
            MessageFlags::FAILED_TO_MENTION_SOME_ROLES_IN_THREAD.bits(),
            1 << 8
        );
        assert_eq!(MessageFlags::SUPPRESS_NOTIFICATIONS.bits(), 1 << 12);
        assert_eq!(MessageFlags::SILENT, MessageFlags::SUPPRESS_NOTIFICATIONS);
        assert_eq!(MessageFlags::VOICE.bits(), 1 << 13);
        assert_eq!(MessageFlags::FORWARDED.bits(), 1 << 14);
        assert_eq!(MessageFlags::COMPONENTS_V2.bits(), 1 << 15);

        let flags = MessageFlags::EPHEMERAL | MessageFlags::HAS_THREAD;
        assert!(flags.ephemeral());
        assert!(flags.has_thread());
        assert!(!flags.voice());
        assert!(MessageFlags::SILENT.silent());
        assert_eq!(!MessageFlags::none(), MessageFlags::all());
    }

    #[test]
    fn attachment_flag_values() {
        assert_eq!(AttachmentFlags::CLIP.bits(), 1 << 0);
        assert_eq!(AttachmentFlags::SPOILER.bits(), 1 << 3);
        assert_eq!(AttachmentFlags::ANIMATED.bits(), 1 << 5);
        assert_eq!(!AttachmentFlags::none(), AttachmentFlags::all());
        assert!(AttachmentFlags::SPOILER.spoiler());
        assert!(!AttachmentFlags::SPOILER.animated());
    }
}
