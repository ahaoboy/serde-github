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
pub struct PullRequestHeadRepoLicense {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "url", deserialize_with = "Option::deserialize")]
    pub url: Option<String>,
    #[serde(rename = "spdx_id", deserialize_with = "Option::deserialize")]
    pub spdx_id: Option<String>,
    #[serde(rename = "node_id")]
    pub node_id: String,
}

impl PullRequestHeadRepoLicense {
    pub fn new(key: String, name: String, url: Option<String>, spdx_id: Option<String>, node_id: String) -> PullRequestHeadRepoLicense {
        PullRequestHeadRepoLicense {
            key,
            name,
            url,
            spdx_id,
            node_id,
        }
    }
}

