use crate::models::ids::{BaseId, TableId, UserId, WorkspaceId};
use crate::models::permissions::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::fmt::Debug;
use surrealdb::sql::Datetime;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WorkspaceUserRole {
    Guest,
    User,
    Admin, // The "owner" should be the Admin with full authority in the workspace
}
// so here, in the roles, if its a guest, we dont need to read the permissions, bc a guest can only
// read, its only when its a user that we start reading its permissions, for a table, or a cell, or
// even a row, we might want to keep things private, or public, wich is smt i have to impl later

#[derive(Serialize, Deserialize, Clone)]
pub struct TableSubPermissions {
    pub table: TablePermissions,
    pub cells: Option<CellPermissions>,
    pub fields: Option<FieldPermissions>,
    pub records: Option<RecordPermissions>,
    pub relations: Option<RelationPermissions>,
}

// Stores fine-grained permissions for tables/bases/etc per workspace user
#[derive(Serialize, Deserialize, Clone)]
pub struct UserPermissions {
    pub tables: Option<HashMap<TableId, TableSubPermissions>>,
    pub bases: Option<HashMap<BaseId, BasePermissions>>,
    pub workspaces: Option<HashMap<WorkspaceId, WorkspacePermissions>>,
    pub workspace_users: Option<HashMap<WorkspaceId, WorkspaceUsersPermissions>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WorkspaceUser {
    pub workspace_id: WorkspaceId, // The workspace this entry belongs to
    pub user_id: UserId,           // Reference to the GLOBAL user
    pub joined_at: Datetime,
    pub last_modified: Datetime,
    pub username: Option<String>, // Optional custom username IN THIS workspace
    pub role: WorkspaceUserRole,  // Guest, User, Admin
    pub permissions: Option<UserPermissions>, // Fine-grained per-table/base/etc permissions
    pub invited_by: Option<UserId>, // Who invited the user, if any
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InsertWorkspaceUser {
    pub workspace_id: WorkspaceId,
    pub user_id: UserId,
    pub role: WorkspaceUserRole,
    pub username: Option<String>,
    pub invited_by: Option<UserId>,
    pub permissions: Option<UserPermissions>,
}

#[derive(Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct WorkspaceUserPatch {
    pub username: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct AdminWorkspaceUserPatch {
    pub username: Option<String>,
    pub role: Option<WorkspaceUserRole>,
    pub permissions: Option<UserPermissions>,
    pub is_soft_deleted: Option<bool>,
}

impl WorkspaceUser {
    pub fn from_insert(input: InsertWorkspaceUser) -> Self {
        let now = Datetime::default();
        Self {
            workspace_id: input.workspace_id,
            user_id: input.user_id,
            joined_at: now.clone(),
            last_modified: now,
            username: input.username,
            role: input.role,
            permissions: input.permissions,
            invited_by: input.invited_by,
        }
    }

    pub fn is_admin(&self) -> bool {
        matches!(self.role, WorkspaceUserRole::Admin)
    }

    pub fn is_guest(&self) -> bool {
        matches!(self.role, WorkspaceUserRole::Guest)
    }

    pub fn is_user(&self) -> bool {
        matches!(self.role, WorkspaceUserRole::User)
    }

    pub fn can(&self, table_id: &TableId, permission: TablePermission) -> bool {
        if self.is_admin() {
            return true;
        }
        if let Some(permissions) = &self.permissions
            && let Some(table_perms) = &permissions.tables
            && let Some(perms) = table_perms.get(table_id)
        {
            return perms.table.contains(permission);
        }
        false
    }
}
