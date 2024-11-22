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
pub struct ProjectsUpdateColumnRequest {
    /// Name of the project column
    #[serde(rename = "name")]
    pub name: String,
}

impl ProjectsUpdateColumnRequest {
    pub fn new(name: String) -> ProjectsUpdateColumnRequest {
        ProjectsUpdateColumnRequest {
            name,
        }
    }
}

