use std::ops::Add;

use axum::Json;
use jsonwebtoken::{encode, Header};
use serde::{Deserialize, Serialize};

use crate::core::{
    error::AppError,
    extract::{
        auth::{Claims, KEY},
        JsonPayload,
    },
    response::{HandlerResult, ResponseBody, ResponseStatus},
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
