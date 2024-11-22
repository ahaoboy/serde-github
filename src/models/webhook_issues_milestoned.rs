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
pub struct WebhookIssuesMilestoned {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "issue")]
    pub issue: Box<models::Issue5>,
    #[serde(rename = "milestone")]
    pub milestone: Box<models::WebhooksMilestone>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
}

impl WebhookIssuesMilestoned {
    pub fn new(action: Action, issue: models::Issue5, milestone: models::WebhooksMilestone, repository: models::RepositoryWebhooks, sender: models::SimpleUserWebhooks) -> WebhookIssuesMilestoned {
        WebhookIssuesMilestoned {
            action,
            enterprise: None,
            installation: None,
            issue: Box::new(issue),
            milestone: Box::new(milestone),
            organization: None,
            repository: Box::new(repository),
            sender: Box::new(sender),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "milestoned")]
    Milestoned,
}

impl Default for Action {
    fn default() -> Action {
        Self::Milestoned
    }
}

