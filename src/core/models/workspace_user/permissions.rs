use std::collections::HashMap;

use crate::{bitmask_serde, core::models::ids::*};
use ::serde::{Deserialize, Serialize};

// alr folks, im going to rewrite this
// * do nothing *
// well, i think we ACTUALLY have to rewrite this
// * plays penumbra phantasm in the bg for epicness *
// yeah no ill actually just write my ideas here * penumbra phantasm still playing *

// 5min pass

// alr so uh, instead that each thing have its own permission, what we'll do is that we have a
// role, but its diff in each base or table, (got self confused abt graphs uh, lemme read this
// again)

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UserPermissions {
    pub workspace: WorkspacePermissions,
    pub workspace_users: HashMap<WorkspaceUserId, WorkspaceUsersPermissions>,
    pub bases: HashMap<BaseId, BaseSubPermissions>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct BaseSubPermissions {
    pub base: BasePermissions,
    pub tables: HashMap<TableId, TableSubPermissions>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TableSubPermissions {
    pub table: TablePermissions,
    pub fields: HashMap<FieldId, FieldPermissions>,
    pub records: HashMap<RecordId, RecordPermissions>,
}

// so for permissions we will need
// can() user do stuff,
// change permissions,
// and presets, like, for new users what they can do, or just for guests what they can do etc
// (we give the workspace role)

bitmask! {
    pub mask WorkspacePermissions: u32 where flags WorkspacePermission {
        Edit               = 1 << 0, // OWNER / ADMIN (limited)
        Delete             = 1 << 1, // OWNER
        ManageRoles        = 1 << 2, // OWNER / ADMIN
        ManageUsers        = 1 << 3, // OWNER / ADMIN
        ExportData         = 1 << 4, // OWNER / ADMIN / USER
        ManageIntegrations = 1 << 5, // OWNER / ADMIN
        ViewAuditLogs      = 1 << 6, // OWNER / ADMIN
    }
}

bitmask! {
    pub mask WorkspaceUsersPermissions: u32 where flags WorkspaceUsersPermission {
        Invite           = 1 << 0, // OWNER / ADMIN / USER
        Desactivate      = 1 << 1, // OWNER / ADMIN
        Activate         = 1 << 2, // OWNER / ADMIN
        Ban              = 1 << 3, // OWNER / ADMIN
        Promote          = 1 << 4, // OWNER
        Demote           = 1 << 5, // OWNER
        ViewDeletedUsers = 1 << 6, // OWNER / ADMIN
    }
}

bitmask! {
    pub mask BasePermissions: u32 where flags BasePermission {
        View         = 1 << 0, // OWNER / ADMIN / USER (base owner)
        Edit         = 1 << 1, // OWNER / ADMIN / USER ...
        Delete       = 1 << 2, // OWNER / ADMIN / USER ...
        ManageTables = 1 << 3, // OWNER / ADMIN / USER ...
        ManageViews  = 1 << 4, // OWNER / ADMIN / USER ...
        ManageUsers  = 1 << 5, // OWNER / ADMIN / USER ...
    } // ↑↑ in a base we only manage user permissions
}

bitmask! {
    pub mask TablePermissions: u32 where flags TablePermission {
        CreateRecords = 1 << 0, // ADMIN / OWNER / USER
        EditRecords   = 1 << 1, // ADMIN / OWNER / USER
        DeleteRecords = 1 << 2, // ADMIN / OWNER / USER
        ViewRecords   = 1 << 3, // ADMIN / OWNER / USER
        CreateFields  = 1 << 4, // ADMIN / OWNER / USER
        EditFields    = 1 << 5, // ADMIN / OWNER / USER
        DeleteFields  = 1 << 6, // ADMIN / OWNER / USER
        ViewFields    = 1 << 7, // ADMIN / OWNER / USER
        CreateViews   = 1 << 8, // ADMIN / OWNER / USER
        EditViews     = 1 << 9, // ADMIN / OWNER / USER
        DeleteViews   = 1 << 10, // ADMIN / OWNER / USER
        ViewViews     = 1 << 11, // ADMIN / OWNER / USER
        Archive       = 1 << 12, // ADMIN / OWNER / USER
        Edit          = 1 << 13, // ADMIN / OWNER / USER
        Delete        = 1 << 14, // ADMIN / OWNER / USER
        View          = 1 << 15, // ADMIN / OWNER / USER
        BulkImport    = 1 << 16, // ADMIN / OWNER / USER
        LockSchema    = 1 << 17, // ADMIN / OWNER / USER
        ExportTable   = 1 << 18, // ADMIN / OWNER / USER
    }
}

bitmask! {
    pub mask FieldPermissions: u32 where flags FieldPermission {
        View           = 1 << 0, // ADMIN / OWNER / USER
        Edit           = 1 << 1, // ADMIN / OWNER / USER
        Delete         = 1 << 2, // ADMIN / OWNER / USER
        Comment        = 1 << 3, // ADMIN / OWNER / USER
    }
}

bitmask! {
    pub mask RecordPermissions: u32 where flags RecordPermission {
        View    = 1 << 0, // ADMIN / OWNER / USER
        Edit    = 1 << 1, // ADMIN / OWNER / USER
        Delete  = 1 << 2, // ADMIN / OWNER / USER
        Comment = 1 << 3, // ADMIN / OWNER / USER
        Share   = 1 << 4, // ADMIN / OWNER / USER
    }
}
bitmask_serde!(WorkspacePermissions);
bitmask_serde!(WorkspaceUsersPermissions);
bitmask_serde!(BasePermissions);
bitmask_serde!(TablePermissions);
bitmask_serde!(FieldPermissions);
bitmask_serde!(RecordPermissions);
