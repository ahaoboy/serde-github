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
pub struct ReposCreateOrUpdateCustomPropertiesValuesRequest {
    /// A list of custom property names and associated values to apply to the repositories.
    #[serde(rename = "properties")]
    pub properties: Vec<models::CustomPropertyValue>,
}

impl ReposCreateOrUpdateCustomPropertiesValuesRequest {
    pub fn new(properties: Vec<models::CustomPropertyValue>) -> ReposCreateOrUpdateCustomPropertiesValuesRequest {
        ReposCreateOrUpdateCustomPropertiesValuesRequest {
            properties,
        }
    }
}
