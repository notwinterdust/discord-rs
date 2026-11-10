//! Webhook enumerations.

use std::fmt;

/// The type of a webhook.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum WebhookType {
    /// A webhook that posts messages to a channel.
    #[default]
    Incoming = 1,
    /// A webhook that follows another channel's messages.
    ChannelFollower = 2,
    /// A webhook owned by an application.
    Application = 3,
}

impl WebhookType {
    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Incoming),
            2 => Some(Self::ChannelFollower),
            3 => Some(Self::Application),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Incoming => "incoming",
            Self::ChannelFollower => "channel_follower",
            Self::Application => "application",
        }
    }

    /// Returns `true` if this is an incoming webhook.
    #[must_use]
    pub const fn is_incoming(self) -> bool {
        matches!(self, Self::Incoming)
    }

    /// Returns `true` if this is a channel follower webhook.
    #[must_use]
    pub const fn is_channel_follower(self) -> bool {
        matches!(self, Self::ChannelFollower)
    }

    /// Returns `true` if this is an application webhook.
    #[must_use]
    pub const fn is_application(self) -> bool {
        matches!(self, Self::Application)
    }
}

impl fmt::Display for WebhookType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::WebhookType;

    #[test]
    fn webhook_type_round_trips() {
        let values = [
            (1, WebhookType::Incoming),
            (2, WebhookType::ChannelFollower),
            (3, WebhookType::Application),
        ];

        for (value, kind) in values {
            assert_eq!(WebhookType::from_value(value), Some(kind));
            assert_eq!(kind.value(), value);
        }

        assert_eq!(WebhookType::from_value(0), None);
        assert_eq!(WebhookType::from_value(4), None);
        assert_eq!(WebhookType::default(), WebhookType::Incoming);
    }

    #[test]
    fn webhook_type_helpers_and_display() {
        assert!(WebhookType::Incoming.is_incoming());
        assert!(!WebhookType::Incoming.is_channel_follower());
        assert!(WebhookType::ChannelFollower.is_channel_follower());
        assert!(WebhookType::Application.is_application());
        assert!(!WebhookType::Application.is_incoming());

        assert_eq!(WebhookType::Incoming.to_string(), "incoming");
        assert_eq!(WebhookType::ChannelFollower.to_string(), "channel_follower");
        assert_eq!(WebhookType::Application.to_string(), "application");
    }
}
