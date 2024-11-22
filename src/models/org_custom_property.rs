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

/// OrgCustomProperty : Custom property defined on an organization
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrgCustomProperty {
    /// The name of the property
    #[serde(rename = "property_name")]
    pub property_name: String,
    /// The type of the value for the property
    #[serde(rename = "value_type")]
    pub value_type: ValueType,
    /// Whether the property is required.
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "default_value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<Option<Box<models::OrgsCreateOrUpdateCustomPropertyRequestDefaultValue>>>,
    /// Short description of the property
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// An ordered list of the allowed values of the property. The property can have up to 200 allowed values.
    #[serde(rename = "allowed_values", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Option<Vec<String>>>,
    /// Who can edit the values of the property
    #[serde(rename = "values_editable_by", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub values_editable_by: Option<Option<ValuesEditableBy>>,
}

impl OrgCustomProperty {
    /// Custom property defined on an organization
    pub fn new(property_name: String, value_type: ValueType) -> OrgCustomProperty {
        OrgCustomProperty {
            property_name,
            value_type,
            required: None,
            default_value: None,
            description: None,
            allowed_values: None,
            values_editable_by: None,
        }
    }
}
/// The type of the value for the property
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ValueType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "single_select")]
    SingleSelect,
}

impl Default for ValueType {
    fn default() -> ValueType {
        Self::String
    }
}
/// Who can edit the values of the property
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ValuesEditableBy {
    #[serde(rename = "org_actors")]
    Actors,
    #[serde(rename = "org_and_repo_actors")]
    AndRepoActors,
}

impl Default for ValuesEditableBy {
    fn default() -> ValuesEditableBy {
        Self::Actors
    }
}
