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

/// PullRequestSimple : Pull Request Simple
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PullRequestSimple {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "diff_url")]
    pub diff_url: String,
    #[serde(rename = "patch_url")]
    pub patch_url: String,
    #[serde(rename = "issue_url")]
    pub issue_url: String,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "review_comments_url")]
    pub review_comments_url: String,
    #[serde(rename = "review_comment_url")]
    pub review_comment_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    #[serde(rename = "number")]
    pub number: i32,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "locked")]
    pub locked: bool,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::NullableSimpleUser>>,
    #[serde(rename = "body", deserialize_with = "Option::deserialize")]
    pub body: Option<String>,
    #[serde(rename = "labels")]
    pub labels: Vec<models::PullRequestSimpleLabelsInner>,
    #[serde(rename = "milestone", deserialize_with = "Option::deserialize")]
    pub milestone: Option<Box<models::NullableMilestone>>,
    #[serde(rename = "active_lock_reason", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub active_lock_reason: Option<Option<String>>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "closed_at", deserialize_with = "Option::deserialize")]
    pub closed_at: Option<String>,
    #[serde(rename = "merged_at", deserialize_with = "Option::deserialize")]
    pub merged_at: Option<String>,
    #[serde(rename = "merge_commit_sha", deserialize_with = "Option::deserialize")]
    pub merge_commit_sha: Option<String>,
    #[serde(rename = "assignee", deserialize_with = "Option::deserialize")]
    pub assignee: Option<Box<models::NullableSimpleUser>>,
    #[serde(rename = "assignees", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Option<Vec<models::SimpleUser>>>,
    #[serde(rename = "requested_reviewers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub requested_reviewers: Option<Option<Vec<models::SimpleUser>>>,
    #[serde(rename = "requested_teams", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub requested_teams: Option<Option<Vec<models::Team>>>,
    #[serde(rename = "head")]
    pub head: Box<models::PullRequestSimpleHead>,
    #[serde(rename = "base")]
    pub base: Box<models::PullRequestSimpleHead>,
    #[serde(rename = "_links")]
    pub _links: Box<models::PullRequestSimpleLinks>,
    #[serde(rename = "author_association")]
    pub author_association: models::AuthorAssociation,
    #[serde(rename = "auto_merge", deserialize_with = "Option::deserialize")]
    pub auto_merge: Option<Box<models::AutoMerge>>,
    /// Indicates whether or not the pull request is a draft.
    #[serde(rename = "draft", skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
}

impl PullRequestSimple {
    /// Pull Request Simple
    pub fn new(url: String, id: i32, node_id: String, html_url: String, diff_url: String, patch_url: String, issue_url: String, commits_url: String, review_comments_url: String, review_comment_url: String, comments_url: String, statuses_url: String, number: i32, state: String, locked: bool, title: String, user: Option<models::NullableSimpleUser>, body: Option<String>, labels: Vec<models::PullRequestSimpleLabelsInner>, milestone: Option<models::NullableMilestone>, created_at: String, updated_at: String, closed_at: Option<String>, merged_at: Option<String>, merge_commit_sha: Option<String>, assignee: Option<models::NullableSimpleUser>, head: models::PullRequestSimpleHead, base: models::PullRequestSimpleHead, _links: models::PullRequestSimpleLinks, author_association: models::AuthorAssociation, auto_merge: Option<models::AutoMerge>) -> PullRequestSimple {
        PullRequestSimple {
            url,
            id,
            node_id,
            html_url,
            diff_url,
            patch_url,
            issue_url,
            commits_url,
            review_comments_url,
            review_comment_url,
            comments_url,
            statuses_url,
            number,
            state,
            locked,
            title,
            user: if let Some(x) = user {Some(Box::new(x))} else {None},
            body,
            labels,
            milestone: if let Some(x) = milestone {Some(Box::new(x))} else {None},
            active_lock_reason: None,
            created_at,
            updated_at,
            closed_at,
            merged_at,
            merge_commit_sha,
            assignee: if let Some(x) = assignee {Some(Box::new(x))} else {None},
            assignees: None,
            requested_reviewers: None,
            requested_teams: None,
            head: Box::new(head),
            base: Box::new(base),
            _links: Box::new(_links),
            author_association,
            auto_merge: if let Some(x) = auto_merge {Some(Box::new(x))} else {None},
            draft: None,
        }
    }
}

