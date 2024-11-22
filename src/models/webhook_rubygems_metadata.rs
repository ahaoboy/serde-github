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
pub struct WebhookRubygemsMetadata {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "readme", skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,
    #[serde(rename = "homepage", skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(rename = "version_info", skip_serializing_if = "Option::is_none")]
    pub version_info: Option<Box<models::WebhookRubygemsMetadataVersionInfo>>,
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "repo", skip_serializing_if = "Option::is_none")]
    pub repo: Option<String>,
    #[serde(rename = "dependencies", skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<std::collections::HashMap<String, String>>>,
    #[serde(rename = "commit_oid", skip_serializing_if = "Option::is_none")]
    pub commit_oid: Option<String>,
}

impl WebhookRubygemsMetadata {
    pub fn new() -> WebhookRubygemsMetadata {
        WebhookRubygemsMetadata {
            name: None,
            description: None,
            readme: None,
            homepage: None,
            version_info: None,
            platform: None,
            metadata: None,
            repo: None,
            dependencies: None,
            commit_oid: None,
        }
    }
}

