/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 *
 * Generated by: https://openapi-generator.tech
 */

/// NodeInfoUsageUsers : NodeInfoUsageUsers contains statistics about the users of this server

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NodeInfoUsageUsers {
    #[serde(rename = "activeHalfyear", skip_serializing_if = "Option::is_none")]
    pub active_halfyear: Option<i64>,
    #[serde(rename = "activeMonth", skip_serializing_if = "Option::is_none")]
    pub active_month: Option<i64>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl NodeInfoUsageUsers {
    /// NodeInfoUsageUsers contains statistics about the users of this server
    pub fn new() -> NodeInfoUsageUsers {
        NodeInfoUsageUsers {
            active_halfyear: None,
            active_month: None,
            total: None,
        }
    }
}
