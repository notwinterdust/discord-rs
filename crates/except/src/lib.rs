//! Discord error types.
//!
//! These types describe failures returned by the REST API and by the
//! transport layer that performs requests.

#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt;
use std::time::Duration;

/// A structured error returned by Discord's REST API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscordError {
    /// Discord's numeric error code.
    pub code: i64,
    /// A human-readable error message.
    pub message: String,
}

impl DiscordError {
    /// Creates a Discord API error.
    #[must_use]
    pub fn new(code: i64, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for DiscordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (Discord error code {})", self.message, self.code)
    }
}

impl Error for DiscordError {}

/// An error raised while performing an HTTP request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HttpError {
    /// The API returned a non-success status code.
    Status {
        /// The HTTP status code.
        status: u16,
        /// Discord's error payload, when one was returned.
        error: Option<DiscordError>,
    },
    /// The request was rate limited.
    RateLimited {
        /// How long the caller should wait before retrying.
        retry_after: Duration,
        /// Whether the rate limit applies to the whole API or only this route.
        global: bool,
    },
    /// The request could not be sent or its response could not be read.
    Transport {
        /// A description of the transport failure.
        message: String,
    },
    /// A response could not be decoded.
    Decode {
        /// A description of the decoding failure.
        message: String,
    },
}

impl HttpError {
    /// Creates a status error, with an optional Discord error payload.
    #[must_use]
    pub const fn status(status: u16, error: Option<DiscordError>) -> Self {
        Self::Status { status, error }
    }

    /// Creates a rate limit error.
    #[must_use]
    pub const fn rate_limited(retry_after: Duration, global: bool) -> Self {
        Self::RateLimited {
            retry_after,
            global,
        }
    }

    /// Creates a transport error.
    #[must_use]
    pub fn transport(message: impl Into<String>) -> Self {
        Self::Transport {
            message: message.into(),
        }
    }

    /// Creates a decoding error.
    #[must_use]
    pub fn decode(message: impl Into<String>) -> Self {
        Self::Decode {
            message: message.into(),
        }
    }

    /// Returns the HTTP status code, if the error came from a response.
    #[must_use]
    pub const fn status_code(&self) -> Option<u16> {
        match self {
            Self::Status { status, .. } => Some(*status),
            Self::RateLimited { .. } => Some(429),
            Self::Transport { .. } | Self::Decode { .. } => None,
        }
    }

    /// Returns the Discord error payload, if the API provided one.
    #[must_use]
    pub const fn discord_error(&self) -> Option<&DiscordError> {
        match self {
            Self::Status { error, .. } => error.as_ref(),
            _ => None,
        }
    }

    /// Returns `true` if the request was rejected because of missing
    /// permissions or access.
    #[must_use]
    pub const fn is_forbidden(&self) -> bool {
        matches!(self.status_code(), Some(403))
    }

    /// Returns `true` if the requested resource does not exist.
    #[must_use]
    pub const fn is_not_found(&self) -> bool {
        matches!(self.status_code(), Some(404))
    }

    /// Returns `true` if the request was rate limited.
    #[must_use]
    pub const fn is_rate_limited(&self) -> bool {
        matches!(self, Self::RateLimited { .. })
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Status { status, error } => match error {
                Some(error) => write!(f, "HTTP status {status}: {error}"),
                None => write!(f, "HTTP status {status}"),
            },
            Self::RateLimited {
                retry_after,
                global,
            } => {
                if *global {
                    write!(
                        f,
                        "globally rate limited for {:.3} seconds",
                        retry_after.as_secs_f64()
                    )
                } else {
                    write!(
                        f,
                        "rate limited for {:.3} seconds",
                        retry_after.as_secs_f64()
                    )
                }
            }
            Self::Transport { message } => write!(f, "transport error: {message}"),
            Self::Decode { message } => write!(f, "decode error: {message}"),
        }
    }
}

impl Error for HttpError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.discord_error()
            .map(|error| error as &(dyn Error + 'static))
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::time::Duration;

    use super::{DiscordError, HttpError};

    #[test]
    fn discord_errors_display_code_and_message() {
        let error = DiscordError::new(50035, "Invalid Form Body");
        assert_eq!(error.code, 50_035);
        assert_eq!(error.message, "Invalid Form Body");
        assert_eq!(
            error.to_string(),
            "Invalid Form Body (Discord error code 50035)"
        );
    }

    #[test]
    fn status_errors_expose_helpers() {
        let error = HttpError::status(403, Some(DiscordError::new(50001, "Missing Access")));
        assert_eq!(error.status_code(), Some(403));
        assert!(error.is_forbidden());
        assert!(!error.is_not_found());
        assert!(!error.is_rate_limited());
        assert_eq!(error.discord_error().map(|error| error.code), Some(50_001));
        assert_eq!(
            error.to_string(),
            "HTTP status 403: Missing Access (Discord error code 50001)"
        );
        assert!(error.source().is_some());

        let bare = HttpError::status(404, None);
        assert!(bare.is_not_found());
        assert_eq!(bare.to_string(), "HTTP status 404");
        assert!(bare.source().is_none());
    }

    #[test]
    fn rate_limit_and_transport_errors_display() {
        let global = HttpError::rate_limited(Duration::from_millis(1_500), true);
        assert_eq!(global.status_code(), Some(429));
        assert!(global.is_rate_limited());
        assert_eq!(
            global.to_string(),
            "globally rate limited for 1.500 seconds"
        );

        let local = HttpError::rate_limited(Duration::from_millis(250), false);
        assert_eq!(local.to_string(), "rate limited for 0.250 seconds");

        let transport = HttpError::transport("connection reset");
        assert_eq!(transport.status_code(), None);
        assert_eq!(transport.to_string(), "transport error: connection reset");

        let decode = HttpError::decode("unexpected end of input");
        assert_eq!(decode.status_code(), None);
        assert_eq!(decode.to_string(), "decode error: unexpected end of input");
    }
}
