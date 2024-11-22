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
pub struct ActionsUpdateRepoVariableRequest {
    /// The name of the variable.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The value of the variable.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ActionsUpdateRepoVariableRequest {
    pub fn new() -> ActionsUpdateRepoVariableRequest {
        ActionsUpdateRepoVariableRequest {
            name: None,
            value: None,
        }
    }
}

