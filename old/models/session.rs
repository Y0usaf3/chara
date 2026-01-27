use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use crate::models::ids::{SessionId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Option<SessionId>,
    pub user: UserId,
    pub token: String,
    pub ip: String,
    pub user_agent: String,
    pub created_at: Datetime,
    pub expires_at: Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertSession {
    pub user: UserId,
    pub token: String,
    pub ip: String,
    pub user_agent: String,
    pub expires_at: Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionPatch {
    pub expires_at: Option<Datetime>,
}

impl Session {
    pub fn from_insert(input: InsertSession) -> Self {
        let now = Datetime::from(chrono::Utc::now());

        Self {
            id: None,
            user: input.user,
            token: input.token,
            ip: input.ip,
            user_agent: input.user_agent,
            created_at: now,
            expires_at: input.expires_at,
        }
    }

    pub fn apply_patch(&mut self, patch: SessionPatch) {
        if let Some(expires_at) = patch.expires_at {
            self.expires_at = expires_at;
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at < Datetime::from(chrono::Utc::now())
    }

    pub fn should_renew(&self, threshold_seconds: i64) -> bool {
        let now = chrono::Utc::now();
        let expires = chrono::DateTime::from(self.expires_at.clone());

        (expires - now).num_seconds() <= threshold_seconds
    }
}
