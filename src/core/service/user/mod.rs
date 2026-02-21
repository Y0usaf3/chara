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

use crate::core::db::{error::Error, DB};
use crate::core::models::user::*;
use crate::HCAUTH;
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
}

impl UserService {
    pub async fn login(method: AuthMethod) -> Result<Self, Error> {
        let ident: Option<crate::core::models::session::Session> = match method {
            AuthMethod::HCA(token) => {
                let auth_identity = HCAUTH
                    .get_identity(token)
                    .await
                    .map_err(|_| Error::User(UserServiceError::BrokenToken))?;

                let mut res = DB
                    .query("SELECT * FROM identity WHERE external_id = $external_id")
                    .bind(("external_id", auth_identity.identity.id))
                    .await?;

                res.take(0)?
            }
            AuthMethod::Session(session) => {
                let mut res = DB
                    .query("SELECT * FROM session WHERE ip = $ip AND `token` = $tokenn AND user_agent = $user_agent AND expires_at > time::now()")
                    .bind(("ip",session.ip))
                    .bind(("tokenn", session.token))
                    .bind(("user_agent",session.agent))
                    .await?;
                res.take(0)?
            }
        };
        let ident = ident.ok_or(Error::User(UserServiceError::UserNonExistant))?;
        let user: Option<User> = DB.select(("user", ident.user.0.key)).await?;

        Ok(UserService {
            user: user.ok_or(Error::User(UserServiceError::UserNonExistant))?,
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

        Ok(UserService {
            user: user.ok_or(Error::User(UserServiceError::UserAlreadyExists))?,
        })
    }

    pub async fn update_self_user(&mut self, mut patch: UserPatch) -> Result<User, Error> {
        if patch.is_deleted == Some(true) {
            patch.is_deleted = Some(false);
        }

        self.user.apply_patch(patch);

        let record_id = self
            .user
            .id
            .as_ref()
            .map(|id| id.0.clone())
            .ok_or(Error::User(UserServiceError::UserNonExistant))?;

        let user: Option<User> = DB
            .update(&record_id)
            .patch(PatchOp::replace(
                "/first_name",
                self.user.first_name.clone(),
            ))
            .patch(PatchOp::replace("/last_name", self.user.last_name.clone()))
            .patch(PatchOp::replace("/email", self.user.email.clone()))
            .await?;

        user.ok_or(Error::User(UserServiceError::UserNonExistant))
    }

    //pub async fn delete_user(&self) {}
}
