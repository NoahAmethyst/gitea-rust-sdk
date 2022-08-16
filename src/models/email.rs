/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Email : Email an email address belonging to a user



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Email {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(rename = "verified", skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

impl Email {
    /// Email an email address belonging to a user
    pub fn new() -> Email {
        Email {
            email: None,
            primary: None,
            verified: None,
        }
    }
}


