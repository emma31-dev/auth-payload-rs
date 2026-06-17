//! # Auth-payload-rs
//!
//! A lightweight, zero-dependency (almost) crate for deserializing and validating OIDC/OAuth2 payloads
//! from Google, GitHub, Facebook, Apple, Discord, LinkedIn, and Microsoft.
//! This project is open for contribution, Please check out the [Repository](https://github.com/emma31-dev/auth-payload-rs).
//!
//! ## Why this exists?
//! Most auth libraries are monolithic. This crate provides the raw, audited `structs` and `validation`
//! logic for Axum backends that want total control over the identity flow. Support for Salvo, Actix,
//! Rocket and more is planned.
//!
//! ## Feature flags
//!
//! Each provider is behind a feature flag. `google` is enabled by default.
//!
//! ```toml
//! [dependencies]
//! auth-payload-rs = { version = "0.1.*", features = ["google", "github", "facebook", "apple", "discord", "linkedin", "microsoft"] }
//! ```
//!
//! ---
//!
//! ## How to use?
//!
//! Every struct derives [`serde::Deserialize`], so you can use it directly as an Axum extractor,
//! deserialize it from a raw JSON response, or pass it to any serde-compatible parser.
//!
//! ```rust,ignore
//! use auth_payload_rs::google::GoogleIdTokenClaims;
//!
//! async fn google_callback(Json(payload): Json<GoogleIdTokenClaims>) {
//!     let user_id = &payload.sub; // stable identifier, always prefer sub over email
//! }
//! ```
//!
//! ---
//!
//! ## Google (`feature = "google"`)
//!
//! ### `GoogleIdTokenClaims`
//! Decoded JWT payload from Google's `id_token` (returned by the token endpoint).
//!
//! ```rust,ignore
//! pub struct GoogleIdTokenClaims {
//!     pub iss: String,                    // Issuer. Example: "https://accounts.google.com"
//!     pub sub: String,                    // Stable user identifier to be used as primary key
//!     pub aud: String,                    // Your Client ID
//!     pub iat: u64,                       // Issued-at (Unix seconds)
//!     pub exp: u64,                       // Expiry (Unix seconds)
//!     pub azp: Option<String>,            // Authorized presenter
//!     pub nonce: Option<String>,          // Replay-protection nonce
//!     pub auth_time: Option<u16>,         // When authentication took place
//!     pub at_hash: Option<u16>,           // Access token hash
//!     pub name: Option<String>,           // Full name (requires `profile` scope)
//!     pub given_name: Option<String>,     // First name
//!     pub family_name: Option<String>,    // Last name
//!     pub picture: Option<String>,        // Avatar URL
//!     pub email: Option<String>,          // Email (requires `email` scope)
//!     pub email_verified: Option<bool>,   // Whether email is verified
//!     pub hd: Option<String>,             // Google Workspace hosted domain
//! }
//! ```
//!
//! ### `GoogleUserInfoResponse`
//! Profile returned by `GET https://openidconnect.googleapis.com/v1/userinfo`.
//!
//! ```rust,ignore
//! pub struct GoogleUserInfoResponse {
//!     pub sub: String,                    // Stable user identifier
//!     pub name: Option<String>,           // more explanation above ^
//!     pub given_name: Option<String>,
//!     pub family_name: Option<String>,
//!     pub picture: Option<String>,
//!     pub email: Option<String>,
//!     pub email_verified: Option<bool>,
//!     pub hud: Option<String>,
//! }
//! ```
//!
//! ---
//!
//! ## GitHub (`feature = "github"`)
//!
//! ### `GitHubUserPayload`
//! Response from `GET https://api.github.com/user`. Requires the `user` scope for private fields.
//!
//! ```rust,ignore
//! pub struct GitHubUserPayload {
//!     pub id: u64,                                  // Stable numeric ID — use as primary key
//!     pub login: String,                            // Username — can change, do not use as key
//!     pub node_id: String,                          // GraphQL node ID
//!     pub avatar_url: String,
//!     pub html_url: String,                         // Profile page URL
//!     pub name: Option<String>,
//!     pub email: Option<String>,                    // Public profile email only — may be null
//!     pub bio: Option<String>,
//!     pub company: Option<String>,
//!     pub blog: Option<String>,
//!     pub location: Option<String>,
//!     pub hireable: Option<bool>,
//!     pub twitter_username: Option<String>,
//!     pub public_repos: u32,
//!     pub public_gists: u32,
//!     pub followers: u32,
//!     pub following: u32,
//!     pub created_at: String,
//!     pub updated_at: String,
//!     // Private fields — only present for the authenticated user with `user` scope:
//!     pub private_gists: Option<u32>,
//!     pub total_private_repos: Option<u32>,
//!     pub owned_private_repos: Option<u32>,
//!     pub disk_usage: Option<u64>,
//!     pub collaborators: Option<u32>,
//!     pub two_factor_authentication: Option<bool>,
//!     pub plan: Option<GitHubPlan>,
//! }
//! ```
//!
//! ### `GitHubEmailItem`
//! One entry from `GET https://api.github.com/user/emails`. Iterate the `Vec` and select
//! the item where `primary == true && verified == true` for the canonical address.
//!
//! ```rust,ignore
//! pub struct GitHubEmailItem {
//!     pub email: String,
//!     pub primary: bool,
//!     pub verified: bool,
//!     pub visibility: Option<String>,   // Some("public") | Some("private") | None
//! }
//! ```
//!
//! ---
//!
//! ## Facebook (`feature = "facebook"`)
//!
//! ### `FacebookUserPayload`
//! Response from `GET https://graph.facebook.com/{user-id}`.
//! Fields beyond `public_profile` must be explicitly requested and require the matching permission.
//!
//! ```rust,ignore
//! pub struct FacebookUserPayload {
//!     pub id: String,                       // App-Scoped User ID to be used as primary key
//!     pub name: String,                     // Full name (public_profile)
//!     pub first_name: Option<String>,       // public_profile
//!     pub middle_name: Option<String>,      // public_profile
//!     pub last_name: Option<String>,        // public_profile
//!     pub short_name: Option<String>,       // public_profile
//!     pub name_format: Option<String>,      // public_profile
//!     pub picture: Option<FacebookPictureContainer>, // public_profile
//!     pub link: Option<String>,             // public_profile
//!     pub age_range: Option<FacebookAgeRange>, // public_profile
//!     pub email: Option<String>,            // requires `email` permission
//!     pub birthday: Option<String>,         // requires `user_birthday`
//!     pub gender: Option<String>,           // requires `user_gender`
//!     pub location: Option<FacebookPage>,   // requires `user_location`
//!     pub hometown: Option<FacebookPage>,   // requires `user_hometown`
//! }
//! ```
//!
//! ---
//!
//! ## Apple (`feature = "apple"`)
//!
//! ### `AppleIdTokenClaims`
//! Decoded JWT payload from Sign in with Apple (`POST https://appleid.apple.com/auth/token`).
//! **Email and name are only provided on the very first authorisation** — cache them immediately.
//!
//! ```rust,ignore
//! pub struct AppleIdTokenClaims {
//!     pub iss: String,                      // Always "https://appleid.apple.com"
//!     pub aud: String,                      // Your bundle ID / Client ID
//!     pub iat: u64,
//!     pub exp: u64,
//!     pub sub: String,                      // Stable user identifier — use as primary key
//!     pub nonce: Option<String>,
//!     pub nonce_supported: Option<bool>,
//!     pub email: Option<String>,            // First auth only; may be a private relay address
//!     pub email_verified: Option<String>,   // String "true"/"false", not a bool
//!     pub is_private_email: Option<String>, // String "true"/"false"
//!     pub real_user_status: Option<u8>,     // 0 = unsupported, 1 = unknown, 2 = likely real
//!     pub transfer_sub: Option<String>,
//! }
//! ```
//!
//! ---
//!
//! ## Discord (`feature = "discord"`)
//!
//! ### `DiscordIdTokenClaims`
//! User object returned via the Discord OAuth2 `identify` scope.
//!
//! ```rust,ignore
//! pub struct DiscordIdTokenClaims {
//!     pub id: String,                       // Snowflake ID to be used as primary key
//!     pub username: String,
//!     pub discriminator: String,            // "0" for Pomelo users, "1234" for legacy
//!     pub global_name: Option<String>,      // Display name
//!     pub avatar: Option<String>,           // Avatar hash
//!     pub email: Option<String>,            // Requires `email` scope
//!     pub verified: Option<bool>,           // Requires `email` scope
//!     pub locale: Option<String>,
//!     pub mfa_enabled: Option<bool>,
//!     pub flags: Option<u64>,
//!     pub public_flags: Option<u64>,
//!     pub premium_type: Option<u8>,         // 0=None, 1=Classic, 2=Nitro, 3=Basic
//!     pub accent_color: Option<u32>,
//!     pub banner: Option<String>,
//!     pub bot: Option<bool>,
//!     pub system: Option<bool>,
//! }
//! ```
//!
//! ---
//!
//! ## LinkedIn (`feature = "linkedin"`)
//!
//! ### `LinkedInIdTokenClaims`
//! OIDC ID token / userinfo response from `GET https://api.linkedin.com/v2/userinfo`.
//!
//! ```rust,ignore
//! pub struct LinkedInIdTokenClaims {
//!     pub iss: String,    // "https://www.linkedin.com"
//!     pub aud: String,    // Your Client ID
//!     pub iat: u64,
//!     pub exp: u64,
//!     pub sub: String,                      // Stable identifier to be used as primary key
//!     pub name: Option<String>,             // Requires `profile` scope
//!     pub given_name: Option<String>,       // Requires `profile` scope
//!     pub family_name: Option<String>,      // Requires `profile` scope
//!     pub picture: Option<String>,          // Requires `profile` scope
//!     pub locale: Option<String>,           // Requires `profile` scope
//!     pub email: Option<String>,            // Requires `email` scope
//!     pub email_verified: Option<bool>,     // Requires `email` scope
//! }
//! ```
//!
//! ---
//!
//! ## Microsoft (`feature = "microsoft"`)
//!
//! ### `MicrosoftIdTokenClaims`
//! Decoded ID token from Azure AD / Entra ID (`POST https://login.microsoftonline.com/{tenant}/oauth2/v2.0/token`).
//!
//! ```rust,ignore
//! pub struct MicrosoftIdTokenClaims {
//!     pub iss: String,                      // Issuer URI including tenant
//!     pub aud: String,                      // Your Client ID
//!     pub iat: u64,
//!     pub exp: u64,
//!     pub nbf: u64,                         // Not-before
//!     pub sub: String,                      // Pairwise identifier (unique per app)
//!     pub oid: Option<String>,              // Object ID, stable across apps in the same tenant
//!     pub tid: Option<String>,              // Tenant ID
//!     pub name: Option<String>,
//!     pub given_name: Option<String>,
//!     pub family_name: Option<String>,
//!     pub email: Option<String>,
//!     pub preferred_username: Option<String>,
//!     pub upn: Option<String>,
//!     pub picture: Option<String>,
//!     pub nonce: Option<String>,
//!     pub acr: Option<String>,
//!     pub amr: Option<Vec<String>>,
//!     pub auth_time: Option<u64>,
//!     pub at_hash: Option<String>,
//!     pub c_hash: Option<String>,
//!     pub roles: Option<Vec<String>>,
//!     pub groups: Option<Vec<String>>,
//! }
//! ```

#[cfg(any(feature = "apple"))]
pub mod apple;
#[cfg(any(feature = "discord"))]
pub mod discord;
#[cfg(any(feature = "facebook"))]
pub mod facebook;
#[cfg(any(feature = "github"))]
pub mod github;
#[cfg(any(feature = "google"))]
pub mod google;
#[cfg(any(feature = "linkedin"))]
pub mod linkedin;
#[cfg(any(feature = "microsoft"))]
pub mod microsoft;
