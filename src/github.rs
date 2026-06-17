use serde::Deserialize;

/// # GitHub User Payload
/// Response from `GET /user` (authenticated) or `GET /users/{username}`.
/// Requires the `user` scope for private profile fields; public fields are available without authentication.
/// Reference: <https://docs.github.com/en/rest/users/users>
#[derive(Debug, Clone, Deserialize)]
pub struct GitHubUserPayload {
    /// The unique numeric identifier for the user. Permanent and never reused.
    /// Always use this as the primary identifier — `login` can change at any time.
    pub id: u64,
    /// The alphanumeric handle (e.g. `"octocat"`). Can be changed by the user; never use as a key.
    pub login: String,
    /// The GraphQL node ID.
    pub node_id: String,
    /// URL of the user's avatar image.
    pub avatar_url: String,
    /// Legacy Gravatar ID. Empty string if not set.
    pub gravatar_id: Option<String>,
    /// GitHub API URL for this user (`https://api.github.com/users/{login}`).
    pub url: String,
    /// Profile page URL (`https://github.com/{login}`).
    pub html_url: String,
    /// API URL for the user's followers list.
    pub followers_url: String,
    /// API URL template for users this user is following.
    pub following_url: String,
    /// API URL template for this user's gists.
    pub gists_url: String,
    /// API URL template for repositories this user has starred.
    pub starred_url: String,
    /// API URL for this user's subscriptions.
    pub subscriptions_url: String,
    /// API URL for this user's organisations.
    pub organizations_url: String,
    /// API URL for this user's repositories.
    pub repos_url: String,
    /// API URL template for this user's events.
    pub events_url: String,
    /// API URL for events received by this user.
    pub received_events_url: String,
    /// Account type — typically `"User"` or `"Organization"`.
    #[serde(rename = "type")]
    pub account_type: String,
    /// Whether the user is a GitHub site administrator.
    pub site_admin: bool,
    /// The user's display name. `None` if not set on their profile.
    pub name: Option<String>,
    /// The user's company name. `None` if not set.
    pub company: Option<String>,
    /// The user's blog or website URL. `None` if not set.
    pub blog: Option<String>,
    /// The user's location. `None` if not set.
    pub location: Option<String>,
    /// The user's publicly visible email address.
    /// `None` if the user has not set a public email.
    ///
    /// **Warning**: This is the public profile email only. Use the [Emails API](https://docs.github.com/en/rest/users/emails)
    /// with the `user:email` scope to reliably retrieve a verified primary address.
    pub email: Option<String>,
    /// Whether the user is available for hire. `None` if not set.
    pub hireable: Option<bool>,
    /// The user's profile bio. `None` if not set.
    pub bio: Option<String>,
    /// The user's Twitter / X username. `None` if not set.
    pub twitter_username: Option<String>,
    /// Number of public repositories owned by the user.
    pub public_repos: u32,
    /// Number of public gists owned by the user.
    pub public_gists: u32,
    /// Number of users following this user.
    pub followers: u32,
    /// Number of users this user is following.
    pub following: u32,
    /// ISO 8601 timestamp of when the account was created.
    pub created_at: String,
    /// ISO 8601 timestamp of the last profile update.
    pub updated_at: String,
    // --- Fields below require the `user` OAuth scope (authenticated user only) ---
    /// Number of private gists. Only present for the authenticated user.
    pub private_gists: Option<u32>,
    /// Total number of private repositories (includes owned + collaborated). Only present for the authenticated user.
    pub total_private_repos: Option<u32>,
    /// Number of private repositories owned by the user. Only present for the authenticated user.
    pub owned_private_repos: Option<u32>,
    /// Disk usage in kilobytes. Only present for the authenticated user.
    pub disk_usage: Option<u64>,
    /// Number of collaborators across private repositories. Only present for the authenticated user.
    pub collaborators: Option<u32>,
    /// Whether two-factor authentication is enabled. Only present for the authenticated user.
    pub two_factor_authentication: Option<bool>,
    /// The user's GitHub plan. Only present for the authenticated user.
    pub plan: Option<GitHubPlan>,
}

/// GitHub subscription plan associated with a user account.
#[derive(Debug, Clone, Deserialize)]
pub struct GitHubPlan {
    /// Plan name (e.g. `"free"`, `"pro"`, `"team"`, `"enterprise"`).
    pub name: String,
    /// Allocated disk space in bytes.
    pub space: u64,
    /// Number of private repositories allowed under this plan.
    pub private_repos: u32,
    /// Number of collaborators allowed under this plan.
    pub collaborators: u32,
}

/// # GitHub Email Item
/// A single entry from `GET /user/emails`.
/// Iterate the returned `Vec<GitHubEmailItem>` and select the entry where
/// `primary == true && verified == true` for the canonical address.
/// Reference: <https://docs.github.com/en/rest/users/emails>
#[derive(Debug, Clone, Deserialize)]
pub struct GitHubEmailItem {
    /// The email address.
    pub email: String,
    /// Whether this is the user's primary email address.
    pub primary: bool,
    /// Whether GitHub has verified ownership of this address.
    pub verified: bool,
    /// Visibility setting — `"public"`, `"private"`, or `None`.
    pub visibility: Option<String>,
}
