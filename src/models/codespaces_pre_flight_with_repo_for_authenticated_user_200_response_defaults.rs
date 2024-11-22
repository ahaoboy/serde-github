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
pub struct CodespacesPreFlightWithRepoForAuthenticatedUser200ResponseDefaults {
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "devcontainer_path", deserialize_with = "Option::deserialize")]
    pub devcontainer_path: Option<String>,
}

impl CodespacesPreFlightWithRepoForAuthenticatedUser200ResponseDefaults {
    pub fn new(location: String, devcontainer_path: Option<String>) -> CodespacesPreFlightWithRepoForAuthenticatedUser200ResponseDefaults {
        CodespacesPreFlightWithRepoForAuthenticatedUser200ResponseDefaults {
            location,
            devcontainer_path,
        }
    }
}

