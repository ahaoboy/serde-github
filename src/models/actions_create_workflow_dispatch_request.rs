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
pub struct ActionsCreateWorkflowDispatchRequest {
    /// The git reference for the workflow. The reference can be a branch or tag name.
    #[serde(rename = "ref")]
    pub r#ref: String,
    /// Input keys and values configured in the workflow file. The maximum number of properties is 10. Any default properties configured in the workflow file will be used when `inputs` are omitted.
    #[serde(rename = "inputs", skip_serializing_if = "Option::is_none")]
    pub inputs: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl ActionsCreateWorkflowDispatchRequest {
    pub fn new(r#ref: String) -> ActionsCreateWorkflowDispatchRequest {
        ActionsCreateWorkflowDispatchRequest {
            r#ref,
            inputs: None,
        }
    }
}
