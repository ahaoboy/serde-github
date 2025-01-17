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

/// RepositoryRuleCommitAuthorEmailPattern : Parameters to be used for the commit_author_email_pattern rule
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryRuleCommitAuthorEmailPattern {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<models::RepositoryRuleCommitMessagePatternParameters>>,
}

impl RepositoryRuleCommitAuthorEmailPattern {
    /// Parameters to be used for the commit_author_email_pattern rule
    pub fn new(r#type: Type) -> RepositoryRuleCommitAuthorEmailPattern {
        RepositoryRuleCommitAuthorEmailPattern {
            r#type,
            parameters: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "commit_author_email_pattern")]
    CommitAuthorEmailPattern,
}

impl Default for Type {
    fn default() -> Type {
        Self::CommitAuthorEmailPattern
    }
}

