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

/// TeamOrganization : Team Organization
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamOrganization {
    #[serde(rename = "login")]
    pub login: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "hooks_url")]
    pub hooks_url: String,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "members_url")]
    pub members_url: String,
    #[serde(rename = "public_members_url")]
    pub public_members_url: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(rename = "blog", skip_serializing_if = "Option::is_none")]
    pub blog: Option<String>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "twitter_username", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub twitter_username: Option<Option<String>>,
    #[serde(rename = "is_verified", skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[serde(rename = "has_organization_projects")]
    pub has_organization_projects: bool,
    #[serde(rename = "has_repository_projects")]
    pub has_repository_projects: bool,
    #[serde(rename = "public_repos")]
    pub public_repos: i32,
    #[serde(rename = "public_gists")]
    pub public_gists: i32,
    #[serde(rename = "followers")]
    pub followers: i32,
    #[serde(rename = "following")]
    pub following: i32,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "total_private_repos", skip_serializing_if = "Option::is_none")]
    pub total_private_repos: Option<i32>,
    #[serde(rename = "owned_private_repos", skip_serializing_if = "Option::is_none")]
    pub owned_private_repos: Option<i32>,
    #[serde(rename = "private_gists", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub private_gists: Option<Option<i32>>,
    #[serde(rename = "disk_usage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub disk_usage: Option<Option<i32>>,
    #[serde(rename = "collaborators", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub collaborators: Option<Option<i32>>,
    #[serde(rename = "billing_email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<Option<String>>,
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<Box<models::OrganizationFullPlan>>,
    #[serde(rename = "default_repository_permission", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_repository_permission: Option<Option<String>>,
    #[serde(rename = "members_can_create_repositories", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub members_can_create_repositories: Option<Option<bool>>,
    #[serde(rename = "two_factor_requirement_enabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub two_factor_requirement_enabled: Option<Option<bool>>,
    #[serde(rename = "members_allowed_repository_creation_type", skip_serializing_if = "Option::is_none")]
    pub members_allowed_repository_creation_type: Option<String>,
    #[serde(rename = "members_can_create_public_repositories", skip_serializing_if = "Option::is_none")]
    pub members_can_create_public_repositories: Option<bool>,
    #[serde(rename = "members_can_create_private_repositories", skip_serializing_if = "Option::is_none")]
    pub members_can_create_private_repositories: Option<bool>,
    #[serde(rename = "members_can_create_internal_repositories", skip_serializing_if = "Option::is_none")]
    pub members_can_create_internal_repositories: Option<bool>,
    #[serde(rename = "members_can_create_pages", skip_serializing_if = "Option::is_none")]
    pub members_can_create_pages: Option<bool>,
    #[serde(rename = "members_can_create_public_pages", skip_serializing_if = "Option::is_none")]
    pub members_can_create_public_pages: Option<bool>,
    #[serde(rename = "members_can_create_private_pages", skip_serializing_if = "Option::is_none")]
    pub members_can_create_private_pages: Option<bool>,
    #[serde(rename = "members_can_fork_private_repositories", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub members_can_fork_private_repositories: Option<Option<bool>>,
    #[serde(rename = "web_commit_signoff_required", skip_serializing_if = "Option::is_none")]
    pub web_commit_signoff_required: Option<bool>,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "archived_at", deserialize_with = "Option::deserialize")]
    pub archived_at: Option<String>,
}

impl TeamOrganization {
    /// Team Organization
    pub fn new(login: String, id: i32, node_id: String, url: String, repos_url: String, events_url: String, hooks_url: String, issues_url: String, members_url: String, public_members_url: String, avatar_url: String, description: Option<String>, has_organization_projects: bool, has_repository_projects: bool, public_repos: i32, public_gists: i32, followers: i32, following: i32, html_url: String, created_at: String, r#type: String, updated_at: String, archived_at: Option<String>) -> TeamOrganization {
        TeamOrganization {
            login,
            id,
            node_id,
            url,
            repos_url,
            events_url,
            hooks_url,
            issues_url,
            members_url,
            public_members_url,
            avatar_url,
            description,
            name: None,
            company: None,
            blog: None,
            location: None,
            email: None,
            twitter_username: None,
            is_verified: None,
            has_organization_projects,
            has_repository_projects,
            public_repos,
            public_gists,
            followers,
            following,
            html_url,
            created_at,
            r#type,
            total_private_repos: None,
            owned_private_repos: None,
            private_gists: None,
            disk_usage: None,
            collaborators: None,
            billing_email: None,
            plan: None,
            default_repository_permission: None,
            members_can_create_repositories: None,
            two_factor_requirement_enabled: None,
            members_allowed_repository_creation_type: None,
            members_can_create_public_repositories: None,
            members_can_create_private_repositories: None,
            members_can_create_internal_repositories: None,
            members_can_create_pages: None,
            members_can_create_public_pages: None,
            members_can_create_private_pages: None,
            members_can_fork_private_repositories: None,
            web_commit_signoff_required: None,
            updated_at,
            archived_at,
        }
    }
}

