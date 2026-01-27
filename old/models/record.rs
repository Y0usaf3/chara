use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use crate::models::ids::{RecordId, TableId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub id: Option<RecordId>,
    pub table: TableId,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertRecord {
    pub table: TableId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordPatch {
    // Extend later if we allow modifying record metadata
}

impl Record {
    pub fn from_insert(input: InsertRecord) -> Self {
        let now = Datetime::from(chrono::Utc::now());
        Self {
            id: None,
            table: input.table,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn touch(&mut self) {
        // GOD DAMN IT I FORGOT TO IMPL TS FEATUR TO OTHER STRUCT OMG
        // Update updated_at timestamp
        self.updated_at = Datetime::from(chrono::Utc::now());
    }
}
