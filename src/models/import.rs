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

/// Import : A repository import from an external source.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Import {
    #[serde(rename = "vcs", deserialize_with = "Option::deserialize")]
    pub vcs: Option<String>,
    #[serde(rename = "use_lfs", skip_serializing_if = "Option::is_none")]
    pub use_lfs: Option<bool>,
    /// The URL of the originating repository.
    #[serde(rename = "vcs_url")]
    pub vcs_url: String,
    #[serde(rename = "svc_root", skip_serializing_if = "Option::is_none")]
    pub svc_root: Option<String>,
    #[serde(rename = "tfvc_project", skip_serializing_if = "Option::is_none")]
    pub tfvc_project: Option<String>,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "status_text", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status_text: Option<Option<String>>,
    #[serde(rename = "failed_step", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub failed_step: Option<Option<String>>,
    #[serde(rename = "error_message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<Option<String>>,
    #[serde(rename = "import_percent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub import_percent: Option<Option<i32>>,
    #[serde(rename = "commit_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub commit_count: Option<Option<i32>>,
    #[serde(rename = "push_percent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub push_percent: Option<Option<i32>>,
    #[serde(rename = "has_large_files", skip_serializing_if = "Option::is_none")]
    pub has_large_files: Option<bool>,
    #[serde(rename = "large_files_size", skip_serializing_if = "Option::is_none")]
    pub large_files_size: Option<i32>,
    #[serde(rename = "large_files_count", skip_serializing_if = "Option::is_none")]
    pub large_files_count: Option<i32>,
    #[serde(rename = "project_choices", skip_serializing_if = "Option::is_none")]
    pub project_choices: Option<Vec<models::ImportProjectChoicesInner>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "authors_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub authors_count: Option<Option<i32>>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "authors_url")]
    pub authors_url: String,
    #[serde(rename = "repository_url")]
    pub repository_url: String,
    #[serde(rename = "svn_root", skip_serializing_if = "Option::is_none")]
    pub svn_root: Option<String>,
}

impl Import {
    /// A repository import from an external source.
    pub fn new(vcs: Option<String>, vcs_url: String, status: Status, url: String, html_url: String, authors_url: String, repository_url: String) -> Import {
        Import {
            vcs,
            use_lfs: None,
            vcs_url,
            svc_root: None,
            tfvc_project: None,
            status,
            status_text: None,
            failed_step: None,
            error_message: None,
            import_percent: None,
            commit_count: None,
            push_percent: None,
            has_large_files: None,
            large_files_size: None,
            large_files_count: None,
            project_choices: None,
            message: None,
            authors_count: None,
            url,
            html_url,
            authors_url,
            repository_url,
            svn_root: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "auth")]
    Auth,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "detecting")]
    Detecting,
    #[serde(rename = "choose")]
    Choose,
    #[serde(rename = "auth_failed")]
    AuthFailed,
    #[serde(rename = "importing")]
    Importing,
    #[serde(rename = "mapping")]
    Mapping,
    #[serde(rename = "waiting_to_push")]
    WaitingToPush,
    #[serde(rename = "pushing")]
    Pushing,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "setup")]
    Setup,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "detection_found_multiple")]
    DetectionFoundMultiple,
    #[serde(rename = "detection_found_nothing")]
    DetectionFoundNothing,
    #[serde(rename = "detection_needs_auth")]
    DetectionNeedsAuth,
}

impl Default for Status {
    fn default() -> Status {
        Self::Auth
    }
}
