use axum::{routing::get, Router};

mod handler;
mod utils;

use handler::{create_post, get_post_by_id, get_post_by_route, get_posts};

pub fn get_router() -> Router {
    Router::new()
        .route("/posts", get(get_posts).post(create_post))
        .route("/posts/:id", get(get_post_by_id))
        .route("/posts/route/:route", get(get_post_by_route))
}
