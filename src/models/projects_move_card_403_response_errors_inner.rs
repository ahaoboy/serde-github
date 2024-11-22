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
pub struct ProjectsMoveCard403ResponseErrorsInner {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}

impl ProjectsMoveCard403ResponseErrorsInner {
    pub fn new() -> ProjectsMoveCard403ResponseErrorsInner {
        ProjectsMoveCard403ResponseErrorsInner {
            code: None,
            message: None,
            resource: None,
            field: None,
        }
    }
}

