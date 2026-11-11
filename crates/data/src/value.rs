//! JSON values.
//!
//! [`Value`] is a small, allocation-friendly JSON tree. Objects preserve
//! insertion order and reject duplicate keys by replacing the previous value.

use std::fmt;

/// A JSON number.
///
/// Integer values are kept as integers until they no longer fit, while values
/// written with a fraction or exponent are kept as floating point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Number {
    /// A signed integer.
    Integer(i64),
    /// An unsigned integer that does not fit in `i64`.
    Unsigned(u64),
    /// A floating-point number.
    Float(f64),
}

impl Number {
    /// Creates a signed integer number.
    #[must_use]
    pub const fn from_i64(value: i64) -> Self {
        Self::Integer(value)
    }

    /// Creates an unsigned integer number.
    #[must_use]
    pub const fn from_u64(value: u64) -> Self {
        Self::Unsigned(value)
    }

    /// Creates a floating-point number, returning `None` for non-finite values.
    #[must_use]
    pub fn from_f64(value: f64) -> Option<Self> {
        value.is_finite().then_some(Self::Float(value))
    }

    /// Returns the value as an `i64`, if it fits.
    #[must_use]
    pub const fn as_i64(self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(value),
            Self::Unsigned(value) => {
                if value <= i64::MAX as u64 {
                    Some(value as i64)
                } else {
                    None
                }
            }
            Self::Float(_) => None,
        }
    }

    /// Returns the value as a `u64`, if it is a non-negative integer.
    #[must_use]
    pub const fn as_u64(self) -> Option<u64> {
        match self {
            Self::Integer(value) => {
                if value >= 0 {
                    Some(value as u64)
                } else {
                    None
                }
            }
            Self::Unsigned(value) => Some(value),
            Self::Float(_) => None,
        }
    }

    /// Returns the value as an `f64`.
    #[must_use]
    pub fn as_f64(self) -> f64 {
        match self {
            Self::Integer(value) => value as f64,
            Self::Unsigned(value) => value as f64,
            Self::Float(value) => value,
        }
    }

    /// Returns `true` if the number has no fractional or exponent part.
    #[must_use]
    pub const fn is_integer(self) -> bool {
        !matches!(self, Self::Float(_))
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(value) => write!(f, "{value}"),
            Self::Unsigned(value) => write!(f, "{value}"),
            Self::Float(value) => {
                if value.is_finite() {
                    write!(f, "{value}")
                } else {
                    f.write_str("null")
                }
            }
        }
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Self::Unsigned(value)
    }
}

/// A JSON object that preserves insertion order.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Object {
    entries: Vec<(String, Value)>,
}

impl Object {
    /// Creates an empty object.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Creates an empty object with room for `capacity` entries.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if the object has no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns `true` if the object contains the given key.
    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        self.entries.iter().any(|(name, _)| name == key)
    }

    /// Returns the value for the given key.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.entries
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value)
    }

    /// Returns a mutable reference to the value for the given key.
    #[must_use]
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.entries
            .iter_mut()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value)
    }

    /// Inserts a key, replacing and returning the previous value if present.
    pub fn insert(&mut self, key: impl Into<String>, value: Value) -> Option<Value> {
        let key = key.into();
        if let Some((_, existing)) = self.entries.iter_mut().find(|(name, _)| name == &key) {
            return Some(std::mem::replace(existing, value));
        }
        self.entries.push((key, value));
        None
    }

    /// Removes a key and returns its value.
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        let index = self.entries.iter().position(|(name, _)| name == key)?;
        Some(self.entries.remove(index).1)
    }

    /// Iterates over the object's key/value pairs in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.entries
            .iter()
            .map(|(key, value)| (key.as_str(), value))
    }

    /// Iterates over the object's keys in insertion order.
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|(key, _)| key.as_str())
    }

    /// Iterates over the object's values in insertion order.
    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.entries.iter().map(|(_, value)| value)
    }
}

impl FromIterator<(String, Value)> for Object {
    fn from_iter<I: IntoIterator<Item = (String, Value)>>(iter: I) -> Self {
        let mut object = Self::new();
        for (key, value) in iter {
            object.insert(key, value);
        }
        object
    }
}

impl Extend<(String, Value)> for Object {
    fn extend<I: IntoIterator<Item = (String, Value)>>(&mut self, iter: I) {
        for (key, value) in iter {
            self.insert(key, value);
        }
    }
}

