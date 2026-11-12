//! Interaction component composition primitives.

use data::{Object, Value};
use std::collections::HashMap;

/// Button style definitions according to Discord API.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonStyle {
    Primary = 1,
    Secondary = 2,
    Success = 3,
    Danger = 4,
    Link = 5,
    Premium = 6,
}

/// Text input style definitions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextInputStyle {
    Short = 1,
    Paragraph = 2,
}

/// A typed button component.
#[derive(Clone, Debug, PartialEq)]
pub struct Button {
    pub style: ButtonStyle,
    pub custom_id: Option<String>,
    pub label: Option<String>,
    pub url: Option<String>,
    pub disabled: bool,
    pub sku_id: Option<u64>,
}

impl Button {
    #[must_use]
    pub const fn new(style: ButtonStyle) -> Self {
        Self {
            style,
            custom_id: None,
            label: None,
            url: None,
            disabled: false,
            sku_id: None,
        }
    }

    #[must_use]
    pub fn primary(custom_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(ButtonStyle::Primary)
            .custom_id(custom_id)
            .label(label)
    }

    #[must_use]
    pub fn secondary(custom_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(ButtonStyle::Secondary)
            .custom_id(custom_id)
            .label(label)
    }

    #[must_use]
    pub fn success(custom_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(ButtonStyle::Success)
            .custom_id(custom_id)
            .label(label)
    }

    #[must_use]
    pub fn danger(custom_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(ButtonStyle::Danger)
            .custom_id(custom_id)
            .label(label)
    }

    #[must_use]
    pub fn link(url: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(ButtonStyle::Link).url(url).label(label)
    }

    #[must_use]
    pub fn custom_id(mut self, custom_id: impl Into<String>) -> Self {
        self.custom_id = Some(custom_id.into());
        self
    }

    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    #[must_use]
    pub const fn sku_id(mut self, sku_id: u64) -> Self {
        self.sku_id = Some(sku_id);
        self
    }

    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert("type", Value::Number(2_u64.into()));
        object.insert("style", Value::Number((self.style as u64).into()));
        if let Some(custom_id) = &self.custom_id {
            object.insert("custom_id", Value::String(custom_id.clone()));
        }
        if let Some(label) = &self.label {
            object.insert("label", Value::String(label.clone()));
        }
        if let Some(url) = &self.url {
            object.insert("url", Value::String(url.clone()));
        }
        if self.disabled {
            object.insert("disabled", Value::Bool(true));
        }
        if let Some(sku) = self.sku_id {
            object.insert("sku_id", Value::String(sku.to_string()));
        }
        Value::Object(object)
    }
}

/// A choice option in a select menu.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub default: bool,
}

impl SelectOption {
    #[must_use]
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            description: None,
            default: false,
        }
    }

    #[must_use]
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    #[must_use]
    pub const fn default(mut self, default: bool) -> Self {
        self.default = default;
        self
    }

    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert("label", Value::String(self.label.clone()));
        object.insert("value", Value::String(self.value.clone()));
        if let Some(desc) = &self.description {
            object.insert("description", Value::String(desc.clone()));
        }
        if self.default {
            object.insert("default", Value::Bool(true));
        }
        Value::Object(object)
    }
}

/// A string select menu.
#[derive(Clone, Debug, PartialEq)]
pub struct StringSelect {
    pub custom_id: String,
    pub options: Vec<SelectOption>,
    pub placeholder: Option<String>,
    pub min_values: Option<u8>,
    pub max_values: Option<u8>,
    pub disabled: bool,
}

impl StringSelect {
    #[must_use]
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            custom_id: custom_id.into(),
            options: Vec::new(),
            placeholder: None,
            min_values: None,
            max_values: None,
            disabled: false,
        }
    }

    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    #[must_use]
    pub const fn min_values(mut self, min: u8) -> Self {
        self.min_values = Some(min);
        self
    }

    #[must_use]
    pub const fn max_values(mut self, max: u8) -> Self {
        self.max_values = Some(max);
        self
    }

    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn add_option(&mut self, option: SelectOption) -> Result<(), &'static str> {
        if self.options.len() >= 25 {
            return Err("select cannot contain more than 25 options");
        }
        self.options.push(option);
        Ok(())
    }

    #[must_use]
    pub fn with_option(mut self, option: SelectOption) -> Self {
        let _ = self.add_option(option);
        self
    }

    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert("type", Value::Number(3_u64.into()));
        object.insert("custom_id", Value::String(self.custom_id.clone()));
        object.insert(
            "options",
            Value::Array(self.options.iter().map(SelectOption::to_value).collect()),
        );
        if let Some(placeholder) = &self.placeholder {
            object.insert("placeholder", Value::String(placeholder.clone()));
        }
        if let Some(min) = self.min_values {
            object.insert(
                "min_values",
                Value::Number(data::Number::from_u64(min as u64)),
            );
        }
        if let Some(max) = self.max_values {
            object.insert(
                "max_values",
                Value::Number(data::Number::from_u64(max as u64)),
            );
        }
        if self.disabled {
            object.insert("disabled", Value::Bool(true));
        }
        Value::Object(object)
    }
}

