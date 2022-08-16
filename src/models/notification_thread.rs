/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NotificationThread : NotificationThread expose Notification on API



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NotificationThread {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "pinned", skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<crate::models::Repository>>,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<Box<crate::models::NotificationSubject>>,
    #[serde(rename = "unread", skip_serializing_if = "Option::is_none")]
    pub unread: Option<bool>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl NotificationThread {
    /// NotificationThread expose Notification on API
    pub fn new() -> NotificationThread {
        NotificationThread {
            id: None,
            pinned: None,
            repository: None,
            subject: None,
            unread: None,
            updated_at: None,
            url: None,
        }
    }
}


