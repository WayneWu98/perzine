use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Serialize)]
enum StatusCode {
    OK = 2000,
    NOT_FOUND = 4041,

    DB_ERROR = 5001,
    UNKNOWN_ERROR = 5004,
}

#[derive(Serialize)]
struct AppResponse<T: IntoResponse> {
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
