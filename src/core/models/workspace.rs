use crate::core::models::{ids::*, user::Name};
use ::serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

// A Workspace represents a collaborative space owned by a single user.
//
// Each workspace has exactly one owner.
// Only the owner is authorized to modify or delete the workspace.
// This restriction is enforced at the service layer; the model itself
// assumes that all applied patches have already been authorized.

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Workspace {
    pub id: Option<WorkspaceId>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub is_deleted: bool,
    pub owner: UserId,
    pub name: Name,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InsertWorkspace {
    pub(crate) owner: UserId,
    pub(crate) name: Name,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct WorkspacePatch {
    pub(crate) name: Option<Name>,
    pub(crate) is_deleted: Option<bool>,
}

impl Workspace {
    pub fn from_insert(insert: InsertWorkspace) -> Self {
        Workspace {
            id: None,
            created_at: Datetime::from(chrono::Utc::now()),
            updated_at: Datetime::from(chrono::Utc::now()),
            is_deleted: false,
            owner: insert.owner,
            name: insert.name,
        }
    }

    pub fn apply_patch(&mut self, patch: WorkspacePatch) {
        if let Some(v) = patch.name {
            self.name = v;
        };
        if let Some(v) = patch.is_deleted {
            self.is_deleted = v;
        };
        self.updated_at = Datetime::from(chrono::Utc::now());
    }
}
