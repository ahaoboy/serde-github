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

/// PullRequestAutoMerge : The status of auto merging a pull request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PullRequestAutoMerge {
    /// Commit message for the merge commit.
    #[serde(rename = "commit_message", deserialize_with = "Option::deserialize")]
    pub commit_message: Option<String>,
    /// Title for the merge commit message.
    #[serde(rename = "commit_title", deserialize_with = "Option::deserialize")]
    pub commit_title: Option<String>,
    #[serde(rename = "enabled_by", deserialize_with = "Option::deserialize")]
    pub enabled_by: Option<Box<models::User>>,
    /// The merge method to use.
    #[serde(rename = "merge_method")]
    pub merge_method: MergeMethod,
}

impl PullRequestAutoMerge {
    /// The status of auto merging a pull request.
    pub fn new(commit_message: Option<String>, commit_title: Option<String>, enabled_by: Option<models::User>, merge_method: MergeMethod) -> PullRequestAutoMerge {
        PullRequestAutoMerge {
            commit_message,
            commit_title,
            enabled_by: if let Some(x) = enabled_by {Some(Box::new(x))} else {None},
            merge_method,
        }
    }
}
/// The merge method to use.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MergeMethod {
    #[serde(rename = "merge")]
    Merge,
    #[serde(rename = "squash")]
    Squash,
    #[serde(rename = "rebase")]
    Rebase,
}

impl Default for MergeMethod {
    fn default() -> MergeMethod {
        Self::Merge
    }
}

