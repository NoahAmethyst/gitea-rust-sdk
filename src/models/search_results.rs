/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SearchResults : SearchResults results of a successful search



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SearchResults {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::Repository>>,
    #[serde(rename = "ok", skip_serializing_if = "Option::is_none")]
    pub ok: Option<bool>,
}

impl SearchResults {
    /// SearchResults results of a successful search
    pub fn new() -> SearchResults {
        SearchResults {
            data: None,
            ok: None,
        }
    }
}

