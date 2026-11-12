//! Shared utilities such as escaping helpers, OAuth URL generation, and worker thread pools.

#![forbid(unsafe_code)]

pub mod pool;
pub use pool::{ThreadPool, ThreadPoolBuilder};

const HEX: &[u8; 16] = b"0123456789ABCDEF";

/// Percent-encodes a string using the URL unreserved character set.
///
/// Letters, digits, `-`, `.`, `_` and `~` are left unchanged. Every other byte
/// is encoded as `%XX`.
#[must_use]
pub fn percent_encode(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    for byte in input.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            output.push(char::from(byte));
        } else {
            output.push('%');
            output.push(char::from(HEX[usize::from(byte >> 4)]));
            output.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
    }
    output
}

/// Decodes a percent-encoded string.
///
/// Returns `None` if a percent escape is malformed or the decoded bytes are not
/// valid UTF-8. A literal `+` is not treated as a space.
#[must_use]
pub fn percent_decode(input: &str) -> Option<String> {
    let bytes = input.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == b'%' {
            let high = hex_value(*bytes.get(index + 1)?)?;
            let low = hex_value(*bytes.get(index + 2)?)?;
            output.push((high << 4) | low);
            index += 3;
        } else {
            output.push(bytes[index]);
            index += 1;
        }
    }

    String::from_utf8(output).ok()
}

/// Builds an `application/x-www-form-urlencoded` query string.
///
/// Pairs are encoded with [`percent_encode`] and joined with `&`.
#[must_use]
pub fn encode_query<I, K, V>(pairs: I) -> String
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    let mut output = String::new();
    for (key, value) in pairs {
        if !output.is_empty() {
            output.push('&');
        }
        output.push_str(&percent_encode(key.as_ref()));
        output.push('=');
        output.push_str(&percent_encode(value.as_ref()));
    }
    output
}

const fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{encode_query, percent_decode, percent_encode};

    #[test]
    fn percent_encoding_round_trips() {
        let cases = [
            ("hello", "hello"),
            ("hello world", "hello%20world"),
            ("a/b?c=d&e", "a%2Fb%3Fc%3Dd%26e"),
            ("café", "caf%C3%A9"),
            ("-._~", "-._~"),
            ("%", "%25"),
        ];
        for (raw, encoded) in cases {
            assert_eq!(percent_encode(raw), encoded);
            assert_eq!(percent_decode(encoded).as_deref(), Some(raw));
        }
    }

    #[test]
    fn malformed_percent_escapes_are_rejected() {
        assert_eq!(percent_decode("%"), None);
        assert_eq!(percent_decode("%2"), None);
        assert_eq!(percent_decode("%GG"), None);
        assert_eq!(percent_decode("%FF"), None);
        assert_eq!(percent_decode("100%"), None);
        assert_eq!(percent_decode("plain"), Some("plain".into()));
    }

    #[test]
    fn query_strings_are_encoded() {
        assert_eq!(encode_query(Vec::<(&str, &str)>::new()), "");
        assert_eq!(
            encode_query([("limit", "50"), ("after", "10 20")]),
            "limit=50&after=10%2020"
        );
        assert_eq!(encode_query([("query", "a&b=c")]), "query=a%26b%3Dc");
    }
}
