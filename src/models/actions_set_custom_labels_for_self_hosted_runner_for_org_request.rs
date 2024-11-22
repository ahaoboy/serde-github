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
pub struct ActionsSetCustomLabelsForSelfHostedRunnerForOrgRequest {
    /// The names of the custom labels to set for the runner. You can pass an empty array to remove all custom labels.
    #[serde(rename = "labels")]
    pub labels: Vec<String>,
}

impl ActionsSetCustomLabelsForSelfHostedRunnerForOrgRequest {
    pub fn new(labels: Vec<String>) -> ActionsSetCustomLabelsForSelfHostedRunnerForOrgRequest {
        ActionsSetCustomLabelsForSelfHostedRunnerForOrgRequest {
            labels,
        }
    }
}
