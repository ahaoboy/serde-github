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

/// RepositoryRuleCommitMessagePattern : Parameters to be used for the commit_message_pattern rule
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryRuleCommitMessagePattern {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<models::RepositoryRuleCommitMessagePatternParameters>>,
}

impl RepositoryRuleCommitMessagePattern {
    /// Parameters to be used for the commit_message_pattern rule
    pub fn new(r#type: Type) -> RepositoryRuleCommitMessagePattern {
        RepositoryRuleCommitMessagePattern {
            r#type,
            parameters: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "commit_message_pattern")]
    CommitMessagePattern,
}

impl Default for Type {
    fn default() -> Type {
        Self::CommitMessagePattern
    }
}

