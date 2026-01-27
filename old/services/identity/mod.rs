use crate::models::identity::*;
use crate::models::ids::*;
use crate::models::user::User;
use crate::services::user::UserService;
use crate::services::{Error, DB};

const IDENTITY: &str = "identity"; // imagine identity crisis

/// Service for user identity actions.
/// Only the identity owner or admin can perform operations.
pub struct IdentityService {
    actor: User,
}

impl IdentityService {
    /// Initialize the service from a session token.
    pub async fn init(session_token: &str) -> Result<Self, Error> {
        let user_service = UserService::init(session_token).await?;
        Ok(Self {
            actor: user_service.actor,
        })
    }

    /// Ensure the actor is allowed to access this identity
    fn assert_access(&self, user_id: &UserId) -> Result<(), Error> {
        if &self.actor.id.clone().ok_or(Error::Forbidden)? != user_id && !self.actor.can_admin() {
            return Err(Error::Forbidden);
        }
        Ok(())
    }

    /// Create a new identity for a user.
    pub async fn create_identity(
        &self,
        user_id: UserId,
        external_user_id: String,
    ) -> Result<Identity, Error> {
        self.assert_access(&user_id)?;

        let identity = Identity::new(user_id, external_user_id);
        DB.create(IDENTITY)
            .content(identity)
            .await?
            .ok_or(Error::Db)
    }

    /// Get an identity by ID.
    pub async fn get_identity_by_id(&self, id: &IdentityId) -> Result<Identity, Error> {
        let identity: Identity = DB
            .select((IDENTITY, id.key()))
            .await?
            .ok_or(Error::NotFound)?;

        self.assert_access(&identity.user)?;
        Ok(identity)
    }

    /// Get an identity by external ID.
    pub async fn get_identity_by_external_id(
        &self,
        external_id: String,
    ) -> Result<Identity, Error> {
        let mut res = DB
            .query(
                r#"
                SELECT *
                FROM identity
                WHERE external_user_id = $external
                LIMIT 1
                "#,
            )
            .bind(("external", external_id))
            .await?;

        let identity: Option<Identity> = res.take(0)?;
        let identity = identity.ok_or(Error::NotFound)?;

        self.assert_access(&identity.user)?;
        Ok(identity)
    }

    /// Delete an identity by ID.
    pub async fn delete_identity(&self, id: &IdentityId) -> Result<Identity, Error> {
        let _identity: Identity = self.get_identity_by_id(id).await?;
        DB.delete((IDENTITY, id.key())).await?.ok_or(Error::Db)
    }

    /// List identities for a given user.
    pub async fn list_identities_by_user(&self, user_id: &UserId) -> Result<Vec<Identity>, Error> {
        self.assert_access(user_id)?;

        let mut res = DB
            .query(
                r#"
                SELECT *
                FROM identity
                WHERE user = $user
                "#,
            )
            .bind(("user", user_id.key()))
            .await?;

        let identities: Vec<Identity> = res.take(0)?;
        Ok(identities)
    }
}
