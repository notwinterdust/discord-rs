//! HTTP request, response and rate-limit primitives.
//!
//! This module models the wire-level pieces of Discord's REST API. It does not
//! perform I/O; a transport implementation can build [`HttpRequest`] values and
//! return [`HttpResponse`] values.

use std::fmt;
use std::time::Duration;

use except::HttpError;
use util::encode_query;

/// An HTTP method used by the Discord REST API.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HttpMethod {
    /// Retrieve a resource.
    #[default]
    Get,
    /// Create a resource.
    Post,
    /// Replace a resource.
    Put,
    /// Partially update a resource.
    Patch,
    /// Delete a resource.
    Delete,
}

impl HttpMethod {
    /// Returns the uppercase method name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
        }
    }

    /// Returns `true` if the method usually carries a request body.
    #[must_use]
    pub const fn has_body(self) -> bool {
        matches!(self, Self::Post | Self::Put | Self::Patch)
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// An HTTP request to Discord's REST API.
///
/// Paths are relative to [`version::API_BASE`]; the query is stored separately
/// so callers can append parameters without rebuilding the path.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HttpRequest {
    /// The request method.
    pub method: HttpMethod,
    /// The path below the API version, such as `/guilds/123`.
    pub path: String,
    /// The query parameters, in insertion order.
    pub query: Vec<(String, String)>,
    /// The request headers, in insertion order.
    pub headers: Vec<(String, String)>,
    /// The request body, if any.
    pub body: Option<Vec<u8>>,
}

impl HttpRequest {
    /// Creates a request with an empty query, headers and body.
    #[must_use]
    pub fn new(method: HttpMethod, path: impl Into<String>) -> Self {
        Self {
            method,
            path: path.into(),
            query: Vec::new(),
            headers: Vec::new(),
            body: None,
        }
    }

    /// Adds a query parameter.
    #[must_use]
    pub fn with_query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.push((key.into(), value.into()));
        self
    }

    /// Adds several query parameters.
    #[must_use]
    pub fn with_queries<I, K, V>(mut self, query: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.query.extend(
            query
                .into_iter()
                .map(|(key, value)| (key.into(), value.into())),
        );
        self
    }

    /// Adds a header.
    ///
    /// Existing headers with the same case-insensitive name are replaced.
    #[must_use]
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        let name = name.into();
        let value = value.into();
        self.headers
            .retain(|(existing, _)| !existing.eq_ignore_ascii_case(&name));
        self.headers.push((name, value));
        self
    }

    /// Sets the request body.
    #[must_use]
    pub fn with_body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Sets a UTF-8 text body.
    #[must_use]
    pub fn with_text_body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into().into_bytes());
        self
    }

    /// Returns the first header with the given case-insensitive name.
    #[must_use]
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(existing, _)| existing.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }

    /// Returns the body length, or zero when there is no body.
    #[must_use]
    pub fn body_len(&self) -> usize {
        self.body.as_ref().map_or(0, Vec::len)
    }

    /// Returns the path with its encoded query string.
    ///
    /// A leading slash is added when the path does not start with one.
    #[must_use]
    pub fn path_and_query(&self) -> String {
        let mut path = if self.path.starts_with('/') {
            self.path.clone()
        } else {
            format!("/{}", self.path)
        };
        if !self.query.is_empty() {
            path.push('?');
            path.push_str(&encode_query(
                self.query
                    .iter()
                    .map(|(key, value)| (key.as_str(), value.as_str())),
            ));
        }
        path
    }

    /// Returns the request line, such as `GET /guilds/1 HTTP/1.1`.
    #[must_use]
    pub fn request_line(&self) -> String {
        format!("{} {} HTTP/1.1", self.method, self.path_and_query())
    }

    /// Returns the absolute API URL for this request.
    #[must_use]
    pub fn api_url(&self) -> String {
        version::api_url(&self.path_and_query())
    }
}

/// An HTTP response from Discord's REST API.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HttpResponse {
    /// The HTTP status code.
    pub status: u16,
    /// The response headers, in the order they were received.
    pub headers: Vec<(String, String)>,
    /// The response body.
    pub body: Vec<u8>,
}

impl HttpResponse {
    /// Creates a response from its status, headers and body.
    #[must_use]
    pub fn new(status: u16, headers: Vec<(String, String)>, body: Vec<u8>) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    /// Returns the first header with the given case-insensitive name.
    #[must_use]
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(existing, _)| existing.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }

    /// Returns `true` if the status is in the `2xx` range.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }

    /// Returns `true` if the response is a rate limit response.
    #[must_use]
    pub const fn is_rate_limited(&self) -> bool {
        self.status == 429
    }

    /// Returns the body as UTF-8 text.
    ///
    /// # Errors
    ///
    /// Returns [`HttpError::Decode`] when the body is not valid UTF-8.
    pub fn text(&self) -> Result<&str, HttpError> {
        std::str::from_utf8(&self.body).map_err(|error| HttpError::decode(error.to_string()))
    }

    /// Parses the rate limit headers in this response, if any are present.
    #[must_use]
    pub fn rate_limit(&self) -> Option<RateLimit> {
        RateLimit::from_headers(
            self.headers
                .iter()
                .map(|(name, value)| (name.as_str(), value.as_str())),
        )
    }

    /// Parses the `Retry-After` header, if present.
    #[must_use]
    pub fn retry_after(&self) -> Option<Duration> {
        self.header("retry-after").and_then(parse_seconds)
    }
}

