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

/// TeamRepository : A team's access to a repository.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamRepository {
    /// Unique identifier of the repository
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "node_id")]
    pub node_id: String,
    /// The name of the repository.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    #[serde(rename = "license", deserialize_with = "Option::deserialize")]
    pub license: Option<Box<models::NullableLicenseSimple>>,
    #[serde(rename = "forks")]
    pub forks: i32,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<models::RepositoryPermissions>>,
    #[serde(rename = "role_name", skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(rename = "owner", deserialize_with = "Option::deserialize")]
    pub owner: Option<Box<models::NullableSimpleUser>>,
    /// Whether the repository is private or public.
    #[serde(rename = "private")]
    pub private: bool,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    #[serde(rename = "fork")]
    pub fork: bool,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "archive_url")]
    pub archive_url: String,
    #[serde(rename = "assignees_url")]
    pub assignees_url: String,
    #[serde(rename = "blobs_url")]
    pub blobs_url: String,
    #[serde(rename = "branches_url")]
    pub branches_url: String,
    #[serde(rename = "collaborators_url")]
    pub collaborators_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "compare_url")]
    pub compare_url: String,
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    #[serde(rename = "contributors_url")]
    pub contributors_url: String,
    #[serde(rename = "deployments_url")]
    pub deployments_url: String,
    #[serde(rename = "downloads_url")]
    pub downloads_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "forks_url")]
    pub forks_url: String,
    #[serde(rename = "git_commits_url")]
    pub git_commits_url: String,
    #[serde(rename = "git_refs_url")]
    pub git_refs_url: String,
    #[serde(rename = "git_tags_url")]
    pub git_tags_url: String,
    #[serde(rename = "git_url")]
    pub git_url: String,
    #[serde(rename = "issue_comment_url")]
    pub issue_comment_url: String,
    #[serde(rename = "issue_events_url")]
    pub issue_events_url: String,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "keys_url")]
    pub keys_url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "languages_url")]
    pub languages_url: String,
    #[serde(rename = "merges_url")]
    pub merges_url: String,
    #[serde(rename = "milestones_url")]
    pub milestones_url: String,
    #[serde(rename = "notifications_url")]
    pub notifications_url: String,
    #[serde(rename = "pulls_url")]
    pub pulls_url: String,
    #[serde(rename = "releases_url")]
    pub releases_url: String,
    #[serde(rename = "ssh_url")]
    pub ssh_url: String,
    #[serde(rename = "stargazers_url")]
    pub stargazers_url: String,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    #[serde(rename = "subscribers_url")]
    pub subscribers_url: String,
    #[serde(rename = "subscription_url")]
    pub subscription_url: String,
    #[serde(rename = "tags_url")]
    pub tags_url: String,
    #[serde(rename = "teams_url")]
    pub teams_url: String,
    #[serde(rename = "trees_url")]
    pub trees_url: String,
    #[serde(rename = "clone_url")]
    pub clone_url: String,
    #[serde(rename = "mirror_url", deserialize_with = "Option::deserialize")]
    pub mirror_url: Option<String>,
    #[serde(rename = "hooks_url")]
    pub hooks_url: String,
    #[serde(rename = "svn_url")]
    pub svn_url: String,
    #[serde(rename = "homepage", deserialize_with = "Option::deserialize")]
    pub homepage: Option<String>,
    #[serde(rename = "language", deserialize_with = "Option::deserialize")]
    pub language: Option<String>,
    #[serde(rename = "forks_count")]
    pub forks_count: i32,
    #[serde(rename = "stargazers_count")]
    pub stargazers_count: i32,
    #[serde(rename = "watchers_count")]
    pub watchers_count: i32,
    #[serde(rename = "size")]
    pub size: i32,
    /// The default branch of the repository.
    #[serde(rename = "default_branch")]
    pub default_branch: String,
    #[serde(rename = "open_issues_count")]
    pub open_issues_count: i32,
    /// Whether this repository acts as a template that can be used to generate new repositories.
    #[serde(rename = "is_template", skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(rename = "topics", skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    /// Whether issues are enabled.
    #[serde(rename = "has_issues")]
    pub has_issues: bool,
    /// Whether projects are enabled.
    #[serde(rename = "has_projects")]
    pub has_projects: bool,
    /// Whether the wiki is enabled.
    #[serde(rename = "has_wiki")]
    pub has_wiki: bool,
    #[serde(rename = "has_pages")]
    pub has_pages: bool,
    /// Whether downloads are enabled.
    #[serde(rename = "has_downloads")]
    pub has_downloads: bool,
    /// Whether the repository is archived.
    #[serde(rename = "archived")]
    pub archived: bool,
    /// Returns whether or not this repository disabled.
    #[serde(rename = "disabled")]
    pub disabled: bool,
    /// The repository visibility: public, private, or internal.
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(rename = "pushed_at", deserialize_with = "Option::deserialize")]
    pub pushed_at: Option<String>,
    #[serde(rename = "created_at", deserialize_with = "Option::deserialize")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", deserialize_with = "Option::deserialize")]
    pub updated_at: Option<String>,
    /// Whether to allow rebase merges for pull requests.
    #[serde(rename = "allow_rebase_merge", skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(rename = "temp_clone_token", skip_serializing_if = "Option::is_none")]
    pub temp_clone_token: Option<String>,
    /// Whether to allow squash merges for pull requests.
    #[serde(rename = "allow_squash_merge", skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    /// Whether to allow Auto-merge to be used on pull requests.
    #[serde(rename = "allow_auto_merge", skip_serializing_if = "Option::is_none")]
    pub allow_auto_merge: Option<bool>,
    /// Whether to delete head branches when pull requests are merged
    #[serde(rename = "delete_branch_on_merge", skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    /// Whether to allow merge commits for pull requests.
    #[serde(rename = "allow_merge_commit", skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    /// Whether to allow forking this repo
    #[serde(rename = "allow_forking", skip_serializing_if = "Option::is_none")]
    pub allow_forking: Option<bool>,
    /// Whether to require contributors to sign off on web-based commits
    #[serde(rename = "web_commit_signoff_required", skip_serializing_if = "Option::is_none")]
    pub web_commit_signoff_required: Option<bool>,
    #[serde(rename = "subscribers_count", skip_serializing_if = "Option::is_none")]
    pub subscribers_count: Option<i32>,
    #[serde(rename = "network_count", skip_serializing_if = "Option::is_none")]
    pub network_count: Option<i32>,
    #[serde(rename = "open_issues")]
    pub open_issues: i32,
    #[serde(rename = "watchers")]
    pub watchers: i32,
    #[serde(rename = "master_branch", skip_serializing_if = "Option::is_none")]
    pub master_branch: Option<String>,
}

