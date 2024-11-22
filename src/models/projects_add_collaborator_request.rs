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
pub struct ProjectsAddCollaboratorRequest {
    /// The permission to grant the collaborator.
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<Permission>,
}

impl ProjectsAddCollaboratorRequest {
    pub fn new() -> ProjectsAddCollaboratorRequest {
        ProjectsAddCollaboratorRequest {
            permission: None,
        }
    }
}
/// The permission to grant the collaborator.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Permission {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "admin")]
    Admin,
}

impl Default for Permission {
    fn default() -> Permission {
        Self::Read
    }
}

