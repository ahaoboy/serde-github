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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecretScanningLocationDetails {
    SecretScanningLocationCommit(Box<models::SecretScanningLocationCommit>),
    SecretScanningLocationWikiCommit(Box<models::SecretScanningLocationWikiCommit>),
    SecretScanningLocationIssueTitle(Box<models::SecretScanningLocationIssueTitle>),
    SecretScanningLocationIssueBody(Box<models::SecretScanningLocationIssueBody>),
    SecretScanningLocationIssueComment(Box<models::SecretScanningLocationIssueComment>),
    SecretScanningLocationDiscussionTitle(Box<models::SecretScanningLocationDiscussionTitle>),
    SecretScanningLocationDiscussionBody(Box<models::SecretScanningLocationDiscussionBody>),
    SecretScanningLocationDiscussionComment(Box<models::SecretScanningLocationDiscussionComment>),
    SecretScanningLocationPullRequestTitle(Box<models::SecretScanningLocationPullRequestTitle>),
    SecretScanningLocationPullRequestBody(Box<models::SecretScanningLocationPullRequestBody>),
    SecretScanningLocationPullRequestComment(Box<models::SecretScanningLocationPullRequestComment>),
    SecretScanningLocationPullRequestReview(Box<models::SecretScanningLocationPullRequestReview>),
    SecretScanningLocationPullRequestReviewComment(Box<models::SecretScanningLocationPullRequestReviewComment>),
}

impl Default for SecretScanningLocationDetails {
    fn default() -> Self {
        Self::SecretScanningLocationCommit(Default::default())
    }
}

