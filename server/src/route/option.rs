use std::collections::HashMap;
use std::sync::Arc;

use axum::{Extension, Json};

use crate::model::site_option::{self, SiteOption};
use crate::route::HandlerResult;
use crate::utils::AppState;

pub async fn get_options(
    Extension(state): Extension<Arc<AppState>>,
) -> HandlerResult<Json<HashMap<String, String>>> {
    let options = site_option::filter_publics(SiteOption::all(&state.pool).await?);
    Ok(Json::from(site_option::map(options)))
}
