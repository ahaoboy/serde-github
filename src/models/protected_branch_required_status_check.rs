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

/// ProtectedBranchRequiredStatusCheck : Protected Branch Required Status Check
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectedBranchRequiredStatusCheck {
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "enforcement_level", skip_serializing_if = "Option::is_none")]
    pub enforcement_level: Option<String>,
    #[serde(rename = "contexts")]
    pub contexts: Vec<String>,
    #[serde(rename = "checks")]
    pub checks: Vec<models::ProtectedBranchRequiredStatusCheckChecksInner>,
    #[serde(rename = "contexts_url", skip_serializing_if = "Option::is_none")]
    pub contexts_url: Option<String>,
    #[serde(rename = "strict", skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

impl ProtectedBranchRequiredStatusCheck {
    /// Protected Branch Required Status Check
    pub fn new(contexts: Vec<String>, checks: Vec<models::ProtectedBranchRequiredStatusCheckChecksInner>) -> ProtectedBranchRequiredStatusCheck {
        ProtectedBranchRequiredStatusCheck {
            url: None,
            enforcement_level: None,
            contexts,
            checks,
            contexts_url: None,
            strict: None,
        }
    }
}

