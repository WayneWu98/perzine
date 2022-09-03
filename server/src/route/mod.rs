pub mod option;

use axum::Extension;
use axum::{routing::get, Json, Router};
use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Clone, Serialize_repr, PartialEq)]
#[repr(u16)]
pub enum StatusCode {
    OK = 2000,
    NOT_FOUND = 4041,

    DB_ERROR = 5001,
    UNKNOWN_ERROR = 5004,
}

#[derive(Serialize)]
pub struct ResponseBody<T> {
    data: T,
    code: StatusCode,
    msg: String,
}

impl<T> ResponseBody<T> {
    pub fn new(data: T, code: StatusCode, msg: String) -> Self {
        Self { data, code, msg }
    }
    pub fn ok(data: T) -> Self {
        Self::new(data, StatusCode::OK, "success".to_string())
    }
}

impl ResponseBody<()> {
    pub fn error(code: StatusCode, msg: String) -> Self {
        Self::new((), code, msg)
    }
}

type HandlerResult<T> = Result<Json<ResponseBody<T>>, crate::utils::error::AppError>;

use crate::utils::AppState;

use self::option::get_options;

pub fn init(state: AppState) -> Router {
    Router::new()
        .route("/options", get(get_options))
        .layer(Extension(std::sync::Arc::new(state)))
}
