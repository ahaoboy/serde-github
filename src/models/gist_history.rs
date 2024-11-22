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

/// GistHistory : Gist History
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GistHistory {
    #[serde(rename = "user", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user: Option<Option<Box<models::NullableSimpleUser>>>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "committed_at", skip_serializing_if = "Option::is_none")]
    pub committed_at: Option<String>,
    #[serde(rename = "change_status", skip_serializing_if = "Option::is_none")]
    pub change_status: Option<Box<models::GistHistoryChangeStatus>>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl GistHistory {
    /// Gist History
    pub fn new() -> GistHistory {
        GistHistory {
            user: None,
            version: None,
            committed_at: None,
            change_status: None,
            url: None,
        }
    }
}

