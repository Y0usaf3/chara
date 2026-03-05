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

use crate::HCAUTH;
use crate::core::db::{DB, error::Error};
use crate::core::models::identity::Identity;
use crate::core::models::ids::UserId;
use crate::core::models::session::Session;
use crate::core::models::user::*;
use surrealdb::opt::PatchOp;
use surrealdb_types::RecordId;
use thiserror::Error;

pub struct SessionI {
    pub token: String,
    pub ip: String,
    pub agent: String,
}

pub enum AuthMethod {
    HCA(String),
    Session(SessionI),
}

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("the user already exists")]
    UserAlreadyExists,
    #[error("the session token doesnt exist")]
    SessionTokenNonExistant,
    #[error("the user doesnt exist")]
    UserNonExistant,
    #[error("the user dont have enough permission")]
    NotEnoughPermission,
    #[error("broken")]
    BrokenToken,
}

#[derive(Debug)]
pub struct UserService {
    pub user: User,
    pub user_record_id: RecordId,
}

impl UserService {
    pub async fn login(method: AuthMethod) -> Result<Self, Error> {
        let user: Option<User> = match method {
            AuthMethod::HCA(token) => {
                let auth_identity = HCAUTH
                    .get_identity(token)
                    .await
                    .map_err(|_| Error::User(UserServiceError::BrokenToken))?;

                let mut res = DB
                    .query("SELECT * FROM identity WHERE external_id = $external_id")
                    .bind(("external_id", auth_identity.identity.id))
                    .await?;

                let ident: Option<Identity> = res.take(0)?;
                let ident = ident.ok_or(Error::User(UserServiceError::UserNonExistant))?;
                DB.select(ident.user.0).await?
            }
            AuthMethod::Session(session) => {
                let mut res = DB
                    .query("SELECT * FROM session WHERE ip = $ip AND `token` = $tokenn AND user_agent = $user_agent AND expires_at > time::now()")
                    .bind(("ip",session.ip))
                    .bind(("tokenn", session.token))
                    .bind(("user_agent",session.agent))
                    .await?;
                let ident: Option<Session> = res.take(0)?;
                let ident = ident.ok_or(Error::User(UserServiceError::UserNonExistant))?;
                DB.select(ident.user.0).await?
            }
        };

        let user = user.ok_or(Error::User(UserServiceError::UserNonExistant))?;

        Ok(UserService {
            user: user.clone(),
            user_record_id: user
                .id
                .as_ref()
                .ok_or(Error::User(UserServiceError::UserNonExistant))?
                .0
                .clone(),
        })
    }

    pub async fn register(token: String) -> Result<UserService, Error> {
        let auth_identity = HCAUTH
            .get_identity(token.clone())
            .await
            .map_err(|_| Error::User(UserServiceError::BrokenToken))?;
        let mut res = DB
            .query(
                "
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
                };
            ",
            )
            .bind(("ext_id", auth_identity.identity.id))
            .bind(("first_name", auth_identity.identity.first_name))
            .bind(("last_name", auth_identity.identity.last_name))
            .bind(("email", auth_identity.identity.primary_email))
            .bind(("access_token", token))
            .await?;
        let user: Option<User> = res.take(0)?;
        let user = user.ok_or(Error::User(UserServiceError::UserAlreadyExists))?;

        Ok(UserService {
            user: user.clone(),
            user_record_id: user
                .id
                .as_ref()
                .ok_or(Error::User(UserServiceError::UserNonExistant))?
                .0
                .clone(),
        })
    }

    pub async fn update_self_user(&mut self, mut patch: UserPatch) -> Result<User, Error> {
        if patch.is_deleted == Some(true) {
            // a user cant delet itself
            patch.is_deleted = Some(false);
        }

        self.user.apply_patch(patch);

        let user: Option<User> = DB
            .update(&self.user_record_id)
            .patch(PatchOp::replace(
                "/first_name",
                self.user.first_name.clone(),
            ))
            .patch(PatchOp::replace("/last_name", self.user.last_name.clone()))
            .patch(PatchOp::replace("/email", self.user.email.clone()))
            .await?;

        user.ok_or(Error::User(UserServiceError::UserNonExistant))
    }

    pub async fn delete_user(&mut self, user_id: &UserId) -> Result<User, Error> {
        if self.is_admin().await.unwrap_or(false) {
            return Err(Error::User(UserServiceError::NotEnoughPermission));
        };
        let user: Option<User> = DB
            .update(&user_id.0)
            .patch(PatchOp::replace("/is_deleted", true))
            .await?;

        user.ok_or(Error::User(UserServiceError::UserNonExistant))
    }

    pub async fn refresh_user(&mut self) -> Result<User, Error> {
        let user: Option<User> = DB.select(&self.user_record_id).await?;
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

    //pub async fn delete_user(&self) {}
}
