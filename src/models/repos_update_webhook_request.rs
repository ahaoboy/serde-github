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
pub struct ReposUpdateWebhookRequest {
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<models::WebhookConfig>>,
    /// Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for. This replaces the entire array of events.
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    /// Determines a list of events to be added to the list of events that the Hook triggers for.
    #[serde(rename = "add_events", skip_serializing_if = "Option::is_none")]
    pub add_events: Option<Vec<String>>,
    /// Determines a list of events to be removed from the list of events that the Hook triggers for.
    #[serde(rename = "remove_events", skip_serializing_if = "Option::is_none")]
    pub remove_events: Option<Vec<String>>,
    /// Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl ReposUpdateWebhookRequest {
    pub fn new() -> ReposUpdateWebhookRequest {
        ReposUpdateWebhookRequest {
            config: None,
            events: None,
            add_events: None,
            remove_events: None,
            active: None,
        }
    }
}

