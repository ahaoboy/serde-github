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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrgsCreateWebhookRequest {
    /// Must be passed as \"web\".
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "config")]
    pub config: Box<models::OrgsCreateWebhookRequestConfig>,
    /// Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for. Set to `[\"*\"]` to receive all possible events.
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    /// Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl OrgsCreateWebhookRequest {
    pub fn new(name: String, config: models::OrgsCreateWebhookRequestConfig) -> OrgsCreateWebhookRequest {
        OrgsCreateWebhookRequest {
            name,
            config: Box::new(config),
            events: None,
            active: None,
        }
    }
}

