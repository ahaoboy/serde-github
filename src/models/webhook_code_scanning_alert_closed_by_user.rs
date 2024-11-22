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
pub struct WebhookCodeScanningAlertClosedByUser {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "alert")]
    pub alert: Box<models::WebhookCodeScanningAlertClosedByUserAlert>,
    /// The commit SHA of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty.
    #[serde(rename = "commit_oid")]
    pub commit_oid: String,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    /// The Git reference of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty.
    #[serde(rename = "ref")]
    pub r#ref: String,
    #[serde(rename = "repository")]
    pub repository: Box<models::RepositoryWebhooks>,
    #[serde(rename = "sender")]
    pub sender: Box<models::SimpleUserWebhooks>,
}

impl WebhookCodeScanningAlertClosedByUser {
    pub fn new(action: Action, alert: models::WebhookCodeScanningAlertClosedByUserAlert, commit_oid: String, r#ref: String, repository: models::RepositoryWebhooks, sender: models::SimpleUserWebhooks) -> WebhookCodeScanningAlertClosedByUser {
        WebhookCodeScanningAlertClosedByUser {
            action,
            alert: Box::new(alert),
            commit_oid,
            enterprise: None,
            installation: None,
            organization: None,
            r#ref,
            repository: Box::new(repository),
            sender: Box::new(sender),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "closed_by_user")]
    ClosedByUser,
}

impl Default for Action {
    fn default() -> Action {
        Self::ClosedByUser
    }
}

