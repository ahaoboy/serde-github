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

/// ReposUpdateRequestSecurityAndAnalysisSecretScanning : Use the `status` property to enable or disable secret scanning for this repository. For more information, see \"[About secret scanning](/code-security/secret-security/about-secret-scanning).\"
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReposUpdateRequestSecurityAndAnalysisSecretScanning {
    /// Can be `enabled` or `disabled`.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl ReposUpdateRequestSecurityAndAnalysisSecretScanning {
    /// Use the `status` property to enable or disable secret scanning for this repository. For more information, see \"[About secret scanning](/code-security/secret-security/about-secret-scanning).\"
    pub fn new() -> ReposUpdateRequestSecurityAndAnalysisSecretScanning {
        ReposUpdateRequestSecurityAndAnalysisSecretScanning {
            status: None,
        }
    }
}

