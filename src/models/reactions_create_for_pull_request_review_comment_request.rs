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
pub struct ReactionsCreateForPullRequestReviewCommentRequest {
    /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the pull request review comment.
    #[serde(rename = "content")]
    pub content: Content,
}

impl ReactionsCreateForPullRequestReviewCommentRequest {
    pub fn new(content: Content) -> ReactionsCreateForPullRequestReviewCommentRequest {
        ReactionsCreateForPullRequestReviewCommentRequest {
            content,
        }
    }
}
/// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the pull request review comment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Content {
    #[serde(rename = "+1")]
    Plus1,
    #[serde(rename = "-1")]
    Variant1,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
}

impl Default for Content {
    fn default() -> Content {
        Self::Plus1
    }
}