/// The scope of a rate limit bucket.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RateLimitScope {
    /// The bucket applies to the current user.
    User,
    /// The bucket applies to the whole API.
    Global,
    /// The bucket is shared between users.
    Shared,
    /// A scope this library does not recognise.
    Unknown(String),
}

impl RateLimitScope {
    /// Converts an API scope name, preserving unrecognised values.
    #[must_use]
    pub fn from_name(name: &str) -> Self {
        match name {
            "user" => Self::User,
            "global" => Self::Global,
            "shared" => Self::Shared,
            other => Self::Unknown(other.to_owned()),
        }
    }

    /// Returns the scope name used by the API.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::User => "user",
            Self::Global => "global",
            Self::Shared => "shared",
            Self::Unknown(name) => name,
        }
    }
}

impl fmt::Display for RateLimitScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Rate limit information attached to a response.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RateLimit {
    /// The number of requests allowed in the bucket.
    pub limit: u32,
    /// The number of requests remaining in the bucket.
    pub remaining: u32,
    /// How long until the bucket resets.
    pub reset_after: Duration,
    /// The opaque bucket identifier, if provided.
    pub bucket: Option<String>,
    /// Whether the rate limit is global.
    pub global: bool,
    /// The bucket's scope, if provided.
    pub scope: Option<RateLimitScope>,
}

impl RateLimit {
    /// Parses rate limit information from response headers.
    ///
    /// Returns `None` when none of the `X-RateLimit-*` headers are present.
    #[must_use]
    pub fn from_headers<I, K, V>(headers: I) -> Option<Self>
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut rate_limit = Self::default();
        let mut found = false;

        for (name, value) in headers {
            let name = name.as_ref();
            let value = value.as_ref();
            if name.eq_ignore_ascii_case("x-ratelimit-limit") {
                if let Ok(limit) = value.trim().parse() {
                    rate_limit.limit = limit;
                    found = true;
                }
            } else if name.eq_ignore_ascii_case("x-ratelimit-remaining") {
                if let Ok(remaining) = value.trim().parse() {
                    rate_limit.remaining = remaining;
                    found = true;
                }
            } else if name.eq_ignore_ascii_case("x-ratelimit-reset-after") {
                if let Some(reset_after) = parse_seconds(value) {
                    rate_limit.reset_after = reset_after;
                    found = true;
                }
            } else if name.eq_ignore_ascii_case("x-ratelimit-bucket") {
                rate_limit.bucket = Some(value.trim().to_owned());
                found = true;
            } else if name.eq_ignore_ascii_case("x-ratelimit-global") {
                rate_limit.global = value.trim().eq_ignore_ascii_case("true");
                found = true;
            } else if name.eq_ignore_ascii_case("x-ratelimit-scope") {
                rate_limit.scope = Some(RateLimitScope::from_name(value.trim()));
                found = true;
            }
        }

        found.then_some(rate_limit)
    }

    /// Returns `true` if no requests remain in the bucket.
    #[must_use]
    pub const fn is_exhausted(&self) -> bool {
        self.remaining == 0
    }

    /// Returns `true` if this limit applies to the whole API.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        self.global
    }
}

