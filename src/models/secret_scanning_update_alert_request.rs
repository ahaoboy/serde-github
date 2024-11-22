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
pub struct SecretScanningUpdateAlertRequest {
    #[serde(rename = "state")]
    pub state: models::SecretScanningAlertState,
    #[serde(rename = "resolution", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Option<models::SecretScanningAlertResolution>>,
    /// An optional comment when closing an alert. Cannot be updated or deleted. Must be `null` when changing `state` to `open`.
    #[serde(rename = "resolution_comment", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub resolution_comment: Option<Option<String>>,
}

impl SecretScanningUpdateAlertRequest {
    pub fn new(state: models::SecretScanningAlertState) -> SecretScanningUpdateAlertRequest {
        SecretScanningUpdateAlertRequest {
            state,
            resolution: None,
            resolution_comment: None,
        }
    }
}

