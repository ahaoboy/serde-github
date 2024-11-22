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
pub struct WebhooksSponsorship {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "maintainer", skip_serializing_if = "Option::is_none")]
    pub maintainer: Option<Box<models::WebhooksSponsorshipMaintainer>>,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "privacy_level")]
    pub privacy_level: String,
    #[serde(rename = "sponsor", deserialize_with = "Option::deserialize")]
    pub sponsor: Option<Box<models::User>>,
    #[serde(rename = "sponsorable", deserialize_with = "Option::deserialize")]
    pub sponsorable: Option<Box<models::User>>,
    #[serde(rename = "tier")]
    pub tier: Box<models::SponsorshipTier>,
}

impl WebhooksSponsorship {
    pub fn new(created_at: String, node_id: String, privacy_level: String, sponsor: Option<models::User>, sponsorable: Option<models::User>, tier: models::SponsorshipTier) -> WebhooksSponsorship {
        WebhooksSponsorship {
            created_at,
            maintainer: None,
            node_id,
            privacy_level,
            sponsor: if let Some(x) = sponsor {Some(Box::new(x))} else {None},
            sponsorable: if let Some(x) = sponsorable {Some(Box::new(x))} else {None},
            tier: Box::new(tier),
        }
    }
}
