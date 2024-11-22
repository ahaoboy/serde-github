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
pub struct WebhooksMarketplacePurchasePlan {
    #[serde(rename = "bullets")]
    pub bullets: Vec<String>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "has_free_trial")]
    pub has_free_trial: bool,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "monthly_price_in_cents")]
    pub monthly_price_in_cents: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "price_model")]
    pub price_model: PriceModel,
    #[serde(rename = "unit_name", deserialize_with = "Option::deserialize")]
    pub unit_name: Option<String>,
    #[serde(rename = "yearly_price_in_cents")]
    pub yearly_price_in_cents: i32,
}

impl WebhooksMarketplacePurchasePlan {
    pub fn new(bullets: Vec<String>, description: String, has_free_trial: bool, id: i32, monthly_price_in_cents: i32, name: String, price_model: PriceModel, unit_name: Option<String>, yearly_price_in_cents: i32) -> WebhooksMarketplacePurchasePlan {
        WebhooksMarketplacePurchasePlan {
            bullets,
            description,
            has_free_trial,
            id,
            monthly_price_in_cents,
            name,
            price_model,
            unit_name,
            yearly_price_in_cents,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PriceModel {
    #[serde(rename = "FREE")]
    Free,
    #[serde(rename = "FLAT_RATE")]
    FlatRate,
    #[serde(rename = "PER_UNIT")]
    PerUnit,
}

impl Default for PriceModel {
    fn default() -> PriceModel {
        Self::Free
    }
}
