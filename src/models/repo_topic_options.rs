/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 *
 * Generated by: https://openapi-generator.tech
 */

/// RepoTopicOptions : RepoTopicOptions a collection of repo topic names

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RepoTopicOptions {
    /// list of topic names
    #[serde(rename = "topics", skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
}

impl RepoTopicOptions {
    /// RepoTopicOptions a collection of repo topic names
    pub fn new() -> RepoTopicOptions {
        RepoTopicOptions { topics: None }
    }
}
