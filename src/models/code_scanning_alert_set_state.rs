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

/// CodeScanningAlertSetState : Sets the state of the code scanning alert. You must provide `dismissed_reason` when you set the state to `dismissed`.
/// Sets the state of the code scanning alert. You must provide `dismissed_reason` when you set the state to `dismissed`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CodeScanningAlertSetState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "dismissed")]
    Dismissed,

}

impl std::fmt::Display for CodeScanningAlertSetState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Dismissed => write!(f, "dismissed"),
        }
    }
}

impl Default for CodeScanningAlertSetState {
    fn default() -> CodeScanningAlertSetState {
        Self::Open
    }
}

