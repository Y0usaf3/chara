use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use crate::models::ids::{BaseId, TableId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub id: Option<TableId>,
    pub base: BaseId,
    pub name: String,
    pub order: i32, // wait wait wait, why did i added a order ??? if that was the list i could
    // just used the created_at lol
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

impl Table {
    pub fn from_insert(input: InsertTable) -> Self {
        Self {
            id: None,
            base: input.base,
            name: input.name,
            order: input.order,
            created_at: Datetime::default(),
            updated_at: Datetime::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertTable {
    pub base: BaseId,
    pub name: String,
    pub order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TablePatch {
    pub name: Option<String>,
    pub order: Option<i32>,
}
