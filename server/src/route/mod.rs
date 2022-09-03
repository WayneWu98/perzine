pub mod option;

use axum::Extension;
use axum::{routing::get, Router};

use crate::core::AppState;

use self::option::get_options;

pub fn init(state: AppState) -> Router {
    Router::new()
        .route("/options", get(get_options))
        .layer(Extension(std::sync::Arc::new(state)))
}
