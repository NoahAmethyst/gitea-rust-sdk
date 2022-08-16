/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 * 
 * Generated by: https://openapi-generator.tech
 */

/// StopWatch : StopWatch represent a running stopwatch



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StopWatch {
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "issue_index", skip_serializing_if = "Option::is_none")]
    pub issue_index: Option<i64>,
    #[serde(rename = "issue_title", skip_serializing_if = "Option::is_none")]
    pub issue_title: Option<String>,
    #[serde(rename = "repo_name", skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    #[serde(rename = "repo_owner_name", skip_serializing_if = "Option::is_none")]
    pub repo_owner_name: Option<String>,
    #[serde(rename = "seconds", skip_serializing_if = "Option::is_none")]
    pub seconds: Option<i64>,
}

impl StopWatch {
    /// StopWatch represent a running stopwatch
    pub fn new() -> StopWatch {
        StopWatch {
            created: None,
            duration: None,
            issue_index: None,
            issue_title: None,
            repo_name: None,
            repo_owner_name: None,
            seconds: None,
        }
    }
}


