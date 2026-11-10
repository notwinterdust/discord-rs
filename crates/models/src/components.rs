//! Message component models.
//!
//! This module covers the components that appear in messages and modals:
//! [`ActionRow`], [`Button`], [`SelectMenu`] and [`TextInput`]. The newer
//! layout components are represented by [`ComponentType`] but do not have
//! dedicated models yet.

use std::fmt;

use abc::Snowflake;
use enums::{ButtonStyle, ChannelType, ComponentType, SelectDefaultValueType, TextStyle};

use crate::PartialEmoji;

/// A child component that can be placed inside an [`ActionRow`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionRowChild {
    /// A button.
    Button(Button),
    /// A select menu.
    Select(SelectMenu),
    /// A text input, used in modals.
    TextInput(TextInput),
}

impl ActionRowChild {
    /// Returns the component's type.
    #[must_use]
    pub const fn component_type(&self) -> ComponentType {
        match self {
            Self::Button(_) => ComponentType::Button,
            Self::Select(select) => select.component_type(),
            Self::TextInput(_) => ComponentType::TextInput,
        }
    }

    /// Returns the button, if this is a button.
    #[must_use]
    pub const fn as_button(&self) -> Option<&Button> {
        match self {
            Self::Button(button) => Some(button),
            _ => None,
        }
    }

    /// Returns the select menu, if this is a select menu.
    #[must_use]
    pub const fn as_select(&self) -> Option<&SelectMenu> {
        match self {
            Self::Select(select) => Some(select),
            _ => None,
        }
    }

    /// Returns the text input, if this is a text input.
    #[must_use]
    pub const fn as_text_input(&self) -> Option<&TextInput> {
        match self {
            Self::TextInput(input) => Some(input),
            _ => None,
        }
    }
}

impl From<Button> for ActionRowChild {
    fn from(button: Button) -> Self {
        Self::Button(button)
    }
}

impl From<SelectMenu> for ActionRowChild {
    fn from(select: SelectMenu) -> Self {
        Self::Select(select)
    }
}

impl From<TextInput> for ActionRowChild {
    fn from(input: TextInput) -> Self {
        Self::TextInput(input)
    }
}

/// A row of up to five components.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ActionRow {
    /// The component's ID, if Discord assigned one.
    pub id: Option<u64>,
    /// The components in the row.
    pub children: Vec<ActionRowChild>,
}

impl ActionRow {
    /// Creates an action row from its children.
    #[must_use]
    pub fn new(children: Vec<ActionRowChild>) -> Self {
        Self { id: None, children }
    }

    /// Sets the component ID.
    #[must_use]
    pub fn with_id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    /// Appends a child component.
    #[must_use]
    pub fn add_child(mut self, child: impl Into<ActionRowChild>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Removes the child at `index`, ignoring an out-of-range index.
    #[must_use]
    pub fn remove_child(mut self, index: usize) -> Self {
        if index < self.children.len() {
            self.children.remove(index);
        }
        self
    }

    /// Returns the component's type.
    #[must_use]
    pub const fn component_type(&self) -> ComponentType {
        ComponentType::ActionRow
    }

    /// Returns every button in the row.
    pub fn buttons(&self) -> impl Iterator<Item = &Button> {
        self.children.iter().filter_map(ActionRowChild::as_button)
    }

    /// Returns every select menu in the row.
    pub fn selects(&self) -> impl Iterator<Item = &SelectMenu> {
        self.children.iter().filter_map(ActionRowChild::as_select)
    }

    /// Returns every text input in the row.
    pub fn text_inputs(&self) -> impl Iterator<Item = &TextInput> {
        self.children
            .iter()
            .filter_map(ActionRowChild::as_text_input)
    }
}

/// A clickable message button.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Button {
    /// The component's ID, if Discord assigned one.
    pub id: Option<u64>,
    /// The button's visual style.
    pub style: ButtonStyle,
    /// The ID received when the button is clicked.
    pub custom_id: Option<String>,
    /// The URL opened by a link button.
    pub url: Option<String>,
    /// Whether the button is disabled.
    pub disabled: bool,
    /// The button's label.
    pub label: Option<String>,
    /// The button's emoji.
    pub emoji: Option<PartialEmoji>,
    /// The SKU opened by a premium button.
    pub sku_id: Option<Snowflake>,
}

