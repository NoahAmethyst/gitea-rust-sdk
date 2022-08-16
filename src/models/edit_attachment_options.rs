/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 *
 * Generated by: https://openapi-generator.tech
 */

/// EditAttachmentOptions : EditAttachmentOptions options for editing attachments

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EditAttachmentOptions {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl EditAttachmentOptions {
    /// EditAttachmentOptions options for editing attachments
    pub fn new() -> EditAttachmentOptions {
        EditAttachmentOptions { name: None }
    }
}
