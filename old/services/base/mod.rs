use crate::models::base::*;
use crate::models::ids::*;
use crate::models::user::User;
use crate::models::workspace::*;
use crate::services::user::UserService;
use crate::services::workspace::WorkspaceService;
use crate::services::{Error, DB};

const BASE: &str = "base";

/// Service for base-related actions.
/// Actor is resolved on init.
/// All actions require workspace ownership.
pub struct BaseService {
    actor: User,
    workspace_service: WorkspaceService,
}

impl BaseService {
    /// Initialize the service from a session token.
    pub async fn init(session_token: &str) -> Result<Self, Error> {
        let user_service = UserService::init(session_token).await?;
        let workspace_service = WorkspaceService::init(session_token).await?;

        Ok(Self {
            actor: user_service.actor,
            workspace_service,
        })
    }

    /// Internal helper: ensure the actor owns the workspace.
    async fn assert_workspace_owner(&self, workspace_id: &WorkspaceId) -> Result<(), Error> {
        let owner_id = self.actor.id.clone().ok_or(Error::Forbidden)?;

        let exists = self
            .workspace_service
            .exists_for_owner(workspace_id, &owner_id)
            .await?;

        if !exists {
            return Err(Error::NotFound);
        }

        Ok(())
    }

    /// Create a base in a workspace.
    pub async fn create_base(
        &self,
        workspace_id: WorkspaceId,
        name: String,
    ) -> Result<Base, Error> {
        self.assert_workspace_owner(&workspace_id).await?;

        let base = InsertBase {
            workspace: workspace_id,
            name,
        };

        DB.create(BASE)
            .content(Base::from_insert(base))
            .await?
            .ok_or(Error::Db)
    }

    /// Get a base by ID.
    pub async fn get_base_by_id(&self, id: &BaseId) -> Result<Base, Error> {
        let base: Base = DB.select((BASE, id.key())).await?.ok_or(Error::NotFound)?;

        self.assert_workspace_owner(&base.workspace).await?;
        Ok(base)
    }

    /// Update a base.
    pub async fn update_base(&self, id: &BaseId, patch: BasePatch) -> Result<Base, Error> {
        let _base = self.get_base_by_id(id).await?;

        let mut res = DB
            .query(
                r#"
                UPDATE base
                SET $patch
                WHERE id = $id
                RETURN AFTER;
                "#,
            )
            .bind(("id", id.key()))
            .bind(("patch", patch))
            .await?;

        let updated: Option<Base> = res.take(0)?;
        updated.ok_or(Error::Db)
    }

    /// Delete a base.
    pub async fn delete_base(&self, id: &BaseId) -> Result<Base, Error> {
        let _base = self.get_base_by_id(id).await?;
        DB.delete((BASE, id.key())).await?.ok_or(Error::Db)
    }

    /// List all bases for a workspace.
    pub async fn list_bases_by_workspace(
        &self,
        workspace_id: &WorkspaceId,
    ) -> Result<Vec<Base>, Error> {
        self.assert_workspace_owner(workspace_id).await?;

        let mut res = DB
            .query(
                r#"
                SELECT *
                FROM base
                WHERE workspace = $workspace;
                "#,
            )
            .bind(("workspace", workspace_id.key()))
            .await?;

        let bases: Vec<Base> = res.take(0)?;
        Ok(bases)
    }
}
