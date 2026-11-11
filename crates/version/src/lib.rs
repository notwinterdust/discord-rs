//! Library and Discord API version information.
//!
//! The constants in this crate keep the REST and gateway URLs in one place.

#![forbid(unsafe_code)]

/// The Discord REST API version used by this library.
pub const DISCORD_API_VERSION: u8 = 10;

/// The base URL of Discord's REST API.
pub const API_BASE: &str = "https://discord.com/api/v10";

/// The base URL of Discord's gateway.
pub const GATEWAY_URL: &str = "wss://gateway.discord.gg";

/// This library's version.
pub const LIBRARY_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The user agent sent with REST requests.
pub const USER_AGENT: &str = concat!("discord-rs/", env!("CARGO_PKG_VERSION"));

/// Builds an absolute REST API URL from a path relative to the API version.
///
/// Leading slashes are ignored so that both `"guilds/1"` and `"/guilds/1"`
/// produce the same URL.
#[must_use]
pub fn api_url(path: &str) -> String {
    let path = path.trim_start_matches('/');
    if path.is_empty() {
        API_BASE.to_owned()
    } else {
        format!("{API_BASE}/{path}")
    }
}

#[cfg(test)]
mod tests {
    use super::{API_BASE, DISCORD_API_VERSION, GATEWAY_URL, LIBRARY_VERSION, USER_AGENT, api_url};

    #[test]
    fn version_constants_are_consistent() {
        assert_eq!(DISCORD_API_VERSION, 10);
        assert!(API_BASE.ends_with("/api/v10"));
        assert!(GATEWAY_URL.starts_with("wss://"));
        assert_eq!(LIBRARY_VERSION, env!("CARGO_PKG_VERSION"));
        assert_eq!(
            USER_AGENT,
            format!("discord-rs/{}", env!("CARGO_PKG_VERSION"))
        );
    }

    #[test]
    fn api_urls_join_paths() {
        assert_eq!(api_url("guilds/1"), format!("{API_BASE}/guilds/1"));
        assert_eq!(api_url("/guilds/1"), format!("{API_BASE}/guilds/1"));
        assert_eq!(api_url(""), API_BASE);
        assert_eq!(api_url("/"), API_BASE);
    }
}
