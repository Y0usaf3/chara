use crate::core::models::{ids::*, workspace_user::permissions::UserPermissions};
use ::serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

mod permissions;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WorkspaceUserRole {
    Guest,
    User,
    Admin, // the admin can have permissions that are admin only
    Owner, // only the owner can change permissions, admin can but cant give admin only permissions
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct WorkspaceUser {
    pub id: Option<WorkspaceUserId>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub is_deleted: bool,
    pub user_id: UserId,
    pub workspace_id: WorkspaceId,
    pub username: Option<String>,
    pub invited_by: WorkspaceUserId,
    pub role: WorkspaceUserRole,
    pub permissions: Option<UserPermissions>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InsertWorkspaceUser {
    pub username: Option<String>,
    pub invited_by: WorkspaceUserId,
    pub role: Option<WorkspaceUserRole>, // if None then its a guest
    pub workspace_id: WorkspaceId,
    pub user_id: UserId,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct WorkspaceUserPatch {
    pub username: Option<String>,
    pub role: Option<WorkspaceUserRole>,
    pub permissions: Option<UserPermissions>,
}

impl WorkspaceUser {
    pub fn from_insert(insert: InsertWorkspaceUser) -> Self {
        WorkspaceUser {
            id: None,
            created_at: Datetime::from(chrono::Utc::now()),
            updated_at: Datetime::from(chrono::Utc::now()),
            is_deleted: false,
            user_id: insert.user_id,
            workspace_id: insert.workspace_id,
            username: insert.username,
            invited_by: insert.invited_by,
            role: insert.role.unwrap_or(WorkspaceUserRole::Guest),
            permissions: None,
        }
    }

    pub fn apply_patch(&mut self, patch: WorkspaceUserPatch, client_role: WorkspaceUserRole) {
        if let Some(v) = patch.username {
            self.username = Some(v)
        };
        if client_role == WorkspaceUserRole::Owner
            && let Some(v) = patch.role
        {
            self.role = v
        };
    }
}
