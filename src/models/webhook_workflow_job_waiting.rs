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
pub struct WebhookWorkflowJobWaiting {
    #[serde(rename = "action")]
    pub action: Action,
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
    #[serde(rename = "workflow_job")]
    pub workflow_job: Box<models::WebhookWorkflowJobWaitingWorkflowJob>,
    #[serde(rename = "deployment", skip_serializing_if = "Option::is_none")]
    pub deployment: Option<Box<models::Deployment>>,
}

impl WebhookWorkflowJobWaiting {
    pub fn new(action: Action, repository: models::RepositoryWebhooks, sender: models::SimpleUserWebhooks, workflow_job: models::WebhookWorkflowJobWaitingWorkflowJob) -> WebhookWorkflowJobWaiting {
        WebhookWorkflowJobWaiting {
            action,
            enterprise: None,
            installation: None,
            organization: None,
            repository: Box::new(repository),
            sender: Box::new(sender),
            workflow_job: Box::new(workflow_job),
            deployment: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "waiting")]
    Waiting,
}

impl Default for Action {
    fn default() -> Action {
        Self::Waiting
    }
}

