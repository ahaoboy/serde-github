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
pub struct GitCreateBlobRequest {
    /// The new blob's content.
    #[serde(rename = "content")]
    pub content: String,
    /// The encoding used for `content`. Currently, `\"utf-8\"` and `\"base64\"` are supported.
    #[serde(rename = "encoding", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
}

impl GitCreateBlobRequest {
    pub fn new(content: String) -> GitCreateBlobRequest {
        GitCreateBlobRequest {
            content,
            encoding: None,
        }
    }
}
