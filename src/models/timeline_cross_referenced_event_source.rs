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
pub struct TimelineCrossReferencedEventSource {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "issue", skip_serializing_if = "Option::is_none")]
    pub issue: Option<Box<models::Issue>>,
}

impl TimelineCrossReferencedEventSource {
    pub fn new() -> TimelineCrossReferencedEventSource {
        TimelineCrossReferencedEventSource {
            r#type: None,
            issue: None,
        }
    }
}
