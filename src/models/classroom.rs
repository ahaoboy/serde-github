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

/// Classroom : A GitHub Classroom classroom
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Classroom {
    /// Unique identifier of the classroom.
    #[serde(rename = "id")]
    pub id: i32,
    /// The name of the classroom.
    #[serde(rename = "name")]
    pub name: String,
    /// Whether classroom is archived.
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "organization")]
    pub organization: Box<models::SimpleClassroomOrganization>,
    /// The URL of the classroom on GitHub Classroom.
    #[serde(rename = "url")]
    pub url: String,
}

impl Classroom {
    /// A GitHub Classroom classroom
    pub fn new(id: i32, name: String, archived: bool, organization: models::SimpleClassroomOrganization, url: String) -> Classroom {
        Classroom {
            id,
            name,
            archived,
            organization: Box::new(organization),
            url,
        }
    }
}

