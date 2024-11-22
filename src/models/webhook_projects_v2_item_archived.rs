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
pub struct WebhookProjectsV2ItemArchived {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "changes")]
    pub changes: Box<models::WebhooksProjectChanges>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization")]
    pub organization: Box<models::OrganizationSimpleWebhooks>,
    #[serde(rename = "projects_v2_item")]
    pub projects_v2_item: Box<models::ProjectsV2Item>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
}

impl WebhookProjectsV2ItemArchived {
    pub fn new(action: Action, changes: models::WebhooksProjectChanges, organization: models::OrganizationSimpleWebhooks, projects_v2_item: models::ProjectsV2Item, sender: models::SimpleUserWebhooks) -> WebhookProjectsV2ItemArchived {
        WebhookProjectsV2ItemArchived {
            action,
            changes: Box::new(changes),
            installation: None,
            organization: Box::new(organization),
            projects_v2_item: Box::new(projects_v2_item),
            sender: Box::new(sender),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "archived")]
    Archived,
}

impl Default for Action {
    fn default() -> Action {
        Self::Archived
    }
}

