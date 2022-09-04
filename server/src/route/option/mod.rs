use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    routing::{get, options, post},
    Extension, Json, Router,
};

use crate::core::AppState;
use crate::model::site_option::{self, SiteOption};

use crate::core::response::{HandlerResult, ResponseBody};

pub async fn get_options(
    Extension(state): Extension<Arc<AppState>>,
) -> HandlerResult<HashMap<String, Option<String>>> {
    let options = SiteOption::select_all(&mut state.rb.clone()).await?;
    let options = site_option::map(options);
    let options = Json(ResponseBody::ok(options));
    Ok(options)
}

pub fn get_router() -> Router {
    Router::new().route("/", get(get_options))
}
