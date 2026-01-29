use crate::core::models::ids::*;
use crate::core::models::{field::kinds::FieldConfig, user::Name};
use ::serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

mod kinds;

/// ['src/core/models/field.md']
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Field {
    id: Option<FieldId>,
    created_at: Datetime,
    updated_at: Datetime,
    is_deleted: bool,
    config: FieldConfig,
    is_primary: bool,
    is_nullable: bool,
    is_unique: bool,
    name: Name,
    description: Option<String>,
}

pub struct InsertField {
    pub(crate) name: Name,
    pub(crate) description: Option<String>,
    pub(crate) is_primary: bool,
    pub(crate) is_nullable: bool,
    pub(crate) is_unique: bool,
    pub(crate) config: FieldConfig,
}

pub struct FieldPatch {
    pub(crate) name: Option<Name>,
    pub(crate) description: Option<String>,
    pub(crate) is_primary: Option<bool>,
    pub(crate) is_nullable: Option<bool>,
    pub(crate) is_unique: Option<bool>,
    pub(crate) config: Option<FieldConfig>,
}

impl Field {
    pub fn from_insert(insert: InsertField) -> Self {
        Field {
            id: None,
            created_at: Datetime::from(chrono::Utc::now()),
            updated_at: Datetime::from(chrono::Utc::now()),
            is_deleted: false,
            config: insert.config,
            is_primary: insert.is_primary,
            is_nullable: insert.is_nullable,
            is_unique: insert.is_unique,
            name: insert.name,
            description: insert.description,
        }
    }
    pub fn apply_patch(&mut self, patch: FieldPatch) {
        // when applying a patch, we're literally
        // migrating, i prefer that we return
        // atleast, a migration enum where we know
        // the migration method, and i should prob
        // make a matrix table for this, so when we
        // apply the patch, we give to the service
        // the migration strategy to convert all of
        // those cells
        if let Some(v) = patch.config {
            self.config = v;
        };
        if let Some(v) = patch.is_primary {
            self.is_primary = v;
        };
        if let Some(v) = patch.is_nullable {
            self.is_nullable = v;
        };
        if let Some(v) = patch.is_unique {
            self.is_unique = v;
        };
        if let Some(v) = patch.name {
            self.name = v;
        };
        if let Some(v) = patch.description {
            self.description = Some(v);
        }
    }
}
