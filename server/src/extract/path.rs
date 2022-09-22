use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};
use serde::de::DeserializeOwned;

use crate::{
    core::error::{AppError, ErrorCode},
    e_code,
};

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
            .map_err(|_| e_code!(ErrorCode::InvalidRequest))?;
        Ok(Self(payload.0))
    }
}
