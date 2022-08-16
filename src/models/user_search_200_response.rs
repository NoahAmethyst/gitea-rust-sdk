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
pub struct UserSearch200Response {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::User>>,
    #[serde(rename = "ok", skip_serializing_if = "Option::is_none")]
    pub ok: Option<bool>,
}

impl UserSearch200Response {
    pub fn new() -> UserSearch200Response {
        UserSearch200Response {
            data: None,
            ok: None,
        }
    }
}

