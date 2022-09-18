use axum::{routing::get, Router};

mod handler;
use handler::{get_option, get_options, update_options};

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(get_options).put(update_options))
        .route("/:name", get(get_option))
    // .route("/:name", get(get_option).options(update_option))
}
