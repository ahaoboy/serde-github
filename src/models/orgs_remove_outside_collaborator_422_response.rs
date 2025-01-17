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
pub struct OrgsRemoveOutsideCollaborator422Response {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "documentation_url", skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
}

impl OrgsRemoveOutsideCollaborator422Response {
    pub fn new() -> OrgsRemoveOutsideCollaborator422Response {
        OrgsRemoveOutsideCollaborator422Response {
            message: None,
            documentation_url: None,
        }
    }
}