impl TeamRepository {
    /// A team's access to a repository.
    pub fn new(id: i32, node_id: String, name: String, full_name: String, license: Option<models::NullableLicenseSimple>, forks: i32, owner: Option<models::NullableSimpleUser>, private: bool, html_url: String, description: Option<String>, fork: bool, url: String, archive_url: String, assignees_url: String, blobs_url: String, branches_url: String, collaborators_url: String, comments_url: String, commits_url: String, compare_url: String, contents_url: String, contributors_url: String, deployments_url: String, downloads_url: String, events_url: String, forks_url: String, git_commits_url: String, git_refs_url: String, git_tags_url: String, git_url: String, issue_comment_url: String, issue_events_url: String, issues_url: String, keys_url: String, labels_url: String, languages_url: String, merges_url: String, milestones_url: String, notifications_url: String, pulls_url: String, releases_url: String, ssh_url: String, stargazers_url: String, statuses_url: String, subscribers_url: String, subscription_url: String, tags_url: String, teams_url: String, trees_url: String, clone_url: String, mirror_url: Option<String>, hooks_url: String, svn_url: String, homepage: Option<String>, language: Option<String>, forks_count: i32, stargazers_count: i32, watchers_count: i32, size: i32, default_branch: String, open_issues_count: i32, has_issues: bool, has_projects: bool, has_wiki: bool, has_pages: bool, has_downloads: bool, archived: bool, disabled: bool, pushed_at: Option<String>, created_at: Option<String>, updated_at: Option<String>, open_issues: i32, watchers: i32) -> TeamRepository {
        TeamRepository {
            id,
            node_id,
            name,
            full_name,
            license: if let Some(x) = license {Some(Box::new(x))} else {None},
            forks,
            permissions: None,
            role_name: None,
            owner: if let Some(x) = owner {Some(Box::new(x))} else {None},
            private,
            html_url,
            description,
            fork,
            url,
            archive_url,
            assignees_url,
            blobs_url,
            branches_url,
            collaborators_url,
            comments_url,
            commits_url,
            compare_url,
            contents_url,
            contributors_url,
            deployments_url,
            downloads_url,
            events_url,
            forks_url,
            git_commits_url,
            git_refs_url,
            git_tags_url,
            git_url,
            issue_comment_url,
            issue_events_url,
            issues_url,
            keys_url,
            labels_url,
            languages_url,
            merges_url,
            milestones_url,
            notifications_url,
            pulls_url,
            releases_url,
            ssh_url,
            stargazers_url,
            statuses_url,
            subscribers_url,
            subscription_url,
            tags_url,
            teams_url,
            trees_url,
            clone_url,
            mirror_url,
            hooks_url,
            svn_url,
            homepage,
            language,
            forks_count,
            stargazers_count,
            watchers_count,
            size,
            default_branch,
            open_issues_count,
            is_template: None,
            topics: None,
            has_issues,
            has_projects,
            has_wiki,
            has_pages,
            has_downloads,
            archived,
            disabled,
            visibility: None,
            pushed_at,
            created_at,
            updated_at,
            allow_rebase_merge: None,
            temp_clone_token: None,
            allow_squash_merge: None,
            allow_auto_merge: None,
            delete_branch_on_merge: None,
            allow_merge_commit: None,
            allow_forking: None,
            web_commit_signoff_required: None,
            subscribers_count: None,
            network_count: None,
            open_issues,
            watchers,
            master_branch: None,
        }
    }
}

