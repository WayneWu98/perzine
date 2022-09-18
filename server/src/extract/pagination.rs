use axum::{
    async_trait,
    extract::{FromRequest, Query, RequestParts},
};
use serde::Deserialize;

use crate::core::error::{AppError, ErrorCode};

#[derive(Deserialize)]
struct OptionalPagination {
    pub page: Option<usize>,
    pub per: Option<usize>,
}

#[derive(Clone, Copy)]
pub struct Pagination {
    pub page: usize,
    pub per: usize,
}

#[async_trait]
impl<T: Send + Sync> FromRequest<T> for Pagination {
    type Rejection = AppError;
    async fn from_request(req: &mut RequestParts<T>) -> Result<Self, Self::Rejection> {
        let Query(OptionalPagination { page, per }) =
            Query::<OptionalPagination>::from_request(req)
                .await
                .map_err(|_| AppError::from_code(ErrorCode::UnkownError, None))?;
        Ok(Pagination {
            page: page.map_or(0, |n| n - 1),
            per: per.unwrap_or(10),
        })
    }
}
