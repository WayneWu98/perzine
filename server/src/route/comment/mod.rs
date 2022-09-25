mod handler;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use handler::{create_comment, get_comments, reply_comment, update_comment};

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(get_comments).post(create_comment))
        .route("/:id", post(update_comment))
        .route("/:id/reply", post(reply_comment))
}
