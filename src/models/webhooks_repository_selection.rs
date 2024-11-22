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

/// WebhooksRepositorySelection : Describe whether all repositories have been selected or there's a selection involved
/// Describe whether all repositories have been selected or there's a selection involved
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WebhooksRepositorySelection {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "selected")]
    Selected,

}

impl std::fmt::Display for WebhooksRepositorySelection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Selected => write!(f, "selected"),
        }
    }
}

impl Default for WebhooksRepositorySelection {
    fn default() -> WebhooksRepositorySelection {
        Self::All
    }
}

