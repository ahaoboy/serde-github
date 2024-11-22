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
pub struct IssueLabelsInnerOneOf {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "node_id", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "color", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<String>>,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

impl IssueLabelsInnerOneOf {
    pub fn new() -> IssueLabelsInnerOneOf {
        IssueLabelsInnerOneOf {
            id: None,
            node_id: None,
            url: None,
            name: None,
            description: None,
            color: None,
            default: None,
        }
    }
}
