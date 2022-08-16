/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PullRequestMeta : PullRequestMeta PR info if an issue is a PR



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PullRequestMeta {
    #[serde(rename = "merged", skip_serializing_if = "Option::is_none")]
    pub merged: Option<bool>,
    #[serde(rename = "merged_at", skip_serializing_if = "Option::is_none")]
    pub merged_at: Option<String>,
}

impl PullRequestMeta {
    /// PullRequestMeta PR info if an issue is a PR
    pub fn new() -> PullRequestMeta {
        PullRequestMeta {
            merged: None,
            merged_at: None,
        }
    }
}


