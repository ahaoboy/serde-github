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

/// ProjectsV2ItemContentType : The type of content tracked in a project item
/// The type of content tracked in a project item
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProjectsV2ItemContentType {
    #[serde(rename = "Issue")]
    Issue,
    #[serde(rename = "PullRequest")]
    PullRequest,
    #[serde(rename = "DraftIssue")]
    DraftIssue,

}

impl std::fmt::Display for ProjectsV2ItemContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Issue => write!(f, "Issue"),
            Self::PullRequest => write!(f, "PullRequest"),
            Self::DraftIssue => write!(f, "DraftIssue"),
        }
    }
}

impl Default for ProjectsV2ItemContentType {
    fn default() -> ProjectsV2ItemContentType {
        Self::Issue
    }
}

