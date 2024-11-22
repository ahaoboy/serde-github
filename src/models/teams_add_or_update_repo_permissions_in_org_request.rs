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
pub struct TeamsAddOrUpdateRepoPermissionsInOrgRequest {
    /// The permission to grant the team on this repository. We accept the following permissions to be set: `pull`, `triage`, `push`, `maintain`, `admin` and you can also specify a custom repository role name, if the owning organization has defined any. If no permission is specified, the team's `permission` attribute will be used to determine what permission to grant the team on this repository.
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

impl TeamsAddOrUpdateRepoPermissionsInOrgRequest {
    pub fn new() -> TeamsAddOrUpdateRepoPermissionsInOrgRequest {
        TeamsAddOrUpdateRepoPermissionsInOrgRequest {
            permission: None,
        }
    }
}

