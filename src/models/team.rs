/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Team : Team represents a team in an organization



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Team {
    #[serde(rename = "can_create_org_repo", skip_serializing_if = "Option::is_none")]
    pub can_create_org_repo: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "includes_all_repositories", skip_serializing_if = "Option::is_none")]
    pub includes_all_repositories: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<crate::models::Organization>>,
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<Permission>,
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<Vec<String>>,
    #[serde(rename = "units_map", skip_serializing_if = "Option::is_none")]
    pub units_map: Option<::std::collections::HashMap<String, String>>,
}

impl Team {
    /// Team represents a team in an organization
    pub fn new() -> Team {
        Team {
            can_create_org_repo: None,
            description: None,
            id: None,
            includes_all_repositories: None,
            name: None,
            organization: None,
            permission: None,
            units: None,
            units_map: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Permission {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "owner")]
    Owner,
}

impl Default for Permission {
    fn default() -> Permission {
        Self::None
    }
}

