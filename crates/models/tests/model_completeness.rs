use std::str::FromStr;

use discord_rust_models as models;

use abc::Snowflake;
use data::Value;
use enums::{ActivityType, Status};
use models::{
    Activity, AutoModRule, Entitlement, GuildTemplate, Onboarding, Presence, Sku, SoundboardSound,
    StageInstance, StagePrivacyLevel, WelcomeScreen,
};

#[test]
fn presence_and_activity_casting() {
    let act_game = Activity::game("Rustaceans Unite");
    assert_eq!(act_game.name, "Rustaceans Unite");
    assert_eq!(act_game.kind, ActivityType::Playing);

    let act_stream = Activity::streaming("Live Coding", "https://twitch.tv/notwinterdust");
    assert_eq!(act_stream.kind, ActivityType::Streaming);
    assert_eq!(
        act_stream.url.as_deref(),
        Some("https://twitch.tv/notwinterdust")
    );

    let act_listen = Activity::listening("Lofi Chill");
    assert_eq!(act_listen.kind, ActivityType::Listening);

    let act_custom = Activity::custom("Coding discord-rs");
    assert_eq!(act_custom.kind, ActivityType::Custom);
    assert_eq!(act_custom.state.as_deref(), Some("Coding discord-rs"));

    let json_str = r#"{
        "user": { "id": "123456789" },
        "guild_id": "987654321",
        "status": "online",
        "activities": [
            {
                "name": "Rust Game",
                "type": 0,
                "details": "In match",
                "state": "Ranked"
            }
        ],
        "client_status": {
            "desktop": "online",
            "mobile": "idle"
        }
    }"#;
    let val = Value::from_str(json_str).unwrap();
    let presence = Presence::from_value(&val, "presence").unwrap();
    assert_eq!(presence.user_id, Snowflake::new(123456789));
    assert_eq!(presence.guild_id, Some(Snowflake::new(987654321)));
    assert_eq!(presence.status, Status::Online);
    assert_eq!(presence.activities.len(), 1);
    assert_eq!(presence.activities[0].name, "Rust Game");
    assert_eq!(presence.activities[0].kind, ActivityType::Playing);
    assert_eq!(presence.activities[0].details.as_deref(), Some("In match"));
    assert_eq!(presence.client_status.desktop, Some(Status::Online));
    assert_eq!(presence.client_status.mobile, Some(Status::Idle));
    assert_eq!(presence.client_status.web, None);
}

#[test]
fn automod_rule_casting() {
    let json_str = r#"{
        "id": "111",
        "guild_id": "222",
        "name": "Block invites",
        "creator_id": "333",
        "event_type": 1,
        "trigger_type": 1,
        "trigger_metadata": {
            "keyword_filter": ["discord.gg/*"],
            "regex_patterns": ["https?://.*"],
            "allow_list": ["discord.gg/rust"],
            "mention_total_limit": 5
        },
        "actions": [
            {
                "type": 1,
                "metadata": {
                    "custom_message": "No invite links allowed!"
                }
            }
        ],
        "enabled": true,
        "exempt_roles": ["444"],
        "exempt_channels": ["555"]
    }"#;
    let val = Value::from_str(json_str).unwrap();
    let rule = AutoModRule::from_value(&val, "automod").unwrap();
    assert_eq!(rule.id, Snowflake::new(111));
    assert_eq!(rule.guild_id, Snowflake::new(222));
    assert_eq!(rule.name, "Block invites");
    assert_eq!(rule.creator_id, Snowflake::new(333));
    assert_eq!(rule.event_type, 1);
    assert_eq!(rule.trigger_type, 1);
    assert_eq!(rule.trigger_metadata.keyword_filter, vec!["discord.gg/*"]);
    assert_eq!(rule.trigger_metadata.mention_total_limit, Some(5));
    assert_eq!(rule.actions.len(), 1);
    assert_eq!(rule.actions[0].action_type, 1);
    assert_eq!(
        rule.actions[0]
            .metadata
            .as_ref()
            .unwrap()
            .custom_message
            .as_deref(),
        Some("No invite links allowed!")
    );
    assert!(rule.enabled);
    assert_eq!(rule.exempt_roles, vec![Snowflake::new(444)]);
    assert_eq!(rule.exempt_channels, vec![Snowflake::new(555)]);
}

#[test]
fn stage_instance_casting() {
    let json_str = r#"{
        "id": "1001",
        "guild_id": "2002",
        "channel_id": "3003",
        "topic": "Rust Conference 2026",
        "privacy_level": 2,
        "guild_scheduled_event_id": "4004"
    }"#;
    let val = Value::from_str(json_str).unwrap();
    let stage = StageInstance::from_value(&val, "stage").unwrap();
    assert_eq!(stage.id, Snowflake::new(1001));
    assert_eq!(stage.guild_id, Snowflake::new(2002));
    assert_eq!(stage.channel_id, Snowflake::new(3003));
    assert_eq!(stage.topic, "Rust Conference 2026");
    assert_eq!(stage.privacy_level, StagePrivacyLevel::GuildOnly);
    assert_eq!(stage.guild_scheduled_event_id, Some(Snowflake::new(4004)));
}

