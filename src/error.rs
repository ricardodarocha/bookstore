use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use serde_with::DisplayFromStr;

/// An API-friendly error type.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// A SQLx call returned an error.
    ///
    /// The exact error contents are not reported to the user in order to avoid leaking
    /// information about databse internals.
    #[error("an internal database error occurred")]
    Sqlx(#[from] sqlx::Error),

    /// Similarly, we don't want to report random `eyre` errors to the user.
    #[error("an internal server error occurred")]
    Eyre(#[from] eyre::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        #[serde_with::serde_as]
        #[serde_with::skip_serializing_none]
        #[derive(serde::Serialize)]
        struct ErrorResponse<'a> {
            // Serialize the `Display` output as the error message
            #[serde_as(as = "DisplayFromStr")]
            error: &'a Error,
        }

        // Normally you wouldn't just print this, but it's useful for debugging without
        // using a logging framework.
        println!("API error: {self:?}");

        (
            self.status_code(),
            Json(ErrorResponse {
                error: &self,
            }),
        )
            .into_response()
    }
}

impl Error {
    fn status_code(&self) -> StatusCode {
        use Error::*;

        match self {
            Sqlx(_) | Eyre(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}