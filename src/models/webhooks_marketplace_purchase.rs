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
pub struct WebhooksMarketplacePurchase {
    #[serde(rename = "account")]
    pub account: Box<models::WebhooksMarketplacePurchaseAccount>,
    #[serde(rename = "billing_cycle")]
    pub billing_cycle: String,
    #[serde(rename = "free_trial_ends_on", deserialize_with = "Option::deserialize")]
    pub free_trial_ends_on: Option<String>,
    #[serde(rename = "next_billing_date", deserialize_with = "Option::deserialize")]
    pub next_billing_date: Option<String>,
    #[serde(rename = "on_free_trial")]
    pub on_free_trial: bool,
    #[serde(rename = "plan")]
    pub plan: Box<models::WebhooksMarketplacePurchasePlan>,
    #[serde(rename = "unit_count")]
    pub unit_count: i32,
}

impl WebhooksMarketplacePurchase {
    pub fn new(account: models::WebhooksMarketplacePurchaseAccount, billing_cycle: String, free_trial_ends_on: Option<String>, next_billing_date: Option<String>, on_free_trial: bool, plan: models::WebhooksMarketplacePurchasePlan, unit_count: i32) -> WebhooksMarketplacePurchase {
        WebhooksMarketplacePurchase {
            account: Box::new(account),
            billing_cycle,
            free_trial_ends_on,
            next_billing_date,
            on_free_trial,
            plan: Box::new(plan),
            unit_count,
        }
    }
}
