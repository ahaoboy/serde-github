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
pub struct WebhookDeploymentReviewRejected {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "approver", skip_serializing_if = "Option::is_none")]
    pub approver: Option<Box<models::WebhooksApprover>>,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization")]
    pub organization: Box<models::OrganizationSimpleWebhooks>,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "reviewers", skip_serializing_if = "Option::is_none")]
    pub reviewers: Option<Vec<models::WebhooksReviewersInner>>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
    #[serde(rename = "since")]
    pub since: String,
    #[serde(rename = "workflow_job_run", skip_serializing_if = "Option::is_none")]
    pub workflow_job_run: Option<Box<models::WebhooksWorkflowJobRun>>,
    #[serde(rename = "workflow_job_runs", skip_serializing_if = "Option::is_none")]
    pub workflow_job_runs: Option<Vec<models::WebhookDeploymentReviewRejectedWorkflowJobRunsInner>>,
    #[serde(rename = "workflow_run", deserialize_with = "Option::deserialize")]
    pub workflow_run: Option<Box<models::DeploymentWorkflowRun2>>,
}

impl WebhookDeploymentReviewRejected {
    pub fn new(action: Action, organization: models::OrganizationSimpleWebhooks, repository: models::RepositoryWebhooks, sender: models::SimpleUserWebhooks, since: String, workflow_run: Option<models::DeploymentWorkflowRun2>) -> WebhookDeploymentReviewRejected {
        WebhookDeploymentReviewRejected {
            action,
            approver: None,
            comment: None,
            enterprise: None,
            installation: None,
            organization: Box::new(organization),
            repository: Box::new(repository),
            reviewers: None,
            sender: Box::new(sender),
            since,
            workflow_job_run: None,
            workflow_job_runs: None,
            workflow_run: if let Some(x) = workflow_run {Some(Box::new(x))} else {None},
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "rejected")]
    Rejected,
}

impl Default for Action {
    fn default() -> Action {
        Self::Rejected
    }
}

