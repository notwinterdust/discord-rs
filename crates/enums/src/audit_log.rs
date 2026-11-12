//! Audit log enumerations.

use std::fmt;

/// The category of an audit log action.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AuditLogActionCategory {
    /// The action created an object.
    Create = 1,
    /// The action deleted an object.
    Delete = 2,
    /// The action updated an object.
    Update = 3,
}

impl AuditLogActionCategory {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Create),
            2 => Some(Self::Delete),
            3 => Some(Self::Update),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Delete => "delete",
            Self::Update => "update",
        }
    }

    /// Returns `true` if this is the create category.
    #[must_use]
    pub const fn is_create(self) -> bool {
        matches!(self, Self::Create)
    }

    /// Returns `true` if this is the delete category.
    #[must_use]
    pub const fn is_delete(self) -> bool {
        matches!(self, Self::Delete)
    }

    /// Returns `true` if this is the update category.
    #[must_use]
    pub const fn is_update(self) -> bool {
        matches!(self, Self::Update)
    }
}

impl fmt::Display for AuditLogActionCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The kind of object an audit log action targets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AuditLogActionTargetType {
    /// The action targets the guild.
    Guild,
    /// The action targets a channel.
    Channel,
    /// The action targets a user.
    User,
    /// The action targets a role.
    Role,
    /// The action targets an invite.
    Invite,
    /// The action targets a webhook.
    Webhook,
    /// The action targets an emoji.
    Emoji,
    /// The action targets a message.
    Message,
    /// The action targets an integration.
    Integration,
    /// The action targets a stage instance.
    StageInstance,
    /// The action targets a sticker.
    Sticker,
    /// The action targets a scheduled event.
    GuildScheduledEvent,
    /// The action targets a thread.
    Thread,
    /// The action targets an integration or application command.
    IntegrationOrAppCommand,
    /// The action targets AutoMod.
    AutoModeration,
    /// The action targets creator monetization.
    CreatorMonetization,
    /// The action targets an onboarding prompt.
    OnboardingPrompt,
    /// The action targets onboarding.
    Onboarding,
    /// The action targets home settings.
    HomeSettings,
}

impl AuditLogActionTargetType {
    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Guild => "guild",
            Self::Channel => "channel",
            Self::User => "user",
            Self::Role => "role",
            Self::Invite => "invite",
            Self::Webhook => "webhook",
            Self::Emoji => "emoji",
            Self::Message => "message",
            Self::Integration => "integration",
            Self::StageInstance => "stage_instance",
            Self::Sticker => "sticker",
            Self::GuildScheduledEvent => "guild_scheduled_event",
            Self::Thread => "thread",
            Self::IntegrationOrAppCommand => "integration_or_app_command",
            Self::AutoModeration => "auto_moderation",
            Self::CreatorMonetization => "creator_monetization",
            Self::OnboardingPrompt => "onboarding_prompt",
            Self::Onboarding => "onboarding",
            Self::HomeSettings => "home_settings",
        }
    }
}

