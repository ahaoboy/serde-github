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
pub struct HookResponse {
    #[serde(rename = "code", deserialize_with = "Option::deserialize")]
    pub code: Option<i32>,
    #[serde(rename = "status", deserialize_with = "Option::deserialize")]
    pub status: Option<String>,
    #[serde(rename = "message", deserialize_with = "Option::deserialize")]
    pub message: Option<String>,
}

impl HookResponse {
    pub fn new(code: Option<i32>, status: Option<String>, message: Option<String>) -> HookResponse {
        HookResponse {
            code,
            status,
            message,
        }
    }
}

