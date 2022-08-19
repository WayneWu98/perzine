pub mod option;

use axum::response::IntoResponse;
use axum::Extension;
use axum::{
    routing::{delete, get, options, post},
    Router,
};
use serde::Serialize;

#[derive(Serialize)]
enum StatusCode {
    OK = 2000,
    NOT_FOUND = 4041,

    DB_ERROR = 5001,
    UNKNOWN_ERROR = 5004,
}

#[derive(Serialize)]
pub struct AppResponse<T: IntoResponse> {
    data: T,
    code: StatusCode,
    msg: String,
}

impl<T: IntoResponse> AppResponse<T> {
    fn new(data: T, code: StatusCode, msg: String) -> Self {
        Self { data, code, msg }
    }
    fn ok(data: T) -> Self {
        Self::new(data, StatusCode::OK, "success".to_string())
    }
}

impl AppResponse<()> {
    fn error(code: StatusCode, msg: String) -> Self {
        Self::new((), code, msg)
    }
}

type HandlerResult<T: IntoResponse> = Result<T, Box<dyn std::error::Error>>;

impl<T: IntoResponse> IntoResponse for HandlerResult<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            Ok(data) => AppResponse::ok(data),
            Err(err) => AppResponse::error(StatusCode::UNKNOWN_ERROR, err.to_string()),
        }
    }
}

use crate::utils::AppState;

use self::option::get_options;

pub fn init(state: AppState) -> Router {
    Router::new()
        .route("/options", get(get_options))
        .layer(Extension(std::sync::Arc::new(state)))
}
