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

/// ReferrerTraffic : Referrer Traffic
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferrerTraffic {
    #[serde(rename = "referrer")]
    pub referrer: String,
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "uniques")]
    pub uniques: i32,
}

impl ReferrerTraffic {
    /// Referrer Traffic
    pub fn new(referrer: String, count: i32, uniques: i32) -> ReferrerTraffic {
        ReferrerTraffic {
            referrer,
            count,
            uniques,
        }
    }
}
