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
pub struct DependencyGraphSpdxSbomSbomCreationInfo {
    /// The date and time the SPDX document was created.
    #[serde(rename = "created")]
    pub created: String,
    /// The tools that were used to generate the SPDX document.
    #[serde(rename = "creators")]
    pub creators: Vec<String>,
}

impl DependencyGraphSpdxSbomSbomCreationInfo {
    pub fn new(created: String, creators: Vec<String>) -> DependencyGraphSpdxSbomSbomCreationInfo {
        DependencyGraphSpdxSbomSbomCreationInfo {
            created,
            creators,
        }
    }
}

