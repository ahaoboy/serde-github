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
pub struct WebhooksLabel {
    /// 6-character hex code, without the leading #, identifying the color
    #[serde(rename = "color")]
    pub color: String,
    #[serde(rename = "default")]
    pub default: bool,
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    #[serde(rename = "id")]
    pub id: i32,
    /// The name of the label.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "node_id")]
    pub node_id: String,
    /// URL for the label
    #[serde(rename = "url")]
    pub url: String,
}

impl WebhooksLabel {
    pub fn new(color: String, default: bool, description: Option<String>, id: i32, name: String, node_id: String, url: String) -> WebhooksLabel {
        WebhooksLabel {
            color,
            default,
            description,
            id,
            name,
            node_id,
            url,
        }
    }
}

