use std::collections::HashMap;

use crate::{bitmask_serde, core::models::ids::*};
use ::serde::{Deserialize, Serialize};

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

impl UserPermissions {
    /// Global check for Workspace actions
    pub fn can_workspace(&self, perm: WorkspacePermission) -> bool {
        self.workspace.contains(perm)
    }

    /// Check if user can perform an action on a specific Base
    pub fn can_base(&self, base_id: &BaseId, perm: BasePermission) -> bool {
        // Workspace Admins/Owners usually bypass Base checks
        if self.workspace.contains(WorkspacePermission::Edit) {
            return true;
        }

        self.bases
            .get(base_id)
            .map(|b| b.base.contains(perm))
            .unwrap_or(false)
    }

    /// Check Table access: Validates Base -> Table
    pub fn can_table(&self, base_id: &BaseId, table_id: &TableId, perm: TablePermission) -> bool {
        if self.workspace.contains(WorkspacePermission::Edit) {
            return true;
        }

        self.bases
            .get(base_id)
            .and_then(|b| b.tables.get(table_id))
            .map(|t| t.table.contains(perm))
            .unwrap_or(false)
    }

    /// Check Field access: Validates Base -> Table -> Field override
    pub fn can_field(
        &self,
        base_id: &BaseId,
        table_id: &TableId,
        field_id: &FieldId,
        perm: FieldPermission,
    ) -> bool {
        if self.workspace.contains(WorkspacePermission::Edit) {
            return true;
        }

        let table_sub = self.bases.get(base_id).and_then(|b| b.tables.get(table_id));

        match table_sub {
            Some(ts) => {
                match ts.fields.get(field_id) {
                    Some(f_perm) => f_perm.contains(perm),
                    None => {
                        // Fallback: If no specific field perm, does the table allow viewing/editing fields generally?
                        match perm {
                            FieldPermission::View => ts.table.contains(TablePermission::ViewFields),
                            FieldPermission::Edit => ts.table.contains(TablePermission::EditFields),
                            _ => false,
                        }
                    }
                }
            }
            None => false,
        }
    }
}

// so for permissions we will need
// can() user do stuff,
// change permissions,
// and presets, like, for new users what they can do, or just for guests what they can do etc
// (we give the workspace role)

bitmask! {
    pub mask WorkspacePermissions: u32 where flags WorkspacePermission {
        Edit = 0x1,
        Delete = 0x2,
        ManageRoles = 0x4,
        ManageUsers = 0x8,
        ExportData = 0x10,
        ManageIntegrations = 0x20,
        ViewAuditLogs = 0x40,
    }
}

bitmask! {
    pub mask WorkspaceUsersPermissions: u32 where flags WorkspaceUsersPermission {
        Invite = 0x1,
        Desactivate = 0x2,
        Activate = 0x4,
        Ban = 0x8,
        Promote = 0x10,
        Demote = 0x20,
        ViewDeletedUsers = 0x40,
    }
}

bitmask! {
    pub mask BasePermissions: u32 where flags BasePermission {
        View = 0x1,
        Edit = 0x2,
        Delete = 0x4,
        ManageTables = 0x8,
        ManageViews = 0x10,
        ManageUsers = 0x20,
    }
}

bitmask! {
    pub mask TablePermissions: u32 where flags TablePermission {
        CreateRecords = 0x1,
        EditRecords = 0x2,
        DeleteRecords = 0x4,
        ViewRecords = 0x8,
        CreateFields = 0x10,
        EditFields = 0x20,
        DeleteFields = 0x40,
        ViewFields = 0x80,
        CreateViews = 0x100,
        EditViews = 0x200,
        DeleteViews = 0x400,
        ViewViews = 0x800,
        Archive = 0x1000,
        Edit = 0x2000,
        Delete = 0x4000,
        View = 0x8000,
        BulkImport = 0x10000,
        LockSchema = 0x20000,
        ExportTable = 0x40000,
    }
}

bitmask! {
    pub mask FieldPermissions: u32 where flags FieldPermission {
        View = 0x1,
        Edit = 0x2,
        Delete = 0x4,
        AddFormula = 0x8,
        HideFromSearch = 0x10,
    }
}

bitmask! {
    pub mask RecordPermissions: u32 where flags RecordPermission {
        View = 0x1,
        Edit = 0x2,
        Delete = 0x4,
        Comment = 0x8,
        Share = 0x10,
    }
}

bitmask_serde!(WorkspacePermissions);
bitmask_serde!(WorkspaceUsersPermissions);
bitmask_serde!(BasePermissions);
bitmask_serde!(TablePermissions);
bitmask_serde!(FieldPermissions);
bitmask_serde!(RecordPermissions);
