/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateIssueOption : CreateIssueOption options to create one issue

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateIssueOption {
    /// deprecated
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(rename = "assignees", skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<String>>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "closed", skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(rename = "due_date", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    /// list of label ids
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<i64>>,
    /// milestone id
    #[serde(rename = "milestone", skip_serializing_if = "Option::is_none")]
    pub milestone: Option<i64>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub _ref: Option<String>,
    #[serde(rename = "title")]
    pub title: String,
}

impl CreateIssueOption {
    /// CreateIssueOption options to create one issue
    pub fn new(title: String) -> CreateIssueOption {
        CreateIssueOption {
            assignee: None,
            assignees: None,
            body: None,
            closed: None,
            due_date: None,
            labels: None,
            milestone: None,
            _ref: None,
            title,
        }
    }
}
