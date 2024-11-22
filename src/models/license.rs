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
pub struct License {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "spdx_id")]
    pub spdx_id: String,
    #[serde(rename = "url", deserialize_with = "Option::deserialize")]
    pub url: Option<String>,
}

impl License {
    pub fn new(key: String, name: String, node_id: String, spdx_id: String, url: Option<String>) -> License {
        License {
            key,
            name,
            node_id,
            spdx_id,
            url,
        }
    }
}

