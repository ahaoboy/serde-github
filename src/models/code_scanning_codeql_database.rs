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

/// CodeScanningCodeqlDatabase : A CodeQL database.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningCodeqlDatabase {
    /// The ID of the CodeQL database.
    #[serde(rename = "id")]
    pub id: i32,
    /// The name of the CodeQL database.
    #[serde(rename = "name")]
    pub name: String,
    /// The language of the CodeQL database.
    #[serde(rename = "language")]
    pub language: String,
    #[serde(rename = "uploader")]
    pub uploader: Box<models::SimpleUser>,
    /// The MIME type of the CodeQL database file.
    #[serde(rename = "content_type")]
    pub content_type: String,
    /// The size of the CodeQL database file in bytes.
    #[serde(rename = "size")]
    pub size: i32,
    /// The date and time at which the CodeQL database was created, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The date and time at which the CodeQL database was last updated, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// The URL at which to download the CodeQL database. The `Accept` header must be set to the value of the `content_type` property.
    #[serde(rename = "url")]
    pub url: String,
    /// The commit SHA of the repository at the time the CodeQL database was created.
    #[serde(rename = "commit_oid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub commit_oid: Option<Option<String>>,
}

impl CodeScanningCodeqlDatabase {
    /// A CodeQL database.
    pub fn new(id: i32, name: String, language: String, uploader: models::SimpleUser, content_type: String, size: i32, created_at: String, updated_at: String, url: String) -> CodeScanningCodeqlDatabase {
        CodeScanningCodeqlDatabase {
            id,
            name,
            language,
            uploader: Box::new(uploader),
            content_type,
            size,
            created_at,
            updated_at,
            url,
            commit_oid: None,
        }
    }
}

