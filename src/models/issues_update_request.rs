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
pub struct IssuesUpdateRequest {
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<Box<models::IssuesUpdateRequestTitle>>>,
    /// The contents of the issue.
    #[serde(rename = "body", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub body: Option<Option<String>>,
    /// Username to assign to this issue. **This field is deprecated.**
    #[serde(rename = "assignee", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Option<String>>,
    /// The open or closed state of the issue.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// The reason for the state change. Ignored unless `state` is changed.
    #[serde(rename = "state_reason", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<Option<StateReason>>,
    #[serde(rename = "milestone", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Option<Box<models::IssuesUpdateRequestMilestone>>>,
    /// Labels to associate with this issue. Pass one or more labels to _replace_ the set of labels on this issue. Send an empty array (`[]`) to clear all labels from the issue. Only users with push access can set labels for issues. Without push access to the repository, label changes are silently dropped.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<models::IssuesCreateRequestLabelsInner>>,
    /// Usernames to assign to this issue. Pass one or more user logins to _replace_ the set of assignees on this issue. Send an empty array (`[]`) to clear all assignees from the issue. Only users with push access can set assignees for new issues. Without push access to the repository, assignee changes are silently dropped.
    #[serde(rename = "assignees", skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<String>>,
}

impl IssuesUpdateRequest {
    pub fn new() -> IssuesUpdateRequest {
        IssuesUpdateRequest {
            title: None,
            body: None,
            assignee: None,
            state: None,
            state_reason: None,
            milestone: None,
            labels: None,
            assignees: None,
        }
    }
}
/// The open or closed state of the issue.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

impl Default for State {
    fn default() -> State {
        Self::Open
    }
}
/// The reason for the state change. Ignored unless `state` is changed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StateReason {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "not_planned")]
    NotPlanned,
    #[serde(rename = "reopened")]
    Reopened,
}

impl Default for StateReason {
    fn default() -> StateReason {
        Self::Completed
    }
}

