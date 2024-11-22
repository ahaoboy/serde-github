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

/// FileCommit : File Commit
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileCommit {
    #[serde(rename = "content", deserialize_with = "Option::deserialize")]
    pub content: Option<Box<models::FileCommitContent>>,
    #[serde(rename = "commit")]
    pub commit: Box<models::FileCommitCommit>,
}

impl FileCommit {
    /// File Commit
    pub fn new(content: Option<models::FileCommitContent>, commit: models::FileCommitCommit) -> FileCommit {
        FileCommit {
            content: if let Some(x) = content {Some(Box::new(x))} else {None},
            commit: Box::new(commit),
        }
    }
}

