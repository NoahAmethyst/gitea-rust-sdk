/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NodeInfoUsage : NodeInfoUsage contains usage statistics for this server



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NodeInfoUsage {
    #[serde(rename = "localComments", skip_serializing_if = "Option::is_none")]
    pub local_comments: Option<i64>,
    #[serde(rename = "localPosts", skip_serializing_if = "Option::is_none")]
    pub local_posts: Option<i64>,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Box<crate::models::NodeInfoUsageUsers>>,
}

impl NodeInfoUsage {
    /// NodeInfoUsage contains usage statistics for this server
    pub fn new() -> NodeInfoUsage {
        NodeInfoUsage {
            local_comments: None,
            local_posts: None,
            users: None,
        }
    }
}


