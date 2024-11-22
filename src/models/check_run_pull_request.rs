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
pub struct CheckRunPullRequest {
    #[serde(rename = "base")]
    pub base: Box<models::CheckRunPullRequestBase>,
    #[serde(rename = "head")]
    pub head: Box<models::CheckRunPullRequestBase>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "number")]
    pub number: i32,
    #[serde(rename = "url")]
    pub url: String,
}

impl CheckRunPullRequest {
    pub fn new(base: models::CheckRunPullRequestBase, head: models::CheckRunPullRequestBase, id: i32, number: i32, url: String) -> CheckRunPullRequest {
        CheckRunPullRequest {
            base: Box::new(base),
            head: Box::new(head),
            id,
            number,
            url,
        }
    }
}

