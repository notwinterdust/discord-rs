//! REST client scaffolding.
//!
//! [`HttpClient`] attaches authentication and standard headers to requests and
//! hands them to a [`Transport`]. The transport performs the actual I/O, which
//! keeps the request-building logic testable without opening sockets.

use except::HttpError;
use version::USER_AGENT;

use crate::http::{HttpMethod, HttpRequest, HttpResponse};

/// Performs HTTP requests for [`HttpClient`].
pub trait Transport {
    /// Sends a request and returns its response.
    ///
    /// # Errors
    ///
    /// Returns an [`HttpError`] when the request cannot be completed.
    fn execute(&self, request: &HttpRequest) -> Result<HttpResponse, HttpError>;
}

/// A Discord REST client.
///
/// The client does not own a connection pool; all I/O is delegated to its
/// transport. Requests are authenticated with the client's token unless they
/// already carry an `Authorization` header.
#[derive(Clone, Debug)]
pub struct HttpClient<T> {
    token: String,
    auth_scheme: String,
    transport: T,
}

impl<T> HttpClient<T> {
    /// Creates a client that authenticates with a bot token.
    #[must_use]
    pub fn new(token: impl Into<String>, transport: T) -> Self {
        Self {
            token: token.into(),
            auth_scheme: String::from("Bot"),
            transport,
        }
    }

    /// Sets the authorization scheme, such as `Bot` or `Bearer`.
    #[must_use]
    pub fn with_auth_scheme(mut self, scheme: impl Into<String>) -> Self {
        self.auth_scheme = scheme.into();
        self
    }

    /// Returns the client's token.
    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Returns the authorization scheme.
    #[must_use]
    pub fn auth_scheme(&self) -> &str {
        &self.auth_scheme
    }

    /// Returns the underlying transport.
    #[must_use]
    pub const fn transport(&self) -> &T {
        &self.transport
    }

    /// Returns a copy of `request` with authentication and standard headers.
    ///
    /// The `Authorization` and `User-Agent` headers are only added when the
    /// request does not already set them. A JSON `Content-Type` is added when
    /// the request has a body and no content type.
    #[must_use]
    pub fn prepare(&self, mut request: HttpRequest) -> HttpRequest {
        if request.header("authorization").is_none() {
            request = request.with_header(
                "Authorization",
                format!("{} {}", self.auth_scheme, self.token),
            );
        }
        if request.header("user-agent").is_none() {
            request = request.with_header("User-Agent", USER_AGENT);
        }
        if request.body.is_some() && request.header("content-type").is_none() {
            request = request.with_header("Content-Type", "application/json");
        }
        request
    }

    /// Creates a `GET` request for the given path.
    #[must_use]
    pub fn get(&self, path: impl Into<String>) -> HttpRequest {
        HttpRequest::new(HttpMethod::Get, path)
    }

    /// Creates a `POST` request for the given path.
    #[must_use]
    pub fn post(&self, path: impl Into<String>) -> HttpRequest {
        HttpRequest::new(HttpMethod::Post, path)
    }

    /// Creates a `PUT` request for the given path.
    #[must_use]
    pub fn put(&self, path: impl Into<String>) -> HttpRequest {
        HttpRequest::new(HttpMethod::Put, path)
    }

    /// Creates a `PATCH` request for the given path.
    #[must_use]
    pub fn patch(&self, path: impl Into<String>) -> HttpRequest {
        HttpRequest::new(HttpMethod::Patch, path)
    }

    /// Creates a `DELETE` request for the given path.
    #[must_use]
    pub fn delete(&self, path: impl Into<String>) -> HttpRequest {
        HttpRequest::new(HttpMethod::Delete, path)
    }
}

impl<T: Transport> HttpClient<T> {
    /// Prepares and sends a request.
    ///
    /// # Errors
    ///
    /// Returns whatever error the transport reports.
    pub fn execute(&self, request: HttpRequest) -> Result<HttpResponse, HttpError> {
        self.transport.execute(&self.prepare(request))
    }

    /// Sends a `GET` request to the given path.
    ///
    /// # Errors
    ///
    /// Returns whatever error the transport reports.
    pub fn execute_get(&self, path: impl Into<String>) -> Result<HttpResponse, HttpError> {
        self.execute(HttpRequest::new(HttpMethod::Get, path))
    }