impl fmt::Display for AuditLogActionTargetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// An action recorded in a guild's audit log.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum AuditLogAction {
    /// The guild was updated.
    GuildUpdate = 1,
    /// A channel was created.
    ChannelCreate = 10,
    /// A channel was updated.
    ChannelUpdate = 11,
    /// A channel was deleted.
    ChannelDelete = 12,
    /// A channel permission overwrite was created.
    OverwriteCreate = 13,
    /// A channel permission overwrite was updated.
    OverwriteUpdate = 14,
    /// A channel permission overwrite was deleted.
    OverwriteDelete = 15,
    /// A member was kicked.
    Kick = 20,
    /// Members were pruned.
    MemberPrune = 21,
    /// A member was banned.
    Ban = 22,
    /// A member was unbanned.
    Unban = 23,
    /// A member was updated.
    MemberUpdate = 24,
    /// A member's roles were updated.
    MemberRoleUpdate = 25,
    /// A member was moved to another voice channel.
    MemberMove = 26,
    /// A member was disconnected from voice.
    MemberDisconnect = 27,
    /// A bot was added to the guild.
    BotAdd = 28,
    /// A role was created.
    RoleCreate = 30,
    /// A role was updated.
    RoleUpdate = 31,
    /// A role was deleted.
    RoleDelete = 32,
    /// An invite was created.
    InviteCreate = 40,
    /// An invite was updated.
    InviteUpdate = 41,
    /// An invite was deleted.
    InviteDelete = 42,
    /// A webhook was created.
    WebhookCreate = 50,
    /// A webhook was updated.
    WebhookUpdate = 51,
    /// A webhook was deleted.
    WebhookDelete = 52,
    /// An emoji was created.
    EmojiCreate = 60,
    /// An emoji was updated.
    EmojiUpdate = 61,
    /// An emoji was deleted.
    EmojiDelete = 62,
    /// A message was deleted.
    MessageDelete = 72,
    /// Messages were bulk deleted.
    MessageBulkDelete = 73,
    /// A message was pinned.
    MessagePin = 74,
    /// A message was unpinned.
    MessageUnpin = 75,
    /// An integration was created.
    IntegrationCreate = 80,
    /// An integration was updated.
    IntegrationUpdate = 81,
    /// An integration was deleted.
    IntegrationDelete = 82,
    /// A stage instance was created.
    StageInstanceCreate = 83,
    /// A stage instance was updated.
    StageInstanceUpdate = 84,
    /// A stage instance was deleted.
    StageInstanceDelete = 85,
    /// A sticker was created.
    StickerCreate = 90,
    /// A sticker was updated.
    StickerUpdate = 91,
    /// A sticker was deleted.
    StickerDelete = 92,
    /// A scheduled event was created.
    ScheduledEventCreate = 100,
    /// A scheduled event was updated.
    ScheduledEventUpdate = 101,
    /// A scheduled event was deleted.
    ScheduledEventDelete = 102,
    /// A thread was created.
    ThreadCreate = 110,
    /// A thread was updated.
    ThreadUpdate = 111,
    /// A thread was deleted.
    ThreadDelete = 112,
    /// An application command's permissions were updated.
    AppCommandPermissionUpdate = 121,
    /// A soundboard sound was created.
    SoundboardSoundCreate = 130,
    /// A soundboard sound was updated.
    SoundboardSoundUpdate = 131,
    /// A soundboard sound was deleted.
    SoundboardSoundDelete = 132,
    /// An AutoMod rule was created.
    AutomodRuleCreate = 140,
    /// An AutoMod rule was updated.
    AutomodRuleUpdate = 141,
    /// An AutoMod rule was deleted.
    AutomodRuleDelete = 142,
    /// An AutoMod rule blocked a message.
    AutomodBlockMessage = 143,
    /// An AutoMod rule flagged a message.
    AutomodFlagMessage = 144,
    /// An AutoMod rule timed out a member.
    AutomodTimeoutMember = 145,
    /// An AutoMod rule quarantined a user.
    AutomodQuarantineUser = 146,
    /// A creator monetization request was created.
    CreatorMonetizationRequestCreated = 150,
    /// Creator monetization terms were accepted.
    CreatorMonetizationTermsAccepted = 151,
    /// An onboarding prompt was created.
    OnboardingPromptCreate = 163,
    /// An onboarding prompt was updated.
    OnboardingPromptUpdate = 164,
    /// An onboarding prompt was deleted.
    OnboardingPromptDelete = 165,
    /// Onboarding was created.
    OnboardingCreate = 166,
    /// Onboarding was updated.
    OnboardingUpdate = 167,
    /// Home settings were created.
    HomeSettingsCreate = 190,
    /// Home settings were updated.
    HomeSettingsUpdate = 191,
}

