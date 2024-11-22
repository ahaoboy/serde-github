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
pub struct MaxFileSizeParameters {
    /// The maximum file size allowed in megabytes. This limit does not apply to Git Large File Storage (Git LFS).
    #[serde(rename = "max_file_size")]
    pub max_file_size: i32,
}

impl MaxFileSizeParameters {
    pub fn new(max_file_size: i32) -> MaxFileSizeParameters {
        MaxFileSizeParameters {
            max_file_size,
        }
    }
}
