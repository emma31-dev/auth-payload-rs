//! # Auth-payload-rs 
//! 
//! A lightweight, zero-dependency (almost) crate for deserializing and validating OIDC/OAuth2 payloads from Google, GitHub, and Facebook. This project is open for contribution for all, check out [Repository](https://github.com/emma31-dev/auth-payload-rs).
//! 
//! ## Why this exists?
//! Most auth libraries are monolithic. This crate provides the raw, audited `structs` and `validation` logic for Axum backends that want total control over the identity flow. We intent to expand support to salvo, actix, Rocket and more as time passes.
//! 
//! ## How to use?
//! 1. For Google's standard JWT Metadata Claims
//! ```rust
//! // Other imports
//! use auth-payload-rs::GoogleIdTokenClaims;
//! 
//! fn googleCallback(Json(payload): Json<GoogleIdTokenClaims>, /* Other arguments */) -> Result<(), ()> {
//!     // Do stuff with payload
//! }
//! ```
//! 2. For Google's UserInfo Endpoint
//! ```rust
//! // Other imports
//! use auth-payload-rs::GoogleUserInfoResponse;
//! 
//! fn googleCallback(Json(payload): Json<GoogleUserInfoResponse>, /* Other arguments */) -> Result<(), ()> {
//!     // Do stuff with payload
//! }
//! ```
//! 
//! ---
//! 
//! ## Google payload
//! 
//! 1.) Decoded ID Token payload structure from Google authentication id_token value returned in responses
//! ```rust
//! pub struct GoogleIdTokenClaims {
//!     pub iss: String, // The Issuer Identifier
//!     pub sub: String, // An identifier for the user
//!     pub azp: Option<String>, // The Client Identifier of the authorized presenter
//!     pub aud: String, // Your application's Client Identifier from google console
//!     pub iat: u64, // The time the ID Token was issued
//!     pub exp: u64, // Expiration time
//!     pub nonce: Option<String>, // The value of the nonce supplied by your app in the authentication request
//!     pub auth_time: Option<u16>, // The time user authentication took place
//!     pub at_hash: Option<u16>, // Access token hash
//!     pub name: Option<String>, // The user's full name
//!     pub given_name: Option<String>, // The user's first name
//!     pub family_name: Option<String>, // The user's surname
//!     pub picture: Option<String>, // The URL of the user's profile picture
//!     pub email: Option<String>, // The user's email address
//!     pub email_verified: bool, // Does it need explanation?
//!     pub hd: Option<String>, // The domain associated with the Google Workspace
//! }
//! ```
//! 
//! 2.) The profile information about the authenticated user UserInfo Endpoint returns
//! ```rust
//! pub struct GoogleUserInfoResponse {
//!     pub sub: String, //The identifier for the user
//!     pub name: Option<String>, // The user's full name
//!     pub given_name: Option<String>, // The user's given name(s) or first name(s).
//!     pub family_name: Option<String>, // The user's surname(s) or last name(s).
//!     pub picture: Option<String>, // The URL of the user's profile picture.
//!     pub email: Option<String>, // The user's email address.
//!     pub email_verified: bool, // Whether the user's email address has been verified.
//!     pub hud: Option<String>, // The hosted domain associated with the user's Google Workspace or Cloud organization.
//! }
//! ```
//! **Warn**: Other payloads are yet to be formalized because of lack or readily available information on them. Be rest assured they will be ready by version 0.2. Thank you :)
//! 
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
    pub email_verified: bool,
    /// The domain associated with the Google Workspace or Cloud organization of the user. 
    /// Provided only if the user belongs to a Google Cloud organization. You must check this 
    /// claim when restricting access to a resource to only members of certain domains. The 
    /// absence of this claim indicates that the account does not belong to a Google hosted domain.
    pub hd: Option<String>,
}

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
    pub email_verified: bool,
    /// The hosted domain associated with the user's Google Workspace or Cloud organization.
    pub hud: Option<String>,
}


/// ```json
/// {
/// "login": "octocat",
/// "id": 1,
/// "node_id": "MDQ6VXNlcjE=",
/// "avatar_url": "https://github.com/images/error/octocat_happy.gif",
/// "gravatar_id": "",
/// "url": "https://api.github.com/users/octocat",
/// "html_url": "https://github.com/octocat",
/// followers_url": "https://api.github.com/users/octocat/followers",
/// "following_url": "https://api.github.com/users/octocat/following{/other_user}",
/// "gists_url": "https://api.github.com/users/octocat/gists{/gist_id}",
/// "starred_url": "https://api.github.com/users/octocat/starred{/owner}{/repo}",
/// "subscriptions_url": "https://api.github.com/users/octocat/subscriptions",
/// "organizations_url": "https://api.github.com/users/octocat/orgs",
/// "repos_url": "https://api.github.com/users/octocat/repos",
/// "events_url": "https://api.github.com/users/octocat/events{/privacy}",
/// "received_events_url": "https://api.github.com/users/octocat/received_events",
/// "type": "User",
/// "site_admin": false,
//   "name": "monalisa octocat",
//   "company": "GitHub",
//   "blog": "https://github.com/blog",
//   "location": "San Francisco",
//   "email": "octocat@github.com",
//   "hireable": false,
//   "bio": "There once was...",
//   "public_repos": 2,
//   "public_gists": 1,
//   "followers": 20,
//   "following": 0,
//   "created_at": "2008-01-14T04:33:35Z",
//   "updated_at": "2008-01-14T04:33:35Z",
//   "private_gists": 81,
//   "total_private_repos": 100,
//   "owned_private_repos": 100,
//   "disk_usage": 10000,
//   "collaborators": 8,
//   "two_factor_authentication": true,
//   "plan": {
//     "name": "Medium",
//     "space": 400,
//     "private_repos": 20,
//     "collaborators": 0
//   }
// }
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct GitHubUserPayload {
    /// The unique numeric identifier. The absolute single source of truth 
    /// for mapping a database user record. Never changes.
    pub id: u64,
    /// The alphanumeric handle (e.g., "octocat"). Users can change this 
    /// at will—never use it as a database primary key.
    pub login: String,
    pub avatar_url: Option<String>,
    pub html_url: String,
    /// Optional field. Can be null if the profile name is left empty.
    pub name: Option<String>,
    /// CRITICAL TRAP: This will be null if the user has marked their email 
    /// as private in GitHub settings, even if your OAuth app requested the `user:email` scope.
    pub email: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubEmailItem {
    pub email: String,
    pub primary: bool,
    pub verified: bool,
    pub visibility: Option<String>,
}

// Logic: Iterate through the returned Vec<GitHubEmailItem> and explicitly 
// extract the item where `primary == true` and `verified == true`.

#[derive(Debug, Clone, Deserialize)]
pub struct FacebookUserPayload {
    /// App-Scoped User ID (ASID). This is NOT a global Facebook ID. 
    /// It is a unique string generated strictly for your specific App ID.
    pub id: String,
    pub name: String,
    /// Optional field. Can be null if the user signed up using a phone number 
    /// instead of an email, or if they opted out of sharing their email.
    pub email: Option<String>,
    pub picture: Option<FacebookPictureContainer>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FacebookPictureContainer {
    pub data: FacebookPictureData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FacebookPictureData {
    pub url: String,
    pub is_silhouette: bool,
}