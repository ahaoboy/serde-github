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

/// RepositoryAdvisoryCredit : A credit given to a user for a repository security advisory.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryAdvisoryCredit {
    #[serde(rename = "user")]
    pub user: Box<models::SimpleUser>,
    #[serde(rename = "type")]
    pub r#type: models::SecurityAdvisoryCreditTypes,
    /// The state of the user's acceptance of the credit.
    #[serde(rename = "state")]
    pub state: State,
}

impl RepositoryAdvisoryCredit {
    /// A credit given to a user for a repository security advisory.
    pub fn new(user: models::SimpleUser, r#type: models::SecurityAdvisoryCreditTypes, state: State) -> RepositoryAdvisoryCredit {
        RepositoryAdvisoryCredit {
            user: Box::new(user),
            r#type,
            state,
        }
    }
}
/// The state of the user's acceptance of the credit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "declined")]
    Declined,
    #[serde(rename = "pending")]
    Pending,
}

impl Default for State {
    fn default() -> State {
        Self::Accepted
    }
}

