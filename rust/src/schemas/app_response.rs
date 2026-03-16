use axum::{extract::FromRequest, http::StatusCode, response::{IntoResponse, Response}};

use crate::schemas::app_error::AppError;

// Create our own JSON extractor by wrapping `axum::Json`. This makes it easy to override the
// rejection and provide our own which formats errors to match our application.
//
// `axum::Json` responds with plain text if the input is invalid.
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(
    pub T
);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

pub struct Created<T>(
    pub T
);

impl<T> IntoResponse for Created<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        (StatusCode::CREATED, AppJson(self.0)).into_response()
    }
}