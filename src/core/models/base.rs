use crate::core::models::{ids::*, user::Name};
use ::serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

// A Base represents a sub-entity within a Workspace.
//
// Each Base belongs to exactly one Workspace, identified by `workspace`.
// Only authorized users of the parent Workspace (typically the owner or admins)
// are allowed to modify or soft-delete a Base.

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Base {
    pub id: Option<BaseId>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub is_deleted: bool,
    pub workspace: WorkspaceId,
    pub name: Name,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InsertBase {
    pub(crate) workspace: WorkspaceId,
    pub(crate) name: Name,
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
            workspace: insert.workspace,
            name: insert.name,
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
