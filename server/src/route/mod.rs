pub mod auth;
pub mod option;
pub mod post;

use axum::Extension;
use axum::Router;

use crate::core::AppState;

pub fn init(state: AppState) -> Router {
    Router::new()
        .nest("/", auth::get_router())
        .nest("/options", option::get_router())
        .nest("/posts", post::get_router())
        .layer(Extension(std::sync::Arc::new(state)))
}
