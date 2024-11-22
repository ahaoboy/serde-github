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

/// Enterprise : An enterprise on GitHub.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Enterprise {
    /// A short description of the enterprise.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "html_url")]
    pub html_url: String,
    /// The enterprise's website URL.
    #[serde(rename = "website_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub website_url: Option<Option<String>>,
    /// Unique identifier of the enterprise
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "node_id")]
    pub node_id: String,
    /// The name of the enterprise.
    #[serde(rename = "name")]
    pub name: String,
    /// The slug url identifier for the enterprise.
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "created_at", deserialize_with = "Option::deserialize")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", deserialize_with = "Option::deserialize")]
    pub updated_at: Option<String>,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
}

impl Enterprise {
    /// An enterprise on GitHub.
    pub fn new(html_url: String, id: i32, node_id: String, name: String, slug: String, created_at: Option<String>, updated_at: Option<String>, avatar_url: String) -> Enterprise {
        Enterprise {
            description: None,
            html_url,
            website_url: None,
            id,
            node_id,
            name,
            slug,
            created_at,
            updated_at,
            avatar_url,
        }
    }
}
