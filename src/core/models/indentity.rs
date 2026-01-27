use crate::core::models::ids::*;
use ::serde::{Deserialize, Serialize};
use surrealdb::sql::*;

// Identity linking a Hack Club external ID (HCID) to an internal UserId.
//
// This mapping is immutable for normal users and may only be modified by
// administrators or internal system processes.
//
// Security invariants:
// - Each HCID is globally unique
// - Each HCID maps to exactly one UserId
// - A UserId may be linked to at most one HCID
//
// This model is used exclusively for Hack Club OAuth.
// User authentication after login relies on session tokens bound to
// expiration, origin (IP / location), and device context.

#[derive(Debug, thiserror::Error)]
pub enum HcidError {
    #[error("invalid HCID format")]
    NotAnID,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HCID(String);

impl HCID {
    pub fn new(id: impl Into<String>) -> Result<Self, HcidError> {
        let id = id.into();
        if id.starts_with("ident!") {
            Ok(Self(id))
        } else {
            Err(HcidError::NotAnID)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Identity {
    pub id: Option<IdentityId>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub is_deleted: bool,
    pub user: UserId,
    pub external_user_id: HCID,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InsertIdentity {
    pub(crate) external_user_id: HCID,
    pub(crate) user: UserId,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct IdentityPatch {
    pub(crate) is_deleted: Option<bool>,
    pub(crate) external_user_id: Option<HCID>,
}

impl Identity {
    pub fn from_insert(insert: InsertIdentity) -> Self {
        Identity {
            id: None,
            created_at: Datetime::from(chrono::Utc::now()),
            updated_at: Datetime::from(chrono::Utc::now()),
            is_deleted: false,
            external_user_id: insert.external_user_id,
            user: insert.user,
        }
    }
    pub fn apply_patch(&mut self, patch: IdentityPatch) {
        if let Some(v) = patch.is_deleted {
            self.is_deleted = v;
        }
        if let Some(v) = patch.external_user_id {
            self.external_user_id = v;
        }
        self.updated_at = Datetime::from(chrono::Utc::now());
    }
}
