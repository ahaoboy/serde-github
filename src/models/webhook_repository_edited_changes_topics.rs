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
pub struct WebhookRepositoryEditedChangesTopics {
    #[serde(rename = "from", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub from: Option<Option<Vec<String>>>,
}

impl WebhookRepositoryEditedChangesTopics {
    pub fn new() -> WebhookRepositoryEditedChangesTopics {
        WebhookRepositoryEditedChangesTopics {
            from: None,
        }
    }
}

