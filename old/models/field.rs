use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use crate::models::ids::{FieldId, TableId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectConfig {
    pub options: Vec<String>,
    pub multiple: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationConfig {
    pub table: TableId,
    pub multiple: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FieldType {
    Text,
    Number,
    Bool,
    Date,
    Select { config: SelectConfig },
    Relation { config: RelationConfig },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationKind {
    Safe,       // can convert automatically without losing data
    NeedsMap,   // requires user-provided mapping (eg, Text -> Select)
    Risky,      // may lose data, must warn
    Impossible, // cannot migrate
}

impl FieldType {
    pub fn is_scalar(&self) -> bool {
        matches!(self, Self::Text | Self::Number | Self::Bool | Self::Date)
    }

    pub fn allows_multiple(&self) -> bool {
        match self {
            FieldType::Select { config } => config.multiple,
            FieldType::Relation { config } => config.multiple,
            _ => false,
        }
    }

    pub fn validate(&self) -> Result<(), FieldTypeError> {
        match self {
            FieldType::Select { config } => {
                if config.options.is_empty() {
                    Err(FieldTypeError::InvalidSelectConfig)
                } else {
                    Ok(())
                }
            }
            FieldType::Relation { .. } => Ok(()),
            _ => Ok(()),
        }
    }

    pub fn migration_kind_to(&self, target: &FieldType) -> MigrationKind {
        match (self, target) {
            // safe
            (FieldType::Text, FieldType::Text)
            | (FieldType::Number, FieldType::Number)
            | (FieldType::Bool, FieldType::Bool)
            | (FieldType::Date, FieldType::Date) => MigrationKind::Safe,

            // require mapping
            (FieldType::Text, FieldType::Select { .. })
            | (FieldType::Text, FieldType::Number)
            | (FieldType::Number, FieldType::Text)
            | (FieldType::Bool, FieldType::Text) => MigrationKind::NeedsMap,

            // relation to relation
            (FieldType::Relation { config: c1 }, FieldType::Relation { config: c2 }) => {
                if c1.table == c2.table {
                    MigrationKind::Safe
                } else {
                    MigrationKind::Risky
                }
            }

            // all other combinations
            _ => MigrationKind::Impossible,
        }
    }
}

/// Error type for FieldType validation
#[derive(Debug, thiserror::Error)]
pub enum FieldTypeError {
    #[error("Invalid select field config")]
    InvalidSelectConfig,

    #[error("Invalid relation field config")]
    InvalidRelationConfig,
}

/// Database representation of a Field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub id: Option<FieldId>,
    pub table: TableId,
    pub name: String,
    pub field_type: FieldType,
    pub order: i32,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

impl Field {
    pub fn from_insert(input: InsertField) -> Self {
        let now = Datetime::from(chrono::Utc::now());
        Self {
            id: None,
            table: input.table,
            name: input.name,
            field_type: input.field_type,
            order: input.order,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertField {
    pub table: TableId,
    pub name: String,
    pub field_type: FieldType,
    pub order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldPatch {
    pub name: Option<String>,
    pub order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MigrationStrategy {
    FailIfDataExists, // only allow if no records exist
    Cast,             // try to automatically convert values
    Map,              // provider mapping rules by the user
    Reset,            // drop old values
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldTypeChange {
    pub from: FieldType,
    pub to: FieldType,
    pub strategy: MigrationStrategy,
}

impl MigrationKind {
    pub fn is_allowed(&self) -> bool {
        match self {
            MigrationKind::Safe | MigrationKind::NeedsMap | MigrationKind::Risky => true,
            _ => false,
        }
    }

    pub fn requires_user_input(&self) -> bool {
        match self {
            MigrationKind::NeedsMap | MigrationKind::Risky => true,
            _ => false,
        }
    }
}
