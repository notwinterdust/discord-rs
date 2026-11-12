//! Interaction component composition primitives.

use data::{Object, Value};

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
