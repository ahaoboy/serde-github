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

/// SecretScanningLocationPullRequestReview : Represents a 'pull_request_review' secret scanning location type. This location type shows that a secret was detected in a review on a pull request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretScanningLocationPullRequestReview {
    /// The API URL to get the pull request review where the secret was detected.
    #[serde(rename = "pull_request_review_url")]
    pub pull_request_review_url: String,
}

impl SecretScanningLocationPullRequestReview {
    /// Represents a 'pull_request_review' secret scanning location type. This location type shows that a secret was detected in a review on a pull request.
    pub fn new(pull_request_review_url: String) -> SecretScanningLocationPullRequestReview {
        SecretScanningLocationPullRequestReview {
            pull_request_review_url,
        }
    }
}
