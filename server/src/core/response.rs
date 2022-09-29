use axum::Json;
use serde::Serialize;

use super::error::ErrorCode;

#[derive(Serialize)]
pub struct ResponseBody<T>
where
    T: Serialize,
{
    data: T,
    code: ErrorCode,
    msg: String,
}

#[derive(Serialize)]
pub struct PaginationData<T> {
    pub total: usize,
    pub pages: usize,
    pub items: T,
}

impl<T> PaginationData<T> {
    pub fn new(items: T, total: usize, pages: usize) -> Self {
        Self {
            items,
            total,
            pages,
        }
    }
}

impl<T: Serialize> ResponseBody<T> {
    pub fn new(data: T, code: ErrorCode, msg: String) -> Self {
        Self { data, code, msg }
    }
    pub fn ok(data: T) -> Self {
        Self::new(data, ErrorCode::OK, "success".to_string())
    }
}

impl<T: Serialize> ResponseBody<PaginationData<T>> {
    pub fn with_pagination_data(data: T, total: usize, pages: usize) -> Self {
        Self::ok(PaginationData::new(data, total, pages))
    }
}

impl ResponseBody<()> {
    pub fn error(code: ErrorCode, msg: String) -> Self {
        Self::new((), code, msg)
    }
}

pub type HandlerResult<T> = Result<Json<ResponseBody<T>>, crate::core::error::AppError>;

#[macro_export]
macro_rules! res_ok {
    ($data: expr) => {
        Ok(axum::Json(crate::core::response::ResponseBody::ok($data)))
    };
    ($data: expr, $type: ty) => {
        match $type {
            _ => res_ok!($data),
        }
    };
}

#[macro_export]
macro_rules! res {
    ($data: expr) => {
        axum::Json(crate::core::response::ResponseBody::ok($data))
    };
    ($data: expr, $type: ty) => {
        match $type {
            _ => res!($data),
        }
    };
}