impl IntoIterator for Object {
    type Item = (String, Value);
    type IntoIter = std::vec::IntoIter<(String, Value)>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<'a> IntoIterator for &'a Object {
    type Item = (&'a str, &'a Value);
    type IntoIter = std::iter::Map<
        std::slice::Iter<'a, (String, Value)>,
        fn(&'a (String, Value)) -> (&'a str, &'a Value),
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.entries
            .iter()
            .map(|(key, value)| (key.as_str(), value))
    }
}

/// A JSON value.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// JSON `null`.
    Null,
    /// A boolean.
    Bool(bool),
    /// A number.
    Number(Number),
    /// A string.
    String(String),
    /// An array.
    Array(Vec<Value>),
    /// An object.
    Object(Object),
}

impl Value {
    /// Creates an empty object value.
    #[must_use]
    pub const fn object() -> Self {
        Self::Object(Object::new())
    }

    /// Creates an empty array value.
    #[must_use]
    pub const fn array() -> Self {
        Self::Array(Vec::new())
    }

    /// Returns `true` if the value is `null`.
    #[must_use]
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Returns `true` if the value is a boolean.
    #[must_use]
    pub const fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    /// Returns `true` if the value is a number.
    #[must_use]
    pub const fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    /// Returns `true` if the value is a string.
    #[must_use]
    pub const fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Returns `true` if the value is an array.
    #[must_use]
    pub const fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    /// Returns `true` if the value is an object.
    #[must_use]
    pub const fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    /// Returns the JSON type name.
    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Bool(_) => "boolean",
            Self::Number(_) => "number",
            Self::String(_) => "string",
            Self::Array(_) => "array",
            Self::Object(_) => "object",
        }
    }

    /// Returns the boolean value, if this is a boolean.
    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the number, if this is a number.
    #[must_use]
    pub const fn as_number(&self) -> Option<Number> {
        match self {
            Self::Number(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the value as an `i64`, if it is an integer that fits.
    #[must_use]
    pub const fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Number(value) => value.as_i64(),
            _ => None,
        }
    }

    /// Returns the value as a `u64`, if it is a non-negative integer.
    #[must_use]
    pub const fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Number(value) => value.as_u64(),
            _ => None,
        }
    }

    /// Returns the value as an `f64`, if it is a number.
    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(value) => Some(value.as_f64()),
            _ => None,
        }
    }

    /// Returns the string, if this is a string.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the array, if this is an array.
    #[must_use]
    pub fn as_array(&self) -> Option<&[Value]> {
        match self {
            Self::Array(value) => Some(value),
            _ => None,
        }
    }

    /// Returns a mutable reference to the array, if this is an array.
    #[must_use]
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Self::Array(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the object, if this is an object.
    #[must_use]
    pub fn as_object(&self) -> Option<&Object> {
        match self {
            Self::Object(value) => Some(value),
            _ => None,
        }
    }

    /// Returns a mutable reference to the object, if this is an object.
    #[must_use]
    pub fn as_object_mut(&mut self) -> Option<&mut Object> {
        match self {
            Self::Object(value) => Some(value),
            _ => None,
        }
    }

    /// Looks up an object key.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.as_object()?.get(key)
    }

    /// Looks up a mutable object key.
    #[must_use]
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.as_object_mut()?.get_mut(key)
    }

    /// Looks up an array index.
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<&Value> {
        self.as_array()?.get(index)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&crate::json::to_string(self))
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Number(Number::Integer(value))
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Number(Number::Integer(i64::from(value)))
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Self::Number(Number::Unsigned(value))
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::Number(Number::Unsigned(u64::from(value)))
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(Number::Float(value))
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Self::Array(value)
    }
}

impl From<Object> for Value {
    fn from(value: Object) -> Self {
        Self::Object(value)
    }
}

impl From<()> for Value {
    fn from((): ()) -> Self {
        Self::Null
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(value: Option<T>) -> Self {
        value.map_or(Self::Null, Into::into)
    }
}

impl std::str::FromStr for Value {
    type Err = crate::json::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        crate::json::from_str(input)
    }
}

#[cfg(test)]
mod tests {
    use super::{Number, Object, Value};

