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
pub enum WebhookPullRequestReviewRequested {
    WebhookPullRequestReviewRequestedOneOf(Box<models::WebhookPullRequestReviewRequestedOneOf>),
    WebhookPullRequestReviewRequestedOneOf1(Box<models::WebhookPullRequestReviewRequestedOneOf1>),
}

impl Default for WebhookPullRequestReviewRequested {
    fn default() -> Self {
        Self::WebhookPullRequestReviewRequestedOneOf(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "review_requested")]
    ReviewRequested,
}

impl Default for Action {
    fn default() -> Action {
        Self::ReviewRequested
    }
}
