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

/// SecurityAdvisoryCreditTypes : The type of credit the user is receiving.
/// The type of credit the user is receiving.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecurityAdvisoryCreditTypes {
    #[serde(rename = "analyst")]
    Analyst,
    #[serde(rename = "finder")]
    Finder,
    #[serde(rename = "reporter")]
    Reporter,
    #[serde(rename = "coordinator")]
    Coordinator,
    #[serde(rename = "remediation_developer")]
    RemediationDeveloper,
    #[serde(rename = "remediation_reviewer")]
    RemediationReviewer,
    #[serde(rename = "remediation_verifier")]
    RemediationVerifier,
    #[serde(rename = "tool")]
    Tool,
    #[serde(rename = "sponsor")]
    Sponsor,
    #[serde(rename = "other")]
    Other,

}

impl std::fmt::Display for SecurityAdvisoryCreditTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Analyst => write!(f, "analyst"),
            Self::Finder => write!(f, "finder"),
            Self::Reporter => write!(f, "reporter"),
            Self::Coordinator => write!(f, "coordinator"),
            Self::RemediationDeveloper => write!(f, "remediation_developer"),
            Self::RemediationReviewer => write!(f, "remediation_reviewer"),
            Self::RemediationVerifier => write!(f, "remediation_verifier"),
            Self::Tool => write!(f, "tool"),
            Self::Sponsor => write!(f, "sponsor"),
            Self::Other => write!(f, "other"),
        }
    }
}

impl Default for SecurityAdvisoryCreditTypes {
    fn default() -> SecurityAdvisoryCreditTypes {
        Self::Analyst
    }
}

