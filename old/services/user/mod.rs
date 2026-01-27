use serde::Deserialize;

use crate::models::ids::Key;
use crate::models::ids::UserId;
use crate::models::session::Session;
use crate::models::user::*;
use crate::services::session::get_session_with_token;
use crate::services::Error;
use crate::services::DB;

const USER: &str = "user";

// alr alr, so there is a lil issue, there is only an admin and an user, one that has some obvoious
// permissions like reading its own data and anoter that can do pretty much everything, but what if
// i added permissions, yea, this could make everything just way better and easier,
// EDIT: its totally not easier wtf did i just write 2 weeks ago
//
// MRAOW I HATE MY LIFE UWU

pub struct UserService {
    pub(crate) actor: User, // u signed the contract, now u can only [act] :3 /silly
}

impl UserService {
    /// Initialize a `UserService` for the user associated with the given session token.
    ///
    /// # Arguments
    /// - `session_token`: The session token of the authenticated user.
    ///
    /// # Returns
    /// - `Ok(Self)` if the session is valid and the user exists.
    /// - `Err(Error::Forbidden)` if the session is expired.
    /// - `Err(Error::NotFound)` if the user does not exist or is deleted.
    pub async fn init(session_token: &str) -> Result<Self, Error> {
        let session: Session = get_session_with_token(session_token.to_string()).await?;
        if session.is_expired() {
            return Err(Error::Forbidden);
        }

        let user: User = DB
            .select((USER, session.user.key()))
            .await?
            .ok_or(Error::NotFound)?;

        if user.is_deleted {
            return Err(Error::NotFound);
        }

        Ok(Self { actor: user })
    }

    /// Update a target user with the given patch.
    ///
    /// The permissions depend on the actor:
    /// - Admin can update any user fields (uses `AdminUserPatch`).
    /// - Regular users can only update their own allowed fields (uses `SelfUserPatch`).
    ///
    /// # Arguments
    /// - `target`: The `UserId` of the user to update.
    /// - `patch`: The patch data with fields to update.
    ///
    /// # Returns
    /// - `Ok(User)` with updated user data.
    /// - `Err(Error::Forbidden)` if the actor cannot update this user.
    /// - `Err(Error::NotFound)` if the user does not exist.
    pub async fn update_user(&self, target: UserId, patch: UserPatch) -> Result<User, Error> {
        if !self.actor.can_edit_user(&target) {
            return Err(Error::Forbidden);
        }

        if self.actor.can_admin() {
            DB.update((USER, target.key()))
                .content(AdminUserPatch::from(patch))
                .await?
                .ok_or(Error::NotFound)?
        } else {
            DB.update((USER, target.key()))
                .content(SelfUserPatch::from(patch))
                .await?
                .ok_or(Error::NotFound)?
        }
    }

    /// Delete a target user (admin only).
    ///
    /// # Arguments
    /// - `target`: The `UserId` of the user to delete.
    ///
    /// # Returns
    /// - `Ok(User)` with the deleted user data.
    /// - `Err(Error::Forbidden)` if the actor cannot delete this user.
    pub async fn delete_user(&self, target: UserId) -> Result<User, Error> {
        if !self.actor.can_delete_user(&target) {
            return Err(Error::Forbidden);
        }
        DB.update((USER, target.key()))
            .content(AdminUserPatch {
                is_deleted: Some(true),
                first_name: None,
                last_name: None,
                username: None,
            })
            .await?
            .ok_or(Error::NotFound)?
    }

    /// List all users (admin only).
    ///
    /// # Returns
    /// - `Ok(Vec<PublicUser>)` containing public info of all users.
    /// - `Err(Error::Forbidden)` if the actor cannot list users.
    pub async fn list_users(&self) -> Result<Vec<PublicUser>, Error> {
        if !self.actor.can_list_users() {
            return Err(Error::Forbidden);
        }
        Ok(DB.select(USER).await?)
    }
}

/// Create a new user in the database.
///
/// # Arguments
/// - `new_user`: The `InsertUser` struct containing the user data to insert.
///
/// # Returns
/// - `Ok(Some(User))` with the created user.
/// - `Err(Error::NotFound)` if the creation fails.
pub async fn create_user(new_user: InsertUser) -> Result<Option<User>, Error> {
    DB.create(USER)
        .content(User::from_insert(new_user))
        .await?
        .ok_or(Error::NotFound)
}

/// Public view of a user.
///
/// Only exposes basic info.
/// Admins may see more internally.
#[derive(Deserialize)]
pub struct PublicUser {
    /// The user's unique ID.
    pub id: UserId,

    /// The user's username.
    pub username: Username,

    /// The user's role (Admin or User).
    pub role: UserRole,

    /// Whether the user is deleted.
    pub is_deleted: bool,
}
