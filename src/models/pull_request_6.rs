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
pub struct PullRequest6 {
    #[serde(rename = "_links")]
    pub _links: Box<models::WebhooksPullRequest5Links>,
    #[serde(rename = "active_lock_reason", deserialize_with = "Option::deserialize")]
    pub active_lock_reason: Option<ActiveLockReason>,
    #[serde(rename = "additions", skip_serializing_if = "Option::is_none")]
    pub additions: Option<i32>,
    #[serde(rename = "assignee", deserialize_with = "Option::deserialize")]
    pub assignee: Option<Box<models::User>>,
    #[serde(rename = "assignees")]
    pub assignees: Vec<models::User>,
    /// How the author is associated with the repository.
    #[serde(rename = "author_association")]
    pub author_association: AuthorAssociation,
    #[serde(rename = "auto_merge", deserialize_with = "Option::deserialize")]
    pub auto_merge: Option<Box<models::PullRequestAutoMerge>>,
    #[serde(rename = "base")]
    pub base: Box<models::PullRequest6Base>,
    #[serde(rename = "body", deserialize_with = "Option::deserialize")]
    pub body: Option<String>,
    #[serde(rename = "changed_files", skip_serializing_if = "Option::is_none")]
    pub changed_files: Option<i32>,
    #[serde(rename = "closed_at", deserialize_with = "Option::deserialize")]
    pub closed_at: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<i32>,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "commits", skip_serializing_if = "Option::is_none")]
    pub commits: Option<i32>,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "deletions", skip_serializing_if = "Option::is_none")]
    pub deletions: Option<i32>,
    #[serde(rename = "diff_url")]
    pub diff_url: String,
    /// Indicates whether or not the pull request is a draft.
    #[serde(rename = "draft")]
    pub draft: bool,
    #[serde(rename = "head")]
    pub head: Box<models::PullRequestBase>,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "issue_url")]
    pub issue_url: String,
    #[serde(rename = "labels")]
    pub labels: Vec<models::Label>,
    #[serde(rename = "locked")]
    pub locked: bool,
    /// Indicates whether maintainers can modify the pull request.
    #[serde(rename = "maintainer_can_modify", skip_serializing_if = "Option::is_none")]
    pub maintainer_can_modify: Option<bool>,
    #[serde(rename = "merge_commit_sha", deserialize_with = "Option::deserialize")]
    pub merge_commit_sha: Option<String>,
    #[serde(rename = "mergeable", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<Option<bool>>,
    #[serde(rename = "mergeable_state", skip_serializing_if = "Option::is_none")]
    pub mergeable_state: Option<String>,
    #[serde(rename = "merged", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub merged: Option<Option<bool>>,
    #[serde(rename = "merged_at", deserialize_with = "Option::deserialize")]
    pub merged_at: Option<String>,
    #[serde(rename = "merged_by", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub merged_by: Option<Option<Box<models::User>>>,
    #[serde(rename = "milestone", deserialize_with = "Option::deserialize")]
    pub milestone: Option<Box<models::Milestone1>>,
    #[serde(rename = "node_id")]
    pub node_id: String,
    /// Number uniquely identifying the pull request within its repository.
    #[serde(rename = "number")]
    pub number: i32,
    #[serde(rename = "patch_url")]
    pub patch_url: String,
    #[serde(rename = "rebaseable", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rebaseable: Option<Option<bool>>,
    #[serde(rename = "requested_reviewers")]
    pub requested_reviewers: Vec<models::PullRequest6RequestedReviewersInner>,
    #[serde(rename = "requested_teams")]
    pub requested_teams: Vec<models::Team1>,
    #[serde(rename = "review_comment_url")]
    pub review_comment_url: String,
    #[serde(rename = "review_comments", skip_serializing_if = "Option::is_none")]
    pub review_comments: Option<i32>,
    #[serde(rename = "review_comments_url")]
    pub review_comments_url: String,
    /// State of this Pull Request. Either `open` or `closed`.
    #[serde(rename = "state")]
    pub state: State,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    /// The title of the pull request.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::User>>,
}

impl PullRequest6 {
    pub fn new(_links: models::WebhooksPullRequest5Links, active_lock_reason: Option<ActiveLockReason>, assignee: Option<models::User>, assignees: Vec<models::User>, author_association: AuthorAssociation, auto_merge: Option<models::PullRequestAutoMerge>, base: models::PullRequest6Base, body: Option<String>, closed_at: Option<String>, comments_url: String, commits_url: String, created_at: String, diff_url: String, draft: bool, head: models::PullRequestBase, html_url: String, id: i32, issue_url: String, labels: Vec<models::Label>, locked: bool, merge_commit_sha: Option<String>, merged_at: Option<String>, milestone: Option<models::Milestone1>, node_id: String, number: i32, patch_url: String, requested_reviewers: Vec<models::PullRequest6RequestedReviewersInner>, requested_teams: Vec<models::Team1>, review_comment_url: String, review_comments_url: String, state: State, statuses_url: String, title: String, updated_at: String, url: String, user: Option<models::User>) -> PullRequest6 {
        PullRequest6 {
            _links: Box::new(_links),
            active_lock_reason,
            additions: None,
            assignee: if let Some(x) = assignee {Some(Box::new(x))} else {None},
            assignees,
            author_association,
            auto_merge: if let Some(x) = auto_merge {Some(Box::new(x))} else {None},
            base: Box::new(base),
            body,
            changed_files: None,
            closed_at,
            comments: None,
            comments_url,
            commits: None,
            commits_url,
            created_at,
            deletions: None,
            diff_url,
            draft,
            head: Box::new(head),
            html_url,
            id,
            issue_url,
            labels,
            locked,
            maintainer_can_modify: None,
            merge_commit_sha,
            mergeable: None,
            mergeable_state: None,
            merged: None,
            merged_at,
            merged_by: None,
            milestone: if let Some(x) = milestone {Some(Box::new(x))} else {None},
            node_id,
            number,
            patch_url,
            rebaseable: None,
            requested_reviewers,
            requested_teams,
            review_comment_url,
            review_comments: None,
            review_comments_url,
            state,
            statuses_url,
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
/// State of this Pull Request. Either `open` or `closed`.
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

