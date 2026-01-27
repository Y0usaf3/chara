use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use crate::models::ids::{BaseId, WorkspaceId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base {
    pub id: Option<BaseId>,
    pub workspace: WorkspaceId,
    pub name: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertBase {
    pub workspace: WorkspaceId,
    pub name: String,
}

impl Base {
    pub fn from_insert(input: InsertBase) -> Self {
        Self {
            id: None,
            workspace: input.workspace,
            name: input.name,
            created_at: Datetime::default(),
            updated_at: Datetime::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasePatch {
    pub name: Option<String>,
}
