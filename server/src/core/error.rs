use axum::{http::StatusCode, Json};
use std::{error::Error, fmt::Display};

use serde_repr::Serialize_repr;

use crate::core::response::ResponseBody;

#[derive(Clone, Serialize_repr, PartialEq, Debug)]
#[repr(u16)]
pub enum ErrorCode {
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

impl ErrorCode {
    pub fn res(&self) -> (StatusCode, String) {
        match self {
            ErrorCode::InvalidRequest => (
                StatusCode::BAD_REQUEST,
                "invalid request, please check request payload or headers.".to_owned(),
            ),
            ErrorCode::Forbidden => (StatusCode::UNAUTHORIZED, "forbidden.".to_owned()),
            ErrorCode::InvalidToken => (StatusCode::UNAUTHORIZED, "invalid token.".to_owned()),
            ErrorCode::ExpiredToken => (StatusCode::UNAUTHORIZED, "token is expired.".to_owned()),
            ErrorCode::NotFound => (
                StatusCode::NOT_FOUND,
                "resources does not exist.".to_owned(),
            ),
            ErrorCode::UnkownError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "unknown error.".to_owned(),
            ),
            ErrorCode::DBError => (StatusCode::INTERNAL_SERVER_ERROR, "DB error.".to_owned()),
            ErrorCode::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "creates token fail.".to_owned(),
            ),
            _ => (StatusCode::OK, "success".to_owned()),
        }
    }
}

#[derive(Debug)]
pub struct AppError {
    pub msg: Option<String>,
    pub code: ErrorCode,
    pub source: Option<Box<dyn Error>>,
}

impl AppError {
    pub fn from_err(err: Box<dyn Error>, code: Option<ErrorCode>) -> Self {
        Self {
            msg: None,
            source: Some(err),
            code: match code {
                Some(code) => code,
                _ => ErrorCode::UnkownError,
            },
        }
    }

    pub fn from_str(msg: String, code: Option<ErrorCode>) -> Self {
        Self {
            msg: Some(msg),
            source: None,
            code: match code {
                Some(code) => code,
                _ => ErrorCode::UnkownError,
            },
        }
    }

    pub fn from_code(code: ErrorCode, msg: Option<String>) -> Self {
        Self {
            msg,
            code,
            source: None,
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<Box<dyn Error>> for AppError {
    fn from(err: Box<dyn Error>) -> Self {
        Self::from_err(err, None)
    }
}

impl From<sea_orm::error::DbErr> for AppError {
    fn from(err: sea_orm::error::DbErr) -> Self {
        Self::from_str(err.to_string(), Some(ErrorCode::DBError))
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (http_code, msg) = self.code.res();
        let msg = match self.msg {
            Some(v) => v,
            None => msg,
        };
        (http_code, Json(ResponseBody::error(self.code, msg))).into_response()
    }
}
