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
pub struct OrgsListOrgRoles200Response {
    /// The total number of organization roles available to the organization.
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// The list of organization roles available to the organization.
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<models::OrganizationRole>>,
}

impl OrgsListOrgRoles200Response {
    pub fn new() -> OrgsListOrgRoles200Response {
        OrgsListOrgRoles200Response {
            total_count: None,
            roles: None,
        }
    }
}
