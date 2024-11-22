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

/// WebhooksIssue : The [issue](https://docs.github.com/rest/issues/issues#get-an-issue) itself.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhooksIssue {
    #[serde(rename = "active_lock_reason", deserialize_with = "Option::deserialize")]
    pub active_lock_reason: Option<ActiveLockReason>,
    #[serde(rename = "assignee", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Option<Box<models::User1>>>,
    #[serde(rename = "assignees")]
    pub assignees: Vec<models::User1>,
    /// How the author is associated with the repository.
    #[serde(rename = "author_association")]
    pub author_association: AuthorAssociation,
    /// Contents of the issue
    #[serde(rename = "body", deserialize_with = "Option::deserialize")]
    pub body: Option<String>,
    #[serde(rename = "closed_at", deserialize_with = "Option::deserialize")]
    pub closed_at: Option<String>,
    #[serde(rename = "comments")]
    pub comments: i32,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "draft", skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<models::Label>>,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(rename = "milestone", deserialize_with = "Option::deserialize")]
    pub milestone: Option<Box<models::Milestone>>,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "number")]
    pub number: i32,
    #[serde(rename = "performed_via_github_app", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<Option<Box<models::App>>>,
    #[serde(rename = "pull_request", skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<Box<models::WebhooksIssuePullRequest>>,
    #[serde(rename = "reactions")]
    pub reactions: Box<models::Reactions>,
    #[serde(rename = "repository_url")]
    pub repository_url: String,
    /// State of the issue; either 'open' or 'closed'
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[serde(rename = "state_reason", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<Option<String>>,
    #[serde(rename = "timeline_url", skip_serializing_if = "Option::is_none")]
    pub timeline_url: Option<String>,
    /// Title of the issue
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// URL for the issue
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::User1>>,
}

impl WebhooksIssue {
    /// The [issue](https://docs.github.com/rest/issues/issues#get-an-issue) itself.
    pub fn new(active_lock_reason: Option<ActiveLockReason>, assignees: Vec<models::User1>, author_association: AuthorAssociation, body: Option<String>, closed_at: Option<String>, comments: i32, comments_url: String, created_at: String, events_url: String, html_url: String, id: i64, labels_url: String, milestone: Option<models::Milestone>, node_id: String, number: i32, reactions: models::Reactions, repository_url: String, title: String, updated_at: String, url: String, user: Option<models::User1>) -> WebhooksIssue {
        WebhooksIssue {
            active_lock_reason,
            assignee: None,
            assignees,
            author_association,
            body,
            closed_at,
            comments,
            comments_url,
            created_at,
            draft: None,
            events_url,
            html_url,
            id,
            labels: None,
            labels_url,
            locked: None,
            milestone: if let Some(x) = milestone {Some(Box::new(x))} else {None},
            node_id,
            number,
            performed_via_github_app: None,
            pull_request: None,
            reactions: Box::new(reactions),
            repository_url,
            state: None,
            state_reason: None,
            timeline_url: None,
            title,
            updated_at,
            url,
            user: if let Some(x) = user {Some(Box::new(x))} else {None},
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActiveLockReason {
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "off-topic")]
    OffTopic,
    #[serde(rename = "too heated")]
    TooHeated,
    #[serde(rename = "spam")]
    Spam,
}

impl Default for ActiveLockReason {
    fn default() -> ActiveLockReason {
        Self::Resolved
    }
}
/// How the author is associated with the repository.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthorAssociation {
    #[serde(rename = "COLLABORATOR")]
    Collaborator,
    #[serde(rename = "CONTRIBUTOR")]
    Contributor,
    #[serde(rename = "FIRST_TIMER")]
    FirstTimer,
    #[serde(rename = "FIRST_TIME_CONTRIBUTOR")]
    FirstTimeContributor,
    #[serde(rename = "MANNEQUIN")]
    Mannequin,
    #[serde(rename = "MEMBER")]
    Member,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "OWNER")]
    Owner,
}

impl Default for AuthorAssociation {
    fn default() -> AuthorAssociation {
        Self::Collaborator
    }
}
/// State of the issue; either 'open' or 'closed'
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

