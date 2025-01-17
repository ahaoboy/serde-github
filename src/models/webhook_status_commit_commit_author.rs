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
pub struct WebhookStatusCommitCommitAuthor {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl WebhookStatusCommitCommitAuthor {
    pub fn new(date: String, email: String, name: String) -> WebhookStatusCommitCommitAuthor {
        WebhookStatusCommitCommitAuthor {
            date,
            email,
            name,
            username: None,
        }
    }
}

