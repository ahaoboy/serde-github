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
pub struct ChecksCreateRequestOutputImagesInner {
    /// The alternative text for the image.
    #[serde(rename = "alt")]
    pub alt: String,
    /// The full URL of the image.
    #[serde(rename = "image_url")]
    pub image_url: String,
    /// A short image description.
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
}

impl ChecksCreateRequestOutputImagesInner {
    pub fn new(alt: String, image_url: String) -> ChecksCreateRequestOutputImagesInner {
        ChecksCreateRequestOutputImagesInner {
            alt,
            image_url,
            caption: None,
        }
    }
}
