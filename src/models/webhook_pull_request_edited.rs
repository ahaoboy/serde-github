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
pub struct WebhookPullRequestEdited {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "changes")]
    pub changes: Box<models::WebhookPullRequestEditedChanges>,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    /// The pull request number.
    #[serde(rename = "number")]
    pub number: i32,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "pull_request")]
    pub pull_request: Box<models::PullRequestWebhook>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<Box<models::SimpleUserWebhooks>>,
}

impl WebhookPullRequestEdited {
    pub fn new(action: Action, changes: models::WebhookPullRequestEditedChanges, number: i32, pull_request: models::PullRequestWebhook, repository: models::RepositoryWebhooks) -> WebhookPullRequestEdited {
        WebhookPullRequestEdited {
            action,
            changes: Box::new(changes),
            enterprise: None,
            installation: None,
            number,
            organization: None,
            pull_request: Box::new(pull_request),
            repository: Box::new(repository),
            sender: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "edited")]
    Edited,
}

impl Default for Action {
    fn default() -> Action {
        Self::Edited
    }
}

