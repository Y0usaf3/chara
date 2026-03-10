use crate::core::models::ids::*;
use ::serde::{Deserialize, Serialize};
use chrono::Duration;
use surrealdb::types::Datetime;
use surrealdb_types::SurrealValue;

#[derive(Deserialize, Serialize, SurrealValue, Debug)]
pub struct Session {
    pub id: Option<SessionId>,
    pub user: UserId,
    pub token: String,
    pub ip: String,
    pub user_agent: String,
    pub created_at: Datetime,
    pub expires_at: Datetime, // make surrealdb drop the session if it expires
}

#[derive(Deserialize, Serialize)]
pub struct InsertSession {
    pub(crate) user: UserId,
    pub(crate) token: String,
    pub(crate) ip: String,
    pub(crate) user_agent: String,
}

// no session patch bc we're not supposed to change this at all

impl Session {
    pub fn from_insert(insert: InsertSession) -> Self {
        Session {
            id: None,
            token: insert.token,
            ip: insert.ip,
            user_agent: insert.user_agent,
            created_at: Datetime::from(chrono::Utc::now()),
            expires_at: Datetime::from(chrono::Utc::now() + Duration::days(2)), // hardened expiration :p
            user: insert.user,
        }
    }
}
