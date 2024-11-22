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
pub struct PullRequest11Base {
    #[serde(rename = "label", deserialize_with = "Option::deserialize")]
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub r#ref: String,
    #[serde(rename = "repo")]
    pub repo: Box<models::Repository3>,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::User>>,
}

impl PullRequest11Base {
    pub fn new(label: Option<String>, r#ref: String, repo: models::Repository3, sha: String, user: Option<models::User>) -> PullRequest11Base {
        PullRequest11Base {
            label,
            r#ref,
            repo: Box::new(repo),
            sha,
            user: if let Some(x) = user {Some(Box::new(x))} else {None},
        }
    }
}
