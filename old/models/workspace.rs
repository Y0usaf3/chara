use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use crate::models::ids::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: Option<WorkspaceId>,
    pub owner: UserId,
    pub name: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub is_soft_deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertWorkspace {
    pub owner: UserId,
    pub name: String,
}

impl Workspace {
    pub fn from_insert(input: InsertWorkspace) -> Self {
        Self {
            id: None,
            owner: input.owner,
            name: input.name,
            created_at: Datetime::default(),
            updated_at: Datetime::default(),
            is_soft_deleted: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspacePatch {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminWorkspacePatch {
    pub is_soft_deleted: bool,
}
