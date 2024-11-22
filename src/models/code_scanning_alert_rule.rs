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
pub struct CodeScanningAlertRule {
    /// A unique identifier for the rule used to detect the alert.
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<String>>,
    /// The name of the rule used to detect the alert.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The severity of the alert.
    #[serde(rename = "severity", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub severity: Option<Option<Severity>>,
    /// The security severity of the alert.
    #[serde(rename = "security_severity_level", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub security_severity_level: Option<Option<SecuritySeverityLevel>>,
    /// A short description of the rule used to detect the alert.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// description of the rule used to detect the alert.
    #[serde(rename = "full_description", skip_serializing_if = "Option::is_none")]
    pub full_description: Option<String>,
    /// A set of tags applicable for the rule.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<String>>>,
    /// Detailed documentation for the rule as GitHub Flavored Markdown.
    #[serde(rename = "help", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub help: Option<Option<String>>,
    /// A link to the documentation for the rule used to detect the alert.
    #[serde(rename = "help_uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub help_uri: Option<Option<String>>,
}

impl CodeScanningAlertRule {
    pub fn new() -> CodeScanningAlertRule {
        CodeScanningAlertRule {
            id: None,
            name: None,
            severity: None,
            security_severity_level: None,
            description: None,
            full_description: None,
            tags: None,
            help: None,
            help_uri: None,
        }
    }
}
/// The severity of the alert.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Severity {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}

impl Default for Severity {
    fn default() -> Severity {
        Self::None
    }
}
/// The security severity of the alert.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecuritySeverityLevel {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "critical")]
    Critical,
}

impl Default for SecuritySeverityLevel {
    fn default() -> SecuritySeverityLevel {
        Self::Low
    }
}

