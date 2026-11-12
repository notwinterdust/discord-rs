//! REST client scaffolding.
//!
//! [`HttpClient`] attaches authentication and standard headers to requests and
//! hands them to a [`Transport`]. The transport performs the actual I/O, which
//! keeps the request-building logic testable without opening sockets.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use data::Value;
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

/// Application-provided secure transport boundary.
pub trait SecureTransport: Transport {
    /// Returns the negotiated peer name.
    fn peer_name(&self) -> &str;
}

/// Retry policy for transient REST failures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetryPolicy {
    /// Maximum number of retries after the initial request.
    pub retries: u32,
    /// Initial delay between attempts.
    pub backoff: Duration,
}

impl RetryPolicy {
    /// Creates a retry policy.
    #[must_use]
    pub const fn new(retries: u32, backoff: Duration) -> Self {
        Self { retries, backoff }
    }

    /// Returns the delay for a zero-based retry attempt.
    #[must_use]
    pub fn delay(&self, attempt: u32) -> Duration {
        self.backoff
            .saturating_mul(1_u32.checked_shl(attempt).unwrap_or(u32::MAX))
    }
}

/// A synchronous HTTP/1.1 transport backed only by the Rust standard library.
///
/// This transport handles plain HTTP. Applications that need HTTPS can provide
/// a TLS-enabled implementation of [`Transport`] without changing [`HttpClient`].
#[derive(Clone, Debug)]
pub struct TcpTransport {
    host: String,
    port: u16,
    timeout: Duration,
}

impl TcpTransport {
    /// Creates a transport for an HTTP host.
    #[must_use]
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
            timeout: Duration::from_secs(30),
        }
    }

    /// Sets the connect and read/write timeout.
    #[must_use]
    pub const fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Returns the configured host.
    #[must_use]
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Returns the configured port.
    #[must_use]
    pub const fn port(&self) -> u16 {
        self.port
    }

    fn connect(&self) -> Result<TcpStream, HttpError> {
        let address = (self.host.as_str(), self.port)
            .to_socket_addrs()
            .map_err(|error| HttpError::transport(error.to_string()))?
            .next()
            .ok_or_else(|| HttpError::transport("host did not resolve"))?;
        let stream = TcpStream::connect_timeout(&address, self.timeout)
            .map_err(|error| HttpError::transport(error.to_string()))?;
        stream
            .set_read_timeout(Some(self.timeout))
            .and_then(|_| stream.set_write_timeout(Some(self.timeout)))
            .map_err(|error| HttpError::transport(error.to_string()))?;
        Ok(stream)
    }
}

