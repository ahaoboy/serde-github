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

/// RepositoryRuleEnforcement : The enforcement level of the ruleset. `evaluate` allows admins to test rules before enforcing them. Admins can view insights on the Rule Insights page (`evaluate` is only available with GitHub Enterprise).
/// The enforcement level of the ruleset. `evaluate` allows admins to test rules before enforcing them. Admins can view insights on the Rule Insights page (`evaluate` is only available with GitHub Enterprise).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RepositoryRuleEnforcement {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "evaluate")]
    Evaluate,

}

impl std::fmt::Display for RepositoryRuleEnforcement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Disabled => write!(f, "disabled"),
            Self::Active => write!(f, "active"),
            Self::Evaluate => write!(f, "evaluate"),
        }
    }
}

impl Default for RepositoryRuleEnforcement {
    fn default() -> RepositoryRuleEnforcement {
        Self::Disabled
    }
}
