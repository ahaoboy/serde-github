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
pub struct TeamsCreateRequest {
    /// The name of the team.
    #[serde(rename = "name")]
    pub name: String,
    /// The description of the team.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// List GitHub IDs for organization members who will become team maintainers.
    #[serde(rename = "maintainers", skip_serializing_if = "Option::is_none")]
    pub maintainers: Option<Vec<String>>,
    /// The full name (e.g., \"organization-name/repository-name\") of repositories to add the team to.
    #[serde(rename = "repo_names", skip_serializing_if = "Option::is_none")]
    pub repo_names: Option<Vec<String>>,
    /// The level of privacy this team should have. The options are:   **For a non-nested team:**    * `secret` - only visible to organization owners and members of this team.    * `closed` - visible to all members of this organization.   Default: `secret`   **For a parent or child team:**    * `closed` - visible to all members of this organization.   Default for child team: `closed`
    #[serde(rename = "privacy", skip_serializing_if = "Option::is_none")]
    pub privacy: Option<Privacy>,
    /// The notification setting the team has chosen. The options are:    * `notifications_enabled` - team members receive notifications when the team is @mentioned.    * `notifications_disabled` - no one receives notifications.   Default: `notifications_enabled`
    #[serde(rename = "notification_setting", skip_serializing_if = "Option::is_none")]
    pub notification_setting: Option<NotificationSetting>,
    /// **Deprecated**. The permission that new repositories will be added to the team with when none is specified.
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<Permission>,
    /// The ID of a team to set as the parent team.
    #[serde(rename = "parent_team_id", skip_serializing_if = "Option::is_none")]
    pub parent_team_id: Option<i32>,
}

impl TeamsCreateRequest {
    pub fn new(name: String) -> TeamsCreateRequest {
        TeamsCreateRequest {
            name,
            description: None,
            maintainers: None,
            repo_names: None,
            privacy: None,
            notification_setting: None,
            permission: None,
            parent_team_id: None,
        }
    }
}
/// The level of privacy this team should have. The options are:   **For a non-nested team:**    * `secret` - only visible to organization owners and members of this team.    * `closed` - visible to all members of this organization.   Default: `secret`   **For a parent or child team:**    * `closed` - visible to all members of this organization.   Default for child team: `closed`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Privacy {
    #[serde(rename = "secret")]
    Secret,
    #[serde(rename = "closed")]
    Closed,
}

impl Default for Privacy {
    fn default() -> Privacy {
        Self::Secret
    }
}
/// The notification setting the team has chosen. The options are:    * `notifications_enabled` - team members receive notifications when the team is @mentioned.    * `notifications_disabled` - no one receives notifications.   Default: `notifications_enabled`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotificationSetting {
    #[serde(rename = "notifications_enabled")]
    Enabled,
    #[serde(rename = "notifications_disabled")]
    Disabled,
}

impl Default for NotificationSetting {
    fn default() -> NotificationSetting {
        Self::Enabled
    }
}
/// **Deprecated**. The permission that new repositories will be added to the team with when none is specified.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Permission {
    #[serde(rename = "pull")]
    Pull,
    #[serde(rename = "push")]
    Push,
}

impl Default for Permission {
    fn default() -> Permission {
        Self::Pull
    }
}
