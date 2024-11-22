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
pub struct ProjectsCreateForOrgRequest {
    /// The name of the project.
    #[serde(rename = "name")]
    pub name: String,
    /// The description of the project.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl ProjectsCreateForOrgRequest {
    pub fn new(name: String) -> ProjectsCreateForOrgRequest {
        ProjectsCreateForOrgRequest {
            name,
            body: None,
        }
    }
}

