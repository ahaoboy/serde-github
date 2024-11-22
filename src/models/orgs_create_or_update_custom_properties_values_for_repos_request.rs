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
pub struct OrgsCreateOrUpdateCustomPropertiesValuesForReposRequest {
    /// The names of repositories that the custom property values will be applied to.
    #[serde(rename = "repository_names")]
    pub repository_names: Vec<String>,
    /// List of custom property names and associated values to apply to the repositories.
    #[serde(rename = "properties")]
    pub properties: Vec<models::CustomPropertyValue>,
}

impl OrgsCreateOrUpdateCustomPropertiesValuesForReposRequest {
    pub fn new(repository_names: Vec<String>, properties: Vec<models::CustomPropertyValue>) -> OrgsCreateOrUpdateCustomPropertiesValuesForReposRequest {
        OrgsCreateOrUpdateCustomPropertiesValuesForReposRequest {
            repository_names,
            properties,
        }
    }
}

