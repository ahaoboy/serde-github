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
pub struct WebhookOrgBlockUnblocked {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "blocked_user", deserialize_with = "Option::deserialize")]
    pub blocked_user: Option<Box<models::WebhooksUser>>,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization")]
    pub organization: Box<models::OrganizationSimpleWebhooks>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<models::RepositoryWebhooks>>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
}

impl WebhookOrgBlockUnblocked {
    pub fn new(action: Action, blocked_user: Option<models::WebhooksUser>, organization: models::OrganizationSimpleWebhooks, sender: models::SimpleUserWebhooks) -> WebhookOrgBlockUnblocked {
        WebhookOrgBlockUnblocked {
            action,
            blocked_user: if let Some(x) = blocked_user {Some(Box::new(x))} else {None},
            enterprise: None,
            installation: None,
            organization: Box::new(organization),
            repository: None,
            sender: Box::new(sender),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "unblocked")]
    Unblocked,
}

impl Default for Action {
    fn default() -> Action {
        Self::Unblocked
    }
}

