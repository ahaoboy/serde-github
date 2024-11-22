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
pub struct WebhooksSecurityAdvisoryVulnerabilitiesInner {
    #[serde(rename = "first_patched_version", deserialize_with = "Option::deserialize")]
    pub first_patched_version: Option<Box<models::WebhooksSecurityAdvisoryVulnerabilitiesInnerFirstPatchedVersion>>,
    #[serde(rename = "package")]
    pub package: Box<models::WebhooksSecurityAdvisoryVulnerabilitiesInnerPackage>,
    #[serde(rename = "severity")]
    pub severity: String,
    #[serde(rename = "vulnerable_version_range")]
    pub vulnerable_version_range: String,
}

impl WebhooksSecurityAdvisoryVulnerabilitiesInner {
    pub fn new(first_patched_version: Option<models::WebhooksSecurityAdvisoryVulnerabilitiesInnerFirstPatchedVersion>, package: models::WebhooksSecurityAdvisoryVulnerabilitiesInnerPackage, severity: String, vulnerable_version_range: String) -> WebhooksSecurityAdvisoryVulnerabilitiesInner {
        WebhooksSecurityAdvisoryVulnerabilitiesInner {
            first_patched_version: if let Some(x) = first_patched_version {Some(Box::new(x))} else {None},
            package: Box::new(package),
            severity,
            vulnerable_version_range,
        }
    }
}

