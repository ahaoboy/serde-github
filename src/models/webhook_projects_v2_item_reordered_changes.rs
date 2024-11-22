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
pub struct WebhookProjectsV2ItemReorderedChanges {
    #[serde(rename = "previous_projects_v2_item_node_id", skip_serializing_if = "Option::is_none")]
    pub previous_projects_v2_item_node_id: Option<Box<models::WebhookMemberEditedChangesPermission>>,
}

impl WebhookProjectsV2ItemReorderedChanges {
    pub fn new() -> WebhookProjectsV2ItemReorderedChanges {
        WebhookProjectsV2ItemReorderedChanges {
            previous_projects_v2_item_node_id: None,
        }
    }
}

