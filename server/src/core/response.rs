use axum::Json;
use serde::Serialize;

use super::error::ErrorCode;

#[derive(Serialize)]
pub struct ResponseBody<T> {
    data: T,
    code: ErrorCode,
    msg: String,
}

#[derive(Serialize)]
pub struct PaginationData<T> {
    pub total: usize,
    pub items: T,
}

impl<T> PaginationData<T> {
    pub fn new(items: T, total: usize) -> Self {
        Self { items, total }
    }
}

impl<T> ResponseBody<T> {
    pub fn new(data: T, code: ErrorCode, msg: String) -> Self {
        Self { data, code, msg }
    }
    pub fn ok(data: T) -> Self {
        Self::new(data, ErrorCode::OK, "success".to_string())
    }
}

impl<T> ResponseBody<PaginationData<T>> {
    pub fn with_pagination_data(data: T, total: usize) -> Self {
        Self::ok(PaginationData::new(data, total))
    }
}

impl ResponseBody<()> {
    pub fn error(code: ErrorCode, msg: String) -> Self {
        Self::new((), code, msg)
    }
}

pub type HandlerResult<T> = Result<Json<ResponseBody<T>>, crate::core::error::AppError>;
