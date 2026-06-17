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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_token_claims_full() {
        let json = r#"{
            "id": "80351110224678912",
            "username": "nelly",
            "discriminator": "0",
            "global_name": "Nelly",
            "avatar": "8342729096ea3675442027381ff50dfe",
            "bot": false,
            "system": false,
            "mfa_enabled": true,
            "banner": "banner-hash",
            "accent_color": 16711680,
            "locale": "en-US",
            "email": "nelly@example.com",
            "verified": true,
            "flags": 64,
            "premium_type": 2,
            "public_flags": 64
        }"#;
        let c: DiscordIdTokenClaims = serde_json::from_str(json).unwrap();
        assert_eq!(c.id, "80351110224678912");
        assert_eq!(c.username, "nelly");
        assert_eq!(c.discriminator, "0");
        assert_eq!(c.global_name.as_deref(), Some("Nelly"));
        assert_eq!(c.email.as_deref(), Some("nelly@example.com"));
        assert_eq!(c.verified, Some(true));
        assert_eq!(c.premium_type, Some(2));
    }

    #[test]
    fn id_token_claims_minimal() {
        let json = r#"{
            "id": "123",
            "username": "ghost",
            "discriminator": "0"
        }"#;
        let c: DiscordIdTokenClaims = serde_json::from_str(json).unwrap();
        assert_eq!(c.id, "123");
        assert!(c.email.is_none());
        assert!(c.avatar.is_none());
        assert!(c.mfa_enabled.is_none());
    }
}
