use axum::{http::StatusCode as HTTPCode, Json};
use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Clone, Serialize_repr, PartialEq, Debug)]
#[repr(u16)]
pub enum ResponseStatus {
    OK = 2000,
    InvalidRequest = 4000,
    Forbidden = 4010,
    InvalidToken = 4011,
    ExpiredToken = 4012,
    NotFound = 4040,
    UnkownError = 5000,
    DBError = 5001,
    TokenCreation = 5002,
}

impl ResponseStatus {
    pub fn res(&self) -> (HTTPCode, String) {
        match self {
            ResponseStatus::InvalidRequest => (
                HTTPCode::BAD_REQUEST,
                "invalid request, please check request payload or headers.".to_owned(),
            ),
            ResponseStatus::Forbidden => (HTTPCode::UNAUTHORIZED, "forbidden.".to_owned()),
            ResponseStatus::InvalidToken => (HTTPCode::UNAUTHORIZED, "invalid token.".to_owned()),
            ResponseStatus::ExpiredToken => {
                (HTTPCode::UNAUTHORIZED, "token is expired.".to_owned())
            }
            ResponseStatus::NotFound => {
                (HTTPCode::NOT_FOUND, "resources does not exist.".to_owned())
            }
            ResponseStatus::UnkownError => {
                (HTTPCode::INTERNAL_SERVER_ERROR, "unknown error.".to_owned())
            }
            ResponseStatus::DBError => (HTTPCode::INTERNAL_SERVER_ERROR, "DB error.".to_owned()),
            ResponseStatus::TokenCreation => (
                HTTPCode::INTERNAL_SERVER_ERROR,
                "creates token fail.".to_owned(),
            ),
            _ => (HTTPCode::OK, "success".to_owned()),
        }
    }
}

#[derive(Serialize)]
pub struct ResponseBody<T> {
    data: T,
    code: ResponseStatus,
    msg: String,
}

impl<T> ResponseBody<T> {
    pub fn new(data: T, code: ResponseStatus, msg: String) -> Self {
        Self { data, code, msg }
    }
    pub fn ok(data: T) -> Self {
        Self::new(data, ResponseStatus::OK, "success".to_string())
    }
}

impl ResponseBody<()> {
    pub fn error(code: ResponseStatus, msg: String) -> Self {
        Self::new((), code, msg)
    }
}

pub type HandlerResult<T> = Result<Json<ResponseBody<T>>, crate::core::error::AppError>;
