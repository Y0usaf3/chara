use crate::core::models::{ids::*, user::Name};
use ::serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

// A Table represents a logical container within a Base.
//
// Each Table belongs to exactly one Base.
// Only authorized users of the parent Base (typically the workspace owner or admins)
// may modify or soft-delete a Table.

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Table {
    pub id: Option<TableId>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub is_deleted: bool,
    pub name: Name,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InsertTable {
    pub(crate) name: Name,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TablePatch {
    pub(crate) name: Option<Name>,
    pub(crate) is_deleted: Option<bool>,
}

impl Table {
    pub fn from_insert(insert: InsertTable) -> Self {
        Table {
            id: None,
            created_at: Datetime::from(chrono::Utc::now()),
            updated_at: Datetime::from(chrono::Utc::now()),
            is_deleted: false,
            name: insert.name,
        }
    }

    pub fn apply_patch(&mut self, patch: TablePatch) {
        if let Some(v) = patch.name {
            self.name = v;
        };
        if let Some(v) = patch.is_deleted {
            self.is_deleted = v;
        };
        self.updated_at = Datetime::from(chrono::Utc::now());
    }
}
