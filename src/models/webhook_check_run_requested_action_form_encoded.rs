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

/// WebhookCheckRunRequestedActionFormEncoded : The check_run.requested_action webhook encoded with URL encoding
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookCheckRunRequestedActionFormEncoded {
    /// A URL-encoded string of the check_run.requested_action JSON payload. The decoded payload is a JSON object.
    #[serde(rename = "payload")]
    pub payload: String,
}

impl WebhookCheckRunRequestedActionFormEncoded {
    /// The check_run.requested_action webhook encoded with URL encoding
    pub fn new(payload: String) -> WebhookCheckRunRequestedActionFormEncoded {
        WebhookCheckRunRequestedActionFormEncoded {
            payload,
        }
    }
}
