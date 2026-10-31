use axum::{extract::{rejection::{JsonRejection, QueryRejection}}, response::{IntoResponse, Response}};
use validator::ValidationErrors;

use crate::schemas::app_error::AppError;

#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    // The request body contained invalid JSON
    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),
    #[error(transparent)]
    QueryRejection(#[from] QueryRejection),
    #[error(transparent)]
    Validation(#[from] ValidationErrors),
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        AppError::from(self).into_response()
    }
}