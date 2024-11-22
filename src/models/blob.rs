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

/// Blob : Blob
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Blob {
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "encoding")]
    pub encoding: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "size", deserialize_with = "Option::deserialize")]
    pub size: Option<i32>,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "highlighted_content", skip_serializing_if = "Option::is_none")]
    pub highlighted_content: Option<String>,
}

impl Blob {
    /// Blob
    pub fn new(content: String, encoding: String, url: String, sha: String, size: Option<i32>, node_id: String) -> Blob {
        Blob {
            content,
            encoding,
            url,
            sha,
            size,
            node_id,
            highlighted_content: None,
        }
    }
}

