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
pub struct WebhookMetaDeletedHookConfig {
    #[serde(rename = "content_type")]
    pub content_type: ContentType,
    #[serde(rename = "insecure_ssl")]
    pub insecure_ssl: String,
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
}

impl WebhookMetaDeletedHookConfig {
    pub fn new(content_type: ContentType, insecure_ssl: String, url: String) -> WebhookMetaDeletedHookConfig {
        WebhookMetaDeletedHookConfig {
            content_type,
            insecure_ssl,
            secret: None,
            url,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "form")]
    Form,
}

impl Default for ContentType {
    fn default() -> ContentType {
        Self::Json
    }
}

