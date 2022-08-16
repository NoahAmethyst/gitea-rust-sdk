/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EditIssueCommentOption : EditIssueCommentOption options for editing a comment



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EditIssueCommentOption {
    #[serde(rename = "body")]
    pub body: String,
}

impl EditIssueCommentOption {
    /// EditIssueCommentOption options for editing a comment
    pub fn new(body: String) -> EditIssueCommentOption {
        EditIssueCommentOption {
            body,
        }
    }
}


