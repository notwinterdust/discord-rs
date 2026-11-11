//! Decoding JSON values into API models.
//!
//! Discord sends most objects as JSON from both the REST API and the gateway.
//! The helpers in this module convert [`data::Value`] trees into the crate's
//! typed models and report the path of the field that failed to decode.

use std::error::Error;
use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use abc::{Colour, Snowflake};
use data::{Object, Value};

/// An error produced while decoding a JSON value into a model.
///
/// Every variant records the path of the offending field, such as
/// `member.user.id`, so callers can report where a payload went wrong.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CastError {
    /// A required field was missing.
    MissingField {
        /// The path of the missing field.
        path: String,
    },
    /// A field had the wrong JSON type.
    InvalidType {
        /// The path of the field.
        path: String,
        /// A description of the expected JSON type.
        expected: &'static str,
    },
    /// A field had the right JSON type but an unusable value.
    InvalidValue {
        /// The path of the field.
        path: String,
        /// A description of the expected value.
        expected: &'static str,
    },
}

impl CastError {
    fn missing(path: String) -> Self {
        Self::MissingField { path }
    }

    fn invalid_type(path: &str, expected: &'static str) -> Self {
        Self::InvalidType {
            path: path.to_owned(),
            expected,
        }
    }

    fn invalid_value(path: &str, expected: &'static str) -> Self {
        Self::InvalidValue {
            path: path.to_owned(),
            expected,
        }
    }
}

impl fmt::Display for CastError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingField { path } => write!(f, "missing field `{path}`"),
            Self::InvalidType { path, expected } => {
                write!(f, "field `{path}` must be {expected}")
            }
            Self::InvalidValue { path, expected } => {
                write!(f, "field `{path}` must be {expected}")
            }
        }
    }
}

impl Error for CastError {}

/// Joins a parent path and a field name.
pub(crate) fn field_path(parent: &str, field: &str) -> String {
    if parent.is_empty() {
        field.to_owned()
    } else {
        format!("{parent}.{field}")
    }
}

/// Joins a parent path and an array index.
pub(crate) fn index_path(parent: &str, index: usize) -> String {
    format!("{parent}[{index}]")
}

/// Looks up a required field, returning its value and path.
pub(crate) fn required<'a>(
    object: &'a Object,
    parent: &str,
    field: &str,
) -> Result<(&'a Value, String), CastError> {
    let path = field_path(parent, field);
    match object.get(field) {
        Some(value) => Ok((value, path)),
        None => Err(CastError::missing(path)),
    }
}

/// Looks up an optional field, treating a JSON `null` as absent.
pub(crate) fn optional<'a>(
    object: &'a Object,
    parent: &str,
    field: &str,
) -> Option<(&'a Value, String)> {
    match object.get(field) {
        None | Some(Value::Null) => None,
        Some(value) => Some((value, field_path(parent, field))),
    }
}

/// Reads an object, reporting the path on failure.
pub(crate) fn object<'a>(value: &'a Value, path: &str) -> Result<&'a Object, CastError> {
    value
        .as_object()
        .ok_or_else(|| CastError::invalid_type(path, "an object"))
}

/// Reads an array, reporting the path on failure.
pub(crate) fn array<'a>(value: &'a Value, path: &str) -> Result<&'a [Value], CastError> {
    value
        .as_array()
        .ok_or_else(|| CastError::invalid_type(path, "an array"))
}

/// Reads a string, reporting the path on failure.
pub(crate) fn string<'a>(value: &'a Value, path: &str) -> Result<&'a str, CastError> {
    value
        .as_str()
        .ok_or_else(|| CastError::invalid_type(path, "a string"))
}

/// Reads a boolean, reporting the path on failure.
pub(crate) fn boolean(value: &Value, path: &str) -> Result<bool, CastError> {
    value
        .as_bool()
        .ok_or_else(|| CastError::invalid_type(path, "a boolean"))
}

