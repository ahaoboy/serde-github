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

/// PageDeploymentId : The ID of the GitHub Pages deployment. This is the Git SHA of the deployed commit.
/// The ID of the GitHub Pages deployment. This is the Git SHA of the deployed commit.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PageDeploymentId {
    Integer(i32),
    String(String),
}

impl Default for PageDeploymentId {
    fn default() -> Self {
        Self::Integer(Default::default())
    }
}

