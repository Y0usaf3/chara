use crate::models::ids::*;
use crate::models::user::*;
use crate::models::workspace::*;
use crate::services::user::UserService;
use crate::services::{Error, DB};

const WORKSPACE: &str = "workspace";

/// Service for workspace-related actions.
/// Binds the current user (actor) on creation.
/// Permissions are checked automatically based on actor.
pub struct WorkspaceService {
    actor: User,
}

impl WorkspaceService {
    /// Initialize the service from a session token.
    ///
    /// Checks that the session is valid and the user is not deleted.
    pub async fn init(session_token: &str) -> Result<Self, Error> {
        let user_service = UserService::init(session_token).await?;
        Ok(Self {
            actor: user_service.actor,
        })
    }

    /// Create a new workspace.
    ///
    /// Only admin users can create workspaces.
    pub async fn create_workspace(&self, name: String) -> Result<Workspace, Error> {
        if !self.actor.can_admin() {
            return Err(Error::NotFound);
        }

        let owner = self.actor.id.clone().ok_or(Error::Forbidden)?;
        let workspace = InsertWorkspace { name, owner };

        DB.create(WORKSPACE)
            .content(Workspace::from_insert(workspace))
            .await?
            .ok_or(Error::AlreadyExists)
    }

    /// Get a workspace by ID.
    ///
    /// Admins can access any workspace. Owners can access their own.
    pub async fn get_workspace(&self, id: &WorkspaceId) -> Result<Workspace, Error> {
        let owner_id = self.actor.id.clone().ok_or(Error::Forbidden)?;
        let workspace: Workspace = DB
            .select((WORKSPACE, id.key()))
            .await?
            .ok_or(Error::NotFound)?;

        if workspace.owner != owner_id && !self.actor.can_admin() {
            return Err(Error::NotFound);
        }

        Ok(workspace)
    }

    // Check if a workspace exists for a given owner.
    pub async fn exists_for_owner(
        &self,
        workspace_id: &WorkspaceId,
        owner_id: &UserId,
    ) -> Result<bool, Error> {
        if !self.actor.can_admin() {
            return Err(Error::Forbidden);
        }
        let mut res = DB
            .query(
                r#"
                SELECT id
                FROM workspace
                WHERE id = $workspace
                  AND owner = $owner
                LIMIT 1
                "#,
            )
            .bind(("workspace", workspace_id.key()))
            .bind(("owner", owner_id.key()))
            .await?;

        let row: Option<serde_json::Value> = res.take(0)?;
        Ok(row.is_some())
    }

    /// Update a workspace by ID.
    ///
    /// Only admins can update workspaces.
    pub async fn update_workspace(
        &self,
        id: &WorkspaceId,
        patch: WorkspacePatch,
    ) -> Result<Workspace, Error> {
        if !self.actor.can_admin() {
            return Err(Error::Forbidden);
        }

        let owner_id = self.actor.id.clone().ok_or(Error::Forbidden)?;
        let mut res = DB
            .query(
                r#"
                UPDATE workspace
                SET $patch
                WHERE id = $workspace_id
                  AND owner = $owner_id
                RETURN AFTER;
                "#,
            )
            .bind(("workspace_id", id.key()))
            .bind(("owner_id", owner_id.key()))
            .bind(("patch", patch))
            .await?;

        let workspace: Option<Workspace> = res.take(0)?;
        let workspace: Workspace = workspace.ok_or(Error::Db)?;
        Ok(workspace)
    }

    /// Soft delete a workspace by ID.
    ///
    /// Only admins can soft delete.
    pub async fn soft_delete_workspace(&self, id: &WorkspaceId) -> Result<Workspace, Error> {
        if !self.actor.can_admin() {
            return Err(Error::Forbidden);
        }

        DB.update((WORKSPACE, id.key()))
            .content(AdminWorkspacePatch {
                is_soft_deleted: true,
            })
            .await?
            .ok_or(Error::Db)
    }

    pub async fn dangerously_delete_workspace_fr_fr_fr(
        &self,
        id: &WorkspaceId,
    ) -> Result<Workspace, Error> {
        if !self.actor.can_admin() {
            return Err(Error::Forbidden);
        }
        DB.delete((WORKSPACE, id.key())).await?.ok_or(Error::Db)
    }

    pub async fn list_all_workspaces(&self) -> Result<Vec<Workspace>, Error> {
        if !self.actor.can_admin() {
            let mut res = DB
                .query(
                    r#"
            SELECT *
            FROM workspace
            WHERE owner = $owner;
            AND is_soft_deleted = false
            "#,
                )
                .bind((
                    "owner",
                    self.actor.id.clone().ok_or(Error::Forbidden)?.key(),
                ))
                .await?;

            let workspaces: Vec<Workspace> = res.take(0)?;
            return Ok(workspaces);
        };
        Ok(DB.select(WORKSPACE).await?)
    }
}
