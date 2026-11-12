//! JSON parsing and serialization.
//!
//! The parser implements RFC 8259 without external dependencies. It preserves
//! object insertion order and rejects invalid syntax, trailing data and
//! excessively deep nesting.

use std::error::Error;
use std::fmt;

use crate::value::{Number, Object, Value};

/// The maximum nesting depth accepted by the parser.
pub const MAX_DEPTH: usize = 128;

/// An error produced while parsing JSON.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    message: String,
    offset: usize,
    line: usize,
    column: usize,
}

impl ParseError {
    fn new(input: &str, offset: usize, message: impl Into<String>) -> Self {
        let offset = offset.min(input.len());
        let prefix = &input[..offset];
        let line = prefix.bytes().filter(|byte| *byte == b'\n').count() + 1;
        let column = prefix
            .rsplit_once('\n')
            .map_or(prefix, |(_, rest)| rest)
            .chars()
            .count()
            + 1;
        Self {
            message: message.into(),
            offset,
            line,
            column,
        }
    }

    /// Returns the human-readable error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the byte offset at which the error occurred.
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the one-based line number of the error.
    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    /// Returns the one-based column number of the error.
    #[must_use]
    pub const fn column(&self) -> usize {
        self.column
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at line {} column {}",
            self.message, self.line, self.column
        )
    }
}

impl Error for ParseError {}

/// Parses a JSON document into a [`Value`].
///
/// # Errors
///
/// Returns a [`ParseError`] when the input is not valid JSON or has trailing
/// non-whitespace data.
pub fn from_str(input: &str) -> Result<Value, ParseError> {
    Parser::new(input).parse_document()
}

/// Serializes a value to compact JSON.
#[must_use]
pub fn to_string(value: &Value) -> String {
    let mut output = String::new();
    write_value(value, &mut output, None);
    output
}

/// Serializes a value to indented JSON using two spaces per level.
#[must_use]
pub fn to_string_pretty(value: &Value) -> String {
    let mut output = String::new();
    write_value(value, &mut output, Some(0));
    output
}

struct Parser<'a> {
    input: &'a str,
    position: usize,
    depth: usize,
}

