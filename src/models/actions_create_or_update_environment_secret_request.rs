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
pub struct ActionsCreateOrUpdateEnvironmentSecretRequest {
    /// Value for your secret, encrypted with [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages) using the public key retrieved from the [Get an environment public key](https://docs.github.com/rest/actions/secrets#get-an-environment-public-key) endpoint.
    #[serde(rename = "encrypted_value")]
    pub encrypted_value: String,
    /// ID of the key you used to encrypt the secret.
    #[serde(rename = "key_id")]
    pub key_id: String,
}

impl ActionsCreateOrUpdateEnvironmentSecretRequest {
    pub fn new(encrypted_value: String, key_id: String) -> ActionsCreateOrUpdateEnvironmentSecretRequest {
        ActionsCreateOrUpdateEnvironmentSecretRequest {
            encrypted_value,
            key_id,
        }
    }
}
