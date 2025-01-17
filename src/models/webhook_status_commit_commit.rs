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
pub struct WebhookStatusCommitCommit {
    #[serde(rename = "author")]
    pub author: Box<models::WebhookStatusCommitCommitAuthor>,
    #[serde(rename = "comment_count")]
    pub comment_count: i32,
    #[serde(rename = "committer")]
    pub committer: Box<models::WebhookStatusCommitCommitAuthor>,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "tree")]
    pub tree: Box<models::ShortBranchCommit>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "verification")]
    pub verification: Box<models::WebhookStatusCommitCommitVerification>,
}

impl WebhookStatusCommitCommit {
    pub fn new(author: models::WebhookStatusCommitCommitAuthor, comment_count: i32, committer: models::WebhookStatusCommitCommitAuthor, message: String, tree: models::ShortBranchCommit, url: String, verification: models::WebhookStatusCommitCommitVerification) -> WebhookStatusCommitCommit {
        WebhookStatusCommitCommit {
            author: Box::new(author),
            comment_count,
            committer: Box::new(committer),
            message,
            tree: Box::new(tree),
            url,
            verification: Box::new(verification),
        }
    }
}

