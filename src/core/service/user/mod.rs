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
use crate::core::models::base::{Base, InsertBase};
use crate::core::models::ids::{BaseId, UserId};
use crate::core::models::user::*;
use crate::core::{
    db::{error::Error, DB},
    service::crypter::encrypt_token,
};
use crate::HCAUTH;
use surrealdb::opt::PatchOp;
use surrealdb::types::RecordId;

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
    user_record_id: UserId,
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
            .ok_or(Error::Auth(AuthError::VerificationFailed))?
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
            .map_err(|_| Error::Auth(AuthError::VerificationFailed))?;
        let encrypted_token = encrypted_token
            .await
            .map_err(|_| Error::Auth(AuthError::InvalidToken))?;

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
        let user = user.ok_or(Error::User(UserError::NotFound))?;
        let record_id = UserId(
            user.id
                .as_ref()
                .ok_or(Error::User(UserError::NotFound))?
                .0
                .clone(),
        );

        Ok(UserService {
            user,
            user_record_id: record_id,
        })
    }

    pub async fn update_self_user(&mut self, patch: UserPatch) -> Result<User, Error> {
        self.user.apply_patch(patch.clone());

        let user: Option<User> = DB
            .update(&self.user_record_id.0)
            .patch(PatchOp::replace(
                "/first_name",
                self.user.first_name.clone(),
            ))
            .patch(PatchOp::replace("/last_name", self.user.last_name.clone()))
            .await?;

        user.ok_or(Error::User(UserError::UpdateFailed(format!("{patch:?}"))))
    }

    pub async fn delete_user(&mut self, user_id: &UserId) -> Result<User, Error> {
        if self.user_record_id == *user_id {
            return Err(Error::User(UserError::CannotDeleteSelf));
        } else if !self.is_admin().await.unwrap_or(false) {
            return Err(Error::Permission(PermissionError::AdminRequired));
        };

        let user: Option<User> = DB
            .query(
                "
LET $is_admin = array::first(
    SELECT role
    FROM user
    WHERE id = $user_id
    AND role = 'admin'
    AND is_deleted = false
);
IF $is_admin.role != 'admin' THEN
    THROW 'Not admin'
END;
IF $self_id == $user_id THEN THROW 'Cannot delete self' END;
UPDATE $user_id SET is_deleted = true;
COMMIT TRANSACTION;",
            )
            .bind(("self_id", self.user_record_id.clone()))
            .bind(("user_id", user_id.0.clone()))
            .await?
            .take(5)?;

        user.ok_or(Error::User(UserError::NotFound))
    }

    pub async fn refresh_user(&mut self) -> Result<User, Error> {
        let user: Option<User> = DB
            .query("SELECT * FROM user WHERE id = $id AND is_deleted = false")
            .bind(("id", self.user_record_id.clone()))
            .await?
            .take(0)?;
        let user = user.ok_or(Error::User(UserError::Deleted))?;
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

    pub async fn create_base(&self, name: String) -> Result<Base, Error> {
        let base = InsertBase {
            name: Name(name),
            owner: self.user_record_id.clone(),
        };
        let res: Option<Base> = DB.create("base").content(Base::from_insert(base)).await?;
        res.ok_or(Error::Database(DatabaseError::QueryFailed(format!(
            "{}: {}",
            file!(),
            line!()
        ))))
    }

    pub async fn delete_base(&self, base: BaseId) -> Result<(), Error> {
        let mut res = DB
            .query(
                "
            BEGIN TRANSACTION;
            
            LET $authorized = (
                SELECT id FROM base WHERE id = $base AND owner = $user
            ) OR (
                SELECT id FROM user WHERE id = $user AND role = 'admin'
            );

            IF count($authorized) == 0 {
                THROW 'Unauthorized: Only the owner or an admin can delete this base.';
            };

            DELETE $base;
            
            COMMIT TRANSACTION;
            ",
            )
            .bind(("user", self.user_record_id.clone()))
            .bind(("base", base))
            .await?;
        res.check()?;
        Ok(())
    }

    //pub async fn update_base(&self, base: BaseId) -> Result<Base, Error> {}
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

// ok so i think there is a new kind of security vulnerability here, what if a session is
// "compromised", or an attacker found a way to steal the token, we should make a session as
// occupied so if anything happens, we block any connection bc it already has a connection, wich is
// i think useful ? but imagine u wanna open the app in a new tab, u cant do that then ...
// hmmmmmmmmm
// yeah ill just write it and enable it if we want to (by decommenting)
// guess what im way too lazy if we ever need it then ill write it, now ima impl the workspace user
// service to interact exclusivly inside a workspace
//
// now the user is the one that makes the bases and access the tables, things will be much much
// easier , now ill have to write a Base Service, it has an owner, and an isolated automatisation
// runner, i gotta work on this asap
