use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};
use serde::de::DeserializeOwned;

use crate::core::{error::AppError, response::ResponseStatus};

pub struct Path<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for Path<T>
where
    T: DeserializeOwned + Send,
    B: Send,
{
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let payload = axum::extract::Path::<T>::from_request(req)
            .await
            .map_err(|_| AppError::from_code(ResponseStatus::InvalidRequest, None))?;
        Ok(Self(payload.0))
    }
}
