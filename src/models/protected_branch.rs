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

/// ProtectedBranch : Branch protections protect branches
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectedBranch {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "required_status_checks", skip_serializing_if = "Option::is_none")]
    pub required_status_checks: Option<Box<models::StatusCheckPolicy>>,
    #[serde(rename = "required_pull_request_reviews", skip_serializing_if = "Option::is_none")]
    pub required_pull_request_reviews: Option<Box<models::ProtectedBranchRequiredPullRequestReviews>>,
    #[serde(rename = "required_signatures", skip_serializing_if = "Option::is_none")]
    pub required_signatures: Option<Box<models::BranchProtectionRequiredSignatures>>,
    #[serde(rename = "enforce_admins", skip_serializing_if = "Option::is_none")]
    pub enforce_admins: Option<Box<models::ProtectedBranchEnforceAdmins>>,
    #[serde(rename = "required_linear_history", skip_serializing_if = "Option::is_none")]
    pub required_linear_history: Option<Box<models::ProtectedBranchRequiredLinearHistory>>,
    #[serde(rename = "allow_force_pushes", skip_serializing_if = "Option::is_none")]
    pub allow_force_pushes: Option<Box<models::ProtectedBranchRequiredLinearHistory>>,
    #[serde(rename = "allow_deletions", skip_serializing_if = "Option::is_none")]
    pub allow_deletions: Option<Box<models::ProtectedBranchRequiredLinearHistory>>,
    #[serde(rename = "restrictions", skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Box<models::BranchRestrictionPolicy>>,
    #[serde(rename = "required_conversation_resolution", skip_serializing_if = "Option::is_none")]
    pub required_conversation_resolution: Option<Box<models::ProtectedBranchRequiredConversationResolution>>,
    #[serde(rename = "block_creations", skip_serializing_if = "Option::is_none")]
    pub block_creations: Option<Box<models::ProtectedBranchRequiredLinearHistory>>,
    #[serde(rename = "lock_branch", skip_serializing_if = "Option::is_none")]
    pub lock_branch: Option<Box<models::ProtectedBranchLockBranch>>,
    #[serde(rename = "allow_fork_syncing", skip_serializing_if = "Option::is_none")]
    pub allow_fork_syncing: Option<Box<models::ProtectedBranchAllowForkSyncing>>,
}

impl ProtectedBranch {
    /// Branch protections protect branches
    pub fn new(url: String) -> ProtectedBranch {
        ProtectedBranch {
            url,
            required_status_checks: None,
            required_pull_request_reviews: None,
            required_signatures: None,
            enforce_admins: None,
            required_linear_history: None,
            allow_force_pushes: None,
            allow_deletions: None,
            restrictions: None,
            required_conversation_resolution: None,
            block_creations: None,
            lock_branch: None,
            allow_fork_syncing: None,
        }
    }
}

