use std::collections::HashMap;
use std::sync::Arc;

use axum::{Extension, Json};

use crate::model::site_option::{self, SiteOption};
use crate::route::HandlerResult;
use crate::utils::AppState;

use super::ResponseBody;

pub async fn get_options(
    Extension(state): Extension<Arc<AppState>>,
) -> HandlerResult<HashMap<String, String>> {
    let options = site_option::filter_publics(SiteOption::all(&state.pool).await?);
    let options = site_option::map(options);
    let options = Json(ResponseBody::ok(options));
    Ok(options)
}
