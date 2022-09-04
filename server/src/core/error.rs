use std::{error::Error, fmt::Display};

use axum::Json;

use crate::core::response::{ResponseBody, ResponseStatus};

#[derive(Debug)]
pub struct AppError {
    pub msg: Option<String>,
    pub code: ResponseStatus,
    pub source: Option<Box<dyn Error>>,
}

impl AppError {
    pub fn from_err(err: Box<dyn Error>, code: Option<ResponseStatus>) -> Self {
        Self {
            msg: None,
            source: Some(err),
            code: match code {
                Some(code) => code,
                _ => ResponseStatus::UnkownError,
            },
        }
    }

    pub fn from_str(msg: String, code: Option<ResponseStatus>) -> Self {
        Self {
            msg: Some(msg),
            source: None,
            code: match code {
                Some(code) => code,
                _ => ResponseStatus::UnkownError,
            },
        }
    }

    pub fn from_code(code: ResponseStatus, msg: Option<String>) -> Self {
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

impl From<deadpool_postgres::PoolError> for AppError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        Self::from_err(Box::new(err), Some(ResponseStatus::DBError))
    }
}

impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> Self {
        Self::from_err(Box::new(err), Some(ResponseStatus::DBError))
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