impl Button {
    /// Creates a button with the given style.
    #[must_use]
    pub fn new(style: ButtonStyle) -> Self {
        Self {
            style,
            ..Self::default()
        }
    }

    /// Sets the component ID.
    #[must_use]
    pub fn with_id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the custom ID sent with interactions.
    #[must_use]
    pub fn with_custom_id(mut self, custom_id: impl Into<String>) -> Self {
        self.custom_id = Some(custom_id.into());
        self
    }

    /// Sets the URL opened by a link button.
    #[must_use]
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Sets the button label.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the button emoji.
    #[must_use]
    pub fn with_emoji(mut self, emoji: PartialEmoji) -> Self {
        self.emoji = Some(emoji);
        self
    }

    /// Sets whether the button is disabled.
    #[must_use]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the SKU opened by a premium button.
    #[must_use]
    pub fn with_sku_id(mut self, sku_id: impl Into<Snowflake>) -> Self {
        self.sku_id = Some(sku_id.into());
        self
    }

    /// Returns the component's type.
    #[must_use]
    pub const fn component_type(&self) -> ComponentType {
        ComponentType::Button
    }

    /// Returns `true` if this is a link button.
    #[must_use]
    pub const fn is_link(&self) -> bool {
        self.style.is_link()
    }

    /// Returns `true` if this is a premium button.
    #[must_use]
    pub const fn is_premium(&self) -> bool {
        matches!(self.style, ButtonStyle::Premium)
    }
}

/// A dropdown menu that allows one or more values to be selected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectMenu {
    /// The component's ID, if Discord assigned one.
    pub id: Option<u64>,
    /// The kind of select menu.
    pub kind: ComponentType,
    /// The ID received when the menu is submitted.
    pub custom_id: String,
    /// Text shown when nothing is selected.
    pub placeholder: Option<String>,
    /// The minimum number of selections.
    pub min_values: u8,
    /// The maximum number of selections.
    pub max_values: u8,
    /// The options available in a string select.
    pub options: Vec<SelectOption>,
    /// Whether the menu is disabled.
    pub disabled: bool,
    /// The channel types allowed in a channel select.
    pub channel_types: Vec<ChannelType>,
    /// The values selected by default.
    pub default_values: Vec<SelectDefaultValue>,
    /// Whether the menu is required in a modal.
    pub required: bool,
}

impl Default for SelectMenu {
    fn default() -> Self {
        Self {
            id: None,
            kind: ComponentType::StringSelect,
            custom_id: String::new(),
            placeholder: None,
            min_values: 1,
            max_values: 1,
            options: Vec::new(),
            disabled: false,
            channel_types: Vec::new(),
            default_values: Vec::new(),
            required: false,
        }
    }
}