    /// Sends a `POST` request to the given path.
    ///
    /// # Errors
    ///
    /// Returns whatever error the transport reports.
    pub fn execute_post(&self, path: impl Into<String>) -> Result<HttpResponse, HttpError> {
        self.execute(HttpRequest::new(HttpMethod::Post, path))
    }

    /// Sends a `PUT` request to the given path.
    ///
    /// # Errors
    ///
    /// Returns whatever error the transport reports.
    pub fn execute_put(&self, path: impl Into<String>) -> Result<HttpResponse, HttpError> {
        self.execute(HttpRequest::new(HttpMethod::Put, path))
    }

    /// Sends a `PATCH` request to the given path.
    ///
    /// # Errors
    ///
    /// Returns whatever error the transport reports.
    pub fn execute_patch(&self, path: impl Into<String>) -> Result<HttpResponse, HttpError> {
        self.execute(HttpRequest::new(HttpMethod::Patch, path))
    }

    /// Sends a `DELETE` request to the given path.
    ///
    /// # Errors
    ///
    /// Returns whatever error the transport reports.
    pub fn execute_delete(&self, path: impl Into<String>) -> Result<HttpResponse, HttpError> {
        self.execute(HttpRequest::new(HttpMethod::Delete, path))
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use except::HttpError;
    use version::USER_AGENT;

    use super::{HttpClient, Transport};
    use crate::http::{HttpMethod, HttpRequest, HttpResponse};

    #[derive(Debug, Default)]
    struct MockTransport {
        requests: RefCell<Vec<HttpRequest>>,
        response: HttpResponse,
    }

    impl Transport for MockTransport {
        fn execute(&self, request: &HttpRequest) -> Result<HttpResponse, HttpError> {
            self.requests.borrow_mut().push(request.clone());
            Ok(self.response.clone())
        }
    }

    #[test]
    fn prepare_adds_authentication_and_standard_headers() {
        let client = HttpClient::new("secret", MockTransport::default());
        assert_eq!(client.token(), "secret");
        assert_eq!(client.auth_scheme(), "Bot");

        let request = client.prepare(
            client
                .post("channels/1/messages")
                .with_query("wait", "true")
                .with_text_body("{}"),
        );
        assert_eq!(request.method, HttpMethod::Post);
        assert_eq!(request.header("authorization"), Some("Bot secret"));
        assert_eq!(request.header("user-agent"), Some(USER_AGENT));
        assert_eq!(request.header("content-type"), Some("application/json"));
        assert_eq!(request.path_and_query(), "/channels/1/messages?wait=true");

        let custom = client.prepare(
            client
                .get("users/@me")
                .with_header("Authorization", "Bearer other")
                .with_header("User-Agent", "custom/1.0"),
        );
        assert_eq!(custom.header("authorization"), Some("Bearer other"));
        assert_eq!(custom.header("user-agent"), Some("custom/1.0"));
        assert_eq!(custom.header("content-type"), None);

        let bearer = client.with_auth_scheme("Bearer");
        assert_eq!(bearer.auth_scheme(), "Bearer");
        assert_eq!(
            bearer
                .prepare(bearer.get("oauth2/@me"))
                .header("authorization"),
            Some("Bearer secret")
        );
    }

    #[test]
    fn execute_forwards_prepared_requests() {
        let transport = MockTransport {
            response: HttpResponse::new(200, Vec::new(), Vec::new()),
            ..MockTransport::default()
        };
        let client = HttpClient::new("secret", transport);
        let response = client.execute_get("gateway").unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(client.transport().requests.borrow().len(), 1);
        let sent = &client.transport().requests.borrow()[0];
        assert_eq!(sent.method, HttpMethod::Get);
        assert_eq!(sent.path, "gateway");
        assert_eq!(sent.header("authorization"), Some("Bot secret"));
    }

    #[test]
    fn execute_helpers_use_the_right_methods() {
        let client = HttpClient::new("secret", MockTransport::default());
        client.execute_get("a").unwrap();
        client.execute_post("b").unwrap();
        client.execute_put("c").unwrap();
        client.execute_patch("d").unwrap();
        client.execute_delete("e").unwrap();

        let requests = client.transport().requests.borrow();
        let methods: Vec<_> = requests.iter().map(|request| request.method).collect();
        assert_eq!(
            methods,
            vec![
                HttpMethod::Get,
                HttpMethod::Post,
                HttpMethod::Put,
                HttpMethod::Patch,
                HttpMethod::Delete,
            ]
        );
    }
}