impl Transport for TcpTransport {
    fn execute(&self, request: &HttpRequest) -> Result<HttpResponse, HttpError> {
        let mut stream = self.connect()?;
        let body = request.body.as_deref().unwrap_or_default();
        let mut request_bytes = format!(
            "{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n",
            request.method,
            request.path_and_query(),
            self.host
        )
        .into_bytes();
        for (name, value) in &request.headers {
            request_bytes.extend_from_slice(format!("{name}: {value}\r\n").as_bytes());
        }
        if !body.is_empty() && request.header("content-length").is_none() {
            request_bytes
                .extend_from_slice(format!("Content-Length: {}\r\n", body.len()).as_bytes());
        }
        request_bytes.extend_from_slice(b"\r\n");
        request_bytes.extend_from_slice(body);
        stream
            .write_all(&request_bytes)
            .map_err(|error| HttpError::transport(error.to_string()))?;

        let mut reader = BufReader::new(stream);
        let mut status_line = String::new();
        reader
            .read_line(&mut status_line)
            .map_err(|error| HttpError::transport(error.to_string()))?;
        let mut status_parts = status_line.split_whitespace();
        let _version = status_parts
            .next()
            .ok_or_else(|| HttpError::decode("missing HTTP version"))?;
        let status = status_parts
            .next()
            .ok_or_else(|| HttpError::decode("missing HTTP status"))?
            .parse::<u16>()
            .map_err(|error| HttpError::decode(error.to_string()))?;

        let mut headers = Vec::new();
        let mut line = String::new();
        loop {
            line.clear();
            reader
                .read_line(&mut line)
                .map_err(|error| HttpError::transport(error.to_string()))?;
            if line == "\r\n" || line == "\n" {
                break;
            }
            let (name, value) = line
                .split_once(':')
                .ok_or_else(|| HttpError::decode("malformed HTTP header"))?;
            headers.push((name.trim().to_owned(), value.trim().to_owned()));
        }

        let mut response_body = Vec::new();
        if let Some(length) = headers
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
            .and_then(|(_, value)| value.parse::<usize>().ok())
        {
            let mut limited = reader.take(length as u64);
            limited
                .read_to_end(&mut response_body)
                .map_err(|error| HttpError::transport(error.to_string()))?;
            if response_body.len() != length {
                return Err(HttpError::decode("HTTP body shorter than Content-Length"));
            }
        } else {
            reader
                .read_to_end(&mut response_body)
                .map_err(|error| HttpError::transport(error.to_string()))?;
        }

        Ok(HttpResponse::new(status, headers, response_body))
    }
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

impl<T: Transport> HttpClient<T> {
    /// Executes a request with a JSON body encoded by the workspace parser.
    pub fn execute_json(
        &self,
        mut request: HttpRequest,
        value: &Value,
    ) -> Result<HttpResponse, HttpError> {
        request = request
            .with_header("Content-Type", "application/json")
            .with_text_body(data::to_string(value));
        self.execute(request)
    }
    /// Executes a request with bounded retries for transport and rate-limit errors.
    pub fn execute_with_retry(
        &self,
        request: HttpRequest,
        policy: RetryPolicy,
    ) -> Result<HttpResponse, HttpError> {
        let mut attempt = 0;
        loop {
            match self.execute(request.clone()) {
                Ok(response) if !response.is_rate_limited() => return Ok(response),
                Ok(response) if attempt >= policy.retries => return Ok(response),
                Ok(response) => {
                    if let Some(delay) = response.retry_after() {
                        std::thread::sleep(delay);
                    } else {
                        std::thread::sleep(policy.delay(attempt));
                    }
                }
                Err(error) if attempt < policy.retries && error.is_rate_limited() => {
                    std::thread::sleep(policy.delay(attempt));
                }
                Err(error) => return Err(error),
            }
            attempt += 1;
        }
    }
}

/// Lifecycle state for a Discord client.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ClientState {
    /// No gateway session is active.
    #[default]
    Disconnected,
    /// The client is establishing a gateway session.
    Connecting,
    /// The client has received READY and can serve events.
    Ready,
    /// The client is shutting down.
    Closing,
    /// The client has shut down.
    Closed,
}

/// A REST client with explicit lifecycle state.
#[derive(Clone, Debug)]
pub struct Client<T> {
    http: HttpClient<T>,
    state: ClientState,
}

impl<T> Client<T> {
    /// Creates a disconnected client.
    #[must_use]
    pub fn new(token: impl Into<String>, transport: T) -> Self {
        Self {
            http: HttpClient::new(token, transport),
            state: ClientState::Disconnected,
        }
    }

    /// Returns the REST client.
    #[must_use]
    pub const fn http(&self) -> &HttpClient<T> {
        &self.http
    }

    /// Returns the REST client mutably.
    pub fn http_mut(&mut self) -> &mut HttpClient<T> {
        &mut self.http
    }

    /// Returns the current lifecycle state.
    #[must_use]
    pub const fn state(&self) -> ClientState {
        self.state
    }

    /// Marks the client as connecting.
    pub fn begin_connect(&mut self) {
        self.state = ClientState::Connecting;
    }

    /// Marks the client as ready.
    pub fn mark_ready(&mut self) {
        self.state = ClientState::Ready;
    }

    /// Marks the client as closing.
    pub fn begin_close(&mut self) {
        self.state = ClientState::Closing;
    }

    /// Marks the client as closed.
    pub fn mark_closed(&mut self) {
        self.state = ClientState::Closed;
    }

    /// Returns whether the client is ready.
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        matches!(self.state, ClientState::Ready)
    }
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

/// A pending response handle for an asynchronous HTTP request executed by [`ThreadedRequestExecutor`].
pub struct PendingResponse {
    receiver: Receiver<Result<HttpResponse, HttpError>>,
}

impl PendingResponse {
    /// Waits for the HTTP response, blocking the current thread until completion.
    ///
    /// # Errors
    ///
    /// Returns [`HttpError`] if the request failed or worker thread disconnected.
    pub fn wait(self) -> Result<HttpResponse, HttpError> {
        self.receiver.recv().map_err(|_| HttpError::transport("worker thread disconnected"))?
    }

