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

/// Workflow : A GitHub Actions workflow
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "state")]
    pub state: State,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "badge_url")]
    pub badge_url: String,
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
}

impl Workflow {
    /// A GitHub Actions workflow
    pub fn new(id: i32, node_id: String, name: String, path: String, state: State, created_at: String, updated_at: String, url: String, html_url: String, badge_url: String) -> Workflow {
        Workflow {
            id,
            node_id,
            name,
            path,
            state,
            created_at,
            updated_at,
            url,
            html_url,
            badge_url,
            deleted_at: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "disabled_fork")]
    DisabledFork,
    #[serde(rename = "disabled_inactivity")]
    DisabledInactivity,
    #[serde(rename = "disabled_manually")]
    DisabledManually,
}

impl Default for State {
    fn default() -> State {
        Self::Active
    }
}
