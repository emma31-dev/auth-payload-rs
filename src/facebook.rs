use serde::Deserialize;

/// # Facebook User Payload
/// Response from `GET /{user-id}` on the Facebook Graph API.
/// Fields beyond the default set must be explicitly requested and require the corresponding permission.
/// Reference: <https://developers.facebook.com/docs/graph-api/reference/user/>
///
/// **Default `public_profile` fields** (always available with a user token):
/// `id`, `name`, `first_name`, `middle_name`, `last_name`, `short_name`, `name_format`, `picture`.
///
/// **Additional fields** require the listed permission to be granted by the user.
#[derive(Debug, Clone, Deserialize)]
pub struct FacebookUserPayload {
    /// App-Scoped User ID (ASID). Unique to your App ID not a global Facebook ID.
    /// This value is stable for a given user + app pair. Always use this as the primary identifier.
    pub id: String,
    /// The user's full name. Part of `public_profile`.
    pub name: String,
    /// The user's first name. Part of `public_profile`.
    pub first_name: Option<String>,
    /// The user's middle name. Part of `public_profile`. `None` if not set.
    pub middle_name: Option<String>,
    /// The user's last name. Part of `public_profile`.
    pub last_name: Option<String>,
    /// A shorter version of the user's name suitable for display. Part of `public_profile`.
    pub short_name: Option<String>,
    /// The user's name formatted to correctly handle Chinese, Japanese, or Korean ordering.
    /// Part of `public_profile`.
    pub name_format: Option<String>,
    /// The user's profile picture. Part of `public_profile`.
    /// Must be requested explicitly via `?fields=picture`.
    pub picture: Option<FacebookPictureContainer>,
    /// The user's primary email address.
    /// `None` if no valid email is available or if the user signed up with a phone number.
    /// Requires the `email` permission.
    ///
    /// **Warning**: Do not use email as a primary identifier; always prefer `id`.
    pub email: Option<String>,
    /// The user's birthday as a fixed-format string.
    /// Format varies by user privacy settings: full `MM/DD/YYYY`, year only `YYYY`, or day+month `MM/DD`.
    /// Requires the `user_birthday` permission.
    pub birthday: Option<String>,
    /// The user's gender as selected on their profile (`"male"` or `"female"`).
    /// Omitted if the user has set a custom neutral pronoun.
    /// Requires the `user_gender` permission.
    pub gender: Option<String>,
    /// The user's current location (a Page object represented as an ID + name pair).
    /// Requires the `user_location` permission.
    pub location: Option<FacebookPage>,
    /// The user's hometown (a Page object represented as an ID + name pair).
    /// Requires the `user_hometown` permission.
    pub hometown: Option<FacebookPage>,
    /// A link to the person's Timeline profile.
    /// Only resolves for a viewer who is a friend of this user and is logged in.
    /// Part of `public_profile`.
    pub link: Option<String>,
    /// The user's age expressed as a range (e.g. `{ "min": 18, "max": 20 }`).
    /// Part of `public_profile`.
    pub age_range: Option<FacebookAgeRange>,
}

/// A minimal Facebook Page reference returned for location/hometown fields.
#[derive(Debug, Clone, Deserialize)]
pub struct FacebookPage {
    /// The Page's unique ID.
    pub id: String,
    /// The Page's name (e.g. city name, hometown string).
    pub name: Option<String>,
}

/// Age range returned by the Graph API for the `age_range` field.
#[derive(Debug, Clone, Deserialize)]
pub struct FacebookAgeRange {
    /// Minimum age (inclusive).
    pub min: Option<u8>,
    /// Maximum age (inclusive). Absent when there is no upper bound.
    pub max: Option<u8>,
}

/// Wrapper object returned when requesting the `picture` field.
#[derive(Debug, Clone, Deserialize)]
pub struct FacebookPictureContainer {
    pub data: FacebookPictureData,
}

/// The actual picture metadata nested inside [`FacebookPictureContainer`].
#[derive(Debug, Clone, Deserialize)]
pub struct FacebookPictureData {
    /// Direct URL to the profile picture image.
    pub url: String,
    /// `true` if the user has no custom avatar and Facebook returned a generic silhouette.
    pub is_silhouette: bool,
    /// Width of the image in pixels. Present when a specific size was requested.
    pub width: Option<u32>,
    /// Height of the image in pixels. Present when a specific size was requested.
    pub height: Option<u32>,
}
