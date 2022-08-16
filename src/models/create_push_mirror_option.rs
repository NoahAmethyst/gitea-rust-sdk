/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreatePushMirrorOption {
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(rename = "remote_address", skip_serializing_if = "Option::is_none")]
    pub remote_address: Option<String>,
    #[serde(rename = "remote_password", skip_serializing_if = "Option::is_none")]
    pub remote_password: Option<String>,
    #[serde(rename = "remote_username", skip_serializing_if = "Option::is_none")]
    pub remote_username: Option<String>,
}

impl CreatePushMirrorOption {
    pub fn new() -> CreatePushMirrorOption {
        CreatePushMirrorOption {
            interval: None,
            remote_address: None,
            remote_password: None,
            remote_username: None,
        }
    }
}
