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

/// AutoMerge : The status of auto merging a pull request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoMerge {
    #[serde(rename = "enabled_by")]
    pub enabled_by: Box<models::SimpleUser>,
    /// The merge method to use.
    #[serde(rename = "merge_method")]
    pub merge_method: MergeMethod,
    /// Title for the merge commit message.
    #[serde(rename = "commit_title")]
    pub commit_title: String,
    /// Commit message for the merge commit.
    #[serde(rename = "commit_message")]
    pub commit_message: String,
}

impl AutoMerge {
    /// The status of auto merging a pull request.
    pub fn new(enabled_by: models::SimpleUser, merge_method: MergeMethod, commit_title: String, commit_message: String) -> AutoMerge {
        AutoMerge {
            enabled_by: Box::new(enabled_by),
            merge_method,
            commit_title,
            commit_message,
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

