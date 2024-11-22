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
pub struct ActivitySetRepoSubscriptionRequest {
    /// Determines if notifications should be received from this repository.
    #[serde(rename = "subscribed", skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
    /// Determines if all notifications should be blocked from this repository.
    #[serde(rename = "ignored", skip_serializing_if = "Option::is_none")]
    pub ignored: Option<bool>,
}

impl ActivitySetRepoSubscriptionRequest {
    pub fn new() -> ActivitySetRepoSubscriptionRequest {
        ActivitySetRepoSubscriptionRequest {
            subscribed: None,
            ignored: None,
        }
    }
}

