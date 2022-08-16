/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GitBlobResponse : GitBlobResponse represents a git blob



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GitBlobResponse {
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "encoding", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(rename = "sha", skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl GitBlobResponse {
    /// GitBlobResponse represents a git blob
    pub fn new() -> GitBlobResponse {
        GitBlobResponse {
            content: None,
            encoding: None,
            sha: None,
            size: None,
            url: None,
        }
    }
}


