use crate::{core::error::ErrorCode, e_code, e_code_err};
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey, EncodingKey, Validation};
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

pub static KEY: Lazy<Key> = Lazy::new(|| Key::new(crate::core::APP_CONFIG.jwt.secret.as_bytes()));

use crate::core::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub nickname: String,
    pub exp: i64,
}

#[async_trait]
impl<T: Send + Sync> FromRequest<T> for Claims {
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<T>) -> Result<Self, Self::Rejection> {
        if !req.headers().contains_key("Authorization") {
            return e_code_err!(ErrorCode::Forbidden);
        }
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| e_code!(ErrorCode::InvalidToken))?;
        let token_data =
            decode(bearer.token(), &KEY.decoding, &Validation::default()).map_err(|err| {
                let code = match err.kind() {
                    ErrorKind::ExpiredSignature => ErrorCode::ExpiredToken,
                    _ => ErrorCode::InvalidToken,
                };
                e_code!(code)
            })?;
        Ok(token_data.claims)
    }
}

pub struct WeekClaims {
    pub claims: Option<Claims>,
}

impl WeekClaims {
    pub fn is_authed(&self) -> bool {
        self.claims.is_some()
    }
}

#[async_trait]
impl<T: Send + Sync> FromRequest<T> for WeekClaims {
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<T>) -> Result<Self, Self::Rejection> {
        match Claims::from_request(req).await {
            Ok(claims) => Ok(Self {
                claims: Some(claims),
            }),
            Err(_) => Ok(Self { claims: None }),
        }
    }
}
