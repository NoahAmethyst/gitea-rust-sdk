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
pub struct TeamSearch200Response {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::Team>>,
    #[serde(rename = "ok", skip_serializing_if = "Option::is_none")]
    pub ok: Option<bool>,
}

impl TeamSearch200Response {
    pub fn new() -> TeamSearch200Response {
        TeamSearch200Response {
            data: None,
            ok: None,
        }
    }
}
