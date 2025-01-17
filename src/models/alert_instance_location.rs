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
pub struct AlertInstanceLocation {
    #[serde(rename = "end_column", skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i32>,
    #[serde(rename = "end_line", skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i32>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "start_column", skip_serializing_if = "Option::is_none")]
    pub start_column: Option<i32>,
    #[serde(rename = "start_line", skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i32>,
}

impl AlertInstanceLocation {
    pub fn new() -> AlertInstanceLocation {
        AlertInstanceLocation {
            end_column: None,
            end_line: None,
            path: None,
            start_column: None,
            start_line: None,
        }
    }
}

