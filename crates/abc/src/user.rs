//! User-related abstract base traits.
//! In Rust the shared behaviour is a trait that
//! `User`, `ClientUser` and `Member` will implement.

use crate::{Asset, HasId, Snowflake};

/// Shared operations on a Discord user.
/// Implementors must also implement [`HasId`].
pub trait User: HasId {
    /// Returns the user's username.
    fn name(&self) -> &str;

    /// Returns the user's discriminator.
    /// This is a legacy concept; new usernames use `"0"`.
    fn discriminator(&self) -> &str;

    /// Returns the user's global nickname, if set.
    fn global_name(&self) -> Option<&str> {
        None
    }

    /// Returns `true` if the user is a bot account.
    fn bot(&self) -> bool {
        false
    }

    /// Returns `true` if the user is a system account.
    fn system(&self) -> bool {
        false
    }

    /// Returns the user's uploaded avatar, if present.
    fn avatar(&self) -> Option<&Asset> {
        None
    }

    /// Returns the user's avatar decoration, if present.
    fn avatar_decoration(&self) -> Option<&Asset> {
        None
    }

    /// Returns the SKU ID of the user's avatar decoration, if present.
    fn avatar_decoration_sku_id(&self) -> Option<Snowflake> {
        None
    }

    /// Returns the user's display name.
    /// The global name takes precedence over the username for regular users.
    /// Models with a guild-specific nickname can override this.
    fn display_name(&self) -> &str {
        self.global_name().unwrap_or_else(|| self.name())
    }

    /// Returns the default avatar for this user.
    fn default_avatar(&self) -> Asset {
        let index = match self.discriminator() {
            "0" | "0000" => ((self.id().get() >> 22) % 6) as u8,
            discriminator => discriminator
                .parse::<u64>()
                .map_or(0, |value| (value % 5) as u8),
        };

        Asset::from_default_avatar(index)
    }

    /// Returns the avatar the user is displaying.
    /// This is the uploaded avatar when one exists, otherwise the default
    /// avatar.
    fn display_avatar(&self) -> Asset {
        self.avatar()
            .cloned()
            .unwrap_or_else(|| self.default_avatar())
    }

    /// Returns a string that mentions this user.
    fn mention(&self) -> String {
        format!("<@{}>", self.id())
    }
}
