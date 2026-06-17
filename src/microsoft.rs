use serde::Deserialize;

/// # Microsoft ID Token Claims
/// Decoded ID Token payload from Microsoft's Azure AD / Entra ID (`/oauth2/v2.0/token`).
/// Fields vary by token version (v1.0 vs v2.0) and requested scopes.
/// Reference: <https://learn.microsoft.com/en-us/entra/identity-platform/id-token-claims-reference>
#[derive(Debug, Clone, Deserialize)]
pub struct MicrosoftIdTokenClaims {
    /// The issuer URI identifying the Security Token Service (STS) that constructed the token,
    /// e.g. `"https://login.microsoftonline.com/{tenant}/v2.0"`.
    pub iss: String,
    /// The audience the token is intended for your application's Client Identifier.
    pub aud: String,
    /// Token issuance time as Unix epoch seconds.
    pub iat: u64,
    /// Token expiration time as Unix epoch seconds.
    pub exp: u64,
    /// Not-before time; the token is invalid before this Unix epoch second.
    pub nbf: u64,
    /// The immutable, unique identifier for the user within their tenant.
    /// Use this as the primary identifier for the user within a single tenant.
    pub sub: String,
    /// The Object ID of the user in Azure AD. Unique within the tenant.
    pub oid: Option<String>,
    /// A GUID representing the Azure AD tenant the user belongs to.
    pub tid: Option<String>,
    /// The user's UPN (User Principal Name), e.g. `"user@contoso.com"`.
    /// May be unverified. Do not rely on it as a secure identifier.
    pub upn: Option<String>,
    /// The user's display name.
    pub name: Option<String>,
    /// The user's given (first) name.
    pub given_name: Option<String>,
    /// The user's family (last) name.
    pub family_name: Option<String>,
    /// The user's email address. Not always present; prefer `upn` or `preferred_username`.
    ///
    /// **Warning**: Do not use email as a primary identifier; always prefer `oid` or `sub`.
    pub email: Option<String>,
    /// The primary username representing the user, typically a UPN or email.
    pub preferred_username: Option<String>,
    /// URL of the user's profile picture (v2.0 tokens only).
    pub picture: Option<String>,
    /// Nonce value echoed from the authentication request, used for replay protection.
    pub nonce: Option<String>,
    /// Authentication-context class reference indicating the authentication method used.
    pub acr: Option<String>,
    /// Authentication Methods References (array of strings, serialised as space-separated in some flows).
    pub amr: Option<Vec<String>>,
    /// Time of user authentication as Unix epoch seconds.
    pub auth_time: Option<u64>,
    /// The at_hash claim links this ID Token to an access token for additional validation.
    pub at_hash: Option<String>,
    /// The c_hash claim links this ID Token to an authorisation code.
    pub c_hash: Option<String>,
    /// The roles assigned to the user for this application (app roles).
    pub roles: Option<Vec<String>>,
    /// Groups the user is a member of, when group claims are configured.
    pub groups: Option<Vec<String>>,
}