impl<'a> Parser<'a> {
    const fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            depth: 0,
        }
    }

    fn parse_document(mut self) -> Result<Value, ParseError> {
        self.skip_whitespace();
        let value = self.parse_value()?;
        self.skip_whitespace();
        if self.position != self.input.len() {
            return Err(self.error("trailing characters"));
        }
        Ok(value)
    }

    fn parse_value(&mut self) -> Result<Value, ParseError> {
        self.skip_whitespace();
        match self.peek() {
            Some(b'{') => self.parse_object(),
            Some(b'[') => self.parse_array(),
            Some(b'"') => self.parse_string().map(Value::String),
            Some(b't') => self.parse_literal("true", Value::Bool(true)),
            Some(b'f') => self.parse_literal("false", Value::Bool(false)),
            Some(b'n') => self.parse_literal("null", Value::Null),
            Some(b'-' | b'0'..=b'9') => self.parse_number(),
            Some(_) => Err(self.error("expected a value")),
            None => Err(self.error("unexpected end of input, expected a value")),
        }
    }

    fn parse_literal(&mut self, literal: &str, value: Value) -> Result<Value, ParseError> {
        if self.input[self.position..].starts_with(literal) {
            self.position += literal.len();
            Ok(value)
        } else {
            Err(self.error("invalid literal"))
        }
    }

    fn parse_object(&mut self) -> Result<Value, ParseError> {
        self.position += 1;
        self.enter()?;
        self.skip_whitespace();

        let mut object = Object::new();
        if self.peek() == Some(b'}') {
            self.position += 1;
            self.depth -= 1;
            return Ok(Value::Object(object));
        }

        loop {
            self.skip_whitespace();
            if self.peek() != Some(b'"') {
                return Err(self.error("expected an object key"));
            }
            let key = self.parse_string()?;

            self.skip_whitespace();
            if self.peek() != Some(b':') {
                return Err(self.error("expected ':' after object key"));
            }
            self.position += 1;

            let value = self.parse_value()?;
            object.insert(key, value);

            self.skip_whitespace();
            match self.peek() {
                Some(b',') => self.position += 1,
                Some(b'}') => {
                    self.position += 1;
                    self.depth -= 1;
                    return Ok(Value::Object(object));
                }
                Some(_) => return Err(self.error("expected ',' or '}'")),
                None => return Err(self.error("unterminated object")),
            }
        }
    }

    fn parse_array(&mut self) -> Result<Value, ParseError> {
        self.position += 1;
        self.enter()?;
        self.skip_whitespace();

        let mut items = Vec::new();
        if self.peek() == Some(b']') {
            self.position += 1;
            self.depth -= 1;
            return Ok(Value::Array(items));
        }

        loop {
            items.push(self.parse_value()?);
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => self.position += 1,
                Some(b']') => {
                    self.position += 1;
                    self.depth -= 1;
                    return Ok(Value::Array(items));
                }
                Some(_) => return Err(self.error("expected ',' or ']'")),
                None => return Err(self.error("unterminated array")),
            }
        }
    }

    fn parse_string(&mut self) -> Result<String, ParseError> {
        debug_assert_eq!(self.peek(), Some(b'"'));
        self.position += 1;
        let mut output = String::new();

        loop {
            let Some(byte) = self.peek() else {
                return Err(self.error("unterminated string"));
            };
            match byte {
                b'"' => {
                    self.position += 1;
                    return Ok(output);
                }
                b'\\' => {
                    self.position += 1;
                    self.parse_escape(&mut output)?;
                }
                0x00..=0x1f => {
                    return Err(self.error("unescaped control character in string"));
                }
                _ => {
                    let character = self.input[self.position..]
                        .chars()
                        .next()
                        .expect("position is always on a character boundary");
                    output.push(character);
                    self.position += character.len_utf8();
                }
            }
        }
    }

    fn parse_escape(&mut self, output: &mut String) -> Result<(), ParseError> {
        let Some(byte) = self.peek() else {
            return Err(self.error("unterminated escape sequence"));
        };
        match byte {
            b'"' => {
                output.push('"');
                self.position += 1;
            }
            b'\\' => {
                output.push('\\');
                self.position += 1;
            }
            b'/' => {
                output.push('/');
                self.position += 1;
            }
            b'b' => {
                output.push('\u{0008}');
                self.position += 1;
            }
            b'f' => {
                output.push('\u{000c}');
                self.position += 1;
            }
            b'n' => {
                output.push('\n');
                self.position += 1;
            }
            b'r' => {
                output.push('\r');
                self.position += 1;
            }
            b't' => {
                output.push('\t');
                self.position += 1;
            }
            b'u' => {
                self.position += 1;
                self.parse_unicode_escape(output)?;
            }
            _ => return Err(self.error("invalid escape sequence")),
        }
        Ok(())
    }

    fn parse_unicode_escape(&mut self, output: &mut String) -> Result<(), ParseError> {
        let first = self.parse_hex_quad()?;

        if (0xd800..=0xdbff).contains(&first) {
            if self.peek() != Some(b'\\') {
                return Err(self.error("unpaired high surrogate"));
            }
            self.position += 1;
            if self.peek() != Some(b'u') {
                return Err(self.error("expected a low surrogate"));
            }
            self.position += 1;
            let second = self.parse_hex_quad()?;
            if !(0xdc00..=0xdfff).contains(&second) {
                return Err(self.error("invalid low surrogate"));
            }
            let code =
                0x1_0000 + ((u32::from(first) - 0xd800) << 10) + (u32::from(second) - 0xdc00);
            if let Some(character) = char::from_u32(code) {
                output.push(character);
            } else {
                return Err(self.error("invalid unicode escape"));
            }
        } else if (0xdc00..=0xdfff).contains(&first) {
            return Err(self.error("unpaired low surrogate"));
        } else if let Some(character) = char::from_u32(u32::from(first)) {
            output.push(character);
        } else {
            return Err(self.error("invalid unicode escape"));
        }

        Ok(())
    }

    fn parse_hex_quad(&mut self) -> Result<u16, ParseError> {
        let mut value = 0_u16;
        for _ in 0..4 {
            let Some(byte) = self.peek() else {
                return Err(self.error("unterminated unicode escape"));
            };
            let Some(digit) = hex_value(byte) else {
                return Err(self.error("invalid unicode escape"));
            };
            value = (value << 4) | u16::from(digit);
            self.position += 1;
        }
        Ok(value)
    }

    fn parse_number(&mut self) -> Result<Value, ParseError> {
        let start = self.position;

        if self.peek() == Some(b'-') {
            self.position += 1;
        }

        match self.peek() {
            Some(b'0') => {
                self.position += 1;
                if matches!(self.peek(), Some(b'0'..=b'9')) {
                    return Err(self.error("leading zeros are not allowed"));
                }
            }
            Some(b'1'..=b'9') => {
                self.consume_digits();
            }
            _ => return Err(self.error("expected a digit")),
        }

        let mut is_float = false;
        if self.peek() == Some(b'.') {
            is_float = true;
            self.position += 1;
            if !self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
                return Err(self.error("expected a digit after the decimal point"));
            }
            self.consume_digits();
        }

        if matches!(self.peek(), Some(b'e' | b'E')) {
            is_float = true;
            self.position += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.position += 1;
            }
            if !self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
                return Err(self.error("expected a digit in the exponent"));
            }
            self.consume_digits();
        }

        let raw = &self.input[start..self.position];
        let number = if is_float {
            Number::Float(self.parse_float(raw)?)
        } else if let Ok(value) = raw.parse::<i64>() {
            Number::Integer(value)
        } else if let Ok(value) = raw.parse::<u64>() {
            Number::Unsigned(value)
        } else {
            Number::Float(self.parse_float(raw)?)
        };

        Ok(Value::Number(number))
    }

    fn parse_float(&self, raw: &str) -> Result<f64, ParseError> {
        let value = raw
            .parse::<f64>()
            .map_err(|_| self.error("invalid number"))?;
        if value.is_finite() {
            Ok(value)
        } else {
            Err(self.error("number is out of range"))
        }
    }

    fn consume_digits(&mut self) {
        while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
            self.position += 1;
        }
    }

    fn enter(&mut self) -> Result<(), ParseError> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            Err(self.error("maximum nesting depth exceeded"))
        } else {
            Ok(())
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.position += 1;
        }
    }

    fn peek(&self) -> Option<u8> {
        self.input.as_bytes().get(self.position).copied()
    }

    fn error(&self, message: impl Into<String>) -> ParseError {
        ParseError::new(self.input, self.position, message)
    }
}

const fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn write_value(value: &Value, output: &mut String, indent: Option<usize>) {
    match value {
        Value::Null => output.push_str("null"),
        Value::Bool(true) => output.push_str("true"),
        Value::Bool(false) => output.push_str("false"),
        Value::Number(number) => output.push_str(&number.to_string()),
        Value::String(string) => write_string(string, output),
        Value::Array(items) => write_array(items, output, indent),
        Value::Object(object) => write_object(object, output, indent),
    }
}

fn write_array(items: &[Value], output: &mut String, indent: Option<usize>) {
    if items.is_empty() {
        output.push_str("[]");
        return;
    }

    output.push('[');
    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        if let Some(level) = indent {
            output.push('\n');
            write_indent(output, level + 1);
        }
        write_value(item, output, indent.map(|level| level + 1));
    }
    if let Some(level) = indent {
        output.push('\n');
        write_indent(output, level);
    }
    output.push(']');
}

fn write_object(object: &Object, output: &mut String, indent: Option<usize>) {
    if object.is_empty() {
        output.push_str("{}");
        return;
    }

    output.push('{');
    for (index, (key, value)) in object.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        if let Some(level) = indent {
            output.push('\n');
            write_indent(output, level + 1);
        }
        write_string(key, output);
        output.push(':');
        if indent.is_some() {
            output.push(' ');
        }
        write_value(value, output, indent.map(|level| level + 1));
    }
    if let Some(level) = indent {
        output.push('\n');
        write_indent(output, level);
    }
    output.push('}');
}

