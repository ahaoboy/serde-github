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

/// Reaction : Reactions to conversations provide a way to help people express their feelings more simply and effectively.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reaction {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::NullableSimpleUser>>,
    /// The reaction to use
    #[serde(rename = "content")]
    pub content: Content,
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl Reaction {
    /// Reactions to conversations provide a way to help people express their feelings more simply and effectively.
    pub fn new(id: i32, node_id: String, user: Option<models::NullableSimpleUser>, content: Content, created_at: String) -> Reaction {
        Reaction {
            id,
            node_id,
            user: if let Some(x) = user {Some(Box::new(x))} else {None},
            content,
            created_at,
        }
    }
}
/// The reaction to use
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Content {
    #[serde(rename = "+1")]
    Plus1,
    #[serde(rename = "-1")]
    Variant1,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
}

impl Default for Content {
    fn default() -> Content {
        Self::Plus1
    }
}

