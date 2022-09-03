use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub struct Key {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Key {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub static KEY: Lazy<Key> = Lazy::new(|| Key::new(crate::core::APP_CONFIG.jwt_secret.as_bytes()));

use crate::core::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub nickname: String,
    pub exp: usize,
}

use super::response::ResponseStatus;
#[async_trait]
impl<T> FromRequest<T> for Claims
where
    T: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<T>) -> Result<Self, Self::Rejection> {
        if !req.headers().contains_key("Authorization") {
            return Err(AppError::from_code(ResponseStatus::Forbidden, None));
        }
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AppError::from_code(ResponseStatus::InvalidToken, None))?;
        let token_data = decode(bearer.token(), &KEY.decoding, &Validation::default())
            .map_err(|_| AppError::from_code(ResponseStatus::InvalidToken, None))?;
        Ok(token_data.claims)
    }
}