fn write_indent(output: &mut String, level: usize) {
    for _ in 0..level {
        output.push_str("  ");
    }
}

fn write_string(value: &str, output: &mut String) {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\u{0008}' => output.push_str("\\b"),
            '\u{000c}' => output.push_str("\\f"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{0000}'..='\u{001f}' => {
                let code = character as usize;
                output.push_str("\\u00");
                output.push(char::from(HEX[(code >> 4) & 0x0f]));
                output.push(char::from(HEX[code & 0x0f]));
            }
            _ => output.push(character),
        }
    }
    output.push('"');
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{MAX_DEPTH, ParseError, from_str, to_string, to_string_pretty};
    use crate::value::{Number, Object, Value};

    #[test]
    fn parses_scalars_and_containers() {
        assert_eq!(from_str("null"), Ok(Value::Null));
        assert_eq!(from_str("true"), Ok(Value::Bool(true)));
        assert_eq!(from_str("false"), Ok(Value::Bool(false)));
        assert_eq!(from_str(" 42 "), Ok(Value::Number(Number::Integer(42))));
        assert_eq!(from_str("-0"), Ok(Value::Number(Number::Integer(0))));
        assert_eq!(
            from_str("18446744073709551615"),
            Ok(Value::Number(Number::Unsigned(u64::MAX)))
        );
        assert_eq!(
            from_str("-9223372036854775808"),
            Ok(Value::Number(Number::Integer(i64::MIN)))
        );
        assert_eq!(from_str("1.5"), Ok(Value::Number(Number::Float(1.5))));
        assert_eq!(from_str("1e2"), Ok(Value::Number(Number::Float(100.0))));
        assert!(matches!(
            from_str("123456789012345678901234567890"),
            Ok(Value::Number(Number::Float(_)))
        ));

        let parsed = from_str(
            r#" {
                "name": "value",
                "list": [1, true, null, {"nested": "yes"}],
                "empty_object": {},
                "empty_array": []
            } "#,
        )
        .unwrap();
        assert_eq!(parsed.get("name").and_then(Value::as_str), Some("value"));
        assert_eq!(
            parsed
                .get("list")
                .and_then(Value::as_array)
                .map(<[Value]>::len),
            Some(4)
        );
        assert_eq!(
            parsed
                .get("list")
                .and_then(|list| list.get_index(3))
                .and_then(|nested| nested.get("nested"))
                .and_then(Value::as_str),
            Some("yes")
        );
        assert!(
            parsed
                .get("empty_object")
                .unwrap()
                .as_object()
                .unwrap()
                .is_empty()
        );
        assert!(
            parsed
                .get("empty_array")
                .unwrap()
                .as_array()
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn parses_string_escapes_and_unicode() {
        let parsed = from_str(r#""a\n\t\"\\\/\b\f\r\u00e9\uD83D\uDE00""#).unwrap();
        assert_eq!(
            parsed.as_str(),
            Some("a\n\t\"\\/\u{8}\u{c}\r\u{e9}\u{1f600}")
        );

        let control = from_str(r#""\u0000\u001f""#).unwrap();
        assert_eq!(control.as_str(), Some("\u{0}\u{1f}"));
    }

    #[test]
    fn duplicate_keys_keep_the_last_value() {
        let parsed = from_str(r#"{"a": 1, "b": 2, "a": 3}"#).unwrap();
        assert_eq!(parsed.get("a").and_then(Value::as_i64), Some(3));
        assert_eq!(parsed.get("b").and_then(Value::as_i64), Some(2));
        assert_eq!(parsed.as_object().unwrap().len(), 2);
    }

    #[test]
    fn rejects_invalid_documents() {
        let invalid = [
            "",
            "{",
            "}",
            "[",
            "[1,]",
            "[1 2]",
            r#"{"a": 1,}"#,
            r#"{"a" 1}"#,
            "{a: 1}",
            "\"unterminated",
            "\"control\u{1}\"",
            r#""\x""#,
            r#""\u12""#,
            r#""\uZZZZ""#,
            r#""\uD800""#,
            r#""\uDC00""#,
            r#""\uD800\u0041""#,
            "01",
            "-",
            "1.",
            ".5",
            "+1",
            "1e",
            "1e+",
            "NaN",
            "Infinity",
            "truex",
            "null null",
            "[1] trailing",
        ];

        for input in invalid {
            assert!(from_str(input).is_err(), "expected error for {input:?}");
        }
    }

    #[test]
    fn enforces_a_nesting_limit() {
        let shallow = format!("{}0{}", "[".repeat(MAX_DEPTH), "]".repeat(MAX_DEPTH));
        assert!(from_str(&shallow).is_ok());

        let deep = format!(
            "{}0{}",
            "[".repeat(MAX_DEPTH + 1),
            "]".repeat(MAX_DEPTH + 1)
        );
        let error = from_str(&deep).unwrap_err();
        assert!(error.message().contains("nesting depth"));
    }

    #[test]
    fn reports_line_and_column() {
        let error = from_str("{\n  \"a\": 1,\n  \"b\": }\n").unwrap_err();
        assert_eq!(error.line(), 3);
        assert_eq!(error.column(), 8);
        assert_eq!(error.offset(), 19);
        assert_eq!(error.to_string(), "expected a value at line 3 column 8");
    }

    #[test]
    fn serializes_compact_json() {
        let mut object = Object::new();
        object.insert("name", Value::from("a\n\"b\""));
        object.insert(
            "values",
            Value::from(vec![Value::from(1_i64), Value::from(true)]),
        );
        object.insert("empty", Value::object());
        let value = Value::Object(object);

        assert_eq!(
            to_string(&value),
            r#"{"name":"a\n\"b\"","values":[1,true],"empty":{}}"#
        );
        assert_eq!(to_string(&Value::Null), "null");
        assert_eq!(to_string(&Value::from(1.5)), "1.5");
        assert_eq!(to_string(&Value::from(f64::NAN)), "null");
        assert_eq!(to_string(&Value::from("\u{1f600}")), "\"\u{1f600}\"");
        assert_eq!(to_string(&Value::from("\u{1}")), "\"\\u0001\"");
    }

    #[test]
    fn serializes_pretty_json() {
        let mut nested = Object::new();
        nested.insert("a", Value::from(1_i64));
        let mut object = Object::new();
        object.insert("nested", Value::Object(nested));
        object.insert("items", Value::from(vec![Value::from(true), Value::Null]));

        assert_eq!(
            to_string_pretty(&Value::Object(object)),
            "{\n  \"nested\": {\n    \"a\": 1\n  },\n  \"items\": [\n    true,\n    null\n  ]\n}"
        );
        assert_eq!(to_string_pretty(&Value::object()), "{}");
        assert_eq!(to_string_pretty(&Value::array()), "[]");
    }

    #[test]
    fn display_and_from_str_round_trip() {
        let value = Value::from_str(r#"{"a":[1,2,3],"b":"x"}"#).unwrap();
        assert_eq!(value.to_string(), r#"{"a":[1,2,3],"b":"x"}"#);
        assert_eq!(Value::from_str(&value.to_string()), Ok(value));
        assert_eq!(
            Value::from_str("["),
            Err(ParseError::new(
                "[",
                1,
                "unexpected end of input, expected a value"
            ))
        );
    }
}
