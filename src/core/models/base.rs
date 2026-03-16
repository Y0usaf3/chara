use crate::core::models::{ids::*, user::Name};
use ::serde::{Deserialize, Serialize};
use surrealdb::types::{Datetime, SurrealValue};

// A Base represents a sub-entity within a Workspace.
//
// Each Base belongs to exactly one Workspace, identified by `workspace`.
// Only authorized users of the parent Workspace (typically the owner or admins)
// are allowed to modify or soft-delete a Base.

#[derive(SurrealValue, Debug, Clone, PartialEq)]
pub struct Base {
    pub id: Option<BaseId>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub is_deleted: bool,
    pub owner: UserId,
    pub name: Name,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InsertBase {
    pub(crate) name: Name,
    pub(crate) owner: UserId,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BasePatch {
    pub(crate) is_deleted: Option<bool>,
    pub(crate) name: Option<Name>,
}

impl Base {
    pub fn from_insert(insert: InsertBase) -> Self {
        Base {
            id: None,
            created_at: Datetime::from(chrono::Utc::now()),
            updated_at: Datetime::from(chrono::Utc::now()),
            is_deleted: false,
            name: insert.name,
            owner: insert.owner,
        }
    }

    pub fn apply_patch(&mut self, patch: BasePatch) {
        if let Some(v) = patch.is_deleted {
            self.is_deleted = v;
        };
        if let Some(v) = patch.name {
            self.name = v;
        };
        self.updated_at = Datetime::from(chrono::Utc::now())
    }
}
