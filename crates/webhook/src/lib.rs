//! Webhook request builders.

#![forbid(unsafe_code)]

/// A webhook message request.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WebhookMessage {
    /// Optional message content.
    pub content: Option<String>,
    /// Whether the message should be delivered without notifications.
    pub silent: bool,
}

impl WebhookMessage {
    /// Creates an empty webhook message.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            content: None,
            silent: false,
        }
    }

    /// Sets message content.
    #[must_use]
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Sets silent delivery.
    #[must_use]
    pub const fn silent(mut self, silent: bool) -> Self {
        self.silent = silent;
        self
    }
}

/// A webhook endpoint identified by its token.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Webhook {
    /// The webhook identifier.
    pub id: u64,
    /// The webhook token.
    pub token: String,
}

impl Webhook {
    /// Creates a webhook endpoint.
    #[must_use]
    pub fn new(id: u64, token: impl Into<String>) -> Self {
        Self {
            id,
            token: token.into(),
        }
    }

    /// Returns the execute-webhook path.
    #[must_use]
    pub fn execute_path(&self) -> String {
        format!("webhooks/{}/{}", self.id, self.token)
    }

    /// Builds a message request payload in the project JSON representation.
    #[must_use]
    pub fn message(&self, message: WebhookMessage) -> String {
        let content = message
            .content
            .map(|value| format!("\"{}\"", escape_json(&value)))
            .unwrap_or_else(|| "null".to_owned());
        format!("{{\"content\":{content},\"silent\":{}}}", message.silent)
    }
}

/// Synchronous webhook operations share the same request model.
pub type SyncWebhook = Webhook;

/// A multipart body assembled without external dependencies.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Multipart {
    boundary: String,
    body: Vec<u8>,
}

impl Multipart {
    /// Creates a multipart body with a boundary.
    #[must_use]
    pub fn new(boundary: impl Into<String>) -> Self {
        Self {
            boundary: boundary.into(),
            body: Vec::new(),
        }
    }

    /// Adds a text field.
    pub fn field(&mut self, name: &str, value: &str) {
        self.body.extend_from_slice(
            format!(
                "--{}\r\nContent-Disposition: form-data; name=\"{}\"\r\n\r\n{}\r\n",
                self.boundary, name, value
            )
            .as_bytes(),
        );
    }

    /// Adds a file field.
    pub fn file(&mut self, name: &str, filename: &str, content_type: &str, content: &[u8]) {
        self.body.extend_from_slice(
            format!(
                "--{}\r\nContent-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\nContent-Type: {}\r\n\r\n",
                self.boundary, name, filename, content_type
            )
            .as_bytes(),
        );
        self.body.extend_from_slice(content);
        self.body.extend_from_slice(b"\r\n");
    }

    /// Finalizes the body.
    #[must_use]
    pub fn finish(mut self) -> (String, Vec<u8>) {
        self.body
            .extend_from_slice(format!("--{}--\r\n", self.boundary).as_bytes());
        (format!("multipart/form-data; boundary={}", self.boundary), self.body)
    }
}

fn escape_json(value: &str) -> String {
    value
        .chars()
        .flat_map(|character| match character {
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\\' => "\\\\".chars().collect(),
            '\n' => "\\n".chars().collect(),
            '\r' => "\\r".chars().collect(),
            '\t' => "\\t".chars().collect(),
            other => vec![other],
        })
        .collect()
}
