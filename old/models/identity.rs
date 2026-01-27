use crate::models::ids::{IdentityId, UserId};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub id: Option<IdentityId>,
    pub user: UserId,
    pub external_user_id: String,
    pub created_at: Datetime,
}

impl Identity {
    pub fn new(user: UserId, external_user_id: String) -> Identity {
        Identity {
            id: None,
            user,
            external_user_id,
            created_at: Datetime::from(chrono::Utc::now()),
        }
    }
}
