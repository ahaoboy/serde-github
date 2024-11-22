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
pub struct WebhookSponsorshipEdited {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "changes")]
    pub changes: Box<models::WebhookSponsorshipEditedChanges>,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<models::RepositoryWebhooks>>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
    #[serde(rename = "sponsorship")]
    pub sponsorship: Box<models::WebhooksSponsorship>,
}

impl WebhookSponsorshipEdited {
    pub fn new(action: Action, changes: models::WebhookSponsorshipEditedChanges, sender: models::SimpleUserWebhooks, sponsorship: models::WebhooksSponsorship) -> WebhookSponsorshipEdited {
        WebhookSponsorshipEdited {
            action,
            changes: Box::new(changes),
            enterprise: None,
            installation: None,
            organization: None,
            repository: None,
            sender: Box::new(sender),
            sponsorship: Box::new(sponsorship),
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