/// A text input modal component.
#[derive(Clone, Debug, PartialEq)]
pub struct TextInput {
    pub custom_id: String,
    pub style: TextInputStyle,
    pub label: String,
    pub min_length: Option<u16>,
    pub max_length: Option<u16>,
    pub required: bool,
    pub value: Option<String>,
    pub placeholder: Option<String>,
}

impl TextInput {
    #[must_use]
    pub fn new(
        custom_id: impl Into<String>,
        label: impl Into<String>,
        style: TextInputStyle,
    ) -> Self {
        Self {
            custom_id: custom_id.into(),
            style,
            label: label.into(),
            min_length: None,
            max_length: None,
            required: true,
            value: None,
            placeholder: None,
        }
    }

    #[must_use]
    pub fn short(custom_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(custom_id, label, TextInputStyle::Short)
    }

    #[must_use]
    pub fn paragraph(custom_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(custom_id, label, TextInputStyle::Paragraph)
    }

    #[must_use]
    pub const fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    #[must_use]
    pub const fn min_length(mut self, min: u16) -> Self {
        self.min_length = Some(min);
        self
    }

    #[must_use]
    pub const fn max_length(mut self, max: u16) -> Self {
        self.max_length = Some(max);
        self
    }

    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert("type", Value::Number(4_u64.into()));
        object.insert("custom_id", Value::String(self.custom_id.clone()));
        object.insert("style", Value::Number((self.style as u64).into()));
        object.insert("label", Value::String(self.label.clone()));
        if !self.required {
            object.insert("required", Value::Bool(false));
        }
        if let Some(min) = self.min_length {
            object.insert(
                "min_length",
                Value::Number(data::Number::from_u64(min as u64)),
            );
        }
        if let Some(max) = self.max_length {
            object.insert(
                "max_length",
                Value::Number(data::Number::from_u64(max as u64)),
            );
        }
        if let Some(placeholder) = &self.placeholder {
            object.insert("placeholder", Value::String(placeholder.clone()));
        }
        if let Some(value) = &self.value {
            object.insert("value", Value::String(value.clone()));
        }
        Value::Object(object)
    }
}

/// An ActionRow containing interactive components (max 5 buttons or 1 select/text input).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ActionRow {
    pub components: Vec<Value>,
}

impl ActionRow {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn add_button(&mut self, button: Button) -> Result<(), &'static str> {
        if self.components.len() >= 5 {
            return Err("action row cannot contain more than 5 buttons");
        }
        if self.components.iter().any(|c| match c {
            Value::Object(obj) => match obj.get("type") {
                Some(Value::Number(n)) => n.as_u64() != Some(2),
                _ => false,
            },
            _ => false,
        }) {
            return Err("buttons cannot be mixed with select menus or text inputs");
        }
        self.components.push(button.to_value());
        Ok(())
    }

    pub fn add_select(&mut self, select: StringSelect) -> Result<(), &'static str> {
        if !self.components.is_empty() {
            return Err("select menu must be the only component in an action row");
        }
        self.components.push(select.to_value());
        Ok(())
    }

    pub fn add_text_input(&mut self, input: TextInput) -> Result<(), &'static str> {
        if !self.components.is_empty() {
            return Err("text input must be the only component in an action row");
        }
        self.components.push(input.to_value());
        Ok(())
    }

    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert("type", Value::Number(1_u64.into()));
        object.insert("components", Value::Array(self.components.clone()));
        Value::Object(object)
    }
}

/// Components V2 Layout Container (max 10 children).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Container {
    pub children: Vec<Value>,
}

