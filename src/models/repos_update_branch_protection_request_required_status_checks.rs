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

/// ReposUpdateBranchProtectionRequestRequiredStatusChecks : Require status checks to pass before merging. Set to `null` to disable.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReposUpdateBranchProtectionRequestRequiredStatusChecks {
    /// Require branches to be up to date before merging.
    #[serde(rename = "strict")]
    pub strict: bool,
    /// **Deprecated**: The list of status checks to require in order to merge into this branch. If any of these checks have recently been set by a particular GitHub App, they will be required to come from that app in future for the branch to merge. Use `checks` instead of `contexts` for more fine-grained control. 
    #[serde(rename = "contexts")]
    pub contexts: Vec<String>,
    /// The list of status checks to require in order to merge into this branch.
    #[serde(rename = "checks", skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<models::ReposUpdateBranchProtectionRequestRequiredStatusChecksChecksInner>>,
}

impl ReposUpdateBranchProtectionRequestRequiredStatusChecks {
    /// Require status checks to pass before merging. Set to `null` to disable.
    pub fn new(strict: bool, contexts: Vec<String>) -> ReposUpdateBranchProtectionRequestRequiredStatusChecks {
        ReposUpdateBranchProtectionRequestRequiredStatusChecks {
            strict,
            contexts,
            checks: None,
        }
    }
}

