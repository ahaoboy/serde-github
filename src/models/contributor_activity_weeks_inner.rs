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
pub struct ContributorActivityWeeksInner {
    #[serde(rename = "w", skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(rename = "a", skip_serializing_if = "Option::is_none")]
    pub a: Option<i32>,
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub d: Option<i32>,
    #[serde(rename = "c", skip_serializing_if = "Option::is_none")]
    pub c: Option<i32>,
}

impl ContributorActivityWeeksInner {
    pub fn new() -> ContributorActivityWeeksInner {
        ContributorActivityWeeksInner {
            w: None,
            a: None,
            d: None,
            c: None,
        }
    }
}