impl Container {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn add(&mut self, component: Value) -> Result<(), &'static str> {
        if self.children.len() >= 10 {
            return Err("container limit of 10 components exceeded");
        }
        self.children.push(component);
        Ok(())
    }

    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert("type", Value::Number(17_u64.into()));
        object.insert("components", Value::Array(self.children.clone()));
        Value::Object(object)
    }
}

/// A component that can be sent in an interaction response.
#[derive(Clone, Debug, PartialEq)]
pub struct Component {
    /// Discord component type.
    pub kind: u8,
    /// Optional custom identifier.
    pub custom_id: Option<String>,
    /// Optional label.
    pub label: Option<String>,
}

impl Component {
    /// Creates a component with a Discord component type.
    #[must_use]
    pub const fn new(kind: u8) -> Self {
        Self {
            kind,
            custom_id: None,
            label: None,
        }
    }

    /// Sets the custom identifier.
    #[must_use]
    pub fn with_custom_id(mut self, custom_id: impl Into<String>) -> Self {
        self.custom_id = Some(custom_id.into());
        self
    }

    /// Sets the visible label.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Encodes the component.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert(
            "type",
            Value::Number(data::Number::from_u64(self.kind as u64)),
        );
        if let Some(custom_id) = &self.custom_id {
            object.insert("custom_id", Value::String(custom_id.clone()));
        }
        if let Some(label) = &self.label {
            object.insert("label", Value::String(label.clone()));
        }
        Value::Object(object)
    }
}

/// A collection of interactive components.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct View {
    components: Vec<Component>,
    timeout_seconds: Option<u64>,
}

/// Routes component custom IDs to callbacks.
#[derive(Default)]
pub struct ComponentRouter {
    handlers: Vec<(String, Box<dyn FnMut(&str)>)>,
}

/// Stores views that can survive a process-level restart boundary.
#[derive(Clone, Debug, Default)]
pub struct PersistentViews {
    views: HashMap<String, View>,
}

impl PersistentViews {
    /// Registers a persistent view.
    pub fn insert(&mut self, name: impl Into<String>, view: View) {
        self.views.insert(name.into(), view);
    }

    /// Retrieves a persistent view.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&View> {
        self.views.get(name)
    }

    /// Removes a persistent view.
    pub fn remove(&mut self, name: &str) -> Option<View> {
        self.views.remove(name)
    }
}

impl ComponentRouter {
    /// Registers a custom-ID handler.
    pub fn on(&mut self, custom_id: impl Into<String>, handler: impl FnMut(&str) + 'static) {
        self.handlers.push((custom_id.into(), Box::new(handler)));
    }

    /// Dispatches a component interaction.
    pub fn dispatch(&mut self, custom_id: &str) -> bool {
        let Some((_, handler)) = self
            .handlers
            .iter_mut()
            .find(|(registered, _)| registered == custom_id)
        else {
            return false;
        };
        handler(custom_id);
        true
    }
}

impl View {
    /// Creates a persistent view.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            components: Vec::new(),
            timeout_seconds: None,
        }
    }

    /// Sets an optional timeout.
    #[must_use]
    pub const fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = Some(timeout_seconds);
        self
    }

    /// Adds a component.
    pub fn add(&mut self, component: Component) -> Result<(), &'static str> {
        if self.components.len() >= 25 {
            return Err("view cannot contain more than 25 components");
        }
        self.components.push(component);
        Ok(())
    }

    /// Returns the components.
    #[must_use]
    pub fn components(&self) -> &[Component] {
        &self.components
    }

    /// Encodes the view's components as a JSON array.
    #[must_use]
    pub fn to_value(&self) -> Value {
        Value::Array(self.components.iter().map(Component::to_value).collect())
    }
}

/// A modal interaction containing form components.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Modal {
    /// The modal custom identifier.
    pub custom_id: String,
    /// The modal title.
    pub title: String,
    /// Modal components.
    pub components: Vec<Component>,
}

impl Modal {
    /// Creates a modal.
    #[must_use]
    pub fn new(custom_id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            custom_id: custom_id.into(),
            title: title.into(),
            components: Vec::new(),
        }
    }

    /// Adds a component.
    pub fn add(&mut self, component: Component) -> Result<(), &'static str> {
        if self.components.len() >= 5 {
            return Err("modal cannot contain more than 5 components");
        }
        self.components.push(component);
        Ok(())
    }

    /// Encodes the modal.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut object = Object::new();
        object.insert("custom_id", Value::String(self.custom_id.clone()));
        object.insert("title", Value::String(self.title.clone()));
        object.insert(
            "components",
            Value::Array(self.components.iter().map(Component::to_value).collect()),
        );
        Value::Object(object)
    }
}

