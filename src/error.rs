// Importing the necessary modules and functions.
use axum::http::StatusCode;
use axum::response::{Response, IntoResponse};
use prisma_client_rust::QueryError;
use prisma_client_rust::prisma_errors::query_engine::{RecordNotFound, UniqueKeyViolation};


// The `EError` enum which represents different types of errors that can occur in the application.
#[derive(thiserror::Error, Debug)]
pub enum EError {

    /// Represents a `400 Bad Request` error.

    /// Represents a `401 Unauthorized` error.
    #[error("Unauthorized : {0}")]
    Unauthorized(String),
    /// Represents a `403 Forbidden` error.
    #[error("Forbidden : {0}")]
    Forbidden(String),

    /// Represents a `404 Not Found` error.
    #[error("Not found : {0}")]
    NotFound(String),

    /// Represents a `500 Internal Server Error`.
    #[error("Internal server error: {0}")]
    InternalServerError(String),

    /// Represents a Prisma error.
    #[error("Prisma error: {0}")]
    PrismaError(#[from] QueryError),

    /// Represents a `400 Bad Request` error.
    #[error("Bad request : {0}")]
    BadRequest(String),

    /// Represents a generic error.
    #[error("Internal server error: {0}")]
    Anyhow(#[from] anyhow::Error),
}


// Implementation of the `IntoResponse` trait for the `EError` enum.
impl IntoResponse for EError {
    // Function to convert an `EError` into a `Response`.
    fn into_response(self) -> Response {
        // Determine the status code based on the type of error.
        let status = match self {

            // Handle Prisma errors
            // If the error is a `UniqueKeyViolation`, return a `409 Conflict` status.
            EError::PrismaError(ref error)
            if error.is_prisma_error::<UniqueKeyViolation>() => {
                StatusCode::CONFLICT
            }
            // If the error is a `RecordNotFound`, return a `404 Not Found` status.
            EError::PrismaError(ref error)
            if error.is_prisma_error::<RecordNotFound>() => {
                StatusCode::NOT_FOUND
            }

            // For other Prisma errors, return a `400 Bad Request` status.
            EError::PrismaError(_) => StatusCode::BAD_REQUEST,
            // For `BadRequest` errors, return a `400 Bad Request` status.
            EError::BadRequest(_) => StatusCode::BAD_REQUEST,
            // For `Unauthorized` errors, return a `401 Unauthorized` status.
            EError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            // For `Forbidden` errors, return a `403 Forbidden` status.
            EError::Forbidden(_) => StatusCode::FORBIDDEN,
            // For `NotFound` errors, return a `404 Not Found` status.
            EError::NotFound(_) => StatusCode::NOT_FOUND,
            // For `InternalServerError` errors, return a `500 Internal Server Error` status.
            EError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            // For generic errors, return a `500 Internal Server Error` status.
            EError::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        // Log the error.
        tracing::error!("{:?}", self);

        // Convert the status code and error message into a `Response`.
        (status, self.to_string()).into_response()
    }
}