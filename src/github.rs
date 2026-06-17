use serde::Deserialize;

/// # GitHub User Payload
/// Response from `GET /user` (authenticated) or `GET /users/{username}`.
/// Requires the `user` scope for private profile fields; public fields are available without authentication.
/// Reference: <https://docs.github.com/en/rest/users/users>
#[derive(Debug, Clone, Deserialize)]
pub struct GitHubUserPayload {
    /// The unique numeric identifier for the user. Permanent and never reused.
    /// Always use this as the primary identifier, `login`, can change at any time.
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
    /// Account type typically `"User"` or `"Organization"`.
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
    /// Visibility setting: `Some("public")`, `Some("private")`, or `None`.
    pub visibility: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_payload_full() {
        let json = r#"{
            "id": 1,
            "login": "octocat",
            "node_id": "MDQ6VXNlcjE=",
            "avatar_url": "https://github.com/images/error/octocat_happy.gif",
            "gravatar_id": "",
            "url": "https://api.github.com/users/octocat",
            "html_url": "https://github.com/octocat",
            "followers_url": "https://api.github.com/users/octocat/followers",
            "following_url": "https://api.github.com/users/octocat/following{/other_user}",
            "gists_url": "https://api.github.com/users/octocat/gists{/gist_id}",
            "starred_url": "https://api.github.com/users/octocat/starred{/owner}{/repo}",
            "subscriptions_url": "https://api.github.com/users/octocat/subscriptions",
            "organizations_url": "https://api.github.com/users/octocat/orgs",
            "repos_url": "https://api.github.com/users/octocat/repos",
            "events_url": "https://api.github.com/users/octocat/events{/privacy}",
            "received_events_url": "https://api.github.com/users/octocat/received_events",
            "type": "User",
            "site_admin": false,
            "name": "monalisa octocat",
            "company": "GitHub",
            "blog": "https://github.com/blog",
            "location": "San Francisco",
            "email": "octocat@github.com",
            "hireable": false,
            "bio": "There once was...",
            "twitter_username": "monatheoctocat",
            "public_repos": 2,
            "public_gists": 1,
            "followers": 20,
            "following": 0,
            "created_at": "2008-01-14T04:33:35Z",
            "updated_at": "2008-01-14T04:33:35Z",
            "private_gists": 81,
            "total_private_repos": 100,
            "owned_private_repos": 100,
            "disk_usage": 10000,
            "collaborators": 8,
            "two_factor_authentication": true,
            "plan": { "name": "Medium", "space": 400, "private_repos": 20, "collaborators": 0 }
        }"#;
        let u: GitHubUserPayload = serde_json::from_str(json).unwrap();
        assert_eq!(u.id, 1);
        assert_eq!(u.login, "octocat");
        assert_eq!(u.account_type, "User");
        assert!(!u.site_admin);
        assert_eq!(u.email.as_deref(), Some("octocat@github.com"));
        assert_eq!(u.two_factor_authentication, Some(true));
        assert_eq!(u.plan.as_ref().unwrap().name, "Medium");
    }

    #[test]
    fn user_payload_minimal() {
        let json = r#"{
            "id": 2,
            "login": "ghost",
            "node_id": "abc",
            "avatar_url": "https://example.com/avatar.png",
            "url": "https://api.github.com/users/ghost",
            "html_url": "https://github.com/ghost",
            "followers_url": "https://api.github.com/users/ghost/followers",
            "following_url": "https://api.github.com/users/ghost/following",
            "gists_url": "https://api.github.com/users/ghost/gists",
            "starred_url": "https://api.github.com/users/ghost/starred",
            "subscriptions_url": "https://api.github.com/users/ghost/subscriptions",
            "organizations_url": "https://api.github.com/users/ghost/orgs",
            "repos_url": "https://api.github.com/users/ghost/repos",
            "events_url": "https://api.github.com/users/ghost/events",
            "received_events_url": "https://api.github.com/users/ghost/received_events",
            "type": "User",
            "site_admin": false,
            "public_repos": 0,
            "public_gists": 0,
            "followers": 0,
            "following": 0,
            "created_at": "2020-01-01T00:00:00Z",
            "updated_at": "2020-01-01T00:00:00Z"
        }"#;
        let u: GitHubUserPayload = serde_json::from_str(json).unwrap();
        assert_eq!(u.id, 2);
        assert!(u.email.is_none());
        assert!(u.plan.is_none());
        assert!(u.two_factor_authentication.is_none());
    }

    #[test]
    fn email_item() {
        let json = r#"[
            {"email": "primary@example.com", "primary": true, "verified": true, "visibility": "private"},
            {"email": "other@example.com", "primary": false, "verified": true, "visibility": null}
        ]"#;
        let items: Vec<GitHubEmailItem> = serde_json::from_str(json).unwrap();
        let primary = items.iter().find(|e| e.primary && e.verified).unwrap();
        assert_eq!(primary.email, "primary@example.com");
    }
}