impl SelectMenu {
    /// Creates a string select menu.
    #[must_use]
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            custom_id: custom_id.into(),
            ..Self::default()
        }
    }

    /// Creates a user select menu.
    #[must_use]
    pub fn user(custom_id: impl Into<String>) -> Self {
        Self::new(custom_id).with_kind(ComponentType::UserSelect)
    }

    /// Creates a role select menu.
    #[must_use]
    pub fn role(custom_id: impl Into<String>) -> Self {
        Self::new(custom_id).with_kind(ComponentType::RoleSelect)
    }

    /// Creates a mentionable select menu.
    #[must_use]
    pub fn mentionable(custom_id: impl Into<String>) -> Self {
        Self::new(custom_id).with_kind(ComponentType::MentionableSelect)
    }

    /// Creates a channel select menu.
    #[must_use]
    pub fn channel(custom_id: impl Into<String>) -> Self {
        Self::new(custom_id).with_kind(ComponentType::ChannelSelect)
    }

    /// Sets the component ID.
    #[must_use]
    pub fn with_id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the select menu kind.
    #[must_use]
    pub fn with_kind(mut self, kind: ComponentType) -> Self {
        self.kind = kind;
        self
    }

    /// Sets the placeholder text.
    #[must_use]
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Sets the minimum and maximum number of selections.
    #[must_use]
    pub fn with_value_range(mut self, min_values: u8, max_values: u8) -> Self {
        self.min_values = min_values;
        self.max_values = max_values;
        self
    }

    /// Sets the options available in a string select.
    #[must_use]
    pub fn with_options(mut self, options: Vec<SelectOption>) -> Self {
        self.options = options;
        self
    }

    /// Appends an option.
    #[must_use]
    pub fn add_option(mut self, option: SelectOption) -> Self {
        self.options.push(option);
        self
    }

    /// Sets whether the menu is disabled.
    #[must_use]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the channel types allowed in a channel select.
    #[must_use]
    pub fn with_channel_types(mut self, channel_types: Vec<ChannelType>) -> Self {
        self.channel_types = channel_types;
        self
    }

    /// Sets the values selected by default.
    #[must_use]
    pub fn with_default_values(mut self, default_values: Vec<SelectDefaultValue>) -> Self {
        self.default_values = default_values;
        self
    }

    /// Sets whether the menu is required in a modal.
    #[must_use]
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Returns the component's type.
    #[must_use]
    pub const fn component_type(&self) -> ComponentType {
        self.kind
    }

    /// Returns `true` if this is a string select menu.
    #[must_use]
    pub const fn is_string_select(&self) -> bool {
        matches!(self.kind, ComponentType::StringSelect)
    }

    /// Returns `true` if this is a user select menu.
    #[must_use]
    pub const fn is_user_select(&self) -> bool {
        matches!(self.kind, ComponentType::UserSelect)
    }

    /// Returns `true` if this is a role select menu.
    #[must_use]
    pub const fn is_role_select(&self) -> bool {
        matches!(self.kind, ComponentType::RoleSelect)
    }

    /// Returns `true` if this is a mentionable select menu.
    #[must_use]
    pub const fn is_mentionable_select(&self) -> bool {
        matches!(self.kind, ComponentType::MentionableSelect)
    }

    /// Returns `true` if this is a channel select menu.
    #[must_use]
    pub const fn is_channel_select(&self) -> bool {
        matches!(self.kind, ComponentType::ChannelSelect)
    }
}

/// An option in a string select menu.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectOption {
    /// The option's label, shown to users.
    pub label: String,
    /// The option's value, sent to the bot.
    pub value: String,
    /// An additional description shown under the label.
    pub description: Option<String>,
    /// The option's emoji.
    pub emoji: Option<PartialEmoji>,
    /// Whether the option is selected by default.
    pub default: bool,
}

impl SelectOption {
    /// Creates an option whose value defaults to its label.
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        let label = label.into();
        Self {
            value: label.clone(),
            label,
            ..Self::default()
        }
    }

    /// Sets the option's value.
    #[must_use]
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// Sets the option's description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the option's emoji.
    #[must_use]
    pub fn with_emoji(mut self, emoji: PartialEmoji) -> Self {
        self.emoji = Some(emoji);
        self
    }

    /// Sets whether the option is selected by default.
    #[must_use]
    pub fn with_default(mut self, default: bool) -> Self {
        self.default = default;
        self
    }
}

impl fmt::Display for SelectOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(emoji) = &self.emoji {
            write!(f, "{emoji} ")?;
        }

        f.write_str(&self.label)?;

        if let Some(description) = &self.description {
            write!(f, "\n{description}")?;
        }

        Ok(())
    }
}

/// A value selected by default in a select menu.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SelectDefaultValue {
    /// The ID of the selected user, role or channel.
    pub id: Snowflake,
    /// The kind of object referenced by [`SelectDefaultValue::id`].
    pub value_type: SelectDefaultValueType,
}

impl SelectDefaultValue {
    /// Creates a default value for an object of the given kind.
    #[must_use]
    pub fn new(id: impl Into<Snowflake>, value_type: SelectDefaultValueType) -> Self {
        Self {
            id: id.into(),
            value_type,
        }
    }

    /// Creates a default value for a user.
    #[must_use]
    pub fn from_user(id: impl Into<Snowflake>) -> Self {
        Self::new(id, SelectDefaultValueType::User)
    }

    /// Creates a default value for a role.
    #[must_use]
    pub fn from_role(id: impl Into<Snowflake>) -> Self {
        Self::new(id, SelectDefaultValueType::Role)
    }

    /// Creates a default value for a channel.
    #[must_use]
    pub fn from_channel(id: impl Into<Snowflake>) -> Self {
        Self::new(id, SelectDefaultValueType::Channel)
    }
}

