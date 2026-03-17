use crate::core::models::ids::*;
use ::serde::{Deserialize, Serialize};
use surrealdb::types::Datetime;
use surrealdb::types::SurrealValue;

#[derive(SurrealValue)]
pub struct Identity {
    pub id: Option<IdentityId>,
    pub user: UserId,
    pub external_user_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: Datetime,

    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub is_deleted: bool,
}

#[derive(Deserialize, Serialize)]
pub struct InsertIdentity {
    pub user: UserId,
    pub external_user_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: Datetime,
}

#[derive(Deserialize, Serialize)]
pub struct IdentityPatch {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<Datetime>,
    pub is_deleted: Option<bool>,
}

impl Identity {
    pub fn from_insert(insert: InsertIdentity) -> Self {
        let now = Datetime::from(chrono::Utc::now());
        Identity {
            id: None,
            user: insert.user,
            external_user_id: insert.external_user_id,
            access_token: insert.access_token,
            refresh_token: insert.refresh_token,
            expires_at: insert.expires_at,
            created_at: now.clone(),
            updated_at: now,
            is_deleted: false,
        }
    }

    pub fn apply_patch(&mut self, patch: IdentityPatch) {
        if let Some(v) = patch.access_token {
            self.access_token = v;
        }
        if let Some(v) = patch.refresh_token {
            self.refresh_token = v;
        }
        if let Some(v) = patch.expires_at {
            self.expires_at = v;
        }
        if let Some(v) = patch.is_deleted {
            self.is_deleted = v;
        }
        self.updated_at = Datetime::from(chrono::Utc::now());
    }
}
