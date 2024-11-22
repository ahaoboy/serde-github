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

/// CommitComment : Commit Comment
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitComment {
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "body")]
    pub body: String,
    #[serde(rename = "path", deserialize_with = "Option::deserialize")]
    pub path: Option<String>,
    #[serde(rename = "position", deserialize_with = "Option::deserialize")]
    pub position: Option<i32>,
    #[serde(rename = "line", deserialize_with = "Option::deserialize")]
    pub line: Option<i32>,
    #[serde(rename = "commit_id")]
    pub commit_id: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::NullableSimpleUser>>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "author_association")]
    pub author_association: models::AuthorAssociation,
    #[serde(rename = "reactions", skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Box<models::ReactionRollup>>,
}

impl CommitComment {
    /// Commit Comment
    pub fn new(html_url: String, url: String, id: i32, node_id: String, body: String, path: Option<String>, position: Option<i32>, line: Option<i32>, commit_id: String, user: Option<models::NullableSimpleUser>, created_at: String, updated_at: String, author_association: models::AuthorAssociation) -> CommitComment {
        CommitComment {
            html_url,
            url,
            id,
            node_id,
            body,
            path,
            position,
            line,
            commit_id,
            user: if let Some(x) = user {Some(Box::new(x))} else {None},
            created_at,
            updated_at,
            author_association,
            reactions: None,
        }
    }
}

