/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EditBranchProtectionOption : EditBranchProtectionOption options for editing a branch protection



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EditBranchProtectionOption {
    #[serde(rename = "approvals_whitelist_teams", skip_serializing_if = "Option::is_none")]
    pub approvals_whitelist_teams: Option<Vec<String>>,
    #[serde(rename = "approvals_whitelist_username", skip_serializing_if = "Option::is_none")]
    pub approvals_whitelist_username: Option<Vec<String>>,
    #[serde(rename = "block_on_official_review_requests", skip_serializing_if = "Option::is_none")]
    pub block_on_official_review_requests: Option<bool>,
    #[serde(rename = "block_on_outdated_branch", skip_serializing_if = "Option::is_none")]
    pub block_on_outdated_branch: Option<bool>,
    #[serde(rename = "block_on_rejected_reviews", skip_serializing_if = "Option::is_none")]
    pub block_on_rejected_reviews: Option<bool>,
    #[serde(rename = "dismiss_stale_approvals", skip_serializing_if = "Option::is_none")]
    pub dismiss_stale_approvals: Option<bool>,
    #[serde(rename = "enable_approvals_whitelist", skip_serializing_if = "Option::is_none")]
    pub enable_approvals_whitelist: Option<bool>,
    #[serde(rename = "enable_merge_whitelist", skip_serializing_if = "Option::is_none")]
    pub enable_merge_whitelist: Option<bool>,
    #[serde(rename = "enable_push", skip_serializing_if = "Option::is_none")]
    pub enable_push: Option<bool>,
    #[serde(rename = "enable_push_whitelist", skip_serializing_if = "Option::is_none")]
    pub enable_push_whitelist: Option<bool>,
    #[serde(rename = "enable_status_check", skip_serializing_if = "Option::is_none")]
    pub enable_status_check: Option<bool>,
    #[serde(rename = "merge_whitelist_teams", skip_serializing_if = "Option::is_none")]
    pub merge_whitelist_teams: Option<Vec<String>>,
    #[serde(rename = "merge_whitelist_usernames", skip_serializing_if = "Option::is_none")]
    pub merge_whitelist_usernames: Option<Vec<String>>,
    #[serde(rename = "protected_file_patterns", skip_serializing_if = "Option::is_none")]
    pub protected_file_patterns: Option<String>,
    #[serde(rename = "push_whitelist_deploy_keys", skip_serializing_if = "Option::is_none")]
    pub push_whitelist_deploy_keys: Option<bool>,
    #[serde(rename = "push_whitelist_teams", skip_serializing_if = "Option::is_none")]
    pub push_whitelist_teams: Option<Vec<String>>,
    #[serde(rename = "push_whitelist_usernames", skip_serializing_if = "Option::is_none")]
    pub push_whitelist_usernames: Option<Vec<String>>,
    #[serde(rename = "require_signed_commits", skip_serializing_if = "Option::is_none")]
    pub require_signed_commits: Option<bool>,
    #[serde(rename = "required_approvals", skip_serializing_if = "Option::is_none")]
    pub required_approvals: Option<i64>,
    #[serde(rename = "status_check_contexts", skip_serializing_if = "Option::is_none")]
    pub status_check_contexts: Option<Vec<String>>,
    #[serde(rename = "unprotected_file_patterns", skip_serializing_if = "Option::is_none")]
    pub unprotected_file_patterns: Option<String>,
}

impl EditBranchProtectionOption {
    /// EditBranchProtectionOption options for editing a branch protection
    pub fn new() -> EditBranchProtectionOption {
        EditBranchProtectionOption {
            approvals_whitelist_teams: None,
            approvals_whitelist_username: None,
            block_on_official_review_requests: None,
            block_on_outdated_branch: None,
            block_on_rejected_reviews: None,
            dismiss_stale_approvals: None,
            enable_approvals_whitelist: None,
            enable_merge_whitelist: None,
            enable_push: None,
            enable_push_whitelist: None,
            enable_status_check: None,
            merge_whitelist_teams: None,
            merge_whitelist_usernames: None,
            protected_file_patterns: None,
            push_whitelist_deploy_keys: None,
            push_whitelist_teams: None,
            push_whitelist_usernames: None,
            require_signed_commits: None,
            required_approvals: None,
            status_check_contexts: None,
            unprotected_file_patterns: None,
        }
    }
}


