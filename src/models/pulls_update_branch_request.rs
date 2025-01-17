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
pub struct PullsUpdateBranchRequest {
    /// The expected SHA of the pull request's HEAD ref. This is the most recent commit on the pull request's branch. If the expected SHA does not match the pull request's HEAD, you will receive a `422 Unprocessable Entity` status. You can use the \"[List commits](https://docs.github.com/rest/commits/commits#list-commits)\" endpoint to find the most recent commit SHA. Default: SHA of the pull request's current HEAD ref.
    #[serde(rename = "expected_head_sha", skip_serializing_if = "Option::is_none")]
    pub expected_head_sha: Option<String>,
}

impl PullsUpdateBranchRequest {
    pub fn new() -> PullsUpdateBranchRequest {
        PullsUpdateBranchRequest {
            expected_head_sha: None,
        }
    }
}

