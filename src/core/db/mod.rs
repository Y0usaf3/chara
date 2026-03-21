use std::sync::LazyLock;

use dotenv_codegen::dotenv;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn init() {
    DB.connect::<Ws>(dotenv!("DB_URL")).await.unwrap();

    DB.signin(Root {
        username: dotenv!("DB_USERNAME").to_string(),
        password: dotenv!("DB_PASSWORD").to_string(),
    })
    .await
    .unwrap();

    DB.use_ns(dotenv!("DB_NAMESPACE"))
        .use_db("main")
        .await
        .unwrap();
}

pub mod error {
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    };
    use chacha20poly1305::Error as EncryptionErr;
    use serde_json::json;
    use thiserror::Error;

    use crate::core::service::errors::{
        AuthError, BaseError, DatabaseError, EncryptionError, PermissionError, TableError, UserError,
    };

    #[derive(Error, Debug)]
    pub enum Irror {
        #[error("database error: {0}")]
        Db(String),

        #[error("authentication error: {0}")]
        Auth(#[from] AuthError),

        #[error("user error: {0}")]
        User(#[from] UserError),

        #[error("permission error: {0}")]
        Permission(#[from] PermissionError),

        #[error("encryption error: {0}")]
        Encryption(#[from] EncryptionError),

        #[error("database error: {0}")]
        Database(#[from] DatabaseError),

        #[error("base error: {0}")]
        Base(#[from] BaseError),

        #[error("table error: {0}")]
        Table(#[from] TableError),
    }

    impl IntoResponse for Irror {
        fn into_response(self) -> Response {
            let (status, message) = match self {
                Self::Auth(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
                Self::Permission(_) => (StatusCode::FORBIDDEN, self.to_string()),
                Self::User(UserError::NotFound)
                | Self::Base(BaseError::NotFound)
                | Self::Table(TableError::NotFound) => (StatusCode::NOT_FOUND, self.to_string()),
                Self::User(UserError::CannotActionSelf) => {
                    (StatusCode::BAD_REQUEST, self.to_string())
                }
                _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            };

            let body = Json(json!({
                "error": message,
                "status": status.as_u16()
            }));

            (status, body).into_response()
        }
    }

    impl From<surrealdb::Error> for Irror {
        fn from(error: surrealdb::Error) -> Self {
            eprintln!("{error:?}");
            Self::Db(error.to_string())
        }
    }

    impl From<EncryptionErr> for Irror {
        fn from(error: EncryptionErr) -> Self {
            eprintln!("{error:?}");
            Self::Encryption(EncryptionError::EncryptionFailed)
        }
    }
}