/// A text input used in modals.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextInput {
    /// The component's ID, if Discord assigned one.
    pub id: Option<u64>,
    /// The ID received when the modal is submitted.
    pub custom_id: String,
    /// The label displayed above the input.
    pub label: Option<String>,
    /// The input's visual style.
    pub style: TextStyle,
    /// Text shown when the input is empty.
    pub placeholder: Option<String>,
    /// The input's default value.
    pub value: Option<String>,
    /// Whether the input must be filled in.
    pub required: bool,
    /// The minimum input length.
    pub min_length: Option<u16>,
    /// The maximum input length.
    pub max_length: Option<u16>,
}

impl Default for TextInput {
    fn default() -> Self {
        Self {
            id: None,
            custom_id: String::new(),
            label: None,
            style: TextStyle::default(),
            placeholder: None,
            value: None,
            required: true,
            min_length: None,
            max_length: None,
        }
    }
}

impl TextInput {
    /// Creates a required text input with the given custom ID and style.
    #[must_use]
    pub fn new(custom_id: impl Into<String>, style: TextStyle) -> Self {
        Self {
            custom_id: custom_id.into(),
            style,
            ..Self::default()
        }
    }

    /// Sets the component ID.
    #[must_use]
    pub fn with_id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the label displayed above the input.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the placeholder text.
    #[must_use]
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Sets the input's default value.
    #[must_use]
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Sets whether the input is required.
    #[must_use]
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Sets the minimum and maximum input lengths.
    #[must_use]
    pub fn with_length_range(mut self, min_length: u16, max_length: u16) -> Self {
        self.min_length = Some(min_length);
        self.max_length = Some(max_length);
        self
    }

    /// Returns the component's type.
    #[must_use]
    pub const fn component_type(&self) -> ComponentType {
        ComponentType::TextInput
    }

