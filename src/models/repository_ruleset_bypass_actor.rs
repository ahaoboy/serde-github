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

/// RepositoryRulesetBypassActor : An actor that can bypass rules in a ruleset
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryRulesetBypassActor {
    /// The ID of the actor that can bypass a ruleset. If `actor_type` is `OrganizationAdmin`, this should be `1`. If `actor_type` is `DeployKey`, this should be null. `OrganizationAdmin` is not applicable for personal repositories. 
    #[serde(rename = "actor_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<Option<i32>>,
    /// The type of actor that can bypass a ruleset. 
    #[serde(rename = "actor_type")]
    pub actor_type: ActorType,
    /// When the specified actor can bypass the ruleset. `pull_request` means that an actor can only bypass rules on pull requests. `pull_request` is not applicable for the `DeployKey` actor type. 
    #[serde(rename = "bypass_mode")]
    pub bypass_mode: BypassMode,
}

impl RepositoryRulesetBypassActor {
    /// An actor that can bypass rules in a ruleset
    pub fn new(actor_type: ActorType, bypass_mode: BypassMode) -> RepositoryRulesetBypassActor {
        RepositoryRulesetBypassActor {
            actor_id: None,
            actor_type,
            bypass_mode,
        }
    }
}
/// The type of actor that can bypass a ruleset. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActorType {
    #[serde(rename = "Integration")]
    Integration,
    #[serde(rename = "OrganizationAdmin")]
    OrganizationAdmin,
    #[serde(rename = "RepositoryRole")]
    RepositoryRole,
    #[serde(rename = "Team")]
    Team,
    #[serde(rename = "DeployKey")]
    DeployKey,
}

impl Default for ActorType {
    fn default() -> ActorType {
        Self::Integration
    }
}
/// When the specified actor can bypass the ruleset. `pull_request` means that an actor can only bypass rules on pull requests. `pull_request` is not applicable for the `DeployKey` actor type. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BypassMode {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "pull_request")]
    PullRequest,
}

impl Default for BypassMode {
    fn default() -> BypassMode {
        Self::Always
    }
}

