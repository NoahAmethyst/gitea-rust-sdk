/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Note : Note contains information related to a git note



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Note {
    #[serde(rename = "commit", skip_serializing_if = "Option::is_none")]
    pub commit: Option<Box<crate::models::Commit>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl Note {
    /// Note contains information related to a git note
    pub fn new() -> Note {
        Note {
            commit: None,
            message: None,
        }
    }
}


