use std::sync::Arc;

use axum::{extract::{Request, rejection::{JsonRejection, QueryRejection}}, http::StatusCode, middleware::Next, response::{IntoResponse, Response}};
use sea_orm::DbErr;
use serde::{Serialize};
use time::{OffsetDateTime, serde::rfc3339};
use validator::ValidationErrors;

use crate::schemas::app_response::AppJson;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    // The request body contained invalid JSON
    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),
    #[error(transparent)]
    QueryRejection(#[from] QueryRejection),
    #[error(transparent)]
    Validation(#[from] ValidationErrors),
    #[error("Something went wrong in the backend")]
    DatabaseError(#[from] DbErr),
    #[error("Ticket {0} not found")]
    TicketNotFound(i32)
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
            #[serde(with = "rfc3339")]
            timestamp: OffsetDateTime
        }

        let (status, message, err) = match &self {
            AppError::JsonRejection(rejection) => {
                (
                    rejection.status(), 
                    rejection.body_text(), 
                    None // No logging
                )
            },
            AppError::QueryRejection(rejection) => {
                (
                    rejection.status(),
                    rejection.body_text(),
                    None // No logging
                )
            },
            AppError::Validation(validation_error) => {
                (
                    StatusCode::BAD_REQUEST,
                    validation_error.to_string(),
                    None // No logging
                )
            },
            AppError::DatabaseError(_) => {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong in the backend".to_string(),
                    Some(self) // We log
                )
            },
            AppError::TicketNotFound(id) => {
                (
                    StatusCode::NOT_FOUND,
                    format!("Ticket {} not found", id),
                    None // No logging
                )
            }
        };

        let error_response = ErrorResponse {
            message,
            timestamp: time::OffsetDateTime::now_utc()
        };

        let mut response = (status, AppJson(error_response)).into_response();

        if let Some(err) = err {
            response.extensions_mut().insert(Arc::new(err));
        }

        response
    }
}

// CONF : Useless with thiserror
// impl From<DbErr> for AppError {
//     fn from(err: DbErr) -> Self {
//         Self::DatabaseError(err)
//     }
// }

// CONF : Useless with thiserror
// impl From<JsonRejection> for AppError {
//     fn from(rejection: JsonRejection) -> Self {
//         Self::JsonRejection(rejection)
//     }
// }

// Our middleware is responsible for logging error details internally
pub async fn log_app_errors(request: Request, next: Next) -> Response {

    let response = next.run(request).await;

    // If the response contains an AppError Extension, log it.
    if let Some(err) = response.extensions().get::<Arc<AppError>>() {
        tracing::error!(?err, "an unexpected error occurred inside a handler");
    }

    response
}