// oookkk thats the most important part now, so for the user i have to know wich functions, bc it
// will be used the most
// so the UserService will have
// login to login the user with an existing acc
// register to create a new user where the email and stuff like that should be unique
// those dont need the self, but for operation that requires self might be to delet a user or smt
// depending on its role so we can update the data, like username etc
// or even deleting the user wich requires admin role
// and in fact, i think workspace user service will be used the most for inside workspace stuff

// oooooooooh shit why did they changed so much stuff iiinn surreeaalllldddbbbb 333...0000
// sooooooooooooooooooooooooooooooon:sob:

// make a function to get record id insteado f having to rerun this shit a million time
use super::errors::*;
use crate::core::models::ids::UserId;
use crate::core::models::user::*;
use crate::core::models::workspace::Workspace;
use crate::core::models::workspace_user::permissions::WorkspacePermission;
use crate::core::models::workspace_user::permissions::WorkspacePermissions;
use crate::core::{
    db::{error::Error, DB},
    service::crypter::encrypt_token,
};
use crate::HCAUTH;
use surrealdb::opt::PatchOp;
use surrealdb_types::RecordId;

pub struct SessionI {
    pub token: String,
    pub ip: String,
    pub agent: String,
}

pub enum AuthMethod {
    HCA(String),
    Session(SessionI),
}

#[derive(Debug)]
pub struct UserService {
    pub user: User,
    pub user_record_id: UserId,
}

impl UserService {
    pub fn id(&self) -> &UserId {
        &self.user_record_id
    }

    pub async fn login(method: AuthMethod) -> Result<Self, Error> {
        let user: User = match method {
            AuthMethod::HCA(token) => {
                let auth_identity = HCAUTH
                    .get_identity(token)
                    .await
                    .map_err(|_| Error::Auth(AuthError::InvalidToken))?;

                let mut res = DB
                    .query("SELECT VALUE user.* FROM identity WHERE external_id = $external_id AND is_deleted = false")
                    .bind(("external_id", auth_identity.identity.id))
                    .await?;

                let ident: Option<User> = res.take(0)?;
                ident.ok_or(Error::Auth(AuthError::VerificationFailed))?
            }
            AuthMethod::Session(session) => {
                let mut res = DB
                    .query(
                        "SELECT VALUE user.* FROM session 
                        WHERE ip = $ip 
                        AND crypto::argon2::compare(`token`, $tokenn)  
                        AND user_agent = $user_agent 
                        AND expires_at > time::now()
                        AND user.is_deleted = false",
                    )
                    .bind(("ip", session.ip))
                    .bind(("tokenn", session.token))
                    .bind(("user_agent", session.agent))
                    .await?;
                let ident: Option<User> = res.take(0)?;
                ident.ok_or(Error::Auth(AuthError::SessionNotFound))?
            }
        };
        let record_id = user
            .id
            .as_ref()
            .ok_or(Error::User(UserServiceError::UserNonExistant))?
            .0
            .clone();

        Ok(UserService {
            user,
            user_record_id: UserId(record_id),
        })
    }

    pub async fn register(token: String) -> Result<UserService, Error> {
        let auth_identity = HCAUTH.get_identity(token.clone());
        let encrypted_token = encrypt_token(&token);

        let auth_identity = auth_identity
            .await
            .map_err(|_| Error::User(UserServiceError::BrokenToken))?;
        let encrypted_token = encrypted_token
            .await
            .map_err(|_| Error::User(UserServiceError::BrokenToken))?;

        let mut res = DB
            .query(
                "
BEGIN TRANSACTION;
LET $existing = (SELECT id FROM identity WHERE external_id = $ext_id LIMIT 1);
IF $existing[0].id = NONE THEN {
    LET $u = (CREATE user CONTENT {
        first_name: $first_name,
        last_name: $last_name,
        email: $email
    });
    CREATE identity CONTENT {
        user: $u[0].id,
        external_id: $ext_id,
        access_token: $access_token
    };
    RETURN $u[0]; 
} ELSE {
    RETURN NONE;
};
COMMIT TRANSACTION;
            ",
            )
            .bind(("ext_id", auth_identity.identity.id))
            .bind(("first_name", auth_identity.identity.first_name))
            .bind(("last_name", auth_identity.identity.last_name))
            .bind(("email", auth_identity.identity.primary_email))
            .bind(("access_token", encrypted_token))
            .await?;
        let user: Option<User> = res.take(0)?;
        let user = user.ok_or(Error::User(UserServiceError::UserAlreadyExists))?;
        let record_id = UserId(
            user.id
                .as_ref()
                .ok_or(Error::User(UserServiceError::UserNonExistant))?
                .0
                .clone(),
        );

        Ok(UserService {
            user,
            user_record_id: record_id,
        })
    }

