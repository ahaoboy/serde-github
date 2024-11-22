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
pub struct SearchUsers200Response {
    #[serde(rename = "total_count")]
    pub total_count: i32,
    #[serde(rename = "incomplete_results")]
    pub incomplete_results: bool,
    #[serde(rename = "items")]
    pub items: Vec<models::UserSearchResultItem>,
}

impl SearchUsers200Response {
    pub fn new(total_count: i32, incomplete_results: bool, items: Vec<models::UserSearchResultItem>) -> SearchUsers200Response {
        SearchUsers200Response {
            total_count,
            incomplete_results,
            items,
        }
    }
}

