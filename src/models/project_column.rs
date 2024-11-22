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

/// ProjectColumn : Project columns contain cards of work.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectColumn {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "project_url")]
    pub project_url: String,
    #[serde(rename = "cards_url")]
    pub cards_url: String,
    /// The unique identifier of the project column
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "node_id")]
    pub node_id: String,
    /// Name of the project column
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl ProjectColumn {
    /// Project columns contain cards of work.
    pub fn new(url: String, project_url: String, cards_url: String, id: i32, node_id: String, name: String, created_at: String, updated_at: String) -> ProjectColumn {
        ProjectColumn {
            url,
            project_url,
            cards_url,
            id,
            node_id,
            name,
            created_at,
            updated_at,
        }
    }
}