impl AuditLogAction {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u16 {
        self as u16
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u16) -> Option<Self> {
        match value {
            1 => Some(Self::GuildUpdate),
            10 => Some(Self::ChannelCreate),
            11 => Some(Self::ChannelUpdate),
            12 => Some(Self::ChannelDelete),
            13 => Some(Self::OverwriteCreate),
            14 => Some(Self::OverwriteUpdate),
            15 => Some(Self::OverwriteDelete),
            20 => Some(Self::Kick),
            21 => Some(Self::MemberPrune),
            22 => Some(Self::Ban),
            23 => Some(Self::Unban),
            24 => Some(Self::MemberUpdate),
            25 => Some(Self::MemberRoleUpdate),
            26 => Some(Self::MemberMove),
            27 => Some(Self::MemberDisconnect),
            28 => Some(Self::BotAdd),
            30 => Some(Self::RoleCreate),
            31 => Some(Self::RoleUpdate),
            32 => Some(Self::RoleDelete),
            40 => Some(Self::InviteCreate),
            41 => Some(Self::InviteUpdate),
            42 => Some(Self::InviteDelete),
            50 => Some(Self::WebhookCreate),
            51 => Some(Self::WebhookUpdate),
            52 => Some(Self::WebhookDelete),
            60 => Some(Self::EmojiCreate),
            61 => Some(Self::EmojiUpdate),
            62 => Some(Self::EmojiDelete),
            72 => Some(Self::MessageDelete),
            73 => Some(Self::MessageBulkDelete),
            74 => Some(Self::MessagePin),
            75 => Some(Self::MessageUnpin),
            80 => Some(Self::IntegrationCreate),
            81 => Some(Self::IntegrationUpdate),
            82 => Some(Self::IntegrationDelete),
            83 => Some(Self::StageInstanceCreate),
            84 => Some(Self::StageInstanceUpdate),
            85 => Some(Self::StageInstanceDelete),
            90 => Some(Self::StickerCreate),
            91 => Some(Self::StickerUpdate),
            92 => Some(Self::StickerDelete),
            100 => Some(Self::ScheduledEventCreate),
            101 => Some(Self::ScheduledEventUpdate),
            102 => Some(Self::ScheduledEventDelete),
            110 => Some(Self::ThreadCreate),
            111 => Some(Self::ThreadUpdate),
            112 => Some(Self::ThreadDelete),
            121 => Some(Self::AppCommandPermissionUpdate),
            130 => Some(Self::SoundboardSoundCreate),
            131 => Some(Self::SoundboardSoundUpdate),
            132 => Some(Self::SoundboardSoundDelete),
            140 => Some(Self::AutomodRuleCreate),
            141 => Some(Self::AutomodRuleUpdate),
            142 => Some(Self::AutomodRuleDelete),
            143 => Some(Self::AutomodBlockMessage),
            144 => Some(Self::AutomodFlagMessage),
            145 => Some(Self::AutomodTimeoutMember),
            146 => Some(Self::AutomodQuarantineUser),
            150 => Some(Self::CreatorMonetizationRequestCreated),
            151 => Some(Self::CreatorMonetizationTermsAccepted),
            163 => Some(Self::OnboardingPromptCreate),
            164 => Some(Self::OnboardingPromptUpdate),
            165 => Some(Self::OnboardingPromptDelete),
            166 => Some(Self::OnboardingCreate),
            167 => Some(Self::OnboardingUpdate),
            190 => Some(Self::HomeSettingsCreate),
            191 => Some(Self::HomeSettingsUpdate),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GuildUpdate => "guild_update",
            Self::ChannelCreate => "channel_create",
            Self::ChannelUpdate => "channel_update",
            Self::ChannelDelete => "channel_delete",
            Self::OverwriteCreate => "overwrite_create",
            Self::OverwriteUpdate => "overwrite_update",
            Self::OverwriteDelete => "overwrite_delete",
            Self::Kick => "kick",
            Self::MemberPrune => "member_prune",
            Self::Ban => "ban",
            Self::Unban => "unban",
            Self::MemberUpdate => "member_update",
            Self::MemberRoleUpdate => "member_role_update",
            Self::MemberMove => "member_move",
            Self::MemberDisconnect => "member_disconnect",
            Self::BotAdd => "bot_add",
            Self::RoleCreate => "role_create",
            Self::RoleUpdate => "role_update",
            Self::RoleDelete => "role_delete",
            Self::InviteCreate => "invite_create",
            Self::InviteUpdate => "invite_update",
            Self::InviteDelete => "invite_delete",
            Self::WebhookCreate => "webhook_create",
            Self::WebhookUpdate => "webhook_update",
            Self::WebhookDelete => "webhook_delete",
            Self::EmojiCreate => "emoji_create",
            Self::EmojiUpdate => "emoji_update",
            Self::EmojiDelete => "emoji_delete",
            Self::MessageDelete => "message_delete",
            Self::MessageBulkDelete => "message_bulk_delete",
            Self::MessagePin => "message_pin",
            Self::MessageUnpin => "message_unpin",
            Self::IntegrationCreate => "integration_create",
            Self::IntegrationUpdate => "integration_update",
            Self::IntegrationDelete => "integration_delete",
            Self::StageInstanceCreate => "stage_instance_create",
            Self::StageInstanceUpdate => "stage_instance_update",
            Self::StageInstanceDelete => "stage_instance_delete",
            Self::StickerCreate => "sticker_create",
            Self::StickerUpdate => "sticker_update",
            Self::StickerDelete => "sticker_delete",
            Self::ScheduledEventCreate => "scheduled_event_create",
            Self::ScheduledEventUpdate => "scheduled_event_update",
            Self::ScheduledEventDelete => "scheduled_event_delete",
            Self::ThreadCreate => "thread_create",
            Self::ThreadUpdate => "thread_update",
            Self::ThreadDelete => "thread_delete",
            Self::AppCommandPermissionUpdate => "app_command_permission_update",
            Self::SoundboardSoundCreate => "soundboard_sound_create",
            Self::SoundboardSoundUpdate => "soundboard_sound_update",
            Self::SoundboardSoundDelete => "soundboard_sound_delete",
            Self::AutomodRuleCreate => "automod_rule_create",
            Self::AutomodRuleUpdate => "automod_rule_update",
            Self::AutomodRuleDelete => "automod_rule_delete",
            Self::AutomodBlockMessage => "automod_block_message",
            Self::AutomodFlagMessage => "automod_flag_message",
            Self::AutomodTimeoutMember => "automod_timeout_member",
            Self::AutomodQuarantineUser => "automod_quarantine_user",
            Self::CreatorMonetizationRequestCreated => "creator_monetization_request_created",
            Self::CreatorMonetizationTermsAccepted => "creator_monetization_terms_accepted",
            Self::OnboardingPromptCreate => "onboarding_prompt_create",
            Self::OnboardingPromptUpdate => "onboarding_prompt_update",
            Self::OnboardingPromptDelete => "onboarding_prompt_delete",
            Self::OnboardingCreate => "onboarding_create",
            Self::OnboardingUpdate => "onboarding_update",
            Self::HomeSettingsCreate => "home_settings_create",
            Self::HomeSettingsUpdate => "home_settings_update",
        }
    }

