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
//! use auth-payload-rs::google::GoogleIdTokenClaims;
//! 
//! fn googleCallback(Json(payload): Json<GoogleIdTokenClaims>, /* Other arguments */) -> Result<(), ()> {
//!     // Do stuff with payload
//! }
//! ```
//! 2. For Google's UserInfo Endpoint
//! ```rust
//! // Other imports
//! use auth-payload-rs::google::GoogleUserInfoResponse;
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
//!     pub email_verified: Option<bool>, // Does it need explanation?
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
//!     pub email_verified: Option<bool>, // Whether the user's email address has been verified.
//!     pub hud: Option<String>, // The hosted domain associated with the user's Google Workspace or Cloud organization.
//! }
//! ```
//! **Warn**: Other payloads are yet to be formalized because of lack or readily available information on them. Be rest assured they will be ready by version 0.2. Thank you :)
//! 

pub mod google;
#[cfg(any(feature = "github"))]
pub mod github;
#[cfg(any(feature = "facebook"))]
pub mod facebook;
#[cfg(any(feature = "apple"))]
pub mod apple;
// #[cfg(any(feature = "discord"))]
pub mod discord;
#[cfg(any(feature = "linkedin"))]
pub mod linkedin;
#[cfg(any(feature = "microsoft"))]
pub mod microsoft;