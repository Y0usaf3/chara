use axum::http::StatusCode;
use axum::response::ErrorResponse;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde::Deserialize;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug, Deserialize)]
pub enum Error {
    #[error("resource already exists")]
    AlreadyExists,

    #[error("resource not found")]
    NotFound,

    #[error("forbidden")]
    Forbidden,

    #[error("meanie request")]
    BadRequest,

    #[error("database error")]
    Db,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            Error::AlreadyExists => (StatusCode::CONFLICT, "already_exists"),
            Error::NotFound => (StatusCode::NOT_FOUND, "not_found"),
            Error::Forbidden => (StatusCode::FORBIDDEN, "forbidden"),
            Error::Db => (StatusCode::INTERNAL_SERVER_ERROR, "database_error"),
            Error::BadRequest => (StatusCode::BAD_REQUEST, "bad request"),
        };

        (status, Json(ErrorResponse::from(error))).0.into_response()
    }
}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        let msg = error.to_string();

        eprintln!("DB error: {msg}");

        if msg.contains("permission") || msg.contains("forbidden") {
            Error::Forbidden
        } else if msg.contains("unique") || msg.contains("already exists") {
            Error::AlreadyExists
        } else if msg.contains("not found") {
            Error::NotFound
        } else {
            Error::Db
        }
    }
}
