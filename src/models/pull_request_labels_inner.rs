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
pub struct PullRequestLabelsInner {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    #[serde(rename = "color")]
    pub color: String,
    #[serde(rename = "default")]
    pub default: bool,
}

impl PullRequestLabelsInner {
    pub fn new(id: i64, node_id: String, url: String, name: String, description: Option<String>, color: String, default: bool) -> PullRequestLabelsInner {
        PullRequestLabelsInner {
            id,
            node_id,
            url,
            name,
            description,
            color,
            default,
        }
    }
}

