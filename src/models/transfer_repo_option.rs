/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferRepoOption : TransferRepoOption options when transfer a repository's ownership



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferRepoOption {
    #[serde(rename = "new_owner")]
    pub new_owner: String,
    /// ID of the team or teams to add to the repository. Teams can only be added to organization-owned repositories.
    #[serde(rename = "team_ids", skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<Vec<i64>>,
}

impl TransferRepoOption {
    /// TransferRepoOption options when transfer a repository's ownership
    pub fn new(new_owner: String) -> TransferRepoOption {
        TransferRepoOption {
            new_owner,
            team_ids: None,
        }
    }
}

