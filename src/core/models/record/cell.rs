use ::serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CellValue {
    id: u16, // is the order of the cell
    created_at: Datetime,
    updated_at: Datetime,
    value: Value,
}

// TODO: make a solid new function for each data kind, enforcing the formating and rules of each
// format, so creating or modifying data would be as ez as butterscotch pie

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Value {}