    #[test]
    fn number_accessors() {
        let signed = Number::Integer(-5);
        assert_eq!(signed.as_i64(), Some(-5));
        assert_eq!(signed.as_u64(), None);
        assert_eq!(signed.as_f64(), -5.0);
        assert!(signed.is_integer());

        let unsigned = Number::Unsigned(u64::MAX);
        assert_eq!(unsigned.as_i64(), None);
        assert_eq!(unsigned.as_u64(), Some(u64::MAX));
        assert_eq!(Number::Unsigned(7).as_i64(), Some(7));
        assert_eq!(Number::Unsigned(7).as_u64(), Some(7));

        let float = Number::Float(1.5);
        assert_eq!(float.as_i64(), None);
        assert_eq!(float.as_u64(), None);
        assert_eq!(float.as_f64(), 1.5);
        assert!(!float.is_integer());
        assert_eq!(Number::from_f64(f64::NAN), None);
        assert_eq!(Number::from_f64(2.0), Some(Number::Float(2.0)));
        assert_eq!(Number::from(3_u64).to_string(), "3");
        assert_eq!(Number::from(-3_i64).to_string(), "-3");
        assert_eq!(Number::Float(f64::INFINITY).to_string(), "null");
    }

    #[test]
    fn objects_replace_duplicate_keys_and_preserve_order() {
        let mut object = Object::with_capacity(3);
        assert!(object.is_empty());
        assert_eq!(object.insert("b", Value::from(1_i64)), None);
        assert_eq!(object.insert("a", Value::from(2_i64)), None);
        assert_eq!(
            object.insert("b", Value::from(3_i64)),
            Some(Value::from(1_i64))
        );

        assert_eq!(object.len(), 2);
        assert!(object.contains_key("a"));
        assert!(!object.contains_key("missing"));
        assert_eq!(object.keys().collect::<Vec<_>>(), vec!["b", "a"]);
        assert_eq!(object.get("b").and_then(Value::as_i64), Some(3));
        *object.get_mut("a").unwrap() = Value::from(4_i64);
        assert_eq!(object.get("a").and_then(Value::as_i64), Some(4));

        assert_eq!(object.remove("a"), Some(Value::from(4_i64)));
        assert_eq!(object.remove("a"), None);
        assert_eq!(object.iter().count(), 1);

        let collected: Object = vec![
            (String::from("x"), Value::from(1_i64)),
            (String::from("y"), Value::from(2_i64)),
        ]
        .into_iter()
        .collect();
        assert_eq!(collected.len(), 2);

        let mut extended = Object::new();
        extended.extend(collected.clone());
        assert_eq!(extended, collected);
        assert_eq!(
            (&collected)
                .into_iter()
                .map(|(key, _)| key)
                .collect::<Vec<_>>(),
            vec!["x", "y"]
        );
        assert_eq!(collected.into_iter().count(), 2);
    }

    #[test]
    fn value_helpers_and_conversions() {
        let mut value = Value::from(1_i64);
        assert!(value.is_number());
        assert_eq!(value.type_name(), "number");
        assert_eq!(value.as_number(), Some(Number::Integer(1)));
        assert_eq!(value.as_i64(), Some(1));
        assert_eq!(value.as_u64(), Some(1));
        assert_eq!(value.as_f64(), Some(1.0));
        assert_eq!(value.as_str(), None);
        assert_eq!(value.as_bool(), None);
        assert!(!value.is_null());

        value = Value::from("text");
        assert!(value.is_string());
        assert_eq!(value.as_str(), Some("text"));

        value = Value::from(vec![Value::from(true), Value::from(2_u32)]);
        assert!(value.is_array());
        assert_eq!(value.as_array().map(<[Value]>::len), Some(2));
        assert_eq!(value.get_index(0).and_then(Value::as_bool), Some(true));
        assert!(value.get_index(2).is_none());
        value.as_array_mut().unwrap().push(Value::from(3_i32));

        let object = Value::object();
        assert!(object.is_object());
        assert!(object.as_object().unwrap().is_empty());
        assert_eq!(Value::array().as_array().map(<[Value]>::len), Some(0));

        let mut object = Value::object();
        *object.as_object_mut().unwrap() = Object::new();
        object
            .as_object_mut()
            .unwrap()
            .insert("key", Value::from(1_u64));
        assert_eq!(object.get("key").and_then(Value::as_u64), Some(1));
        *object.get_mut("key").unwrap() = Value::from(2_u64);
        assert_eq!(object.get("key").and_then(Value::as_u64), Some(2));
        assert_eq!(object.get("missing"), None);
        assert_eq!(object.get_mut("missing"), None);

        assert!(Value::from(()).is_null());
        assert!(Value::from(Option::<i64>::None).is_null());
        assert_eq!(Value::from(Some(5_i64)).as_i64(), Some(5));
        assert_eq!(Value::from(1.5).as_f64(), Some(1.5));
        assert_eq!(Value::from(String::from("s")).as_str(), Some("s"));
        assert_eq!(Value::from(1_u64).to_string(), "1");
    }
}
