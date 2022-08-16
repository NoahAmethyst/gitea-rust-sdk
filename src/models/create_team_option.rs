/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateTeamOption : CreateTeamOption options for creating a team

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateTeamOption {
    #[serde(
        rename = "can_create_org_repo",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_create_org_repo: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "includes_all_repositories",
        skip_serializing_if = "Option::is_none"
    )]
    pub includes_all_repositories: Option<bool>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<Permission>,
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<Vec<String>>,
    #[serde(rename = "units_map", skip_serializing_if = "Option::is_none")]
    pub units_map: Option<::std::collections::HashMap<String, String>>,
}

impl CreateTeamOption {
    /// CreateTeamOption options for creating a team
    pub fn new(name: String) -> CreateTeamOption {
        CreateTeamOption {
            can_create_org_repo: None,
            description: None,
            includes_all_repositories: None,
            name,
            permission: None,
            units: None,
            units_map: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Permission {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "admin")]
    Admin,
}

impl Default for Permission {
    fn default() -> Permission {
        Self::Read
    }
}
