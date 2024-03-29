use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    BoxError,
};
use serde::de::DeserializeOwned;

use crate::{
    core::error::{AppError, ErrorCode},
    e_code,
};

pub struct Query<T>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for Query<T>
where
    B: axum::body::HttpBody + Send,
    T: DeserializeOwned,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let payload = axum::extract::Query::<T>::from_request(req)
            .await
            .map_err(|_| e_code!(ErrorCode::InvalidRequest))?;
        Ok(Self(payload.0))
    }
}
