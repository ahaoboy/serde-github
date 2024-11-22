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
pub struct WebhookReleaseEditedChanges {
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<models::WebhookProjectEditedChangesBody>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<models::WebhookLabelEditedChangesName>>,
    #[serde(rename = "make_latest", skip_serializing_if = "Option::is_none")]
    pub make_latest: Option<Box<models::WebhookReleaseEditedChangesMakeLatest>>,
}

impl WebhookReleaseEditedChanges {
    pub fn new() -> WebhookReleaseEditedChanges {
        WebhookReleaseEditedChanges {
            body: None,
            name: None,
            make_latest: None,
        }
    }
}

