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
pub struct ReposCreateOrUpdateEnvironmentRequestReviewersInner {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::DeploymentReviewerType>,
    /// The id of the user or team who can review the deployment
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl ReposCreateOrUpdateEnvironmentRequestReviewersInner {
    pub fn new() -> ReposCreateOrUpdateEnvironmentRequestReviewersInner {
        ReposCreateOrUpdateEnvironmentRequestReviewersInner {
            r#type: None,
            id: None,
        }
    }
}

