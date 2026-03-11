use std::sync::LazyLock;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub mod error {
    use crate::core::service::errors::*;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use axum::response::Response;
    use axum::Json;
    use chacha20poly1305::Error as EncryptionErr;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("database error: {0}")]
        Db(String),

        #[error("authentication error: {0}")]
        Auth(#[from] AuthError),

        #[error("user error: {0}")]
        User(#[from] UserError),

        #[error("permission error: {0}")]
        Permission(#[from] PermissionError),

        #[error("workspace error: {0}")]
        Workspace(#[from] WorkspaceError),

        #[error("encryption error: {0}")]
        Encryption(#[from] EncryptionError),

        #[error("database error: {0}")]
        Database(#[from] DatabaseError),
    }

    impl IntoResponse for Error {
        fn into_response(self) -> Response {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
        }
    }

    impl From<surrealdb::Error> for Error {
        fn from(error: surrealdb::Error) -> Self {
            eprintln!("{error:?}");
            Self::Db(error.to_string())
        }
    }

    impl From<EncryptionErr> for Error {
        fn from(error: EncryptionErr) -> Self {
            eprintln!("{error:?}");
            Self::Encryption(EncryptionError::EncryptionFailed)
        }
    }
}