fn parse_seconds(value: &str) -> Option<Duration> {
    let seconds = value.trim().parse::<f64>().ok()?;
    if !seconds.is_finite() || seconds < 0.0 {
        return None;
    }
    Duration::try_from_secs_f64(seconds).ok()
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{HttpMethod, HttpRequest, HttpResponse, RateLimit, RateLimitScope};

    fn headers(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        pairs
            .iter()
            .map(|(name, value)| ((*name).to_owned(), (*value).to_owned()))
            .collect()
    }

    #[test]
    fn methods_display_and_report_bodies() {
        assert_eq!(HttpMethod::Get.to_string(), "GET");
        assert_eq!(HttpMethod::Post.as_str(), "POST");
        assert!(!HttpMethod::Get.has_body());
        assert!(HttpMethod::Post.has_body());
        assert!(HttpMethod::Put.has_body());
        assert!(HttpMethod::Patch.has_body());
        assert!(!HttpMethod::Delete.has_body());
    }

    #[test]
    fn requests_build_paths_queries_and_headers() {
        let request = HttpRequest::new(HttpMethod::Get, "channels/1/messages")
            .with_query("limit", "50")
            .with_queries([("after", "10 20")])
            .with_header("Authorization", "Bot token")
            .with_header("authorization", "Bot replacement")
            .with_text_body("{}");

        assert_eq!(request.method, HttpMethod::Get);
        assert_eq!(request.path, "channels/1/messages");
        assert_eq!(request.query.len(), 2);
        assert_eq!(request.header("AUTHORIZATION"), Some("Bot replacement"));
        assert_eq!(request.headers.len(), 1);
        assert_eq!(request.body_len(), 2);
        assert_eq!(request.body.as_deref(), Some(b"{}".as_slice()));
        assert_eq!(
            request.path_and_query(),
            "/channels/1/messages?limit=50&after=10%2020"
        );
        assert_eq!(
            request.request_line(),
            "GET /channels/1/messages?limit=50&after=10%2020 HTTP/1.1"
        );
        assert_eq!(
            request.api_url(),
            "https://discord.com/api/v10/channels/1/messages?limit=50&after=10%2020"
        );
    }

    #[test]
    fn requests_without_queries_or_leading_slash() {
        let request = HttpRequest::new(HttpMethod::Delete, "/guilds/1");
        assert_eq!(request.path_and_query(), "/guilds/1");
        assert_eq!(request.request_line(), "DELETE /guilds/1 HTTP/1.1");
        assert_eq!(request.body_len(), 0);
        assert_eq!(request.header("missing"), None);

        assert_eq!(HttpRequest::default().path_and_query(), "/");
        assert_eq!(HttpRequest::default().request_line(), "GET / HTTP/1.1");
    }

    #[test]
    fn responses_expose_status_headers_and_text() {
        let response = HttpResponse::new(
            204,
            headers(&[("Content-Type", "application/json")]),
            Vec::new(),
        );
        assert!(response.is_success());
        assert!(!response.is_rate_limited());
        assert_eq!(response.header("content-type"), Some("application/json"));
        assert_eq!(response.text(), Ok(""));

        let invalid = HttpResponse::new(200, Vec::new(), vec![0xff]);
        assert!(invalid.text().is_err());

        let limited = HttpResponse::new(429, Vec::new(), Vec::new());
        assert!(limited.is_rate_limited());
    }

    #[test]
    fn rate_limits_parse_headers() {
        let parsed = RateLimit::from_headers(headers(&[
            ("X-RateLimit-Limit", "10"),
            ("X-RateLimit-Remaining", "0"),
            ("X-RateLimit-Reset-After", "1.5"),
            ("X-RateLimit-Bucket", "abc"),
            ("X-RateLimit-Global", "true"),
            ("X-RateLimit-Scope", "shared"),
        ]))
        .unwrap();

        assert_eq!(parsed.limit, 10);
        assert_eq!(parsed.remaining, 0);
        assert_eq!(parsed.reset_after, Duration::from_millis(1_500));
        assert_eq!(parsed.bucket.as_deref(), Some("abc"));
        assert!(parsed.is_global());
        assert!(parsed.is_exhausted());
        assert_eq!(parsed.scope, Some(RateLimitScope::Shared));

        assert!(RateLimit::from_headers(Vec::<(&str, &str)>::new()).is_none());
        assert!(RateLimit::from_headers([("X-RateLimit-Limit", "not-a-number")]).is_none());
        assert!(RateLimit::from_headers([("X-RateLimit-Reset-After", "-1")]).is_none());
    }

    #[test]
    fn rate_limit_scopes_round_trip() {
        assert_eq!(RateLimitScope::from_name("user"), RateLimitScope::User);
        assert_eq!(RateLimitScope::from_name("global"), RateLimitScope::Global);
        assert_eq!(RateLimitScope::from_name("shared"), RateLimitScope::Shared);
        assert_eq!(
            RateLimitScope::from_name("future"),
            RateLimitScope::Unknown("future".into())
        );
        assert_eq!(
            RateLimitScope::Unknown("future".into()).to_string(),
            "future"
        );
        assert_eq!(RateLimitScope::User.to_string(), "user");
    }

    #[test]
    fn responses_parse_rate_limits_and_retry_after() {
        let response = HttpResponse::new(
            429,
            headers(&[
                ("X-RateLimit-Limit", "5"),
                ("X-RateLimit-Remaining", "0"),
                ("X-RateLimit-Reset-After", "0.25"),
                ("Retry-After", "2.5"),
            ]),
            Vec::new(),
        );

        let rate_limit = response.rate_limit().unwrap();
        assert_eq!(rate_limit.limit, 5);
        assert_eq!(rate_limit.reset_after, Duration::from_millis(250));
        assert_eq!(response.retry_after(), Some(Duration::from_millis(2_500)));

        let no_headers = HttpResponse::default();
        assert!(no_headers.rate_limit().is_none());
        assert_eq!(no_headers.retry_after(), None);
    }
}
