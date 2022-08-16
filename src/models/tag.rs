/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 *
 * Generated by: https://openapi-generator.tech
 */

/// Tag : Tag represents a repository tag

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "commit", skip_serializing_if = "Option::is_none")]
    pub commit: Option<Box<crate::models::CommitMeta>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "tarball_url", skip_serializing_if = "Option::is_none")]
    pub tarball_url: Option<String>,
    #[serde(rename = "zipball_url", skip_serializing_if = "Option::is_none")]
    pub zipball_url: Option<String>,
}

impl Tag {
    /// Tag represents a repository tag
    pub fn new() -> Tag {
        Tag {
            commit: None,
            id: None,
            message: None,
            name: None,
            tarball_url: None,
            zipball_url: None,
        }
    }
}
