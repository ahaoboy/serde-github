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

/// CopilotCancelCopilotSeatAssignmentForTeams200Response : The total number of seat assignments cancelled.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CopilotCancelCopilotSeatAssignmentForTeams200Response {
    #[serde(rename = "seats_cancelled")]
    pub seats_cancelled: i32,
}

impl CopilotCancelCopilotSeatAssignmentForTeams200Response {
    /// The total number of seat assignments cancelled.
    pub fn new(seats_cancelled: i32) -> CopilotCancelCopilotSeatAssignmentForTeams200Response {
        CopilotCancelCopilotSeatAssignmentForTeams200Response {
            seats_cancelled,
        }
    }
}

