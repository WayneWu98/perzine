pub mod auth;
pub mod option;
pub mod post;
pub mod taxonomy;

use axum::Extension;
use axum::Router;

use crate::core::AppState;

pub fn init(state: AppState) -> Router {
    Router::new()
        .nest("/", auth::get_router())
        .nest("/", taxonomy::get_router())
        .nest("/", post::get_router())
        .nest("/options", option::get_router())
        // .nest("/posts", post::get_router())
        .layer(Extension(std::sync::Arc::new(state)))
    // .layer(axum::error_handling::HandleErrorLayer::new(handle_error))
}
