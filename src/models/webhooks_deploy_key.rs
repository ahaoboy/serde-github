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

/// WebhooksDeployKey : The [`deploy key`](https://docs.github.com/rest/deploy-keys/deploy-keys#get-a-deploy-key) resource.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhooksDeployKey {
    #[serde(rename = "added_by", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub added_by: Option<Option<String>>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "last_used", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_used: Option<Option<String>>,
    #[serde(rename = "read_only")]
    pub read_only: bool,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "verified")]
    pub verified: bool,
}

impl WebhooksDeployKey {
    /// The [`deploy key`](https://docs.github.com/rest/deploy-keys/deploy-keys#get-a-deploy-key) resource.
    pub fn new(created_at: String, id: i32, key: String, read_only: bool, title: String, url: String, verified: bool) -> WebhooksDeployKey {
        WebhooksDeployKey {
            added_by: None,
            created_at,
            id,
            key,
            last_used: None,
            read_only,
            title,
            url,
            verified,
        }
    }
}

