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

/// PorterAuthor : Porter Author
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PorterAuthor {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "remote_id")]
    pub remote_id: String,
    #[serde(rename = "remote_name")]
    pub remote_name: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "import_url")]
    pub import_url: String,
}

impl PorterAuthor {
    /// Porter Author
    pub fn new(id: i32, remote_id: String, remote_name: String, email: String, name: String, url: String, import_url: String) -> PorterAuthor {
        PorterAuthor {
            id,
            remote_id,
            remote_name,
            email,
            name,
            url,
            import_url,
        }
    }
}

