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
pub struct WorkflowRunPullRequestsInner {
    #[serde(rename = "base")]
    pub base: Box<models::CheckRunPullRequestBase>,
    #[serde(rename = "head")]
    pub head: Box<models::CheckRunPullRequestBase>,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "number")]
    pub number: f64,
    #[serde(rename = "url")]
    pub url: String,
}

impl WorkflowRunPullRequestsInner {
    pub fn new(base: models::CheckRunPullRequestBase, head: models::CheckRunPullRequestBase, id: f64, number: f64, url: String) -> WorkflowRunPullRequestsInner {
        WorkflowRunPullRequestsInner {
            base: Box::new(base),
            head: Box::new(head),
            id,
            number,
            url,
        }
    }
}