    /// Returns the action's category, if it has one.
    #[must_use]
    pub const fn category(self) -> Option<AuditLogActionCategory> {
        match self {
            Self::GuildUpdate
            | Self::ChannelUpdate
            | Self::OverwriteUpdate
            | Self::MemberUpdate
            | Self::MemberRoleUpdate
            | Self::RoleUpdate
            | Self::InviteUpdate
            | Self::WebhookUpdate
            | Self::EmojiUpdate
            | Self::IntegrationUpdate
            | Self::StageInstanceUpdate
            | Self::StickerUpdate
            | Self::ScheduledEventUpdate
            | Self::ThreadUpdate
            | Self::AppCommandPermissionUpdate
            | Self::SoundboardSoundUpdate
            | Self::AutomodRuleUpdate
            | Self::OnboardingPromptUpdate
            | Self::OnboardingUpdate
            | Self::HomeSettingsUpdate => Some(AuditLogActionCategory::Update),
            Self::ChannelCreate
            | Self::OverwriteCreate
            | Self::RoleCreate
            | Self::InviteCreate
            | Self::WebhookCreate
            | Self::EmojiCreate
            | Self::IntegrationCreate
            | Self::StageInstanceCreate
            | Self::StickerCreate
            | Self::ScheduledEventCreate
            | Self::ThreadCreate
            | Self::SoundboardSoundCreate
            | Self::AutomodRuleCreate
            | Self::OnboardingPromptCreate
            | Self::OnboardingCreate
            | Self::HomeSettingsCreate => Some(AuditLogActionCategory::Create),
            Self::ChannelDelete
            | Self::OverwriteDelete
            | Self::RoleDelete
            | Self::InviteDelete
            | Self::WebhookDelete
            | Self::EmojiDelete
            | Self::MessageDelete
            | Self::MessageBulkDelete
            | Self::IntegrationDelete
            | Self::StageInstanceDelete
            | Self::StickerDelete
            | Self::ScheduledEventDelete
            | Self::ThreadDelete
            | Self::SoundboardSoundDelete
            | Self::AutomodRuleDelete
            | Self::OnboardingPromptDelete => Some(AuditLogActionCategory::Delete),
            Self::Kick
            | Self::MemberPrune
            | Self::Ban
            | Self::Unban
            | Self::MemberMove
            | Self::MemberDisconnect
            | Self::BotAdd
            | Self::MessagePin
            | Self::MessageUnpin
            | Self::AutomodBlockMessage
            | Self::AutomodFlagMessage
            | Self::AutomodTimeoutMember
            | Self::AutomodQuarantineUser
            | Self::CreatorMonetizationRequestCreated
            | Self::CreatorMonetizationTermsAccepted => None,
        }
    }

