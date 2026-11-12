//! Discord error types.
//!
//! These types describe failures returned by the REST API, gateway connections,
//! and transport layers.

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

/// A 404 Not Found error representing a missing Discord resource.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotFound {
    pub message: String,
    pub code: Option<i64>,
}

impl NotFound {
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: None,
        }
    }

    #[must_use]
    pub fn with_code(mut self, code: i64) -> Self {
        self.code = Some(code);
        self
    }
}

impl fmt::Display for NotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code {
            Some(code) => write!(f, "Not Found: {} (Discord error code {code})", self.message),
            None => write!(f, "Not Found: {}", self.message),
        }
    }
}

impl Error for NotFound {}

/// A 403 Forbidden error representing lack of permissions or access.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forbidden {
    pub message: String,
    pub code: Option<i64>,
}

impl Forbidden {
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: None,
        }
    }

    #[must_use]
    pub fn with_code(mut self, code: i64) -> Self {
        self.code = Some(code);
        self
    }
}

impl fmt::Display for Forbidden {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code {
            Some(code) => write!(f, "Forbidden: {} (Discord error code {code})", self.message),
            None => write!(f, "Forbidden: {}", self.message),
        }
    }
}

impl Error for Forbidden {}

/// A 401 Unauthorized error representing an invalid or missing authentication token.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unauthorized {
    pub message: String,
}

impl Unauthorized {
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for Unauthorized {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unauthorized: {}", self.message)
    }
}

impl Error for Unauthorized {}

/// A rate-limiting error indicating the caller must wait before retrying.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RateLimitedError {
    pub retry_after: Duration,
    pub global: bool,
}

impl RateLimitedError {
    #[must_use]
    pub const fn new(retry_after: Duration, global: bool) -> Self {
        Self {
            retry_after,
            global,
        }
    }
}

impl fmt::Display for RateLimitedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.global {
            write!(
                f,
                "Globally rate limited for {:.3} seconds",
                self.retry_after.as_secs_f64()
            )
        } else {
            write!(
                f,
                "Rate limited for {:.3} seconds",
                self.retry_after.as_secs_f64()
            )
        }
    }
}

impl Error for RateLimitedError {}

/// An invalid data / unprocessable entity error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidData {
    pub message: String,
}

impl InvalidData {
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for InvalidData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Data: {}", self.message)
    }
}

impl Error for InvalidData {}

/// An error indicating the Discord gateway endpoint could not be found or resolved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GatewayNotFound {
    pub url: String,
}

impl GatewayNotFound {
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

impl fmt::Display for GatewayNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Gateway Not Found: {}", self.url)
    }
}

impl Error for GatewayNotFound {}

/// Granular client exception hierarchy unifying all Discord error scenarios.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClientException {
    NotFound(NotFound),
    Forbidden(Forbidden),
    Unauthorized(Unauthorized),
    RateLimited(RateLimitedError),
    InvalidData(InvalidData),
    GatewayNotFound(GatewayNotFound),
    Http(HttpError),
}

impl fmt::Display for ClientException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(e) => write!(f, "{e}"),
            Self::Forbidden(e) => write!(f, "{e}"),
            Self::Unauthorized(e) => write!(f, "{e}"),
            Self::RateLimited(e) => write!(f, "{e}"),
            Self::InvalidData(e) => write!(f, "{e}"),
            Self::GatewayNotFound(e) => write!(f, "{e}"),
            Self::Http(e) => write!(f, "{e}"),
        }
    }
}

impl Error for ClientException {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFound(e) => Some(e),
            Self::Forbidden(e) => Some(e),
            Self::Unauthorized(e) => Some(e),
            Self::RateLimited(e) => Some(e),
            Self::InvalidData(e) => Some(e),
            Self::GatewayNotFound(e) => Some(e),
            Self::Http(e) => Some(e),
        }
    }
}

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

    /// Converts this HTTP error into a granular [`ClientException`].
    #[must_use]
    pub fn to_client_exception(&self) -> ClientException {
        match self {
            Self::Status { status: 404, error } => ClientException::NotFound(NotFound {
                message: error
                    .as_ref()
                    .map(|e| e.message.clone())
                    .unwrap_or_else(|| "Resource not found".into()),
                code: error.as_ref().map(|e| e.code),
            }),
            Self::Status { status: 403, error } => ClientException::Forbidden(Forbidden {
                message: error
                    .as_ref()
                    .map(|e| e.message.clone())
                    .unwrap_or_else(|| "Missing Permissions".into()),
                code: error.as_ref().map(|e| e.code),
            }),
            Self::Status { status: 401, error } => ClientException::Unauthorized(Unauthorized {
                message: error
                    .as_ref()
                    .map(|e| e.message.clone())
                    .unwrap_or_else(|| "401: Unauthorized".into()),
            }),
            Self::RateLimited {
                retry_after,
                global,
            } => ClientException::RateLimited(RateLimitedError {
                retry_after: *retry_after,
                global: *global,
            }),
            Self::Decode { message } => ClientException::InvalidData(InvalidData {
                message: message.clone(),
            }),
            _ => ClientException::Http(self.clone()),
        }
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

    use super::{
        ClientException, DiscordError, Forbidden, GatewayNotFound, HttpError, InvalidData,
        NotFound, RateLimitedError, Unauthorized,
    };

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

    #[test]
    fn granular_error_hierarchy_and_conversions() {
        let not_found = NotFound::new("Unknown Channel").with_code(10003);
        assert_eq!(
            not_found.to_string(),
            "Not Found: Unknown Channel (Discord error code 10003)"
        );

        let forbidden = Forbidden::new("Missing Permissions").with_code(50013);
        assert_eq!(
            forbidden.to_string(),
            "Forbidden: Missing Permissions (Discord error code 50013)"
        );

        let unauthorized = Unauthorized::new("Invalid token");
        assert_eq!(unauthorized.to_string(), "Unauthorized: Invalid token");

        let rate_limit = RateLimitedError::new(Duration::from_secs(5), false);
        assert_eq!(rate_limit.to_string(), "Rate limited for 5.000 seconds");

        let invalid = InvalidData::new("Malformed JSON");
        assert_eq!(invalid.to_string(), "Invalid Data: Malformed JSON");

        let gw = GatewayNotFound::new("wss://gateway.discord.gg");
        assert_eq!(
            gw.to_string(),
            "Gateway Not Found: wss://gateway.discord.gg"
        );

        // Conversion from HttpError to ClientException
        let http_404 = HttpError::status(404, Some(DiscordError::new(10003, "Channel not found")));
        let ex_404 = http_404.to_client_exception();
        assert!(matches!(ex_404, ClientException::NotFound(_)));
        assert!(ex_404.source().is_some());

        let http_403 = HttpError::status(403, None);
        let ex_403 = http_403.to_client_exception();
        assert!(matches!(ex_403, ClientException::Forbidden(_)));

        let http_429 = HttpError::rate_limited(Duration::from_secs(1), true);
        let ex_429 = http_429.to_client_exception();
        assert!(matches!(ex_429, ClientException::RateLimited(_)));

        let http_decode = HttpError::decode("corrupt");
        let ex_decode = http_decode.to_client_exception();
        assert!(matches!(ex_decode, ClientException::InvalidData(_)));
    }
}
