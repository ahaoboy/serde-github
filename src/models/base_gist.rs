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

/// BaseGist : Base Gist
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseGist {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "forks_url")]
    pub forks_url: String,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "git_pull_url")]
    pub git_pull_url: String,
    #[serde(rename = "git_push_url")]
    pub git_push_url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "files")]
    pub files: std::collections::HashMap<String, models::BaseGistFilesValue>,
    #[serde(rename = "public")]
    pub public: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    #[serde(rename = "comments")]
    pub comments: i32,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::NullableSimpleUser>>,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<models::SimpleUser>>,
    #[serde(rename = "truncated", skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
    #[serde(rename = "forks", skip_serializing_if = "Option::is_none")]
    pub forks: Option<Vec<serde_json::Value>>,
    #[serde(rename = "history", skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<serde_json::Value>>,
}

impl BaseGist {
    /// Base Gist
    pub fn new(url: String, forks_url: String, commits_url: String, id: String, node_id: String, git_pull_url: String, git_push_url: String, html_url: String, files: std::collections::HashMap<String, models::BaseGistFilesValue>, public: bool, created_at: String, updated_at: String, description: Option<String>, comments: i32, user: Option<models::NullableSimpleUser>, comments_url: String) -> BaseGist {
        BaseGist {
            url,
            forks_url,
            commits_url,
            id,
            node_id,
            git_pull_url,
            git_push_url,
            html_url,
            files,
            public,
            created_at,
            updated_at,
            description,
            comments,
            user: if let Some(x) = user {Some(Box::new(x))} else {None},
            comments_url,
            owner: None,
            truncated: None,
            forks: None,
            history: None,
        }
    }
}

