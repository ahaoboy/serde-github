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
pub struct ReposUpdateBranchProtectionRequest {
    #[serde(rename = "required_status_checks", deserialize_with = "Option::deserialize")]
    pub required_status_checks: Option<Box<models::ReposUpdateBranchProtectionRequestRequiredStatusChecks>>,
    /// Enforce all configured restrictions for administrators. Set to `true` to enforce required status checks for repository administrators. Set to `null` to disable.
    #[serde(rename = "enforce_admins", deserialize_with = "Option::deserialize")]
    pub enforce_admins: Option<bool>,
    #[serde(rename = "required_pull_request_reviews", deserialize_with = "Option::deserialize")]
    pub required_pull_request_reviews: Option<Box<models::ReposUpdateBranchProtectionRequestRequiredPullRequestReviews>>,
    #[serde(rename = "restrictions", deserialize_with = "Option::deserialize")]
    pub restrictions: Option<Box<models::ReposUpdateBranchProtectionRequestRestrictions>>,
    /// Enforces a linear commit Git history, which prevents anyone from pushing merge commits to a branch. Set to `true` to enforce a linear commit history. Set to `false` to disable a linear commit Git history. Your repository must allow squash merging or rebase merging before you can enable a linear commit history. Default: `false`. For more information, see \"[Requiring a linear commit history](https://docs.github.com/github/administering-a-repository/requiring-a-linear-commit-history)\" in the GitHub Help documentation.
    #[serde(rename = "required_linear_history", skip_serializing_if = "Option::is_none")]
    pub required_linear_history: Option<bool>,
    /// Permits force pushes to the protected branch by anyone with write access to the repository. Set to `true` to allow force pushes. Set to `false` or `null` to block force pushes. Default: `false`. For more information, see \"[Enabling force pushes to a protected branch](https://docs.github.com/github/administering-a-repository/enabling-force-pushes-to-a-protected-branch)\" in the GitHub Help documentation.\"
    #[serde(rename = "allow_force_pushes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub allow_force_pushes: Option<Option<bool>>,
    /// Allows deletion of the protected branch by anyone with write access to the repository. Set to `false` to prevent deletion of the protected branch. Default: `false`. For more information, see \"[Enabling force pushes to a protected branch](https://docs.github.com/github/administering-a-repository/enabling-force-pushes-to-a-protected-branch)\" in the GitHub Help documentation.
    #[serde(rename = "allow_deletions", skip_serializing_if = "Option::is_none")]
    pub allow_deletions: Option<bool>,
    /// If set to `true`, the `restrictions` branch protection settings which limits who can push will also block pushes which create new branches, unless the push is initiated by a user, team, or app which has the ability to push. Set to `true` to restrict new branch creation. Default: `false`.
    #[serde(rename = "block_creations", skip_serializing_if = "Option::is_none")]
    pub block_creations: Option<bool>,
    /// Requires all conversations on code to be resolved before a pull request can be merged into a branch that matches this rule. Set to `false` to disable. Default: `false`.
    #[serde(rename = "required_conversation_resolution", skip_serializing_if = "Option::is_none")]
    pub required_conversation_resolution: Option<bool>,
    /// Whether to set the branch as read-only. If this is true, users will not be able to push to the branch. Default: `false`.
    #[serde(rename = "lock_branch", skip_serializing_if = "Option::is_none")]
    pub lock_branch: Option<bool>,
    /// Whether users can pull changes from upstream when the branch is locked. Set to `true` to allow fork syncing. Set to `false` to prevent fork syncing. Default: `false`.
    #[serde(rename = "allow_fork_syncing", skip_serializing_if = "Option::is_none")]
    pub allow_fork_syncing: Option<bool>,
}

impl ReposUpdateBranchProtectionRequest {
    pub fn new(required_status_checks: Option<models::ReposUpdateBranchProtectionRequestRequiredStatusChecks>, enforce_admins: Option<bool>, required_pull_request_reviews: Option<models::ReposUpdateBranchProtectionRequestRequiredPullRequestReviews>, restrictions: Option<models::ReposUpdateBranchProtectionRequestRestrictions>) -> ReposUpdateBranchProtectionRequest {
        ReposUpdateBranchProtectionRequest {
            required_status_checks: if let Some(x) = required_status_checks {Some(Box::new(x))} else {None},
            enforce_admins,
            required_pull_request_reviews: if let Some(x) = required_pull_request_reviews {Some(Box::new(x))} else {None},
            restrictions: if let Some(x) = restrictions {Some(Box::new(x))} else {None},
            required_linear_history: None,
            allow_force_pushes: None,
            allow_deletions: None,
            block_creations: None,
            required_conversation_resolution: None,
            lock_branch: None,
            allow_fork_syncing: None,
        }
    }
}

