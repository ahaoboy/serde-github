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
pub struct IssuesCreateRequest {
    #[serde(rename = "title")]
    pub title: Box<models::IssuesCreateRequestTitle>,
    /// The contents of the issue.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Login for the user that this issue should be assigned to. _NOTE: Only users with push access can set the assignee for new issues. The assignee is silently dropped otherwise. **This field is deprecated.**_
    #[serde(rename = "assignee", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Option<String>>,
    #[serde(rename = "milestone", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Option<Box<models::IssuesCreateRequestMilestone>>>,
    /// Labels to associate with this issue. _NOTE: Only users with push access can set labels for new issues. Labels are silently dropped otherwise._
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<models::IssuesCreateRequestLabelsInner>>,
    /// Logins for Users to assign to this issue. _NOTE: Only users with push access can set assignees for new issues. Assignees are silently dropped otherwise._
    #[serde(rename = "assignees", skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<String>>,
}

impl IssuesCreateRequest {
    pub fn new(title: models::IssuesCreateRequestTitle) -> IssuesCreateRequest {
        IssuesCreateRequest {
            title: Box::new(title),
            body: None,
            assignee: None,
            milestone: None,
            labels: None,
            assignees: None,
        }
    }
}

