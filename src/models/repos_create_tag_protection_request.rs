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
pub struct ReposCreateTagProtectionRequest {
    /// An optional glob pattern to match against when enforcing tag protection.
    #[serde(rename = "pattern")]
    pub pattern: String,
}

impl ReposCreateTagProtectionRequest {
    pub fn new(pattern: String) -> ReposCreateTagProtectionRequest {
        ReposCreateTagProtectionRequest {
            pattern,
        }
    }
}

