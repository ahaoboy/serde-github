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

/// RepositoryRulesetConditionsRepositoryIdTarget : Parameters for a repository ID condition
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryRulesetConditionsRepositoryIdTarget {
    #[serde(rename = "repository_id")]
    pub repository_id: Box<models::RepositoryRulesetConditionsRepositoryIdTargetRepositoryId>,
}

impl RepositoryRulesetConditionsRepositoryIdTarget {
    /// Parameters for a repository ID condition
    pub fn new(repository_id: models::RepositoryRulesetConditionsRepositoryIdTargetRepositoryId) -> RepositoryRulesetConditionsRepositoryIdTarget {
        RepositoryRulesetConditionsRepositoryIdTarget {
            repository_id: Box::new(repository_id),
        }
    }
}
