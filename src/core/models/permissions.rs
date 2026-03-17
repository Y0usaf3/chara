use crate::bitmask_serde;
use ::serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

// alr folks, im going to rewrite this
// * do nothing *
// well, i think we ACTUALLY have to rewrite this
// * plays penumbra phantasm in the bg for epicness *
// yeah no ill actually just write my ideas here * penumbra phantasm still playing *

// 5min pass

// alr so uh, instead that each thing have its own permission, what we'll do is that we have a
// role, but its diff in each base or table, (got self confused abt graphs uh, lemme read this
// again)

// so for permissions we will need
// can() user do stuff,
// change permissions,
// and presets, like, for new users what they can do, or just for guests what they can do etc
// (we give the workspace role)

// ok so instead we'll make each permissions as a relation , and bc surreal db store them as
// tables, we'll need to know how the table will be

// btw welcome to my macro insanity lol

// just realised how much insanity i put in this single file

#[macro_export]
macro_rules! relation {
    ( $( $x:ident ,$y:ident), * ) => {
        #[derive(Deserialize, Serialize, PartialEq, Eq, SurrealValue)]
        $(pub struct $x {
            pub perm: $y,
        })*
    };
}

bitmask! {
    pub mask BasePermissions: i32 where flags BasePermission {
        Admin = 1 << 0,
        View = 1 << 1,
        Edit = 1 << 2,
        Delete = 1 << 3,

        ManageTables = 1 << 4,
        ManageViews = 1 << 5,
        ManagerUserPermissions = 1 << 6,
        ManageAutomatisations = 1 << 7
    }
}
bitmask_serde!(BasePermissions);
relation!(CanAccessBase, BasePermissions);

bitmask! {
    pub mask TablePermissions: i32 where flags TablePermission {
        Admin = 1 << 0,
        View = 1 << 1,
        Edit = 1 << 2,
        Delete = 1 << 3,

        BulkImport = 1 << 4,
        LockFields = 1 << 5,
        Export = 1 << 6,
        Archive = 1 << 7
    }
}
bitmask_serde!(TablePermissions);
relation!(CanAccessTable, TablePermissions);

bitmask! {
    pub mask FieldPermissions: i32 where flags FieldPermission {
        Admin = 1 << 0,
        View = 1 << 1,
        Edit = 1 << 2,
        Delete = 1 << 3,

        Comment = 1 << 4,
        Lock = 1 << 5,
        // XXXXXX = 1 << 6,
        // XXXXXX = 1 << 7
    }
}
bitmask_serde!(FieldPermissions);
relation!(CanAccessField, FieldPermissions);
