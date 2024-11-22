/*
 * GitHub's official OpenAPI spec + Octokit extension
 *
 * OpenAPI specs from https://github.com/github/rest-api-description with the 'x-octokit' extension required by the Octokit SDKs
 *
 * The version of the OpenAPI document: 16.6.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// WebhookCodeScanningAlertReopenedByUserAlert : The code scanning alert involved in the event.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookCodeScanningAlertReopenedByUserAlert {
    /// The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ.`
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The time that the alert was dismissed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "dismissed_at", deserialize_with = "Option::deserialize")]
    pub dismissed_at: Option<serde_json::Value>,
    #[serde(rename = "dismissed_by", deserialize_with = "Option::deserialize")]
    pub dismissed_by: Option<serde_json::Value>,
    /// The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`.
    #[serde(rename = "dismissed_reason", deserialize_with = "Option::deserialize")]
    pub dismissed_reason: Option<serde_json::Value>,
    /// The GitHub URL of the alert resource.
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "most_recent_instance", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub most_recent_instance: Option<Option<Box<models::AlertInstance>>>,
    /// The code scanning alert number.
    #[serde(rename = "number")]
    pub number: i32,
    #[serde(rename = "rule")]
    pub rule: Box<models::WebhookCodeScanningAlertAppearedInBranchAlertRule>,
    /// State of a code scanning alert.
    #[serde(rename = "state")]
    pub state: State,
    #[serde(rename = "tool")]
    pub tool: Box<models::WebhookCodeScanningAlertAppearedInBranchAlertTool>,
    #[serde(rename = "url")]
    pub url: String,
}

impl WebhookCodeScanningAlertReopenedByUserAlert {
    /// The code scanning alert involved in the event.
    pub fn new(created_at: String, dismissed_at: Option<serde_json::Value>, dismissed_by: Option<serde_json::Value>, dismissed_reason: Option<serde_json::Value>, html_url: String, number: i32, rule: models::WebhookCodeScanningAlertAppearedInBranchAlertRule, state: State, tool: models::WebhookCodeScanningAlertAppearedInBranchAlertTool, url: String) -> WebhookCodeScanningAlertReopenedByUserAlert {
        WebhookCodeScanningAlertReopenedByUserAlert {
            created_at,
            dismissed_at,
            dismissed_by,
            dismissed_reason,
            html_url,
            most_recent_instance: None,
            number,
            rule: Box::new(rule),
            state,
            tool: Box::new(tool),
            url,
        }
    }
}
/// State of a code scanning alert.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "fixed")]
    Fixed,
}

impl Default for State {
    fn default() -> State {
        Self::Open
    }
}

