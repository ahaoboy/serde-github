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
pub struct CopilotAddCopilotSeatsForUsersRequest {
    /// The usernames of the organization members to be granted access to GitHub Copilot.
    #[serde(rename = "selected_usernames")]
    pub selected_usernames: Vec<String>,
}

impl CopilotAddCopilotSeatsForUsersRequest {
    pub fn new(selected_usernames: Vec<String>) -> CopilotAddCopilotSeatsForUsersRequest {
        CopilotAddCopilotSeatsForUsersRequest {
            selected_usernames,
        }
    }
}

