use crate::models::ids::*;
use crate::models::permissions::*;
use crate::models::workspace_users::*;
use crate::services::{Error, DB};

const WORKSPACE_USER: &str = "workspace_user";

/// Service for managing users within a workspace.
/// Bound to a workspace user (actor) and scoped to a specific workspace.
pub struct WorkspaceUserService {
    /// The workspace this service is bound to.
    pub workspace_id: WorkspaceId,

    /// The workspace user performing actions.
    pub actor: WorkspaceUser,
}

impl WorkspaceUserService {
    /// Initialize the service for a workspace and user.
    ///
    /// Ensures that the workspace user exists and is part of the workspace.
    pub async fn init(workspace_id: &WorkspaceId, user_id: &UserId) -> Result<Self, Error> {
        let workspace_user: WorkspaceUser = DB
            .select((
                WORKSPACE_USER,
                format!("{}:{}", workspace_id.key(), user_id.key()),
            ))
            .await?
            .ok_or(Error::NotFound)?;

        Ok(Self {
            workspace_id: workspace_id.clone(),
            actor: workspace_user,
        })
    }

    /// Check if the actor is an admin in this workspace.
    pub fn is_admin(&self) -> bool {
        self.actor.is_admin()
    }

    /// Check if the actor is a guest in this workspace.
    pub fn is_guest(&self) -> bool {
        self.actor.is_guest()
    }

    /// Check if the actor is a regular user in this workspace.
    pub fn is_user(&self) -> bool {
        self.actor.is_user()
    }

    /// Check if the actor has a specific permission on a table.
    pub fn can(&self, table_id: &TableId, permission: TablePermission) -> bool {
        self.actor.can(table_id, permission)
    }

    // ---------------- Workspace User Management ----------------

    /// Add a new user to the workspace.
    ///
    /// Only admins can add new users.
    pub async fn add_user(&self, input: InsertWorkspaceUser) -> Result<WorkspaceUser, Error> {
        if !self.is_admin() {
            return Err(Error::Forbidden);
        }

        // Enforce workspace scope
        if input.workspace_id != self.workspace_id {
            return Err(Error::Forbidden);
        }

        DB.create(WORKSPACE_USER)
            .content(WorkspaceUser::from_insert(input))
            .await?
            .ok_or(Error::AlreadyExists)
    }

    /// Get a workspace user info.
    ///
    /// Admins can see any user, others only themselves.
    pub async fn get_user(&self, user_id: &UserId) -> Result<WorkspaceUser, Error> {
        let key = format!("{}:{}", self.workspace_id.key(), user_id.key());
        let user: WorkspaceUser = DB
            .select((WORKSPACE_USER, key))
            .await?
            .ok_or(Error::NotFound)?;

        if !self.is_admin() && self.actor.user_id != *user_id {
            return Err(Error::Forbidden);
        }

        Ok(user)
    }

    /// Update a workspace user.
    ///
    /// Only admins can update other users.
    pub async fn update_user(
        &self,
        user_id: &UserId,
        patch: AdminWorkspaceUserPatch, // define this struct
    ) -> Result<WorkspaceUser, Error> {
        if !self.is_admin() {
            return Err(Error::Forbidden);
        }

        let mut res = DB
            .query(
                r#"
                UPDATE workspace_user
                SET $patch
                WHERE workspace_id = $workspace_id
                  AND user_id = $user_id
                RETURN AFTER;
                "#,
            )
            .bind(("workspace_id", self.workspace_id.key()))
            .bind(("user_id", user_id.key()))
            .bind(("patch", patch))
            .await?;

        let user: Option<WorkspaceUser> = res.take(0)?;
        user.ok_or(Error::Db)
    }

    /// Remove (hard delete) a user from the workspace.
    ///
    /// Only admins can remove users.
    pub async fn remove_user(&self, user_id: &UserId) -> Result<WorkspaceUser, Error> {
        if !self.is_admin() {
            return Err(Error::Forbidden);
        }

        let key = format!("{}:{}", self.workspace_id.key(), user_id.key());
        DB.delete((WORKSPACE_USER, key)).await?.ok_or(Error::Db)
    }

    /// Soft delete a user (mark them as deleted but keep data).
    pub async fn soft_delete_user(&self, user_id: &UserId) -> Result<WorkspaceUser, Error> {
        if !self.is_admin() {
            return Err(Error::Forbidden);
        }

        let key = format!("{}:{}", self.workspace_id.key(), user_id.key());
        DB.update((WORKSPACE_USER, key))
            .content(AdminWorkspaceUserPatch {
                is_soft_deleted: Some(true),
                role: None,
                permissions: None,
                username: None,
            })
            .await?
            .ok_or(Error::Db)
    }

    /// List all users in the workspace.
    ///
    /// Admins see all users, others see only themselves.
    pub async fn list_users(&self) -> Result<Vec<WorkspaceUser>, Error> {
        if self.is_admin() {
            let users: Vec<WorkspaceUser> = DB
                .query("SELECT * FROM workspace_user WHERE workspace_id = $workspace_id")
                .bind(("workspace_id", self.workspace_id.key()))
                .await?
                .take(0)?;
            Ok(users)
        } else {
            Ok(vec![self.actor.clone()])
        }
    }
}