    /// Waits for the HTTP response up to the given timeout.
    ///
    /// # Errors
    ///
    /// Returns [`HttpError`] on request failure, timeout, or worker disconnection.
    pub fn wait_timeout(self, timeout: Duration) -> Result<HttpResponse, HttpError> {
        match self.receiver.recv_timeout(timeout) {
            Ok(result) => result,
            Err(RecvTimeoutError::Timeout) => Err(HttpError::transport("request timed out waiting for response")),
            Err(RecvTimeoutError::Disconnected) => Err(HttpError::transport("worker thread disconnected")),
        }
    }
}

enum RequestMessage {
    Execute {
        request: HttpRequest,
        response_sender: Sender<Result<HttpResponse, HttpError>>,
    },
    Shutdown,
}

/// A threaded background executor for HTTP requests.
///
/// Dispatches requests to background worker threads so HTTP I/O does not block
/// the main thread or gateway event processing loop.
pub struct ThreadedRequestExecutor {
    sender: Option<Sender<RequestMessage>>,
    workers: Vec<JoinHandle<()>>,
}

impl ThreadedRequestExecutor {
    /// Spawns a new threaded request executor with the given number of worker threads.
    #[must_use]
    pub fn new<T: Transport + Send + Sync + 'static>(transport: Arc<T>, worker_count: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let count = worker_count.max(1);
        let mut workers = Vec::with_capacity(count);

        for _ in 0..count {
            let rx = Arc::clone(&receiver);
            let tx_transport = Arc::clone(&transport);

            let handle = thread::spawn(move || {
                loop {
                    let msg = {
                        let lock = rx.lock().unwrap();
                        lock.recv()
                    };

                    match msg {
                        Ok(RequestMessage::Execute {
                            request,
                            response_sender,
                        }) => {
                            let result = tx_transport.execute(&request);
                            let _ = response_sender.send(result);
                        }
                        Ok(RequestMessage::Shutdown) | Err(_) => break,
                    }
                }
            });
            workers.push(handle);
        }

        Self {
            sender: Some(sender),
            workers,
        }
    }

    /// Enqueues an HTTP request for asynchronous execution and returns a pending handle.
    ///
    /// # Errors
    ///
    /// Returns [`HttpError`] if the executor has shut down.
    pub fn submit(&self, request: HttpRequest) -> Result<PendingResponse, HttpError> {
        if let Some(ref sender) = self.sender {
            let (tx, rx) = mpsc::channel();
            sender
                .send(RequestMessage::Execute {
                    request,
                    response_sender: tx,
                })
                .map_err(|_| HttpError::transport("executor shut down"))?;
            Ok(PendingResponse { receiver: rx })
        } else {
            Err(HttpError::transport("executor is closed"))
        }
    }

    /// Shuts down all worker threads gracefully.
    pub fn shutdown(&mut self) {
        if let Some(sender) = self.sender.take() {
            for _ in 0..self.workers.len() {
                let _ = sender.send(RequestMessage::Shutdown);
            }
        }
        for handle in self.workers.drain(..) {
            let _ = handle.join();
        }
    }
}

impl Drop for ThreadedRequestExecutor {
    fn drop(&mut self) {
        self.shutdown();
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

    #[test]
    fn threaded_request_executor_executes_requests_asynchronously() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        struct ThreadSafeMockTransport {
            count: AtomicUsize,
        }

        impl Transport for ThreadSafeMockTransport {
            fn execute(&self, _request: &HttpRequest) -> Result<HttpResponse, HttpError> {
                self.count.fetch_add(1, Ordering::SeqCst);
                Ok(HttpResponse::new(200, Vec::new(), Vec::new()))
            }
        }

        let transport = Arc::new(ThreadSafeMockTransport {
            count: AtomicUsize::new(0),
        });

        let mut executor = super::ThreadedRequestExecutor::new(Arc::clone(&transport), 2);
        let pending1 = executor.submit(HttpRequest::new(HttpMethod::Get, "users/@me")).unwrap();
        let pending2 = executor.submit(HttpRequest::new(HttpMethod::Get, "gateway")).unwrap();

        let resp1 = pending1.wait().unwrap();
        let resp2 = pending2.wait_timeout(std::time::Duration::from_secs(2)).unwrap();

        assert_eq!(resp1.status, 200);
        assert_eq!(resp2.status, 200);
        assert_eq!(transport.count.load(Ordering::SeqCst), 2);

        executor.shutdown();
    }
}
