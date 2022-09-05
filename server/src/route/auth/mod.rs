use axum::routing::post;
use axum::Json;
use axum::{routing::get, Router};
use jsonwebtoken::{encode, Header};
use serde::{Deserialize, Serialize};

use crate::core::{
    error::AppError,
    response::{HandlerResult, ResponseBody, ResponseStatus},
};

use crate::extract::{
    auth::{Claims, KEY},
    JsonPayload,
};

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    email: String,
    password: String,
}

use crate::core::APP_CONFIG;

pub async fn login(JsonPayload(payload): JsonPayload<AuthPayload>) -> HandlerResult<AuthBody> {
    println!("login payload: {:#?}", payload);

    let claims = Claims {
        email: "wayne-wu@163.com".to_owned(),
        nickname: "wayne".to_owned(),
        exp: chrono::Local::now().timestamp() + APP_CONFIG.clone().jwt.expires,
    };

    let token = encode(&Header::default(), &claims, &KEY.encoding)
        .map_err(|_| AppError::from_code(ResponseStatus::TokenCreation, None))?;

    Ok(Json(ResponseBody::ok(AuthBody::new(token))))
}

pub async fn protected(claims: Claims) -> HandlerResult<String> {
    Ok(Json(ResponseBody::ok(format!("{:#?}", claims).to_string())))
}

pub fn get_router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/protected", get(protected))
}
