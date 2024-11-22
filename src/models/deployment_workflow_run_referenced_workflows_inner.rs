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
pub struct DeploymentWorkflowRunReferencedWorkflowsInner {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    #[serde(rename = "sha")]
    pub sha: String,
}

impl DeploymentWorkflowRunReferencedWorkflowsInner {
    pub fn new(path: String, sha: String) -> DeploymentWorkflowRunReferencedWorkflowsInner {
        DeploymentWorkflowRunReferencedWorkflowsInner {
            path,
            r#ref: None,
            sha,
        }
    }
}
