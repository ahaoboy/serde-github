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

/// CopilotSeatDetailsAssignee : The assignee that has been granted access to GitHub Copilot.
/// The assignee that has been granted access to GitHub Copilot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CopilotSeatDetailsAssignee {
    SimpleUser(Box<models::SimpleUser>),
    Team(Box<models::Team>),
    Organization(Box<models::Organization>),
}

impl Default for CopilotSeatDetailsAssignee {
    fn default() -> Self {
        Self::SimpleUser(Default::default())
    }
}
