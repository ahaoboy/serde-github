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
pub struct ReposSetAppAccessRestrictionsRequestOneOf {
    /// The GitHub Apps that have push access to this branch. Use the slugified version of the app name. **Note**: The list of users, apps, and teams in total is limited to 100 items.
    #[serde(rename = "apps")]
    pub apps: Vec<String>,
}

impl ReposSetAppAccessRestrictionsRequestOneOf {
    pub fn new(apps: Vec<String>) -> ReposSetAppAccessRestrictionsRequestOneOf {
        ReposSetAppAccessRestrictionsRequestOneOf {
            apps,
        }
    }
}

