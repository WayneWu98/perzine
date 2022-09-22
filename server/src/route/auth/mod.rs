use axum::routing::post;
use axum::Router;
use jsonwebtoken::{encode, Header};
use serde::Deserialize;

use crate::core::{error::ErrorCode, response::HandlerResult};

use crate::extract::{
    auth::{Claims, KEY},
    JsonPayload,
};
use crate::{e_code, res_ok};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    email: String,
    password: String,
}

use crate::core::APP_CONFIG;

pub async fn login(JsonPayload(_payload): JsonPayload<AuthPayload>) -> HandlerResult<String> {
    let claims = Claims {
        email: "wayne-wu@163.com".to_owned(),
        nickname: "wayne".to_owned(),
        exp: chrono::Local::now().timestamp() + APP_CONFIG.clone().jwt.expires,
    };

    let token = encode(&Header::default(), &claims, &KEY.encoding)
        .map_err(|_| e_code!(ErrorCode::TokenCreation))?;

    res_ok!(token)
}

pub fn get_router() -> Router {
    Router::new().route("/login", post(login))
}
