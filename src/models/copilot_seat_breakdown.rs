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

/// CopilotSeatBreakdown : The breakdown of Copilot Business seats for the organization.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CopilotSeatBreakdown {
    /// The total number of seats being billed for the organization as of the current billing cycle.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// Seats added during the current billing cycle.
    #[serde(rename = "added_this_cycle", skip_serializing_if = "Option::is_none")]
    pub added_this_cycle: Option<i32>,
    /// The number of seats that are pending cancellation at the end of the current billing cycle.
    #[serde(rename = "pending_cancellation", skip_serializing_if = "Option::is_none")]
    pub pending_cancellation: Option<i32>,
    /// The number of seats that have been assigned to users that have not yet accepted an invitation to this organization.
    #[serde(rename = "pending_invitation", skip_serializing_if = "Option::is_none")]
    pub pending_invitation: Option<i32>,
    /// The number of seats that have used Copilot during the current billing cycle.
    #[serde(rename = "active_this_cycle", skip_serializing_if = "Option::is_none")]
    pub active_this_cycle: Option<i32>,
    /// The number of seats that have not used Copilot during the current billing cycle.
    #[serde(rename = "inactive_this_cycle", skip_serializing_if = "Option::is_none")]
    pub inactive_this_cycle: Option<i32>,
}

impl CopilotSeatBreakdown {
    /// The breakdown of Copilot Business seats for the organization.
    pub fn new() -> CopilotSeatBreakdown {
        CopilotSeatBreakdown {
            total: None,
            added_this_cycle: None,
            pending_cancellation: None,
            pending_invitation: None,
            active_this_cycle: None,
            inactive_this_cycle: None,
        }
    }
}
