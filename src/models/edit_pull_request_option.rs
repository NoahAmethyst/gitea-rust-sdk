/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EditPullRequestOption : EditPullRequestOption options when modify pull request



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EditPullRequestOption {
    #[serde(rename = "allow_maintainer_edit", skip_serializing_if = "Option::is_none")]
    pub allow_maintainer_edit: Option<bool>,
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(rename = "assignees", skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<String>>,
    #[serde(rename = "base", skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "due_date", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<i64>>,
    #[serde(rename = "milestone", skip_serializing_if = "Option::is_none")]
    pub milestone: Option<i64>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "unset_due_date", skip_serializing_if = "Option::is_none")]
    pub unset_due_date: Option<bool>,
}

impl EditPullRequestOption {
    /// EditPullRequestOption options when modify pull request
    pub fn new() -> EditPullRequestOption {
        EditPullRequestOption {
            allow_maintainer_edit: None,
            assignee: None,
            assignees: None,
            base: None,
            body: None,
            due_date: None,
            labels: None,
            milestone: None,
            state: None,
            title: None,
            unset_due_date: None,
        }
    }
}


