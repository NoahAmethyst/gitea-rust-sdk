/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateLabelOption : CreateLabelOption options for creating a label



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateLabelOption {
    #[serde(rename = "color")]
    pub color: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateLabelOption {
    /// CreateLabelOption options for creating a label
    pub fn new(color: String, name: String) -> CreateLabelOption {
        CreateLabelOption {
            color,
            description: None,
            name,
        }
    }
}

