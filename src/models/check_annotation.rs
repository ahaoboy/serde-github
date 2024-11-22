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

/// CheckAnnotation : Check Annotation
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckAnnotation {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "start_line")]
    pub start_line: i32,
    #[serde(rename = "end_line")]
    pub end_line: i32,
    #[serde(rename = "start_column", deserialize_with = "Option::deserialize")]
    pub start_column: Option<i32>,
    #[serde(rename = "end_column", deserialize_with = "Option::deserialize")]
    pub end_column: Option<i32>,
    #[serde(rename = "annotation_level", deserialize_with = "Option::deserialize")]
    pub annotation_level: Option<String>,
    #[serde(rename = "title", deserialize_with = "Option::deserialize")]
    pub title: Option<String>,
    #[serde(rename = "message", deserialize_with = "Option::deserialize")]
    pub message: Option<String>,
    #[serde(rename = "raw_details", deserialize_with = "Option::deserialize")]
    pub raw_details: Option<String>,
    #[serde(rename = "blob_href")]
    pub blob_href: String,
}

impl CheckAnnotation {
    /// Check Annotation
    pub fn new(path: String, start_line: i32, end_line: i32, start_column: Option<i32>, end_column: Option<i32>, annotation_level: Option<String>, title: Option<String>, message: Option<String>, raw_details: Option<String>, blob_href: String) -> CheckAnnotation {
        CheckAnnotation {
            path,
            start_line,
            end_line,
            start_column,
            end_column,
            annotation_level,
            title,
            message,
            raw_details,
            blob_href,
        }
    }
}

