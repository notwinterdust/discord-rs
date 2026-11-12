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

    /// Returns the path for editing a webhook message.
    #[must_use]
    pub fn edit_message_path(&self, message_id: u64) -> String {
        format!("{}/messages/{message_id}", self.execute_path())
    }

    /// Returns the path for deleting a webhook message.
    #[must_use]
    pub fn delete_message_path(&self, message_id: u64) -> String {
        self.edit_message_path(message_id)
    }

    /// Returns the path for fetching a specific webhook message.
    #[must_use]
    pub fn fetch_message_path(&self, message_id: u64) -> String {
        self.edit_message_path(message_id)
    }

    /// Returns the base endpoint path for editing this webhook.
    #[must_use]
    pub fn edit_path(&self) -> String {
        self.execute_path()
    }

    /// Returns the base endpoint path for deleting this webhook.
    #[must_use]
    pub fn delete_path(&self) -> String {
        self.execute_path()
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
        (
            format!("multipart/form-data; boundary={}", self.boundary),
            self.body,
        )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn webhook_paths_and_message_encoding() {
        let wh = Webhook::new(12345, "secret_token");
        assert_eq!(wh.execute_path(), "webhooks/12345/secret_token");
        assert_eq!(wh.edit_path(), "webhooks/12345/secret_token");
        assert_eq!(wh.delete_path(), "webhooks/12345/secret_token");
        assert_eq!(
            wh.edit_message_path(6789),
            "webhooks/12345/secret_token/messages/6789"
        );
        assert_eq!(
            wh.fetch_message_path(6789),
            "webhooks/12345/secret_token/messages/6789"
        );
        assert_eq!(
            wh.delete_message_path(6789),
            "webhooks/12345/secret_token/messages/6789"
        );

        let msg = WebhookMessage::new()
            .with_content("hello\nworld")
            .silent(true);
        let encoded = wh.message(msg);
        assert!(encoded.contains("\"content\":\"hello\\nworld\""));
        assert!(encoded.contains("\"silent\":true"));
    }

    #[test]
    fn multipart_body_assembly() {
        let mut mp = Multipart::new("xyzBoundary");
        mp.field("username", "TestBot");
        mp.file("payload_json", "message.json", "application/json", b"{}");
        let (content_type, body) = mp.finish();

        assert_eq!(content_type, "multipart/form-data; boundary=xyzBoundary");
        let body_str = String::from_utf8_lossy(&body);
        assert!(body_str.contains("--xyzBoundary\r\n"));
        assert!(body_str.contains("name=\"username\""));
        assert!(body_str.contains("TestBot"));
        assert!(body_str.contains("--xyzBoundary--\r\n"));
    }
}
