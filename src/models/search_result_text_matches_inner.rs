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
pub struct SearchResultTextMatchesInner {
    #[serde(rename = "object_url", skip_serializing_if = "Option::is_none")]
    pub object_url: Option<String>,
    #[serde(rename = "object_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<Option<String>>,
    #[serde(rename = "property", skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    #[serde(rename = "fragment", skip_serializing_if = "Option::is_none")]
    pub fragment: Option<String>,
    #[serde(rename = "matches", skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<models::SearchResultTextMatchesInnerMatchesInner>>,
}

impl SearchResultTextMatchesInner {
    pub fn new() -> SearchResultTextMatchesInner {
        SearchResultTextMatchesInner {
            object_url: None,
            object_type: None,
            property: None,
            fragment: None,
            matches: None,
        }
    }
}

