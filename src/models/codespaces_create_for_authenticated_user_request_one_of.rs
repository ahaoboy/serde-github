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
pub struct CodespacesCreateForAuthenticatedUserRequestOneOf {
    /// Repository id for this codespace
    #[serde(rename = "repository_id")]
    pub repository_id: i32,
    /// Git ref (typically a branch name) for this codespace
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    /// The requested location for a new codespace. Best efforts are made to respect this upon creation. Assigned by IP if not provided.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// The geographic area for this codespace. If not specified, the value is assigned by IP. This property replaces `location`, which is being deprecated.
    #[serde(rename = "geo", skip_serializing_if = "Option::is_none")]
    pub geo: Option<Geo>,
    /// IP for location auto-detection when proxying a request
    #[serde(rename = "client_ip", skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    /// Machine type to use for this codespace
    #[serde(rename = "machine", skip_serializing_if = "Option::is_none")]
    pub machine: Option<String>,
    /// Path to devcontainer.json config to use for this codespace
    #[serde(rename = "devcontainer_path", skip_serializing_if = "Option::is_none")]
    pub devcontainer_path: Option<String>,
    /// Whether to authorize requested permissions from devcontainer.json
    #[serde(rename = "multi_repo_permissions_opt_out", skip_serializing_if = "Option::is_none")]
    pub multi_repo_permissions_opt_out: Option<bool>,
    /// Working directory for this codespace
    #[serde(rename = "working_directory", skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    /// Time in minutes before codespace stops from inactivity
    #[serde(rename = "idle_timeout_minutes", skip_serializing_if = "Option::is_none")]
    pub idle_timeout_minutes: Option<i32>,
    /// Display name for this codespace
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Duration in minutes after codespace has gone idle in which it will be deleted. Must be integer minutes between 0 and 43200 (30 days).
    #[serde(rename = "retention_period_minutes", skip_serializing_if = "Option::is_none")]
    pub retention_period_minutes: Option<i32>,
}

impl CodespacesCreateForAuthenticatedUserRequestOneOf {
    pub fn new(repository_id: i32) -> CodespacesCreateForAuthenticatedUserRequestOneOf {
        CodespacesCreateForAuthenticatedUserRequestOneOf {
            repository_id,
            r#ref: None,
            location: None,
            geo: None,
            client_ip: None,
            machine: None,
            devcontainer_path: None,
            multi_repo_permissions_opt_out: None,
            working_directory: None,
            idle_timeout_minutes: None,
            display_name: None,
            retention_period_minutes: None,
        }
    }
}
/// The geographic area for this codespace. If not specified, the value is assigned by IP. This property replaces `location`, which is being deprecated.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Geo {
    #[serde(rename = "EuropeWest")]
    EuropeWest,
    #[serde(rename = "SoutheastAsia")]
    SoutheastAsia,
    #[serde(rename = "UsEast")]
    UsEast,
    #[serde(rename = "UsWest")]
    UsWest,
}

impl Default for Geo {
    fn default() -> Geo {
        Self::EuropeWest
    }
}

