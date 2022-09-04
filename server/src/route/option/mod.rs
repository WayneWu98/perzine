use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    routing::{get, options, post},
    Extension, Json, Router,
};
use once_cell::sync::Lazy;

use crate::core::AppState;
use crate::model::site_option::{self, SiteOption};

use crate::core::response::{HandlerResult, ResponseBody};

pub async fn get_options(
    Extension(state): Extension<Arc<AppState>>,
) -> HandlerResult<HashMap<String, String>> {
    let options = site_option::filter_publics(SiteOption::all(&state.pool).await?);
    let options = site_option::map(options);
    let options = Json(ResponseBody::ok(options));
    Ok(options)
}

pub fn get_router() -> Router {
    Router::new().route("/", get(get_options))
}
