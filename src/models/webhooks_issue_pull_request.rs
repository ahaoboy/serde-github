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
pub struct WebhooksIssuePullRequest {
    #[serde(rename = "diff_url", skip_serializing_if = "Option::is_none")]
    pub diff_url: Option<String>,
    #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(rename = "merged_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub merged_at: Option<Option<String>>,
    #[serde(rename = "patch_url", skip_serializing_if = "Option::is_none")]
    pub patch_url: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl WebhooksIssuePullRequest {
    pub fn new() -> WebhooksIssuePullRequest {
        WebhooksIssuePullRequest {
            diff_url: None,
            html_url: None,
            merged_at: None,
            patch_url: None,
            url: None,
        }
    }
}

