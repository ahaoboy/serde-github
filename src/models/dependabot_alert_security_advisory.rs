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

/// DependabotAlertSecurityAdvisory : Details for the GitHub Security Advisory.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DependabotAlertSecurityAdvisory {
    /// The unique GitHub Security Advisory ID assigned to the advisory.
    #[serde(rename = "ghsa_id")]
    pub ghsa_id: String,
    /// The unique CVE ID assigned to the advisory.
    #[serde(rename = "cve_id", deserialize_with = "Option::deserialize")]
    pub cve_id: Option<String>,
    /// A short, plain text summary of the advisory.
    #[serde(rename = "summary")]
    pub summary: String,
    /// A long-form Markdown-supported description of the advisory.
    #[serde(rename = "description")]
    pub description: String,
    /// Vulnerable version range information for the advisory.
    #[serde(rename = "vulnerabilities")]
    pub vulnerabilities: Vec<models::DependabotAlertSecurityVulnerability>,
    /// The severity of the advisory.
    #[serde(rename = "severity")]
    pub severity: Severity,
    #[serde(rename = "cvss")]
    pub cvss: Box<models::DependabotAlertSecurityAdvisoryCvss>,
    /// Details for the advisory pertaining to Common Weakness Enumeration.
    #[serde(rename = "cwes")]
    pub cwes: Vec<models::DependabotAlertSecurityAdvisoryCwesInner>,
    /// Values that identify this advisory among security information sources.
    #[serde(rename = "identifiers")]
    pub identifiers: Vec<models::DependabotAlertSecurityAdvisoryIdentifiersInner>,
    /// Links to additional advisory information.
    #[serde(rename = "references")]
    pub references: Vec<models::DependabotAlertSecurityAdvisoryReferencesInner>,
    /// The time that the advisory was published in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "published_at")]
    pub published_at: String,
    /// The time that the advisory was last modified in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// The time that the advisory was withdrawn in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "withdrawn_at", deserialize_with = "Option::deserialize")]
    pub withdrawn_at: Option<String>,
}

impl DependabotAlertSecurityAdvisory {
    /// Details for the GitHub Security Advisory.
    pub fn new(ghsa_id: String, cve_id: Option<String>, summary: String, description: String, vulnerabilities: Vec<models::DependabotAlertSecurityVulnerability>, severity: Severity, cvss: models::DependabotAlertSecurityAdvisoryCvss, cwes: Vec<models::DependabotAlertSecurityAdvisoryCwesInner>, identifiers: Vec<models::DependabotAlertSecurityAdvisoryIdentifiersInner>, references: Vec<models::DependabotAlertSecurityAdvisoryReferencesInner>, published_at: String, updated_at: String, withdrawn_at: Option<String>) -> DependabotAlertSecurityAdvisory {
        DependabotAlertSecurityAdvisory {
            ghsa_id,
            cve_id,
            summary,
            description,
            vulnerabilities,
            severity,
            cvss: Box::new(cvss),
            cwes,
            identifiers,
            references,
            published_at,
            updated_at,
            withdrawn_at,
        }
    }
}
/// The severity of the advisory.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Severity {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "critical")]
    Critical,
}

impl Default for Severity {
    fn default() -> Severity {
        Self::Low
    }
}
