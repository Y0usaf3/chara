use std::sync::LazyLock;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub mod error {
    use crate::core::service::user::UserServiceError;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use axum::response::Response;
    use axum::Json;
    use chacha20poly1305::Error as EncryptionErr;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("database error")]
        Db,

        #[error("user service error: {0}")]
        User(#[from] UserServiceError),
    }

    impl IntoResponse for Error {
        fn into_response(self) -> Response {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
        }
    }

    impl From<surrealdb::Error> for Error {
        fn from(error: surrealdb::Error) -> Self {
            eprintln!("{error:?}");
            Self::Db
        }
    }

    impl From<EncryptionErr> for Error {
        fn from(error: EncryptionErr) -> Self {
            eprintln!("{error:?}");
            Self::User(UserServiceError::EncryptionError)
        }
    }
}
