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
pub struct WebhookCommitCommentCreated {
    /// The action performed. Can be `created`.
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "comment")]
    pub comment: Box<models::WebhookCommitCommentCreatedComment>,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
}

impl WebhookCommitCommentCreated {
    pub fn new(action: Action, comment: models::WebhookCommitCommentCreatedComment, repository: models::RepositoryWebhooks, sender: models::SimpleUserWebhooks) -> WebhookCommitCommentCreated {
        WebhookCommitCommentCreated {
            action,
            comment: Box::new(comment),
            enterprise: None,
            installation: None,
            organization: None,
            repository: Box::new(repository),
            sender: Box::new(sender),
        }
    }
}
/// The action performed. Can be `created`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "created")]
    Created,
}

impl Default for Action {
    fn default() -> Action {
        Self::Created
    }
}

