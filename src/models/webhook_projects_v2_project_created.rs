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

/// WebhookProjectsV2ProjectCreated : A project was created
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookProjectsV2ProjectCreated {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization")]
    pub organization: Box<models::OrganizationSimpleWebhooks>,
    #[serde(rename = "projects_v2")]
    pub projects_v2: Box<models::ProjectsV2>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
}

impl WebhookProjectsV2ProjectCreated {
    /// A project was created
    pub fn new(action: Action, organization: models::OrganizationSimpleWebhooks, projects_v2: models::ProjectsV2, sender: models::SimpleUserWebhooks) -> WebhookProjectsV2ProjectCreated {
        WebhookProjectsV2ProjectCreated {
            action,
            installation: None,
            organization: Box::new(organization),
            projects_v2: Box::new(projects_v2),
            sender: Box::new(sender),
        }
    }
}
/// 
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

