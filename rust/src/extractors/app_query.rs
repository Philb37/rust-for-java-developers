use axum::extract::FromRequestParts;

use crate::{extractors::http_error::HttpError};

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(HttpError))]
pub struct AppQuery<T>(pub T);