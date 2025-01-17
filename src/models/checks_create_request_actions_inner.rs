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
pub struct ChecksCreateRequestActionsInner {
    /// The text to be displayed on a button in the web UI. The maximum size is 20 characters.
    #[serde(rename = "label")]
    pub label: String,
    /// A short explanation of what this action would do. The maximum size is 40 characters.
    #[serde(rename = "description")]
    pub description: String,
    /// A reference for the action on the integrator's system. The maximum size is 20 characters.
    #[serde(rename = "identifier")]
    pub identifier: String,
}

impl ChecksCreateRequestActionsInner {
    pub fn new(label: String, description: String, identifier: String) -> ChecksCreateRequestActionsInner {
        ChecksCreateRequestActionsInner {
            label,
            description,
            identifier,
        }
    }
}

