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
pub struct WebhookRepositoryRulesetDeleted {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<models::RepositoryWebhooks>>,
    #[serde(rename = "repository_ruleset")]
    pub repository_ruleset: Box<models::RepositoryRuleset>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
}

impl WebhookRepositoryRulesetDeleted {
    pub fn new(action: Action, repository_ruleset: models::RepositoryRuleset, sender: models::SimpleUserWebhooks) -> WebhookRepositoryRulesetDeleted {
        WebhookRepositoryRulesetDeleted {
            action,
            enterprise: None,
            installation: None,
            organization: None,
            repository: None,
            repository_ruleset: Box::new(repository_ruleset),
            sender: Box::new(sender),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "deleted")]
    Deleted,
}

impl Default for Action {
    fn default() -> Action {
        Self::Deleted
    }
}

