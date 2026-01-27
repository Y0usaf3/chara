use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use crate::models::ids::{FieldId, RecordId, RelationId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub id: Option<RelationId>,
    pub from_record: RecordId,
    pub to_record: RecordId,
    pub field: FieldId,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertRelation {
    pub from_record: RecordId,
    pub to_record: RecordId,
    pub field: FieldId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationPatch {
    // For now, we may allow updating the field link
    pub field: Option<FieldId>,
}

impl Relation {
    pub fn from_insert(input: InsertRelation) -> Self {
        let now = Datetime::from(chrono::Utc::now());
        Self {
            id: None,
            from_record: input.from_record,
            to_record: input.to_record,
            field: input.field,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn apply_patch(&mut self, patch: RelationPatch) {
        if let Some(field) = patch.field {
            self.field = field;
            self.updated_at = Datetime::from(chrono::Utc::now());
        }
    }
}

// i feel like i fucked up the architecture of each service so bad
