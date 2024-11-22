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
pub struct OrgsListAppInstallations200Response {
    #[serde(rename = "total_count")]
    pub total_count: i32,
    #[serde(rename = "installations")]
    pub installations: Vec<models::Installation>,
}

impl OrgsListAppInstallations200Response {
    pub fn new(total_count: i32, installations: Vec<models::Installation>) -> OrgsListAppInstallations200Response {
        OrgsListAppInstallations200Response {
            total_count,
            installations,
        }
    }
}

