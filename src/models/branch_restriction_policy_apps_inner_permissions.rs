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
pub struct BranchRestrictionPolicyAppsInnerPermissions {
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(rename = "contents", skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<String>,
    #[serde(rename = "single_file", skip_serializing_if = "Option::is_none")]
    pub single_file: Option<String>,
}

impl BranchRestrictionPolicyAppsInnerPermissions {
    pub fn new() -> BranchRestrictionPolicyAppsInnerPermissions {
        BranchRestrictionPolicyAppsInnerPermissions {
            metadata: None,
            contents: None,
            issues: None,
            single_file: None,
        }
    }
}
