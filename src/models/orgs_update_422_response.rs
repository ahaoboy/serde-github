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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrgsUpdate422Response {
    ValidationError(Box<models::ValidationError>),
    ValidationErrorSimple(Box<models::ValidationErrorSimple>),
}

impl Default for OrgsUpdate422Response {
    fn default() -> Self {
        Self::ValidationError(Default::default())
    }
}
