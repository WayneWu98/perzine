use std::{error::Error, fmt::Display};

use axum::Json;

use crate::route::ResponseBody;

#[derive(Debug)]
pub struct AppError {
    pub msg: Option<String>,
    pub source: Option<Box<dyn Error>>,
}

impl AppError {
    fn from_err(err: Box<dyn Error>) -> Self {
        Self {
            msg: None,
            source: Some(err),
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
        Self::from_err(err)
    }
}

impl From<deadpool_postgres::PoolError> for AppError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        Self::from_err(Box::new(err))
    }
}

impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> Self {
        Self::from_err(Box::new(err))
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        Json(ResponseBody::error(
            crate::route::StatusCode::UNKNOWN_ERROR,
            self.to_string(),
        ))
        .into_response()
    }
}
