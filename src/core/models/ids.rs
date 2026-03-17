use serde::{Deserialize, Serialize};
use surrealdb::types::record_id::RecordId as Thing;
use surrealdb::types::SurrealValue;

macro_rules! define_ids {
    ($($name:ident),*) => {
        $(
            #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, SurrealValue)]
            pub struct $name(pub  Thing);
        )*
    };
}

// Now you can define all of them at once:
define_ids!(
    BaseId, TableId, UserId, CellId, RowId, RecordId, FieldId, RelationId, IdentityId, SessionId,
    ViewId
);

pub trait Key {
    fn key(&self) -> String;
}

//macro_rules! impl_key { // god dang it macros are so damn hard to read, i wrote this tho...
//    ($($t:ty),*) => {
//        $(
//            impl Key for $t {
//                fn key(&self) -> String {
//                    self.to_str
//                }
//            }
//        )*
//    }
//}
//
//// Inchelligence ☉ ‿ ⚆ // oh lmao
//impl_key!(
//    UserId,
//    WorkspaceId,
//    BaseId,
//    TableId,
//    CellId,
//    RecordId,
//    FieldId,
//    RelationId,
//    IdentityId,
//    SessionId,
//    ViewId,
//    WorkspaceUserId
//);
