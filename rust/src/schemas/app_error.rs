use std::sync::Arc;

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use time::{OffsetDateTime, serde::rfc3339};

use crate::{extractors::http_error::HttpError, repository::database_error::DatabaseError, schemas::app_response::AppJson, services::domain_error::DomainError};

// This is the equivalent of @ExceptionHandler in Springboot and @ServerExceptionMapper in Quarkus

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Http(#[from] HttpError),
    #[error(transparent)]
    Database(#[from] DatabaseError),
    #[error(transparent)]
    Domain(#[from] DomainError)
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
            #[serde(with = "rfc3339")]
            timestamp: OffsetDateTime,
        }

        let (status, message, err) = match &self {
            AppError::Http(HttpError::Validation(e)) => 
                (
                    StatusCode::BAD_REQUEST, 
                    e.to_string(), 
                    None
                ),
            AppError::Http(HttpError::JsonRejection(r)) => 
                (
                    r.status(), 
                    r.body_text(), 
                    None
                ),
            AppError::Http(HttpError::QueryRejection(r)) => 
                (
                    r.status(), 
                    r.body_text(), 
                    None
                ),
            AppError::Database(_) =>
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong in the backend".to_owned(),
                    Some(self), // We log
                ),
            AppError::Domain(DomainError::TicketNotFound(_)) =>
                (
                    StatusCode::NOT_FOUND,
                    self.to_string(),
                    None, // No logging
                )
        };

        let error_response = ErrorResponse {
            message,
            timestamp: time::OffsetDateTime::now_utc(),
        };

        let mut response = (status, AppJson(error_response)).into_response();

        if let Some(err) = err {
            response.extensions_mut().insert(Arc::new(err));
        }

        response
    }
}

// Our middleware is responsible for logging error details internally
pub async fn log_app_errors(request: Request, next: Next) -> Response {
    let response = next.run(request).await;

    // If the response contains an AppError Extension, log it.
    if let Some(err) = response.extensions().get::<Arc<AppError>>() {
        tracing::error!(?err, "an unexpected error occurred inside a handler");
    }

    response
}
