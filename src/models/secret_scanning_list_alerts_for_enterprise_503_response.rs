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
pub struct SecretScanningListAlertsForEnterprise503Response {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "documentation_url", skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
}

impl SecretScanningListAlertsForEnterprise503Response {
    pub fn new() -> SecretScanningListAlertsForEnterprise503Response {
        SecretScanningListAlertsForEnterprise503Response {
            code: None,
            message: None,
            documentation_url: None,
        }
    }
}

