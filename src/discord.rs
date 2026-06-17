use serde::Deserialize;

/// # Discord ID Token Claims
/// Decoded ID Token payload returned from Discord's OAuth2 token endpoint when using the `identify` scope.
/// Reference: <https://discord.com/developers/docs/topics/oauth2>
#[derive(Debug, Clone, Deserialize)]
pub struct DiscordIdTokenClaims {
    /// The user's unique Discord Snowflake ID. Never changes across renames or email updates.
    /// Always use this as the primary identifier for the user.
    pub id: String,
    /// The user's username (not unique across the platform).
    pub username: String,
    /// The user's Discord-tag discriminator (e.g. `"0"` for pomelo users, `"1234"` for legacy).
    pub discriminator: String,
    /// The hash of the user's avatar. Use with
    /// `https://cdn.discordapp.com/avatars/{id}/{avatar}.png` to build the URL.
    /// `None` if the user has no custom avatar.
    pub avatar: Option<String>,
    /// Whether the user belongs to an OAuth2 application (i.e. is a bot account).
    pub bot: Option<bool>,
    /// Whether the user is an Official Discord System user.
    pub system: Option<bool>,
    /// Whether the user has two-factor authentication enabled.
    pub mfa_enabled: Option<bool>,
    /// The user's banner hash. `None` if no custom banner is set.
    pub banner: Option<String>,
    /// The user's banner colour encoded as an integer representation of a hex colour code.
    pub accent_color: Option<u32>,
    /// The user's chosen language option (e.g. `"en-US"`).
    pub locale: Option<String>,
    /// The user's email address. Only present when the `email` scope is requested.
    ///
    /// **Warning**: Do not use email as a primary identifier; always prefer `id`.
    pub email: Option<String>,
    /// Whether the user's email address has been verified. Only present when the `email` scope is requested.
    pub verified: Option<bool>,
    /// The flags on the user's account as a bitfield integer.
    pub flags: Option<u64>,
    /// The type of Nitro subscription: `0` = None, `1` = Classic, `2` = Nitro, `3` = Basic.
    pub premium_type: Option<u8>,
    /// The public flags on the user's account as a bitfield integer (shown on profiles).
    pub public_flags: Option<u64>,
    /// The user's display name if one is set. Supersedes `username` in UI contexts.
    pub global_name: Option<String>,
}
