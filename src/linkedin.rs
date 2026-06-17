use serde::Deserialize;

/// # LinkedIn ID Token Claims
/// Decoded ID Token payload from LinkedIn's OpenID Connect flow (`/oauth/v2/userinfo`).
/// Requires the `openid` scope; additional scopes (`profile`, `email`) unlock optional fields.
/// Reference: <https://learn.microsoft.com/en-us/linkedin/consumer/integrations/self-serve/sign-in-with-linkedin-v2>
#[derive(Debug, Clone, Deserialize)]
pub struct LinkedInIdTokenClaims {
    /// The issuer identifier — always `"https://www.linkedin.com"`.
    pub iss: String,
    /// The audience — your application's Client Identifier.
    pub aud: String,
    /// Token issuance time as Unix epoch seconds.
    pub iat: u64,
    /// Token expiration time as Unix epoch seconds.
    pub exp: u64,
    /// The user's unique LinkedIn subject identifier. Stable and never reused.
    /// Always use this as the primary identifier for the user.
    pub sub: String,
    /// The user's full name. Present when `profile` scope is granted.
    pub name: Option<String>,
    /// The user's given (first) name. Present when `profile` scope is granted.
    pub given_name: Option<String>,
    /// The user's family (last) name. Present when `profile` scope is granted.
    pub family_name: Option<String>,
    /// URL of the user's profile picture. Present when `profile` scope is granted.
    pub picture: Option<String>,
    /// The user's preferred locale (e.g. `"en_US"`). Present when `profile` scope is granted.
    pub locale: Option<String>,
    /// The user's email address. Present when `email` scope is granted.
    ///
    /// **Warning**: Do not rely on email as a primary identifier; always prefer `sub`.
    pub email: Option<String>,
    /// Whether the user's email address has been verified. Present when `email` scope is granted.
    pub email_verified: Option<bool>,
}