// ---------------------------------------------------------------------------
// Multi-page paginator
// ---------------------------------------------------------------------------

/// An interactive paginator managing multi-page embeds or data with navigation buttons.
#[derive(Clone, Debug, PartialEq)]
pub struct Paginator<T> {
    pages: Vec<T>,
    current_page: usize,
    custom_id_prefix: String,
    show_first_last: bool,
}

impl<T> Paginator<T> {
    /// Creates a paginator with the given pages.
    #[must_use]
    pub fn new(pages: Vec<T>) -> Self {
        Self {
            pages,
            current_page: 0,
            custom_id_prefix: "paginator".to_owned(),
            show_first_last: true,
        }
    }

    /// Sets the custom ID prefix used for button interactions.
    #[must_use]
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.custom_id_prefix = prefix.into();
        self
    }

    /// Sets whether to include First (⏮) and Last (⏭) navigation buttons.
    #[must_use]
    pub const fn show_first_last(mut self, show: bool) -> Self {
        self.show_first_last = show;
        self
    }

    /// Returns the current page content, if any pages exist.
    #[must_use]
    pub fn current_page(&self) -> Option<&T> {
        self.pages.get(self.current_page)
    }

    /// Returns the current 0-based page index.
    #[must_use]
    pub const fn current_index(&self) -> usize {
        self.current_page
    }

    /// Returns the total number of pages.
    #[must_use]
    pub fn total_pages(&self) -> usize {
        self.pages.len()
    }

    /// Advances to the next page. Returns `true` if the page changed.
    pub fn next(&mut self) -> bool {
        if self.current_page + 1 < self.pages.len() {
            self.current_page += 1;
            true
        } else {
            false
        }
    }

    /// Moves to the previous page. Returns `true` if the page changed.
    pub fn prev(&mut self) -> bool {
        if self.current_page > 0 {
            self.current_page -= 1;
            true
        } else {
            false
        }
    }

    /// Jumps to the first page. Returns `true` if the page changed.
    pub fn first(&mut self) -> bool {
        if self.current_page != 0 && !self.pages.is_empty() {
            self.current_page = 0;
            true
        } else {
            false
        }
    }

    /// Jumps to the last page. Returns `true` if the page changed.
    pub fn last(&mut self) -> bool {
        if !self.pages.is_empty() && self.current_page != self.pages.len() - 1 {
            self.current_page = self.pages.len() - 1;
            true
        } else {
            false
        }
    }

    /// Jumps to a specific 0-based page index.
    pub fn go_to(&mut self, index: usize) -> bool {
        if index < self.pages.len() && self.current_page != index {
            self.current_page = index;
            true
        } else {
            false
        }
    }

    /// Handles an incoming component interaction `custom_id`.
    /// Returns `true` if the paginator handled the action and updated its page.
    pub fn handle_interaction(&mut self, custom_id: &str) -> bool {
        let prefix = format!("{}:", self.custom_id_prefix);
        if let Some(action) = custom_id.strip_prefix(&prefix) {
            match action {
                "first" => self.first(),
                "prev" => self.prev(),
                "next" => self.next(),
                "last" => self.last(),
                _ => false,
            }
        } else {
            false
        }
    }

    /// Generates an [`ActionRow`] with navigation buttons appropriately enabled or disabled.
    #[must_use]
    pub fn action_row(&self) -> ActionRow {
        let mut row = ActionRow::new();
        let is_first = self.current_page == 0;
        let is_last = self.pages.is_empty() || self.current_page >= self.pages.len() - 1;

        if self.show_first_last {
            let _ = row.add_button(
                Button::secondary(format!("{}:first", self.custom_id_prefix), "⏮")
                    .disabled(is_first),
            );
        }

        let _ = row.add_button(
            Button::secondary(format!("{}:prev", self.custom_id_prefix), "◀").disabled(is_first),
        );

        let label = format!("{}/{}", self.current_page + 1, self.pages.len().max(1));
        let _ = row.add_button(
            Button::secondary(format!("{}:indicator", self.custom_id_prefix), label).disabled(true),
        );

        let _ = row.add_button(
            Button::secondary(format!("{}:next", self.custom_id_prefix), "▶").disabled(is_last),
        );

        if self.show_first_last {
            let _ = row.add_button(
                Button::secondary(format!("{}:last", self.custom_id_prefix), "⏭").disabled(is_last),
            );
        }

        row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_builder_preserves_values_and_styles() {
        let btn = Button::primary("btn_confirm", "Confirm")
            .disabled(true)
            .sku_id(12345);

        let val = btn.to_value();
        match val {
            Value::Object(obj) => {
                assert_eq!(obj.get("type"), Some(&Value::Number(2_u64.into())));
                assert_eq!(obj.get("style"), Some(&Value::Number(1_u64.into())));
                assert_eq!(
                    obj.get("custom_id"),
                    Some(&Value::String("btn_confirm".into()))
                );
                assert_eq!(obj.get("label"), Some(&Value::String("Confirm".into())));
                assert_eq!(obj.get("disabled"), Some(&Value::Bool(true)));
                assert_eq!(obj.get("sku_id"), Some(&Value::String("12345".into())));
            }
            _ => panic!("expected object"),
        }
    }

    #[test]
    fn select_menu_options_and_limits() {
        let mut select = StringSelect::new("choices")
            .placeholder("Pick something")
            .min_values(1)
            .max_values(3);

        for i in 0..25 {
            assert!(
                select
                    .add_option(SelectOption::new(format!("Choice {i}"), format!("val_{i}")))
                    .is_ok()
            );
        }
        assert!(
            select
                .add_option(SelectOption::new("overflow", "overflow"))
                .is_err()
        );

        let val = select.to_value();
        match val {
            Value::Object(obj) => {
                assert_eq!(obj.get("type"), Some(&Value::Number(3_u64.into())));
                assert_eq!(
                    obj.get("placeholder"),
                    Some(&Value::String("Pick something".into()))
                );
                assert_eq!(obj.get("min_values"), Some(&Value::Number(1_u64.into())));
                assert_eq!(obj.get("max_values"), Some(&Value::Number(3_u64.into())));
            }
            _ => panic!("expected object"),
        }
    }

    #[test]
    fn text_input_builder_and_action_row_validation() {
        let input = TextInput::short("name_field", "Your Name")
            .required(true)
            .min_length(2)
            .max_length(32);

        let mut row = ActionRow::new();
        assert!(row.add_text_input(input).is_ok());
        // ActionRow cannot contain anything else if it already has a text input
        assert!(row.add_button(Button::primary("c", "l")).is_err());

        let mut btn_row = ActionRow::new();
        for i in 0..5 {
            assert!(
                btn_row
                    .add_button(Button::secondary(format!("b{i}"), "B"))
                    .is_ok()
            );
        }
        assert!(
            btn_row
                .add_button(Button::secondary("overflow", "B"))
                .is_err()
        );

        let mut container = Container::new();
        for _ in 0..10 {
            assert!(container.add(Value::Null).is_ok());
        }
        assert!(container.add(Value::Null).is_err());
    }

    #[test]
    fn paginator_navigation_and_interactions() {
        let pages = vec!["Page 1", "Page 2", "Page 3"];
        let mut paginator = Paginator::new(pages).prefix("p");

        assert_eq!(paginator.total_pages(), 3);
        assert_eq!(paginator.current_index(), 0);
        assert_eq!(paginator.current_page(), Some(&"Page 1"));

        // First row at start: prev and first disabled
        let row = paginator.action_row();
        assert_eq!(row.components.len(), 5);

        // Interaction next
        assert!(paginator.handle_interaction("p:next"));
        assert_eq!(paginator.current_index(), 1);
        assert_eq!(paginator.current_page(), Some(&"Page 2"));

        // Interaction last
        assert!(paginator.handle_interaction("p:last"));
        assert_eq!(paginator.current_index(), 2);
        assert!(!paginator.handle_interaction("p:next")); // already last

        // Interaction prev
        assert!(paginator.handle_interaction("p:prev"));
        assert_eq!(paginator.current_index(), 1);

        // Interaction first
        assert!(paginator.handle_interaction("p:first"));
        assert_eq!(paginator.current_index(), 0);
        assert!(!paginator.handle_interaction("p:first")); // already first
    }
}
