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
pub struct RepositoryRuleDetailedOneOf8 {
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The type of source for the ruleset that includes this rule.
    #[serde(rename = "ruleset_source_type", skip_serializing_if = "Option::is_none")]
    pub ruleset_source_type: Option<RulesetSourceType>,
    /// The name of the source of the ruleset that includes this rule.
    #[serde(rename = "ruleset_source", skip_serializing_if = "Option::is_none")]
    pub ruleset_source: Option<String>,
    /// The ID of the ruleset that includes this rule.
    #[serde(rename = "ruleset_id", skip_serializing_if = "Option::is_none")]
    pub ruleset_id: Option<i32>,
}

impl RepositoryRuleDetailedOneOf8 {
    pub fn new(r#type: Type) -> RepositoryRuleDetailedOneOf8 {
        RepositoryRuleDetailedOneOf8 {
            r#type,
            ruleset_source_type: None,
            ruleset_source: None,
            ruleset_id: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "non_fast_forward")]
    NonFastForward,
}

impl Default for Type {
    fn default() -> Type {
        Self::NonFastForward
    }
}
/// The type of source for the ruleset that includes this rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RulesetSourceType {
    #[serde(rename = "Repository")]
    Repository,
    #[serde(rename = "Organization")]
    Organization,
}

impl Default for RulesetSourceType {
    fn default() -> RulesetSourceType {
        Self::Repository
    }
}