    pub async fn update_self_user(&mut self, patch: UserPatch) -> Result<User, Error> {
        self.user.apply_patch(patch);

        let user: Option<User> = DB
            .update(&self.user_record_id.0)
            .patch(PatchOp::replace(
                "/first_name",
                self.user.first_name.clone(),
            ))
            .patch(PatchOp::replace("/last_name", self.user.last_name.clone()))
            .await?;

        user.ok_or(Error::User(UserServiceError::UserNonExistant))
    }

    pub async fn delete_user(&mut self, user_id: &UserId) -> Result<User, Error> {
        if !self.is_admin().await.unwrap_or(false) {
            return Err(Error::User(UserServiceError::NotEnoughPermission));
        };
        if self.user_record_id == *user_id {
            return Err(Error::User(UserServiceError::NotEnoughPermission));
        };
        let user: Option<User> = DB
            .update(&user_id.0)
            .patch(PatchOp::replace("/is_deleted", true))
            .await?;

        user.ok_or(Error::User(UserServiceError::UserNonExistant))
    }

    pub async fn refresh_user(&mut self) -> Result<User, Error> {
        let user: Option<User> = DB
            .query("SELECT * FROM user WHERE id = $id AND is_deleted = false")
            .bind(("id", self.user_record_id.clone()))
            .await?
            .take(0)?;
        let user = user.ok_or(Error::User(UserServiceError::UserNonExistant))?;
        self.user = user.clone();
        Ok(user)
    }

    pub async fn is_admin(&self) -> Result<bool, Error> {
        let mut res = DB
            .query("SELECT (role = 'admin') AS value FROM user WHERE id = $user;")
            .bind(("user", self.user_record_id.clone()))
            .await?;
        let value: Option<IsAdmin> = res.take(0)?;
        Ok(value.unwrap_or_default().value())
    }

    // make it return a user service instead
    pub async fn create_workspace(&self, name: String) -> Result<Workspace, Error> {
        let mut admin_perm: WorkspacePermissions = WorkspacePermissions::none();
        admin_perm.set(WorkspacePermission::Admin);

        let mut res = DB
            .query(
                "
                BEGIN TRANSACTION;
                -- create the workspace
                LET $ws = (CREATE workspace CONTENT {
                    name: $name,
                    owner: $user_id,
                });

                -- create the workspace_user entry for the owner
                LET $ws_user = (CREATE workspace_user CONTENT {
                    workspace_id: $ws[0].id,
                    user: $user_id,
                    username: $first_name -- defaulting to user's first name
                });

                -- create the permission relation (the graph edge)
                RELATE $ws_user->can_access_workspace->$ws 
                SET perms = $admin_perm;

                COMMIT TRANSACTION;

                SELECT * FROM $ws[0].id;
            ",
            )
            .bind(("name", name))
            .bind(("user_id", self.user_record_id.clone()))
            .bind(("first_name", self.user.first_name.clone()))
            .bind((
                "admin_perm",
                <WorkspacePermissions as Into<i32>>::into(admin_perm),
            ))
            .await?;

        let workspace: Option<Workspace> = res.take(5)?;
        workspace.ok_or(Error::User(UserServiceError::BrokenToken))
    }

    pub async fn delete_workspace(&self, workspace_id: RecordId) -> Result<(), Error> {
        let mut res = DB
            .query(
                "
            BEGIN TRANSACTION;

            -- soft delete the workspace 
            LET $ws = (
                UPDATE workspace 
                SET is_deleted = true, updated_at = time::now()
                WHERE id = $workspace_id AND (owner = $user_id OR $is_admin = true)
                RETURN id
            );

            COMMIT TRANSACTION;

            SELECT VALUE id FROM $ws[0];
            ",
            )
            .bind(("workspace_id", workspace_id))
            .bind(("user_id", self.user_record_id.clone()))
            .bind(("is_admin", self.is_admin().await.unwrap_or(false)))
            .await?;

        let deleted_id: Option<RecordId> = res.take(3)?;

        if deleted_id.is_none() {
            return Err(Error::User(UserServiceError::NotEnoughPermission));
        }

        Ok(())
    }
}

// ok, i gotta learn how argon2 works again, dam i forgot how it works its been like, 6months or
// smt ? huh
// ok it makes sense hehe
// ok so uh, it was ez to impl argon2 hehe, not that hard, now ima add encryption for tokens for
// identity, especially  access and refresh tokens
// ima use AES , so chacha20poly1305 ig (wth is that name)
//
// alr alr, so i have to impl AES correctly, store the nonce in the correct way etc, ill do it when
// i have time
