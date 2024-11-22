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

/// SecretScanningLocationIssueTitle : Represents an 'issue_title' secret scanning location type. This location type shows that a secret was detected in the title of an issue.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretScanningLocationIssueTitle {
    /// The API URL to get the issue where the secret was detected.
    #[serde(rename = "issue_title_url")]
    pub issue_title_url: String,
}

impl SecretScanningLocationIssueTitle {
    /// Represents an 'issue_title' secret scanning location type. This location type shows that a secret was detected in the title of an issue.
    pub fn new(issue_title_url: String) -> SecretScanningLocationIssueTitle {
        SecretScanningLocationIssueTitle {
            issue_title_url,
        }
    }
}
