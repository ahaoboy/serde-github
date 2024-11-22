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
pub enum WebhooksPullRequest5RequestedReviewersInner {
    User(Box<models::User>),
    Team(Box<models::Team>),
}

impl Default for WebhooksPullRequest5RequestedReviewersInner {
    fn default() -> Self {
        Self::User(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Bot")]
    Bot,
    #[serde(rename = "User")]
    User,
    #[serde(rename = "Organization")]
    Organization,
}

impl Default for Type {
    fn default() -> Type {
        Self::Bot
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Privacy {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "secret")]
    Secret,
}

impl Default for Privacy {
    fn default() -> Privacy {
        Self::Open
    }
}

