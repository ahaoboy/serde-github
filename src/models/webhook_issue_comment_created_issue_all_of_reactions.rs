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
pub struct WebhookIssueCommentCreatedIssueAllOfReactions {
    #[serde(rename = "+1", skip_serializing_if = "Option::is_none")]
    pub plus_1: Option<i32>,
    #[serde(rename = "-1", skip_serializing_if = "Option::is_none")]
    pub _1: Option<i32>,
    #[serde(rename = "confused", skip_serializing_if = "Option::is_none")]
    pub confused: Option<i32>,
    #[serde(rename = "eyes", skip_serializing_if = "Option::is_none")]
    pub eyes: Option<i32>,
    #[serde(rename = "heart", skip_serializing_if = "Option::is_none")]
    pub heart: Option<i32>,
    #[serde(rename = "hooray", skip_serializing_if = "Option::is_none")]
    pub hooray: Option<i32>,
    #[serde(rename = "laugh", skip_serializing_if = "Option::is_none")]
    pub laugh: Option<i32>,
    #[serde(rename = "rocket", skip_serializing_if = "Option::is_none")]
    pub rocket: Option<i32>,
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl WebhookIssueCommentCreatedIssueAllOfReactions {
    pub fn new() -> WebhookIssueCommentCreatedIssueAllOfReactions {
        WebhookIssueCommentCreatedIssueAllOfReactions {
            plus_1: None,
            _1: None,
            confused: None,
            eyes: None,
            heart: None,
            hooray: None,
            laugh: None,
            rocket: None,
            total_count: None,
            url: None,
        }
    }
}