    /// Returns the kind of object the action targets, if known.
    #[must_use]
    pub const fn target_type(self) -> Option<AuditLogActionTargetType> {
        match self {
            Self::GuildUpdate => Some(AuditLogActionTargetType::Guild),
            Self::ChannelCreate
            | Self::ChannelUpdate
            | Self::ChannelDelete
            | Self::OverwriteCreate
            | Self::OverwriteUpdate
            | Self::OverwriteDelete
            | Self::MessageBulkDelete => Some(AuditLogActionTargetType::Channel),
            Self::Kick
            | Self::MemberPrune
            | Self::Ban
            | Self::Unban
            | Self::MemberUpdate
            | Self::MemberRoleUpdate
            | Self::MemberMove
            | Self::MemberDisconnect
            | Self::BotAdd
            | Self::AutomodBlockMessage
            | Self::AutomodFlagMessage
            | Self::AutomodTimeoutMember
            | Self::AutomodQuarantineUser => Some(AuditLogActionTargetType::User),
            Self::RoleCreate | Self::RoleUpdate | Self::RoleDelete => {
                Some(AuditLogActionTargetType::Role)
            }
            Self::InviteCreate | Self::InviteUpdate | Self::InviteDelete => {
                Some(AuditLogActionTargetType::Invite)
            }
            Self::WebhookCreate | Self::WebhookUpdate | Self::WebhookDelete => {
                Some(AuditLogActionTargetType::Webhook)
            }
            Self::EmojiCreate | Self::EmojiUpdate | Self::EmojiDelete => {
                Some(AuditLogActionTargetType::Emoji)
            }
            Self::MessageDelete | Self::MessagePin | Self::MessageUnpin => {
                Some(AuditLogActionTargetType::Message)
            }
            Self::IntegrationCreate
            | Self::IntegrationUpdate
            | Self::IntegrationDelete
            | Self::AppCommandPermissionUpdate => {
                Some(AuditLogActionTargetType::IntegrationOrAppCommand)
            }
            Self::StageInstanceCreate | Self::StageInstanceUpdate | Self::StageInstanceDelete => {
                Some(AuditLogActionTargetType::StageInstance)
            }
            Self::StickerCreate | Self::StickerUpdate | Self::StickerDelete => {
                Some(AuditLogActionTargetType::Sticker)
            }
            Self::ScheduledEventCreate
            | Self::ScheduledEventUpdate
            | Self::ScheduledEventDelete => Some(AuditLogActionTargetType::GuildScheduledEvent),
            Self::ThreadCreate | Self::ThreadUpdate | Self::ThreadDelete => {
                Some(AuditLogActionTargetType::Thread)
            }
            Self::SoundboardSoundCreate
            | Self::SoundboardSoundUpdate
            | Self::SoundboardSoundDelete => Some(AuditLogActionTargetType::Guild),
            Self::AutomodRuleCreate | Self::AutomodRuleUpdate | Self::AutomodRuleDelete => {
                Some(AuditLogActionTargetType::AutoModeration)
            }
            Self::CreatorMonetizationRequestCreated | Self::CreatorMonetizationTermsAccepted => {
                Some(AuditLogActionTargetType::CreatorMonetization)
            }
            Self::OnboardingPromptCreate
            | Self::OnboardingPromptUpdate
            | Self::OnboardingPromptDelete => Some(AuditLogActionTargetType::OnboardingPrompt),
            Self::OnboardingCreate | Self::OnboardingUpdate => {
                Some(AuditLogActionTargetType::Onboarding)
            }
            Self::HomeSettingsCreate | Self::HomeSettingsUpdate => {
                Some(AuditLogActionTargetType::HomeSettings)
            }
        }
    }

