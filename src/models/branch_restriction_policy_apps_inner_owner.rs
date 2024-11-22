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
pub struct BranchRestrictionPolicyAppsInnerOwner {
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "node_id", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "repos_url", skip_serializing_if = "Option::is_none")]
    pub repos_url: Option<String>,
    #[serde(rename = "events_url", skip_serializing_if = "Option::is_none")]
    pub events_url: Option<String>,
    #[serde(rename = "hooks_url", skip_serializing_if = "Option::is_none")]
    pub hooks_url: Option<String>,
    #[serde(rename = "issues_url", skip_serializing_if = "Option::is_none")]
    pub issues_url: Option<String>,
    #[serde(rename = "members_url", skip_serializing_if = "Option::is_none")]
    pub members_url: Option<String>,
    #[serde(rename = "public_members_url", skip_serializing_if = "Option::is_none")]
    pub public_members_url: Option<String>,
    #[serde(rename = "avatar_url", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "gravatar_id", skip_serializing_if = "Option::is_none")]
    pub gravatar_id: Option<String>,
    #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(rename = "followers_url", skip_serializing_if = "Option::is_none")]
    pub followers_url: Option<String>,
    #[serde(rename = "following_url", skip_serializing_if = "Option::is_none")]
    pub following_url: Option<String>,
    #[serde(rename = "gists_url", skip_serializing_if = "Option::is_none")]
    pub gists_url: Option<String>,
    #[serde(rename = "starred_url", skip_serializing_if = "Option::is_none")]
    pub starred_url: Option<String>,
    #[serde(rename = "subscriptions_url", skip_serializing_if = "Option::is_none")]
    pub subscriptions_url: Option<String>,
    #[serde(rename = "organizations_url", skip_serializing_if = "Option::is_none")]
    pub organizations_url: Option<String>,
    #[serde(rename = "received_events_url", skip_serializing_if = "Option::is_none")]
    pub received_events_url: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "site_admin", skip_serializing_if = "Option::is_none")]
    pub site_admin: Option<bool>,
}

impl BranchRestrictionPolicyAppsInnerOwner {
    pub fn new() -> BranchRestrictionPolicyAppsInnerOwner {
        BranchRestrictionPolicyAppsInnerOwner {
            login: None,
            id: None,
            node_id: None,
            url: None,
            repos_url: None,
            events_url: None,
            hooks_url: None,
            issues_url: None,
            members_url: None,
            public_members_url: None,
            avatar_url: None,
            description: None,
            gravatar_id: None,
            html_url: None,
            followers_url: None,
            following_url: None,
            gists_url: None,
            starred_url: None,
            subscriptions_url: None,
            organizations_url: None,
            received_events_url: None,
            r#type: None,
            site_admin: None,
        }
    }
}

