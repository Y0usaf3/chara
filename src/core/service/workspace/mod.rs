use super::errors::*;
use crate::core::models::{
    ids::{BaseId, UserId, WorkspaceId, WorkspaceUserId},
    workspace::{Workspace, WorkspacePatch},
    workspace_user::WorkspaceUser,
};

#[derive(Debug)]
pub struct WorkspaceService {
    pub workspace_user: WorkspaceUser,
    workspace_user_record_id: WorkspaceUserId,
    pub workspace: Workspace,
    workspace_record_id: WorkspaceId,
}

impl WorkspaceService {
    pub async fn new(user: &UserId, workspace: WorkspaceId) -> Result<Self, Error> {}
    pub async fn create_base(&self, name: String) -> Result<Self, Error> {}
    pub async fn delete_base(&self, base: BaseId) -> Result<Self, Error> {}
    pub async fn open_base(&self, base: BaseId) -> Result<Self, Error> {}
    pub async fn edit_workspace_user(
        &self,
        workspace_user: WorkspaceUserId,
    ) -> Result<Self, Error> {
    }
    pub async fn delete_workspace_user(
        &self,
        workspace_user: WorkspaceUserId,
    ) -> Result<Self, Error> {
    }
    pub async fn edit_workspace_name(&self, patch: WorkspacePatcht) -> Result<Self, Error> {}
}
