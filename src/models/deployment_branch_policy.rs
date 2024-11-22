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

/// DeploymentBranchPolicy : Details of a deployment branch or tag policy.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentBranchPolicy {
    /// The unique identifier of the branch or tag policy.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "node_id", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// The name pattern that branches or tags must match in order to deploy to the environment.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether this rule targets a branch or tag.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl DeploymentBranchPolicy {
    /// Details of a deployment branch or tag policy.
    pub fn new() -> DeploymentBranchPolicy {
        DeploymentBranchPolicy {
            id: None,
            node_id: None,
            name: None,
            r#type: None,
        }
    }
}
/// Whether this rule targets a branch or tag.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "branch")]
    Branch,
    #[serde(rename = "tag")]
    Tag,
}

impl Default for Type {
    fn default() -> Type {
        Self::Branch
    }
}

