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
pub struct CheckSuitePreferencePreferencesAutoTriggerChecksInner {
    #[serde(rename = "app_id")]
    pub app_id: i32,
    #[serde(rename = "setting")]
    pub setting: bool,
}

impl CheckSuitePreferencePreferencesAutoTriggerChecksInner {
    pub fn new(app_id: i32, setting: bool) -> CheckSuitePreferencePreferencesAutoTriggerChecksInner {
        CheckSuitePreferencePreferencesAutoTriggerChecksInner {
            app_id,
            setting,
        }
    }
}

