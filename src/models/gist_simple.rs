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

/// GistSimple : Gist Simple
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GistSimple {
    #[serde(rename = "forks", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub forks: Option<Option<Vec<models::GistSimpleForksInner>>>,
    #[serde(rename = "history", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub history: Option<Option<Vec<models::GistHistory>>>,
    #[serde(rename = "fork_of", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub fork_of: Option<Option<Box<models::Gist>>>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "forks_url", skip_serializing_if = "Option::is_none")]
    pub forks_url: Option<String>,
    #[serde(rename = "commits_url", skip_serializing_if = "Option::is_none")]
    pub commits_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "node_id", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "git_pull_url", skip_serializing_if = "Option::is_none")]
    pub git_pull_url: Option<String>,
    #[serde(rename = "git_push_url", skip_serializing_if = "Option::is_none")]
    pub git_push_url: Option<String>,
    #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<std::collections::HashMap<String, models::GistSimpleFilesValue>>,
    #[serde(rename = "public", skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<i32>,
    #[serde(rename = "user", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user: Option<Option<String>>,
    #[serde(rename = "comments_url", skip_serializing_if = "Option::is_none")]
    pub comments_url: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<models::SimpleUser>>,
    #[serde(rename = "truncated", skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

impl GistSimple {
    /// Gist Simple
    pub fn new() -> GistSimple {
        GistSimple {
            forks: None,
            history: None,
            fork_of: None,
            url: None,
            forks_url: None,
            commits_url: None,
            id: None,
            node_id: None,
            git_pull_url: None,
            git_push_url: None,
            html_url: None,
            files: None,
            public: None,
            created_at: None,
            updated_at: None,
            description: None,
            comments: None,
            user: None,
            comments_url: None,
            owner: None,
            truncated: None,
        }
    }
}

