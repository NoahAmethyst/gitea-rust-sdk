/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PrBranchInfo : PRBranchInfo information about a branch



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PrBranchInfo {
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub _ref: Option<String>,
    #[serde(rename = "repo", skip_serializing_if = "Option::is_none")]
    pub repo: Option<Box<crate::models::Repository>>,
    #[serde(rename = "repo_id", skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<i64>,
    #[serde(rename = "sha", skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
}

impl PrBranchInfo {
    /// PRBranchInfo information about a branch
    pub fn new() -> PrBranchInfo {
        PrBranchInfo {
            label: None,
            _ref: None,
            repo: None,
            repo_id: None,
            sha: None,
        }
    }
}

