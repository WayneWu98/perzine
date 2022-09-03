use axum::Json;
use jsonwebtoken::{encode, Header};
use serde::{Deserialize, Serialize};

use crate::core::{
    auth::{Claims, KEY},
    error::AppError,
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

pub async fn login(Json(payload): Json<AuthPayload>) -> HandlerResult<AuthBody> {
    println!("login payload: {:#?}", payload);

    let claims = Claims {
        email: "wayne-wu@163.com".to_owned(),
        nickname: "wayne".to_owned(),
        exp: 60 * 60 * 12,
    };

    let token = encode(&Header::default(), &claims, &KEY.encoding)
        .map_err(|_| AppError::from_code(ResponseStatus::TokenCreation, None))?;

    Ok(Json(ResponseBody::ok(AuthBody::new(token))))
}

pub async fn protected(claims: Claims) -> HandlerResult<String> {
    Ok(Json(ResponseBody::ok(format!("{:#?}", claims).to_string())))
}
