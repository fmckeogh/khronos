//! Error handling

use {
    crate::auth::AuthLevel,
    axum::{
        extract::rejection::ExtensionRejection,
        http::StatusCode,
        response::{IntoResponse, Response},
    },
    tracing::error,
};

/// Route error, presented to user through `IntoResponse` impl
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum Error {
    /// File {0:?} not found
    FileNotFound(String),

    /// Database error
    DatabaseError(#[from] sqlx::Error),

    /// Invalid group format
    InvalidGroupFormat(String),

    /// Failed to acquire keys from request extension: {0}
    KeysExtension(ExtensionRejection),
    /// Missing cookie or token
    AuthRequired,
    /// JSON web token error: {0}
    Token(#[from] jsonwebtoken::errors::Error),
    /// Level {got:?} attempted action requiring {expected:?}
    InsufficientPrivilege { expected: AuthLevel, got: AuthLevel },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("Error occurred when handling request: {:?}", self);

        let status_code = match self {
            Error::InvalidGroupFormat(_) => StatusCode::BAD_REQUEST,
            Error::FileNotFound(_) => StatusCode::NOT_FOUND,
            Error::DatabaseError(_) | Error::KeysExtension(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::AuthRequired | Error::Token(_) | Error::InsufficientPrivilege { .. } => {
                StatusCode::UNAUTHORIZED
            }
        };

        (status_code, self.to_string()).into_response()
    }
}
