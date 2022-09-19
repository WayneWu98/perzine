use axum::{routing::get, Router};

mod handler;

pub fn get_router() -> Router {
    Router::new().route("/posts", get(handler::get_contents))
}
