/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 *
 * Generated by: https://openapi-generator.tech
 */

/// MigrateRepoOptions : MigrateRepoOptions options for migrating repository's this is used to interact with api v1

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MigrateRepoOptions {
    #[serde(rename = "auth_password", skip_serializing_if = "Option::is_none")]
    pub auth_password: Option<String>,
    #[serde(rename = "auth_token", skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(rename = "auth_username", skip_serializing_if = "Option::is_none")]
    pub auth_username: Option<String>,
    #[serde(rename = "clone_addr")]
    pub clone_addr: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<bool>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<bool>,
    #[serde(rename = "lfs", skip_serializing_if = "Option::is_none")]
    pub lfs: Option<bool>,
    #[serde(rename = "lfs_endpoint", skip_serializing_if = "Option::is_none")]
    pub lfs_endpoint: Option<String>,
    #[serde(rename = "milestones", skip_serializing_if = "Option::is_none")]
    pub milestones: Option<bool>,
    #[serde(rename = "mirror", skip_serializing_if = "Option::is_none")]
    pub mirror: Option<bool>,
    #[serde(rename = "mirror_interval", skip_serializing_if = "Option::is_none")]
    pub mirror_interval: Option<String>,
    #[serde(rename = "private", skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(rename = "pull_requests", skip_serializing_if = "Option::is_none")]
    pub pull_requests: Option<bool>,
    #[serde(rename = "releases", skip_serializing_if = "Option::is_none")]
    pub releases: Option<bool>,
    #[serde(rename = "repo_name")]
    pub repo_name: String,
    /// Name of User or Organisation who will own Repo after migration
    #[serde(rename = "repo_owner", skip_serializing_if = "Option::is_none")]
    pub repo_owner: Option<String>,
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
    /// deprecated (only for backwards compatibility)
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<i64>,
    #[serde(rename = "wiki", skip_serializing_if = "Option::is_none")]
    pub wiki: Option<bool>,
}

impl MigrateRepoOptions {
    /// MigrateRepoOptions options for migrating repository's this is used to interact with api v1
    pub fn new(clone_addr: String, repo_name: String) -> MigrateRepoOptions {
        MigrateRepoOptions {
            auth_password: None,
            auth_token: None,
            auth_username: None,
            clone_addr,
            description: None,
            issues: None,
            labels: None,
            lfs: None,
            lfs_endpoint: None,
            milestones: None,
            mirror: None,
            mirror_interval: None,
            private: None,
            pull_requests: None,
            releases: None,
            repo_name,
            repo_owner: None,
            service: None,
            uid: None,
            wiki: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Service {
    #[serde(rename = "git")]
    Git,
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "gitea")]
    Gitea,
    #[serde(rename = "gitlab")]
    Gitlab,
}

impl Default for Service {
    fn default() -> Service {
        Self::Git
    }
}
