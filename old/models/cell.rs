use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use crate::models::ids::{CellId, FieldId, RecordId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub id: Option<CellId>,
    pub record: RecordId,
    pub field: FieldId,
    pub value: serde_json::Value,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertCell {
    pub record: RecordId,
    pub field: FieldId,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellPatch {
    pub value: Option<serde_json::Value>,
}

impl Cell {
    pub fn from_insert(input: InsertCell) -> Self {
        let now = Datetime::from(chrono::Utc::now());
        Self {
            id: None,
            record: input.record,
            field: input.field,
            value: input.value,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn apply_patch(&mut self, patch: CellPatch) {
        if let Some(value) = patch.value {
            self.value = value;
            self.updated_at = Datetime::from(chrono::Utc::now());
        }
    }
}
