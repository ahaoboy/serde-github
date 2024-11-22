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
pub struct ActivityMarkNotificationsAsReadRequest {
    /// Describes the last point that notifications were checked. Anything updated since this time will not be marked as read. If you omit this parameter, all notifications are marked as read. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. Default: The current timestamp.
    #[serde(rename = "last_read_at", skip_serializing_if = "Option::is_none")]
    pub last_read_at: Option<String>,
    /// Whether the notification has been read.
    #[serde(rename = "read", skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
}

impl ActivityMarkNotificationsAsReadRequest {
    pub fn new() -> ActivityMarkNotificationsAsReadRequest {
        ActivityMarkNotificationsAsReadRequest {
            last_read_at: None,
            read: None,
        }
    }
}

