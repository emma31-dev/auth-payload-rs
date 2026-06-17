use serde::Deserialize;

/// # Standard JWT Metadata Claims
/// Decoded ID Token payload structure from Google authentication id_token value returned in responses
#[derive(Debug, Clone, Deserialize)]
pub struct GoogleIdTokenClaims {
    /// The Issuer Identifier for the Issuer of the response.
    /// Typically https://accounts.google.com; however, accounts.google.com
    /// is also returned for legacy implementations.
    pub iss: String,
    /// An identifier for the user, unique among all Google Accounts and never reused.
    /// A Google Account can have multiple email addresses at different points in time,
    /// but the sub value is never changed. Use sub within your application as the unique-identifier
    /// key for the user. Maximum length of 255 case-sensitive ASCII characters.
    pub sub: String,
    /// The Client Identifier of the authorized presenter, obtained from the Google Cloud console.
    /// This claim is only needed when the party requesting the ID Token is not the same
    /// as the audience of the ID Token.
    pub azp: Option<String>,
    /// The audience for which the ID Token is intended.
    /// This is your application's Client Identifier, obtained from the Google Cloud console.
    pub aud: String,
    /// The time the ID Token was issued. Represented in Unix epoch time (integer seconds).
    pub iat: u64,
    /// Expiration time on or after which the ID Token must not be accepted.
    /// Represented in Unix epoch time (integer seconds).
    pub exp: u64,
    /// The value of the nonce supplied by your app in the authentication request.
    /// You should protect against replay attacks by presenting this value only once.
    pub nonce: Option<String>,
    /// The time user authentication took place, a JSON number representing the number of seconds
    /// that have elapsed since the Unix epoch (January 1, 1970, 00:00:00 UTC). Provided when the
    /// auth_time claim is included in the authentication request claims parameter.
    pub auth_time: Option<u16>,
    /// Access token hash. Provides validation that the Access Token is tied to the identity token.
    /// If the ID Token is issued with an access_token value in the server flow, this claim is always included.
    pub at_hash: Option<u16>,
    /// The user's full name, in a displayable form. Might be provided when the request scope
    /// included the string profile or the ID Token is returned from a token refresh.
    pub name: Option<String>,
    /// The user's given name(s) or first name(s). Might be provided when a name claim is present.
    pub given_name: Option<String>,
    /// The user's surname(s) or last name(s). Might be provided when a name claim is present.
    pub family_name: Option<String>,
    /// The URL of the user's profile picture. Might be provided when the request scope
    /// included the string profile or the ID token is returned from a token refresh.
    pub picture: Option<String>,
    /// The user's email address. Provided only if you included the email scope in your request.
    /// The value of this claim may not be unique to this account and could change over time,
    /// therefore you shouldn't use this value as the primary identifier to link to your user record.
    /// You also can't rely on the domain of the email claim to identify users of Google Workspace or Cloud
    /// organizations; use the hd claim instead.
    ///
    /// **Warning**: Don't use email address as an identifier because a Google Account can have multiple
    /// email addresses at different points in time. Always use the sub field as the identifier for the user.
    pub email: Option<String>,
    /// True if the user's email address has been verified; otherwise false.
    pub email_verified: Option<bool>,
    /// The domain associated with the Google Workspace or Cloud organization of the user.
    /// Provided only if the user belongs to a Google Cloud organization. You must check this
    /// claim when restricting access to a resource to only members of certain domains. The
    /// absence of this claim indicates that the account does not belong to a Google hosted domain.
    pub hd: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GoogleUserInfoResponse {
    ///An identifier for the user, unique among all Google Accounts and never reused.
    /// Case-sensitive string not exceeding 255 characters.
    pub sub: String,
    /// The user's full name, in a displayable form.
    pub name: Option<String>,
    /// The user's given name(s) or first name(s).
    pub given_name: Option<String>,
    /// The user's surname(s) or last name(s).
    pub family_name: Option<String>,
    /// The URL of the user's profile picture.
    pub picture: Option<String>,
    /// The user's email address.
    pub email: Option<String>,
    /// Whether the user's email address has been verified.
    pub email_verified: Option<bool>,
    /// The hosted domain associated with the user's Google Workspace or Cloud organization.
    pub hud: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_token_claims_full() {
        let json = r#"{
            "iss": "https://accounts.google.com",
            "sub": "1234567890",
            "aud": "your-client-id.apps.googleusercontent.com",
            "iat": 1700000000,
            "exp": 1700003600,
            "azp": "your-client-id.apps.googleusercontent.com",
            "nonce": "abc123",
            "auth_time": 1700,
            "at_hash": 42,
            "name": "Jane Doe",
            "given_name": "Jane",
            "family_name": "Doe",
            "picture": "https://example.com/photo.jpg",
            "email": "jane@example.com",
            "email_verified": true,
            "hd": "example.com"
        }"#;
        let c: GoogleIdTokenClaims = serde_json::from_str(json).unwrap();
        assert_eq!(c.iss, "https://accounts.google.com");
        assert_eq!(c.sub, "1234567890");
        assert_eq!(c.aud, "your-client-id.apps.googleusercontent.com");
        assert_eq!(c.iat, 1700000000);
        assert_eq!(c.exp, 1700003600);
        assert_eq!(c.email.as_deref(), Some("jane@example.com"));
        assert_eq!(c.email_verified, Some(true));
        assert_eq!(c.hd.as_deref(), Some("example.com"));
    }

    #[test]
    fn id_token_claims_minimal() {
        let json = r#"{
            "iss": "https://accounts.google.com",
            "sub": "1234567890",
            "aud": "client-id",
            "iat": 1700000000,
            "exp": 1700003600
        }"#;
        let c: GoogleIdTokenClaims = serde_json::from_str(json).unwrap();
        assert_eq!(c.sub, "1234567890");
        assert!(c.email.is_none());
        assert!(c.hd.is_none());
        assert!(c.name.is_none());
    }

    #[test]
    fn userinfo_response_full() {
        let json = r#"{
            "sub": "1234567890",
            "name": "Jane Doe",
            "given_name": "Jane",
            "family_name": "Doe",
            "picture": "https://example.com/photo.jpg",
            "email": "jane@example.com",
            "email_verified": true,
            "hud": "example.com"
        }"#;
        let u: GoogleUserInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(u.sub, "1234567890");
        assert_eq!(u.email.as_deref(), Some("jane@example.com"));
        assert_eq!(u.hud.as_deref(), Some("example.com"));
    }

    #[test]
    fn userinfo_response_minimal() {
        let json = r#"{ "sub": "1234567890" }"#;
        let u: GoogleUserInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(u.sub, "1234567890");
        assert!(u.email.is_none());
        assert!(u.name.is_none());
    }
}
