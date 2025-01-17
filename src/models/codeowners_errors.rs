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

/// CodeownersErrors : A list of errors found in a repo's CODEOWNERS file
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeownersErrors {
    #[serde(rename = "errors")]
    pub errors: Vec<models::CodeownersErrorsErrorsInner>,
}

impl CodeownersErrors {
    /// A list of errors found in a repo's CODEOWNERS file
    pub fn new(errors: Vec<models::CodeownersErrorsErrorsInner>) -> CodeownersErrors {
        CodeownersErrors {
            errors,
        }
    }
}