/// Reads an unsigned integer, reporting the path on failure.
pub(crate) fn u64(value: &Value, path: &str) -> Result<u64, CastError> {
    value
        .as_u64()
        .ok_or_else(|| CastError::invalid_type(path, "a non-negative integer"))
}

/// Reads a signed integer, reporting the path on failure.
pub(crate) fn i64(value: &Value, path: &str) -> Result<i64, CastError> {
    value
        .as_i64()
        .ok_or_else(|| CastError::invalid_type(path, "an integer"))
}

/// Reads a snowflake from either its string or numeric representation.
pub(crate) fn snowflake(value: &Value, path: &str) -> Result<Snowflake, CastError> {
    if let Some(text) = value.as_str() {
        return text
            .parse::<u64>()
            .map(Snowflake::new)
            .map_err(|_| CastError::invalid_value(path, "a snowflake"));
    }
    value
        .as_u64()
        .map(Snowflake::new)
        .ok_or_else(|| CastError::invalid_type(path, "a snowflake"))
}

/// Reads an ISO 8601 timestamp.
pub(crate) fn timestamp(value: &Value, path: &str) -> Result<SystemTime, CastError> {
    let text = value
        .as_str()
        .ok_or_else(|| CastError::invalid_type(path, "an ISO 8601 timestamp"))?;
    parse_timestamp(text)
        .ok_or_else(|| CastError::invalid_value(path, "a valid ISO 8601 timestamp"))
}

/// Reads a permission bitfield from a decimal string or an integer.
pub(crate) fn permission_bits(value: &Value, path: &str) -> Result<u64, CastError> {
    if let Some(text) = value.as_str() {
        return text
            .parse::<u64>()
            .map_err(|_| CastError::invalid_value(path, "a permission bitfield"));
    }
    value
        .as_u64()
        .ok_or_else(|| CastError::invalid_type(path, "a permission bitfield"))
}

/// Reads a 24-bit colour.
pub(crate) fn colour(value: &Value, path: &str) -> Result<Colour, CastError> {
    let raw = u64(value, path)?;
    let raw = u32::try_from(raw)
        .ok()
        .filter(|raw| *raw <= 0xFF_FFFF)
        .ok_or_else(|| CastError::invalid_value(path, "a 24-bit colour"))?;
    Ok(Colour::new(raw))
}

/// Reads a required string field.
pub(crate) fn required_string<'a>(
    object: &'a Object,
    parent: &str,
    field: &str,
) -> Result<&'a str, CastError> {
    let (value, path) = required(object, parent, field)?;
    string(value, &path)
}

/// Reads an optional string field.
pub(crate) fn optional_string<'a>(
    object: &'a Object,
    parent: &str,
    field: &str,
) -> Result<Option<&'a str>, CastError> {
    optional(object, parent, field)
        .map(|(value, path)| string(value, &path))
        .transpose()
}

/// Reads a required object field.
pub(crate) fn required_object<'a>(
    object: &'a Object,
    parent: &str,
    field: &str,
) -> Result<&'a Object, CastError> {
    let (value, path) = required(object, parent, field)?;
    self::object(value, &path)
}

/// Reads an optional object field.
pub(crate) fn optional_object<'a>(
    object: &'a Object,
    parent: &str,
    field: &str,
) -> Result<Option<&'a Object>, CastError> {
    optional(object, parent, field)
        .map(|(value, path)| self::object(value, &path))
        .transpose()
}

/// Reads an optional array field.
pub(crate) fn optional_array<'a>(
    object: &'a Object,
    parent: &str,
    field: &str,
) -> Result<Option<&'a [Value]>, CastError> {
    optional(object, parent, field)
        .map(|(value, path)| array(value, &path))
        .transpose()
}

