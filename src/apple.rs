use serde::Deserialize;

/// # Apple ID Token Claims
/// Decoded ID Token payload from Sign in with Apple (`/auth/token`).
/// Apple returns a minimal set of claims; name and email are only provided on the **first** authorisation.
/// Reference: <https://developer.apple.com/documentation/sign_in_with_apple/sign_in_with_apple_rest_api/authenticating_users_with_sign_in_with_apple>
#[derive(Debug, Clone, Deserialize)]
pub struct AppleIdTokenClaims {
    /// The issuer that always `"https://appleid.apple.com"`.
    pub iss: String,
    /// The audience i.e your application's bundle ID / Client Identifier.
    pub aud: String,
    /// Token issuance time as Unix epoch seconds.
    pub iat: u64,
    /// Token expiration time as Unix epoch seconds.
    pub exp: u64,
    /// The unique, stable identifier for the user. Never reused across Apple IDs.
    /// Always use this as the primary identifier for the user.
    pub sub: String,
    /// Nonce value echoed from the authentication request, used for replay protection.
    /// Present only when a nonce was supplied in the request.
    pub nonce: Option<String>,
    /// Whether the nonce contained in the token was suppressed by Apple for security.
    pub nonce_supported: Option<bool>,
    /// The user's email address or an Apple-generated private relay address.
    /// **Only returned on the first authentication**; cache it immediately.
    ///
    /// **Warning**: Do not use email as a primary identifier; always prefer `sub`.
    pub email: Option<String>,
    /// Whether the email address has been verified by Apple.
    /// The value is a string `"true"` or `"false"` in Apple's response.
    pub email_verified: Option<String>,
    /// Whether the email address is a private relay (anonymised) address managed by Apple.
    /// The value is a string `"true"` or `"false"` in Apple's response.
    pub is_private_email: Option<String>,
    /// Whether the user appears to be a real person (`0` = unsupported, `1` = unknown, `2` = likely real).
    pub real_user_status: Option<u8>,
    /// A token the app can use to transfer the user's account to another app in the same team.
    pub transfer_sub: Option<String>,
}