    /// Returns `true` if the action created an object.
    #[must_use]
    pub const fn is_create(self) -> bool {
        matches!(self.category(), Some(AuditLogActionCategory::Create))
    }

    /// Returns `true` if the action deleted an object.
    #[must_use]
    pub const fn is_delete(self) -> bool {
        matches!(self.category(), Some(AuditLogActionCategory::Delete))
    }

    /// Returns `true` if the action updated an object.
    #[must_use]
    pub const fn is_update(self) -> bool {
        matches!(self.category(), Some(AuditLogActionCategory::Update))
    }
}

impl fmt::Display for AuditLogAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::{AuditLogAction, AuditLogActionCategory, AuditLogActionTargetType};

    #[test]
    fn actions_round_trip() {
        let values = [
            1, 10, 11, 12, 13, 14, 15, 20, 21, 22, 23, 24, 25, 26, 27, 28, 30, 31, 32, 40, 41, 42,
            50, 51, 52, 60, 61, 62, 72, 73, 74, 75, 80, 81, 82, 83, 84, 85, 90, 91, 92, 100, 101,
            102, 110, 111, 112, 121, 130, 131, 132, 140, 141, 142, 143, 144, 145, 146, 150, 151,
            163, 164, 165, 166, 167, 190, 191,
        ];

        for value in values {
            let action = AuditLogAction::from_value(value).unwrap();
            assert_eq!(action.value(), value);
        }

        assert_eq!(AuditLogAction::from_value(0), None);
        assert_eq!(AuditLogAction::from_value(2), None);
        assert_eq!(AuditLogAction::from_value(192), None);
        assert_eq!(AuditLogAction::GuildUpdate.to_string(), "guild_update");
    }

    #[test]
    fn categories_and_targets() {
        assert_eq!(
            AuditLogAction::ChannelCreate.category(),
            Some(AuditLogActionCategory::Create)
        );
        assert_eq!(
            AuditLogAction::ChannelUpdate.category(),
            Some(AuditLogActionCategory::Update)
        );
        assert_eq!(
            AuditLogAction::ChannelDelete.category(),
            Some(AuditLogActionCategory::Delete)
        );
        assert_eq!(AuditLogAction::Kick.category(), None);
        assert!(AuditLogAction::RoleCreate.is_create());
        assert!(AuditLogAction::RoleUpdate.is_update());
        assert!(AuditLogAction::RoleDelete.is_delete());
        assert!(!AuditLogAction::Kick.is_create());

        assert_eq!(
            AuditLogAction::GuildUpdate.target_type(),
            Some(AuditLogActionTargetType::Guild)
        );
        assert_eq!(
            AuditLogAction::MessageDelete.target_type(),
            Some(AuditLogActionTargetType::Message)
        );
        assert_eq!(
            AuditLogAction::AppCommandPermissionUpdate.target_type(),
            Some(AuditLogActionTargetType::IntegrationOrAppCommand)
        );
        assert_eq!(
            AuditLogAction::AutomodRuleCreate.target_type(),
            Some(AuditLogActionTargetType::AutoModeration)
        );
        assert_eq!(
            AuditLogAction::ScheduledEventCreate.target_type(),
            Some(AuditLogActionTargetType::GuildScheduledEvent)
        );
        assert_eq!(AuditLogActionCategory::Create.to_string(), "create");
        assert_eq!(
            AuditLogActionTargetType::GuildScheduledEvent.to_string(),
            "guild_scheduled_event"
        );
    }
}
