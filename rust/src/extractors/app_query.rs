use axum::extract::FromRequestParts;

use crate::schemas::app_error::AppError;

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(AppError))]
pub struct AppQuery<T>(pub T);