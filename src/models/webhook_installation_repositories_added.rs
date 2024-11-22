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
pub struct WebhookInstallationRepositoriesAdded {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation")]
    pub installation: Box<models::Installation>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    /// An array of repository objects, which were added to the installation.
    #[serde(rename = "repositories_added")]
    pub repositories_added: Vec<models::WebhooksRepositoriesInner>,
    /// An array of repository objects, which were removed from the installation.
    #[serde(rename = "repositories_removed")]
    pub repositories_removed: Vec<models::WebhookInstallationRepositoriesAddedRepositoriesRemovedInner>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<models::RepositoryWebhooks>>,
    #[serde(rename = "repository_selection")]
    pub repository_selection: models::WebhooksRepositorySelection,
    #[serde(rename = "requester", deserialize_with = "Option::deserialize")]
    pub requester: Option<Box<models::WebhooksUser>>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
}

impl WebhookInstallationRepositoriesAdded {
    pub fn new(action: Action, installation: models::Installation, repositories_added: Vec<models::WebhooksRepositoriesInner>, repositories_removed: Vec<models::WebhookInstallationRepositoriesAddedRepositoriesRemovedInner>, repository_selection: models::WebhooksRepositorySelection, requester: Option<models::WebhooksUser>, sender: models::SimpleUserWebhooks) -> WebhookInstallationRepositoriesAdded {
        WebhookInstallationRepositoriesAdded {
            action,
            enterprise: None,
            installation: Box::new(installation),
            organization: None,
            repositories_added,
            repositories_removed,
            repository: None,
            repository_selection,
            requester: if let Some(x) = requester {Some(Box::new(x))} else {None},
            sender: Box::new(sender),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "added")]
    Added,
}

impl Default for Action {
    fn default() -> Action {
        Self::Added
    }
}

