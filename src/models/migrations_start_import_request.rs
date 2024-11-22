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
pub struct MigrationsStartImportRequest {
    /// The URL of the originating repository.
    #[serde(rename = "vcs_url")]
    pub vcs_url: String,
    /// The originating VCS type. Without this parameter, the import job will take additional time to detect the VCS type before beginning the import. This detection step will be reflected in the response.
    #[serde(rename = "vcs", skip_serializing_if = "Option::is_none")]
    pub vcs: Option<Vcs>,
    /// If authentication is required, the username to provide to `vcs_url`.
    #[serde(rename = "vcs_username", skip_serializing_if = "Option::is_none")]
    pub vcs_username: Option<String>,
    /// If authentication is required, the password to provide to `vcs_url`.
    #[serde(rename = "vcs_password", skip_serializing_if = "Option::is_none")]
    pub vcs_password: Option<String>,
    /// For a tfvc import, the name of the project that is being imported.
    #[serde(rename = "tfvc_project", skip_serializing_if = "Option::is_none")]
    pub tfvc_project: Option<String>,
}

impl MigrationsStartImportRequest {
    pub fn new(vcs_url: String) -> MigrationsStartImportRequest {
        MigrationsStartImportRequest {
            vcs_url,
            vcs: None,
            vcs_username: None,
            vcs_password: None,
            tfvc_project: None,
        }
    }
}
/// The originating VCS type. Without this parameter, the import job will take additional time to detect the VCS type before beginning the import. This detection step will be reflected in the response.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Vcs {
    #[serde(rename = "subversion")]
    Subversion,
    #[serde(rename = "git")]
    Git,
    #[serde(rename = "mercurial")]
    Mercurial,
    #[serde(rename = "tfvc")]
    Tfvc,
}

impl Default for Vcs {
    fn default() -> Vcs {
        Self::Subversion
    }
}

