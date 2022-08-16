/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 *
 * Generated by: https://openapi-generator.tech
 */

/// MarkdownOption : MarkdownOption markdown options

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MarkdownOption {
    /// Context to render  in: body
    #[serde(rename = "Context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Mode to render  in: body
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Text markdown to render  in: body
    #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Is it a wiki page ?  in: body
    #[serde(rename = "Wiki", skip_serializing_if = "Option::is_none")]
    pub wiki: Option<bool>,
}

impl MarkdownOption {
    /// MarkdownOption markdown options
    pub fn new() -> MarkdownOption {
        MarkdownOption {
            context: None,
            mode: None,
            text: None,
            wiki: None,
        }
    }
}
