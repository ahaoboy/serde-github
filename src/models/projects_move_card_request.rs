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
pub struct ProjectsMoveCardRequest {
    /// The position of the card in a column. Can be one of: `top`, `bottom`, or `after:<card_id>` to place after the specified card.
    #[serde(rename = "position")]
    pub position: String,
    /// The unique identifier of the column the card should be moved to
    #[serde(rename = "column_id", skip_serializing_if = "Option::is_none")]
    pub column_id: Option<i32>,
}

impl ProjectsMoveCardRequest {
    pub fn new(position: String) -> ProjectsMoveCardRequest {
        ProjectsMoveCardRequest {
            position,
            column_id: None,
        }
    }
}