    /// Returns the input's default value.
    #[must_use]
    pub fn default_value(&self) -> Option<&str> {
        self.value.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use abc::Snowflake;
    use enums::{ButtonStyle, ChannelType, ComponentType, SelectDefaultValueType, TextStyle};

    use super::{
        ActionRow, ActionRowChild, Button, SelectDefaultValue, SelectMenu, SelectOption, TextInput,
    };
    use crate::PartialEmoji;

    #[test]
    fn button_builders_and_helpers() {
        let button = Button::new(ButtonStyle::Primary)
            .with_id(1)
            .with_custom_id("confirm")
            .with_label("Confirm")
            .with_emoji(PartialEmoji::unicode("✅"))
            .with_disabled(true)
            .with_sku_id(10_u64);

        assert_eq!(button.id, Some(1));
        assert_eq!(button.style, ButtonStyle::Primary);
        assert_eq!(button.custom_id.as_deref(), Some("confirm"));
        assert_eq!(button.label.as_deref(), Some("Confirm"));
        assert_eq!(
            button.emoji.as_ref().map(ToString::to_string),
            Some("✅".to_owned())
        );
        assert!(button.disabled);
        assert_eq!(button.sku_id, Some(Snowflake::new(10)));
        assert_eq!(button.component_type(), ComponentType::Button);
        assert!(!button.is_link());
        assert!(!button.is_premium());

        let link = Button::new(ButtonStyle::Link).with_url("https://example.com");
        assert_eq!(link.url.as_deref(), Some("https://example.com"));
        assert!(link.is_link());

        let premium = Button::new(ButtonStyle::Premium).with_sku_id(11_u64);
        assert!(premium.is_premium());
    }

    #[test]
    fn action_row_children_and_queries() {
        let row = ActionRow::new(vec![
            ActionRowChild::from(Button::new(ButtonStyle::Primary).with_custom_id("one")),
            ActionRowChild::from(SelectMenu::new("select")),
        ])
        .with_id(5)
        .add_child(TextInput::new("input", TextStyle::Short));

        assert_eq!(row.id, Some(5));
        assert_eq!(row.children.len(), 3);
        assert_eq!(row.component_type(), ComponentType::ActionRow);
        assert_eq!(row.buttons().count(), 1);
        assert_eq!(row.selects().count(), 1);
        assert_eq!(row.text_inputs().count(), 1);
        assert_eq!(row.children[0].component_type(), ComponentType::Button);
        assert_eq!(row.children[2].component_type(), ComponentType::TextInput);
        assert!(row.children[0].as_button().is_some());
        assert!(row.children[0].as_select().is_none());

        let row = row.remove_child(0).remove_child(10);
        assert_eq!(row.children.len(), 2);
    }

    #[test]
    fn select_menu_builders() {
        let menu = SelectMenu::new("choose")
            .with_id(3)
            .with_placeholder("Pick one")
            .with_value_range(1, 2)
            .with_disabled(true)
            .with_required(true)
            .add_option(SelectOption::new("First").with_value("first"))
            .with_channel_types(vec![ChannelType::Text])
            .with_default_values(vec![SelectDefaultValue::from_user(7_u64)]);

        assert_eq!(menu.id, Some(3));
        assert_eq!(menu.kind, ComponentType::StringSelect);
        assert_eq!(menu.custom_id, "choose");
        assert_eq!(menu.placeholder.as_deref(), Some("Pick one"));
        assert_eq!(menu.min_values, 1);
        assert_eq!(menu.max_values, 2);
        assert_eq!(menu.options.len(), 1);
        assert!(menu.disabled);
        assert!(menu.required);
        assert_eq!(menu.channel_types, vec![ChannelType::Text]);
        assert_eq!(
            menu.default_values[0].value_type,
            SelectDefaultValueType::User
        );
        assert_eq!(menu.component_type(), ComponentType::StringSelect);
        assert!(menu.is_string_select());
        assert!(!menu.is_channel_select());

        assert!(SelectMenu::user("u").is_user_select());
        assert!(SelectMenu::role("r").is_role_select());
        assert!(SelectMenu::mentionable("m").is_mentionable_select());
        assert!(SelectMenu::channel("c").is_channel_select());
        assert_eq!(SelectMenu::default().min_values, 1);
        assert_eq!(SelectMenu::default().kind, ComponentType::StringSelect);
    }

    #[test]
    fn select_option_defaults_and_display() {
        let option = SelectOption::new("First");

        assert_eq!(option.label, "First");
        assert_eq!(option.value, "First");
        assert!(!option.default);
        assert_eq!(option.to_string(), "First");

        let option = option
            .with_value("first")
            .with_description("The first option")
            .with_emoji(PartialEmoji::unicode("1️⃣"))
            .with_default(true);

        assert_eq!(option.value, "first");
        assert_eq!(option.description.as_deref(), Some("The first option"));
        assert!(option.default);
        assert_eq!(option.to_string(), "1️⃣ First\nThe first option");
    }

    #[test]
    fn select_default_values() {
        let user = SelectDefaultValue::from_user(1_u64);
        let role = SelectDefaultValue::from_role(2_u64);
        let channel = SelectDefaultValue::from_channel(3_u64);
        let explicit = SelectDefaultValue::new(4_u64, SelectDefaultValueType::Role);

        assert_eq!(user.id, Snowflake::new(1));
        assert_eq!(user.value_type, SelectDefaultValueType::User);
        assert_eq!(role.value_type, SelectDefaultValueType::Role);
        assert_eq!(channel.value_type, SelectDefaultValueType::Channel);
        assert_eq!(explicit.id, Snowflake::new(4));
        assert_eq!(explicit.value_type, SelectDefaultValueType::Role);
    }

    #[test]
    fn text_input_builders() {
        let input = TextInput::new("answer", TextStyle::Paragraph)
            .with_id(8)
            .with_label("Answer")
            .with_placeholder("Type here")
            .with_value("default")
            .with_required(false)
            .with_length_range(1, 100);

        assert_eq!(input.id, Some(8));
        assert_eq!(input.custom_id, "answer");
        assert_eq!(input.label.as_deref(), Some("Answer"));
        assert_eq!(input.style, TextStyle::Paragraph);
        assert_eq!(input.placeholder.as_deref(), Some("Type here"));
        assert_eq!(input.default_value(), Some("default"));
        assert!(!input.required);
        assert_eq!(input.min_length, Some(1));
        assert_eq!(input.max_length, Some(100));
        assert_eq!(input.component_type(), ComponentType::TextInput);

        let required = TextInput::new("name", TextStyle::Short);
        assert!(required.required);
        assert_eq!(required.default_value(), None);
        assert!(TextInput::default().required);
    }
}
