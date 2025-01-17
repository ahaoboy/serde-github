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

/// WebhookBranchProtectionRuleEditedChanges : If the action was `edited`, the changes to the rule.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookBranchProtectionRuleEditedChanges {
    #[serde(rename = "admin_enforced", skip_serializing_if = "Option::is_none")]
    pub admin_enforced: Option<Box<models::WebhookBranchProtectionRuleEditedChangesAdminEnforced>>,
    #[serde(rename = "authorized_actor_names", skip_serializing_if = "Option::is_none")]
    pub authorized_actor_names: Option<Box<models::WebhookBranchProtectionRuleEditedChangesAuthorizedActorNames>>,
    #[serde(rename = "authorized_actors_only", skip_serializing_if = "Option::is_none")]
    pub authorized_actors_only: Option<Box<models::WebhookBranchProtectionRuleEditedChangesAdminEnforced>>,
    #[serde(rename = "authorized_dismissal_actors_only", skip_serializing_if = "Option::is_none")]
    pub authorized_dismissal_actors_only: Option<Box<models::WebhookBranchProtectionRuleEditedChangesAdminEnforced>>,
    #[serde(rename = "linear_history_requirement_enforcement_level", skip_serializing_if = "Option::is_none")]
    pub linear_history_requirement_enforcement_level: Option<Box<models::WebhookBranchProtectionRuleEditedChangesLinearHistoryRequirementEnforcementLevel>>,
    #[serde(rename = "required_status_checks", skip_serializing_if = "Option::is_none")]
    pub required_status_checks: Option<Box<models::WebhookBranchProtectionRuleEditedChangesAuthorizedActorNames>>,
    #[serde(rename = "required_status_checks_enforcement_level", skip_serializing_if = "Option::is_none")]
    pub required_status_checks_enforcement_level: Option<Box<models::WebhookBranchProtectionRuleEditedChangesLinearHistoryRequirementEnforcementLevel>>,
}

impl WebhookBranchProtectionRuleEditedChanges {
    /// If the action was `edited`, the changes to the rule.
    pub fn new() -> WebhookBranchProtectionRuleEditedChanges {
        WebhookBranchProtectionRuleEditedChanges {
            admin_enforced: None,
            authorized_actor_names: None,
            authorized_actors_only: None,
            authorized_dismissal_actors_only: None,
            linear_history_requirement_enforcement_level: None,
            required_status_checks: None,
            required_status_checks_enforcement_level: None,
        }
    }
}

