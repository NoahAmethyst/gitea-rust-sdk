/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CommitDateOptions : CommitDateOptions store dates for GIT_AUTHOR_DATE and GIT_COMMITTER_DATE



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommitDateOptions {
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "committer", skip_serializing_if = "Option::is_none")]
    pub committer: Option<String>,
}

impl CommitDateOptions {
    /// CommitDateOptions store dates for GIT_AUTHOR_DATE and GIT_COMMITTER_DATE
    pub fn new() -> CommitDateOptions {
        CommitDateOptions {
            author: None,
            committer: None,
        }
    }
}


