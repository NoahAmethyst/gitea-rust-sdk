/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 *
 * Generated by: https://openapi-generator.tech
 */

/// EditReleaseOption : EditReleaseOption options when editing a release

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EditReleaseOption {
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "draft", skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "prerelease", skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<bool>,
    #[serde(rename = "tag_name", skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    #[serde(rename = "target_commitish", skip_serializing_if = "Option::is_none")]
    pub target_commitish: Option<String>,
}

impl EditReleaseOption {
    /// EditReleaseOption options when editing a release
    pub fn new() -> EditReleaseOption {
        EditReleaseOption {
            body: None,
            draft: None,
            name: None,
            prerelease: None,
            tag_name: None,
            target_commitish: None,
        }
    }
}