#[test]
fn welcome_screen_and_onboarding_casting() {
    let welcome_json = r#"{
        "description": "Welcome to our coding community!",
        "welcome_channels": [
            {
                "channel_id": "101",
                "description": "Read the rules",
                "emoji_name": "📜"
            }
        ]
    }"#;
    let welcome_val = Value::from_str(welcome_json).unwrap();
    let welcome = WelcomeScreen::from_value(&welcome_val, "welcome").unwrap();
    assert_eq!(
        welcome.description.as_deref(),
        Some("Welcome to our coding community!")
    );
    assert_eq!(welcome.welcome_channels.len(), 1);
    assert_eq!(welcome.welcome_channels[0].channel_id, Snowflake::new(101));
    assert_eq!(welcome.welcome_channels[0].description, "Read the rules");

    let onboarding_json = r#"{
        "guild_id": "999",
        "prompts": [
            {
                "id": "888",
                "type": 0,
                "title": "What languages do you write?",
                "single_select": false,
                "required": true,
                "in_onboarding": true,
                "options": [
                    {
                        "id": "777",
                        "title": "Rust",
                        "description": "Systems programming",
                        "channel_ids": ["111"],
                        "role_ids": ["222"]
                    }
                ]
            }
        ],
        "default_channel_ids": ["101"],
        "enabled": true,
        "mode": 1
    }"#;
    let onboarding_val = Value::from_str(onboarding_json).unwrap();
    let onboarding = Onboarding::from_value(&onboarding_val, "onboarding").unwrap();
    assert_eq!(onboarding.guild_id, Snowflake::new(999));
    assert!(onboarding.enabled);
    assert_eq!(onboarding.prompts.len(), 1);
    assert_eq!(onboarding.prompts[0].title, "What languages do you write?");
    assert!(!onboarding.prompts[0].single_select);
    assert!(onboarding.prompts[0].required);
    assert_eq!(onboarding.prompts[0].options[0].title, "Rust");
}

#[test]
fn guild_template_casting() {
    let json_str = r#"{
        "code": "rust-guild-123",
        "name": "Rust Community Template",
        "description": "Standard setup",
        "usage_count": 42,
        "creator_id": "555",
        "creator": {
            "id": "555",
            "username": "ferris",
            "discriminator": "0001"
        },
        "source_guild_id": "999",
        "serialized_source_guild": {
            "name": "Original Rust Guild"
        }
    }"#;
    let val = Value::from_str(json_str).unwrap();
    let template = GuildTemplate::from_value(&val, "template").unwrap();
    assert_eq!(template.code, "rust-guild-123");
    assert_eq!(template.name, "Rust Community Template");
    assert_eq!(template.usage_count, 42);
    assert_eq!(template.creator_id, Snowflake::new(555));
    assert_eq!(template.creator.name, "ferris");
    assert_eq!(template.source_guild_id, Snowflake::new(999));
}

#[test]
fn monetization_sku_and_entitlement_casting() {
    let sku_json = r#"{
        "id": "123000",
        "type": 1,
        "application_id": "456000",
        "name": "Pro Tier Monthly",
        "slug": "pro-tier",
        "flags": 0
    }"#;
    let sku_val = Value::from_str(sku_json).unwrap();
    let sku = Sku::from_value(&sku_val, "sku").unwrap();
    assert_eq!(sku.id, Snowflake::new(123000));
    assert_eq!(sku.sku_type, 1);
    assert_eq!(sku.name, "Pro Tier Monthly");
    assert_eq!(sku.slug, "pro-tier");

    let ent_json = r#"{
        "id": "789000",
        "sku_id": "123000",
        "application_id": "456000",
        "user_id": "333",
        "type": 1,
        "deleted": false,
        "consumed": false
    }"#;
    let ent_val = Value::from_str(ent_json).unwrap();
    let ent = Entitlement::from_value(&ent_val, "entitlement").unwrap();
    assert_eq!(ent.id, Snowflake::new(789000));
    assert_eq!(ent.sku_id, Snowflake::new(123000));
    assert_eq!(ent.user_id, Some(Snowflake::new(333)));
    assert!(!ent.deleted);
    assert!(!ent.consumed);
}

#[test]
fn soundboard_sound_casting() {
    let sound_json = r#"{
        "sound_id": "888000",
        "name": "Airhorn",
        "volume": 0.85,
        "emoji_name": "📢",
        "guild_id": "999",
        "available": true,
        "user_id": "333"
    }"#;
    let sound_val = Value::from_str(sound_json).unwrap();
    let sound = SoundboardSound::from_value(&sound_val, "sound").unwrap();
    assert_eq!(sound.sound_id, Snowflake::new(888000));
    assert_eq!(sound.name, "Airhorn");
    assert!((sound.volume - 0.85).abs() < 1e-6);
    assert_eq!(sound.emoji_name.as_deref(), Some("📢"));
    assert_eq!(sound.guild_id, Some(Snowflake::new(999)));
    assert!(!sound.is_default());
    assert!(sound.available);

    let default_sound_json = r#"{
        "sound_id": "111000",
        "name": "Discord Default Sound",
        "volume": 1.0
    }"#;
    let default_val = Value::from_str(default_sound_json).unwrap();
    let default_sound = SoundboardSound::from_value(&default_val, "default_sound").unwrap();
    assert!(default_sound.is_default());
}
