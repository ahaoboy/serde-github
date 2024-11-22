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
pub struct WebhookCustomPropertyDeletedDefinition {
    /// The name of the property that was deleted.
    #[serde(rename = "property_name")]
    pub property_name: String,
}

impl WebhookCustomPropertyDeletedDefinition {
    pub fn new(property_name: String) -> WebhookCustomPropertyDeletedDefinition {
        WebhookCustomPropertyDeletedDefinition {
            property_name,
        }
    }
}

