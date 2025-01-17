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

/// Discussion : A Discussion in a repository.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Discussion {
    #[serde(rename = "active_lock_reason", deserialize_with = "Option::deserialize")]
    pub active_lock_reason: Option<String>,
    #[serde(rename = "answer_chosen_at", deserialize_with = "Option::deserialize")]
    pub answer_chosen_at: Option<String>,
    #[serde(rename = "answer_chosen_by", deserialize_with = "Option::deserialize")]
    pub answer_chosen_by: Option<Box<models::User>>,
    #[serde(rename = "answer_html_url", deserialize_with = "Option::deserialize")]
    pub answer_html_url: Option<String>,
    /// How the author is associated with the repository.
    #[serde(rename = "author_association")]
    pub author_association: AuthorAssociation,
    #[serde(rename = "body")]
    pub body: String,
    #[serde(rename = "category")]
    pub category: Box<models::DiscussionCategory>,
    #[serde(rename = "comments")]
    pub comments: i32,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "locked")]
    pub locked: bool,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "number")]
    pub number: i32,
    #[serde(rename = "reactions", skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Box<models::Reactions>>,
    #[serde(rename = "repository_url")]
    pub repository_url: String,
    /// The current state of the discussion. `converting` means that the discussion is being converted from an issue. `transferring` means that the discussion is being transferred from another repository.
    #[serde(rename = "state")]
    pub state: State,
    /// The reason for the current state
    #[serde(rename = "state_reason", deserialize_with = "Option::deserialize")]
    pub state_reason: Option<StateReason>,
    #[serde(rename = "timeline_url", skip_serializing_if = "Option::is_none")]
    pub timeline_url: Option<String>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::User>>,
}

impl Discussion {
    /// A Discussion in a repository.
    pub fn new(active_lock_reason: Option<String>, answer_chosen_at: Option<String>, answer_chosen_by: Option<models::User>, answer_html_url: Option<String>, author_association: AuthorAssociation, body: String, category: models::DiscussionCategory, comments: i32, created_at: String, html_url: String, id: i32, locked: bool, node_id: String, number: i32, repository_url: String, state: State, state_reason: Option<StateReason>, title: String, updated_at: String, user: Option<models::User>) -> Discussion {
        Discussion {
            active_lock_reason,
            answer_chosen_at,
            answer_chosen_by: if let Some(x) = answer_chosen_by {Some(Box::new(x))} else {None},
            answer_html_url,
            author_association,
            body,
            category: Box::new(category),
            comments,
            created_at,
            html_url,
            id,
            locked,
            node_id,
            number,
            reactions: None,
            repository_url,
            state,
            state_reason,
            timeline_url: None,
            title,
            updated_at,
            user: if let Some(x) = user {Some(Box::new(x))} else {None},
        }
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
/// The current state of the discussion. `converting` means that the discussion is being converted from an issue. `transferring` means that the discussion is being transferred from another repository.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "converting")]
    Converting,
    #[serde(rename = "transferring")]
    Transferring,
}

impl Default for State {
    fn default() -> State {
        Self::Open
    }
}
/// The reason for the current state
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StateReason {
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "outdated")]
    Outdated,
    #[serde(rename = "duplicate")]
    Duplicate,
    #[serde(rename = "reopened")]
    Reopened,
}

impl Default for StateReason {
    fn default() -> StateReason {
        Self::Resolved
    }
}

