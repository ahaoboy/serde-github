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
pub struct WebhookPackagePublishedPackagePackageVersionNugetMetadataInnerValueOneOf {
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "branch", skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(rename = "commit", skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl WebhookPackagePublishedPackagePackageVersionNugetMetadataInnerValueOneOf {
    pub fn new() -> WebhookPackagePublishedPackagePackageVersionNugetMetadataInnerValueOneOf {
        WebhookPackagePublishedPackagePackageVersionNugetMetadataInnerValueOneOf {
            url: None,
            branch: None,
            commit: None,
            r#type: None,
        }
    }
}

