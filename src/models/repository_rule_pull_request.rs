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

/// RepositoryRulePullRequest : Require all commits be made to a non-target branch and submitted via a pull request before they can be merged.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryRulePullRequest {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<models::RepositoryRulePullRequestParameters>>,
}

impl RepositoryRulePullRequest {
    /// Require all commits be made to a non-target branch and submitted via a pull request before they can be merged.
    pub fn new(r#type: Type) -> RepositoryRulePullRequest {
        RepositoryRulePullRequest {
            r#type,
            parameters: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "pull_request")]
    PullRequest,
}

impl Default for Type {
    fn default() -> Type {
        Self::PullRequest
    }
}

