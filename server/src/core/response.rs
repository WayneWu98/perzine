use axum::{http::StatusCode as HTTPCode, Json};
use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Clone, Serialize_repr, PartialEq, Debug)]
#[repr(u16)]
pub enum ResponseStatus {
    OK = 2000,
    Forbidden = 4001,
    InvalidToken = 40002,
    ExpiredToken = 40003,
    TokenCreation = 40004,
    NotFound = 4004,
    DBError = 5001,
    UnkownError = 5004,
}

impl ResponseStatus {
    pub fn res(&self) -> (HTTPCode, String) {
        match self {
            ResponseStatus::Forbidden => (HTTPCode::UNAUTHORIZED, "forbidden.".to_string()),
            _ => (HTTPCode::OK, "success".to_string()),
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