/// Reads a required snowflake field.
pub(crate) fn required_snowflake(
    object: &Object,
    parent: &str,
    field: &str,
) -> Result<Snowflake, CastError> {
    let (value, path) = required(object, parent, field)?;
    snowflake(value, &path)
}

/// Reads an optional snowflake field.
pub(crate) fn optional_snowflake(
    object: &Object,
    parent: &str,
    field: &str,
) -> Result<Option<Snowflake>, CastError> {
    optional(object, parent, field)
        .map(|(value, path)| snowflake(value, &path))
        .transpose()
}

/// Reads an optional boolean field, using `default` when absent.
pub(crate) fn optional_bool(
    object: &Object,
    parent: &str,
    field: &str,
    default: bool,
) -> Result<bool, CastError> {
    optional(object, parent, field)
        .map(|(value, path)| boolean(value, &path))
        .transpose()
        .map(|value| value.unwrap_or(default))
}

/// Reads an optional unsigned integer field.
pub(crate) fn optional_u64(
    object: &Object,
    parent: &str,
    field: &str,
) -> Result<Option<u64>, CastError> {
    optional(object, parent, field)
        .map(|(value, path)| u64(value, &path))
        .transpose()
}

/// Reads an optional signed integer field.
pub(crate) fn optional_i64(
    object: &Object,
    parent: &str,
    field: &str,
) -> Result<Option<i64>, CastError> {
    optional(object, parent, field)
        .map(|(value, path)| i64(value, &path))
        .transpose()
}

/// Reads an optional colour field.
pub(crate) fn optional_colour(
    object: &Object,
    parent: &str,
    field: &str,
) -> Result<Option<Colour>, CastError> {
    optional(object, parent, field)
        .map(|(value, path)| colour(value, &path))
        .transpose()
}

/// Reads an optional permission bitfield.
pub(crate) fn optional_permission_bits(
    object: &Object,
    parent: &str,
    field: &str,
) -> Result<Option<u64>, CastError> {
    optional(object, parent, field)
        .map(|(value, path)| permission_bits(value, &path))
        .transpose()
}

/// Reads an optional ISO 8601 timestamp.
pub(crate) fn optional_timestamp(
    object: &Object,
    parent: &str,
    field: &str,
) -> Result<Option<SystemTime>, CastError> {
    optional(object, parent, field)
        .map(|(value, path)| timestamp(value, &path))
        .transpose()
}

/// Reads a flag that is represented by the mere presence of its key.
///
/// Discord sends some role tags as `null` when they apply. A boolean value is
/// also accepted so hand-written payloads behave predictably.
pub(crate) fn optional_presence_flag(
    object: &Object,
    parent: &str,
    field: &str,
) -> Result<bool, CastError> {
    match object.get(field) {
        None => Ok(false),
        Some(Value::Null) => Ok(true),
        Some(value) => boolean(value, &field_path(parent, field)),
    }
}

