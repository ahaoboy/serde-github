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
pub struct ReposCreateDeployKeyRequest {
    /// A name for the key.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The contents of the key.
    #[serde(rename = "key")]
    pub key: String,
    /// If `true`, the key will only be able to read repository contents. Otherwise, the key will be able to read and write.      Deploy keys with write access can perform the same actions as an organization member with admin access, or a collaborator on a personal repository. For more information, see \"[Repository permission levels for an organization](https://docs.github.com/articles/repository-permission-levels-for-an-organization/)\" and \"[Permission levels for a user account repository](https://docs.github.com/articles/permission-levels-for-a-user-account-repository/).\"
    #[serde(rename = "read_only", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

impl ReposCreateDeployKeyRequest {
    pub fn new(key: String) -> ReposCreateDeployKeyRequest {
        ReposCreateDeployKeyRequest {
            title: None,
            key,
            read_only: None,
        }
    }
}

