//! Message component enumerations.

use std::fmt;

/// The type of a Discord message component.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ComponentType {
    /// A container for up to five components.
    #[default]
    ActionRow = 1,
    /// A clickable button.
    Button = 2,
    /// A dropdown allowing one or more strings to be selected.
    StringSelect = 3,
    /// A text input used in modals.
    TextInput = 4,
    /// A dropdown allowing users to be selected.
    UserSelect = 5,
    /// A dropdown allowing roles to be selected.
    RoleSelect = 6,
    /// A dropdown allowing users and roles to be selected.
    MentionableSelect = 7,
    /// A dropdown allowing channels to be selected.
    ChannelSelect = 8,
    /// A section containing text and accessories.
    Section = 9,
    /// A block of text.
    TextDisplay = 10,
    /// A thumbnail image.
    Thumbnail = 11,
    /// A gallery of media items.
    MediaGallery = 12,
    /// A file attachment.
    File = 13,
    /// A visual separator.
    Separator = 14,
    /// A container grouping other components.
    Container = 17,
    /// A label wrapping an input component.
    Label = 18,
    /// A file upload input.
    FileUpload = 19,
    /// A group of radio buttons.
    RadioGroup = 21,
    /// A group of checkboxes.
    CheckboxGroup = 22,
    /// A single checkbox.
    Checkbox = 23,
}

impl ComponentType {
    /// Alias of [`ComponentType::StringSelect`].
    pub const SELECT: Self = Self::StringSelect;

    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::ActionRow),
            2 => Some(Self::Button),
            3 => Some(Self::StringSelect),
            4 => Some(Self::TextInput),
            5 => Some(Self::UserSelect),
            6 => Some(Self::RoleSelect),
            7 => Some(Self::MentionableSelect),
            8 => Some(Self::ChannelSelect),
            9 => Some(Self::Section),
            10 => Some(Self::TextDisplay),
            11 => Some(Self::Thumbnail),
            12 => Some(Self::MediaGallery),
            13 => Some(Self::File),
            14 => Some(Self::Separator),
            17 => Some(Self::Container),
            18 => Some(Self::Label),
            19 => Some(Self::FileUpload),
            21 => Some(Self::RadioGroup),
            22 => Some(Self::CheckboxGroup),
            23 => Some(Self::Checkbox),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActionRow => "action_row",
            Self::Button => "button",
            Self::StringSelect => "string_select",
            Self::TextInput => "text_input",
            Self::UserSelect => "user_select",
            Self::RoleSelect => "role_select",
            Self::MentionableSelect => "mentionable_select",
            Self::ChannelSelect => "channel_select",
            Self::Section => "section",
            Self::TextDisplay => "text_display",
            Self::Thumbnail => "thumbnail",
            Self::MediaGallery => "media_gallery",
            Self::File => "file",
            Self::Separator => "separator",
            Self::Container => "container",
            Self::Label => "label",
            Self::FileUpload => "file_upload",
            Self::RadioGroup => "radio_group",
            Self::CheckboxGroup => "checkbox_group",
            Self::Checkbox => "checkbox",
        }
    }

    /// Returns `true` if this is one of the select menu types.
    #[must_use]
    pub const fn is_select(self) -> bool {
        matches!(
            self,
            Self::StringSelect
                | Self::UserSelect
                | Self::RoleSelect
                | Self::MentionableSelect
                | Self::ChannelSelect
        )
    }
}

impl fmt::Display for ComponentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The visual style of a button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ButtonStyle {
    /// A blurple primary button.
    #[default]
    Primary = 1,
    /// A grey secondary button.
    Secondary = 2,
    /// A green success button.
    Success = 3,
    /// A red danger button.
    Danger = 4,
    /// A link button that opens a URL.
    Link = 5,
    /// A premium button that opens a SKU.
    Premium = 6,
}

impl ButtonStyle {
    /// Alias of [`ButtonStyle::Primary`].
    pub const BLURPLE: Self = Self::Primary;
    /// Alias of [`ButtonStyle::Secondary`].
    pub const GREY: Self = Self::Secondary;
    /// American spelling of [`ButtonStyle::GREY`].
    pub const GRAY: Self = Self::Secondary;
    /// Alias of [`ButtonStyle::Success`].
    pub const GREEN: Self = Self::Success;
    /// Alias of [`ButtonStyle::Danger`].
    pub const RED: Self = Self::Danger;
    /// Alias of [`ButtonStyle::Link`].
    pub const URL: Self = Self::Link;

    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Primary),
            2 => Some(Self::Secondary),
            3 => Some(Self::Success),
            4 => Some(Self::Danger),
            5 => Some(Self::Link),
            6 => Some(Self::Premium),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Success => "success",
            Self::Danger => "danger",
            Self::Link => "link",
            Self::Premium => "premium",
        }
    }

    /// Returns `true` if buttons with this style open a URL.
    #[must_use]
    pub const fn is_link(self) -> bool {
        matches!(self, Self::Link)
    }
}

