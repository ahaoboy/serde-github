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

/// PageBuildStatus : Page Build Status
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageBuildStatus {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "status")]
    pub status: String,
}

impl PageBuildStatus {
    /// Page Build Status
    pub fn new(url: String, status: String) -> PageBuildStatus {
        PageBuildStatus {
            url,
            status,
        }
    }
}

