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

pub struct JsonPayload<T>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for JsonPayload<T>
where
    B: axum::body::HttpBody + Send,
    T: DeserializeOwned,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // let body = req.;
        let payload = axum::Json::<T>::from_request(req).await.map_err(|err| {
            println!("{:?}", err);
            e_code!(ErrorCode::InvalidRequest)
        })?;
        Ok(Self(payload.0))
    }
}