/// Parses an ISO 8601 timestamp into a [`SystemTime`].
///
/// The accepted format is `YYYY-MM-DDTHH:MM:SS` with an optional fractional
/// second and an optional `Z` or `±HH:MM` offset. A missing offset is treated
/// as UTC.
fn parse_timestamp(text: &str) -> Option<SystemTime> {
    if !text.is_ascii() || text.len() < 19 {
        return None;
    }
    let bytes = text.as_bytes();
    if bytes[4] != b'-'
        || bytes[7] != b'-'
        || !matches!(bytes[10], b'T' | b't' | b' ')
        || bytes[13] != b':'
        || bytes[16] != b':'
    {
        return None;
    }

    let year = i64::from(parse_digits(&text[0..4])?);
    let month = parse_digits(&text[5..7])?;
    let day = parse_digits(&text[8..10])?;
    let hour = parse_digits(&text[11..13])?;
    let minute = parse_digits(&text[14..16])?;
    let second = parse_digits(&text[17..19])?;

    if !(1..=12).contains(&month) {
        return None;
    }
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => return None,
    };
    if day == 0 || day > max_day || hour > 23 || minute > 59 || second > 60 {
        return None;
    }
    if !(1..=9999).contains(&year) {
        return None;
    }

    let mut index = 19;
    let mut nanoseconds = 0_u32;
    if index < bytes.len() && bytes[index] == b'.' {
        index += 1;
        let start = index;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
        if index == start {
            return None;
        }
        let digits = &text[start..index];
        let mut value = parse_digits(digits)?;
        if digits.len() > 9 {
            value /= 10_u32.pow(u32::try_from(digits.len() - 9).ok()?);
        } else {
            value *= 10_u32.pow(u32::try_from(9 - digits.len()).ok()?);
        }
        nanoseconds = value;
    }

    let offset = if index == bytes.len() {
        0
    } else if matches!(bytes[index], b'Z' | b'z') {
        if index + 1 != bytes.len() {
            return None;
        }
        0
    } else if matches!(bytes[index], b'+' | b'-') {
        let sign = if bytes[index] == b'-' { -1 } else { 1 };
        index += 1;
        let remainder = &text[index..];
        let (hours, minutes) = match remainder.len() {
            5 if remainder.as_bytes()[2] == b':' => (&remainder[0..2], &remainder[3..5]),
            4 => (&remainder[0..2], &remainder[2..4]),
            _ => return None,
        };
        let hours = i64::from(parse_digits(hours)?);
        let minutes = i64::from(parse_digits(minutes)?);
        if hours > 23 || minutes > 59 {
            return None;
        }
        sign * (hours * 3600 + minutes * 60)
    } else {
        return None;
    };

    let days = days_from_civil(year, month, day);
    let seconds =
        days * 86_400 + i64::from(hour) * 3600 + i64::from(minute) * 60 + i64::from(second)
            - offset;
    let total_nanos = i128::from(seconds) * 1_000_000_000 + i128::from(nanoseconds);
    let magnitude = total_nanos.unsigned_abs();
    let duration = Duration::new(
        u64::try_from(magnitude / 1_000_000_000).ok()?,
        u32::try_from(magnitude % 1_000_000_000).ok()?,
    );
    if total_nanos >= 0 {
        UNIX_EPOCH.checked_add(duration)
    } else {
        UNIX_EPOCH.checked_sub(duration)
    }
}

fn parse_digits(text: &str) -> Option<u32> {
    if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    text.parse().ok()
}

const fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let month_prime = i64::from((month + 9) % 12);
    let day_of_year = (153 * month_prime + 2) / 5 + i64::from(day) - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, UNIX_EPOCH};

    use abc::Snowflake;
    use data::{Object, Value, json};

    use super::{
        CastError, field_path, index_path, optional_permission_bits, permission_bits, snowflake,
        timestamp,
    };

    #[test]
    fn timestamps_accept_offsets_and_fractions() {
        let expected = UNIX_EPOCH + Duration::new(1_627_821_296, 789_000_000);
        assert_eq!(
            timestamp(
                &Value::from("2021-08-01T12:34:56.789000+00:00"),
                "timestamp"
            )
            .unwrap(),
            expected
        );
        assert_eq!(
            timestamp(&Value::from("2021-08-01T12:34:56.789Z"), "timestamp").unwrap(),
            expected
        );
        assert_eq!(
            timestamp(&Value::from("2021-08-01T14:34:56.789+02:00"), "timestamp").unwrap(),
            expected
        );
        assert_eq!(
            timestamp(&Value::from("2021-08-01T12:34:56.789"), "timestamp").unwrap(),
            expected
        );
        assert_eq!(
            timestamp(&Value::from("2021-08-01T12:34:56.5Z"), "timestamp").unwrap(),
            UNIX_EPOCH + Duration::new(1_627_821_296, 500_000_000)
        );
    }

    #[test]
    fn timestamps_handle_epochs_leap_days_and_errors() {
        assert_eq!(
            timestamp(&Value::from("1970-01-01T00:00:00Z"), "timestamp").unwrap(),
            UNIX_EPOCH
        );
        assert_eq!(
            timestamp(&Value::from("1969-12-31T23:59:59Z"), "timestamp").unwrap(),
            UNIX_EPOCH - Duration::from_secs(1)
        );
        assert!(timestamp(&Value::from("2020-02-29T00:00:00Z"), "timestamp").is_ok());
        assert!(timestamp(&Value::from("2021-02-29T00:00:00Z"), "timestamp").is_err());
        assert!(timestamp(&Value::from("2021-13-01T00:00:00Z"), "timestamp").is_err());
        assert!(timestamp(&Value::from("2021-00-01T00:00:00Z"), "timestamp").is_err());
        assert!(timestamp(&Value::from("2021-01-32T00:00:00Z"), "timestamp").is_err());
        assert!(timestamp(&Value::from("2021-01-01T24:00:00Z"), "timestamp").is_err());
        assert!(timestamp(&Value::from("2021-01-01T00:00:00"), "timestamp").is_ok());
        assert!(timestamp(&Value::from("not a timestamp"), "timestamp").is_err());
        assert!(timestamp(&Value::from(1_u64), "timestamp").is_err());
        assert!(timestamp(&Value::from("2021-01-01T00:00:00+25:00"), "timestamp").is_err());
        assert!(timestamp(&Value::from("2021-01-01T00:00:00.Z"), "timestamp").is_err());
    }

    #[test]
    fn snowflakes_accept_strings_and_numbers() {
        assert_eq!(
            snowflake(&Value::from("123"), "id").unwrap(),
            Snowflake::new(123)
        );
        assert_eq!(
            snowflake(&Value::from(123_u64), "id").unwrap(),
            Snowflake::new(123)
        );
        assert!(matches!(
            snowflake(&Value::from("abc"), "id"),
            Err(CastError::InvalidValue { .. })
        ));
        assert!(matches!(
            snowflake(&Value::from(true), "id"),
            Err(CastError::InvalidType { .. })
        ));
    }

    #[test]
    fn permission_bits_accept_strings_and_numbers() {
        assert_eq!(
            permission_bits(&Value::from("8"), "permissions").unwrap(),
            8
        );
        assert_eq!(
            permission_bits(&Value::from(8_u64), "permissions").unwrap(),
            8
        );

        let mut object = Object::new();
        object.insert("permissions", Value::from("16"));
        assert_eq!(
            optional_permission_bits(&object, "role", "permissions").unwrap(),
            Some(16)
        );
        assert_eq!(
            optional_permission_bits(&Object::new(), "role", "permissions").unwrap(),
            None
        );
        assert!(permission_bits(&Value::from("-1"), "permissions").is_err());
    }

    #[test]
    fn paths_and_error_messages_are_reported() {
        assert_eq!(field_path("member", "user"), "member.user");
        assert_eq!(field_path("", "user"), "user");
        assert_eq!(index_path("member.roles", 2), "member.roles[2]");

        let error = CastError::MissingField {
            path: "member.user.id".to_owned(),
        };
        assert_eq!(error.to_string(), "missing field `member.user.id`");

        let error = CastError::InvalidType {
            path: "id".to_owned(),
            expected: "a string",
        };
        assert_eq!(error.to_string(), "field `id` must be a string");

        let error = CastError::InvalidValue {
            path: "id".to_owned(),
            expected: "a snowflake",
        };
        assert_eq!(error.to_string(), "field `id` must be a snowflake");
    }

    #[test]
    fn json_objects_can_be_decoded() {
        let value = json::from_str(r#"{"id":"1","name":"winter"}"#).unwrap();
        assert_eq!(
            snowflake(value.get("id").unwrap(), "id").unwrap(),
            Snowflake::new(1)
        );
    }
}
