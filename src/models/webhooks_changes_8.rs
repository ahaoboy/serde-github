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
pub struct WebhooksChanges8 {
    #[serde(rename = "tier")]
    pub tier: Box<models::WebhooksChanges8Tier>,
}

impl WebhooksChanges8 {
    pub fn new(tier: models::WebhooksChanges8Tier) -> WebhooksChanges8 {
        WebhooksChanges8 {
            tier: Box::new(tier),
        }
    }
}

