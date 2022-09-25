use axum::{routing::get, Router};

mod handler;
mod utils;

use handler::{
    create_post, delete_post, get_post_by_id, get_post_by_route, get_posts, update_post,
};

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(get_posts).post(create_post))
        .route(
            "/:id",
            get(get_post_by_id).put(update_post).delete(delete_post),
        )
        .route("/route/:route", get(get_post_by_route))
}
