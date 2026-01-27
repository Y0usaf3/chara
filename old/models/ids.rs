use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkspaceId(pub Thing);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BaseId(pub Thing);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TableId(pub Thing);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserId(pub Thing);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CellId(pub Thing);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RecordId(pub Thing);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FieldId(pub Thing);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RelationId(pub Thing);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IdentityId(pub Thing);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SessionId(pub Thing);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ViewId(pub Thing);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkspaceUserId(pub Thing);

pub trait Key {
    fn key(&self) -> String;
}

macro_rules! impl_key { // god dang it macros are so damn hard to read, i wrote this tho...
    ($($t:ty),*) => {
        $(
            impl Key for $t {
                fn key(&self) -> String {
                    self.0.id.to_string()
                }
            }
        )*
    }
}

// Inchelligence ☉ ‿ ⚆ // oh lmao
impl_key!(
    UserId,
    WorkspaceId,
    BaseId,
    TableId,
    CellId,
    RecordId,
    FieldId,
    RelationId,
    IdentityId,
    SessionId,
    ViewId,
    WorkspaceUserId
);
