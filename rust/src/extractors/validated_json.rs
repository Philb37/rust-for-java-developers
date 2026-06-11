use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::schemas::{app_error::AppError, app_response::AppJson};

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T> 
where
    T: DeserializeOwned + Validate,
    S: Send + Sync
{
    type Rejection = AppError;

    async fn from_request(
            req: Request,
            state: &S,
        ) -> Result<Self, Self::Rejection> {
        
        let AppJson(value) = AppJson::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}