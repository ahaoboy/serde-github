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

/// DependabotSecret : Set secrets for Dependabot.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DependabotSecret {
    /// The name of the secret.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl DependabotSecret {
    /// Set secrets for Dependabot.
    pub fn new(name: String, created_at: String, updated_at: String) -> DependabotSecret {
        DependabotSecret {
            name,
            created_at,
            updated_at,
        }
    }
}
