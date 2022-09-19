use axum::{routing::get, Router};

mod handler;
mod utils;

pub fn get_router() -> Router {
    Router::new().route("/posts", get(handler::get_contents))
}
