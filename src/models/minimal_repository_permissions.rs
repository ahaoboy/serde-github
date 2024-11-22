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
pub struct MinimalRepositoryPermissions {
    #[serde(rename = "admin", skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[serde(rename = "maintain", skip_serializing_if = "Option::is_none")]
    pub maintain: Option<bool>,
    #[serde(rename = "push", skip_serializing_if = "Option::is_none")]
    pub push: Option<bool>,
    #[serde(rename = "triage", skip_serializing_if = "Option::is_none")]
    pub triage: Option<bool>,
    #[serde(rename = "pull", skip_serializing_if = "Option::is_none")]
    pub pull: Option<bool>,
}

impl MinimalRepositoryPermissions {
    pub fn new() -> MinimalRepositoryPermissions {
        MinimalRepositoryPermissions {
            admin: None,
            maintain: None,
            push: None,
            triage: None,
            pull: None,
        }
    }
}