impl fmt::Display for ButtonStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The style of a modal text input.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TextStyle {
    /// A single-line input.
    #[default]
    Short = 1,
    /// A multi-line input.
    Paragraph = 2,
}

impl TextStyle {
    /// Alias of [`TextStyle::Paragraph`].
    pub const LONG: Self = Self::Paragraph;

    /// Returns the raw API value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Converts a raw API value, returning `None` for unknown values.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Short),
            2 => Some(Self::Paragraph),
            _ => None,
        }
    }

    /// Returns the lowercase name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Short => "short",
            Self::Paragraph => "paragraph",
        }
    }
}

impl fmt::Display for TextStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The type of object referenced by a select menu's default value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SelectDefaultValueType {
    /// A user.
    #[default]
    User,
    /// A role.
    Role,
    /// A channel.
    Channel,
}

impl SelectDefaultValueType {
    /// Returns the name used by the API.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Role => "role",
            Self::Channel => "channel",
        }
    }

    /// Converts an API name, returning `None` for unknown values.
    #[must_use]
    pub fn from_name(value: &str) -> Option<Self> {
        match value {
            "user" => Some(Self::User),
            "role" => Some(Self::Role),
            "channel" => Some(Self::Channel),
            _ => None,
        }
    }
}

impl fmt::Display for SelectDefaultValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::{ButtonStyle, ComponentType, SelectDefaultValueType, TextStyle};

    #[test]
    fn component_type_round_trips() {
        let values = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 17, 18, 19, 21, 22, 23,
        ];

        for value in values {
            let component_type = ComponentType::from_value(value).unwrap();
            assert_eq!(component_type.value(), value);
        }

        assert_eq!(ComponentType::from_value(0), None);
        assert_eq!(ComponentType::from_value(15), None);
        assert_eq!(ComponentType::from_value(20), None);
        assert_eq!(ComponentType::from_value(24), None);
        assert_eq!(ComponentType::default(), ComponentType::ActionRow);
        assert_eq!(ComponentType::SELECT, ComponentType::StringSelect);
        assert!(ComponentType::StringSelect.is_select());
        assert!(ComponentType::ChannelSelect.is_select());
        assert!(!ComponentType::Button.is_select());
        assert_eq!(ComponentType::StringSelect.to_string(), "string_select");
    }

    #[test]
    fn button_style_values() {
        let values = [
            (1, ButtonStyle::Primary),
            (2, ButtonStyle::Secondary),
            (3, ButtonStyle::Success),
            (4, ButtonStyle::Danger),
            (5, ButtonStyle::Link),
            (6, ButtonStyle::Premium),
        ];

        for (value, style) in values {
            assert_eq!(ButtonStyle::from_value(value), Some(style));
            assert_eq!(style.value(), value);
        }

        assert_eq!(ButtonStyle::from_value(0), None);
        assert_eq!(ButtonStyle::from_value(7), None);
        assert_eq!(ButtonStyle::default(), ButtonStyle::Primary);
        assert_eq!(ButtonStyle::BLURPLE, ButtonStyle::Primary);
        assert_eq!(ButtonStyle::GREEN, ButtonStyle::Success);
        assert_eq!(ButtonStyle::URL, ButtonStyle::Link);
        assert!(ButtonStyle::Link.is_link());
        assert!(!ButtonStyle::Primary.is_link());
        assert_eq!(ButtonStyle::Secondary.to_string(), "secondary");
    }

    #[test]
    fn text_style_values() {
        assert_eq!(TextStyle::from_value(1), Some(TextStyle::Short));
        assert_eq!(TextStyle::from_value(2), Some(TextStyle::Paragraph));
        assert_eq!(TextStyle::from_value(3), None);
        assert_eq!(TextStyle::default(), TextStyle::Short);
        assert_eq!(TextStyle::LONG, TextStyle::Paragraph);
        assert_eq!(TextStyle::Paragraph.to_string(), "paragraph");
    }

    #[test]
    fn select_default_value_type_names() {
        assert_eq!(
            SelectDefaultValueType::from_name("user"),
            Some(SelectDefaultValueType::User)
        );
        assert_eq!(
            SelectDefaultValueType::from_name("role"),
            Some(SelectDefaultValueType::Role)
        );
        assert_eq!(
            SelectDefaultValueType::from_name("channel"),
            Some(SelectDefaultValueType::Channel)
        );
        assert_eq!(SelectDefaultValueType::from_name("other"), None);
        assert_eq!(
            SelectDefaultValueType::default(),
            SelectDefaultValueType::User
        );
        assert_eq!(SelectDefaultValueType::Role.to_string(), "role");
    }
}
