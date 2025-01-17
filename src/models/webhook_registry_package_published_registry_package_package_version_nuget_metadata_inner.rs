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
pub struct WebhookRegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataInner {
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<Box<models::WebhookRegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataInnerId>>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<models::WebhookPackagePublishedPackagePackageVersionNugetMetadataInnerValue>>,
}

impl WebhookRegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataInner {
    pub fn new() -> WebhookRegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataInner {
        WebhookRegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataInner {
            id: None,
            name: None,
            value: None,
        }
    }
}

