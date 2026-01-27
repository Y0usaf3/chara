use crate::core::models::ids::*;
use ::serde::{Deserialize, Serialize};
use email_address::EmailAddress;
use surrealdb::sql::*;

// The User struct represents a global platform user.
// Users are created with the User role by default.
// Administrative privileges are granted explicitly by the system. (soon™)
// Only Administators can apply patches to Users.

#[derive(Debug, thiserror::Error)]
pub enum NameError {
    #[error("Name is too short: {0}")]
    TooShort(usize),
    #[error("Name contains invalid characters")]
    InvalidCharacters,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Name(String);
// a name must be :
// - ascii only
// - alphabetic only
// - can only have more than 3 chars

impl Name {
    pub fn new(value: String) -> Result<Self, NameError> {
        let len = value.chars().count();
        if len < 3 {
            return Err(NameError::TooShort(len));
        }

        if !value.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(NameError::InvalidCharacters);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct User {
    pub id: Option<UserId>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub is_deleted: bool,
    pub first_name: Name,
    pub last_name: Name,
    pub email: EmailAddress,
    pub role: UserRole,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InsertUser {
    pub(crate) first_name: Name,
    pub(crate) last_name: Name,
    pub(crate) email: EmailAddress,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UserPatch {
    pub(crate) is_deleted: Option<bool>,
    pub(crate) first_name: Option<Name>,
    pub(crate) last_name: Option<Name>,
}

impl User {
    pub fn from_insert(insert: InsertUser) -> Self {
        User {
            id: None,
            created_at: Datetime::from(chrono::Utc::now()),
            updated_at: Datetime::from(chrono::Utc::now()),
            is_deleted: false,
            first_name: insert.first_name,
            last_name: insert.last_name,
            email: insert.email,
            role: UserRole::User,
        }
    }

    pub fn apply_patch(&mut self, patch: UserPatch) {
        if let Some(v) = patch.is_deleted {
            self.is_deleted = v;
        };
        if let Some(v) = patch.first_name {
            self.first_name = v;
        };
        if let Some(v) = patch.last_name {
            self.last_name = v;
        };
        self.updated_at = Datetime::from(chrono::Utc::now());
    }
}
