use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    routing::{get, options, post},
    Extension, Json, Router,
};
use serde::Deserialize;

use crate::extract::Path;
use crate::model::site_option::{self, SiteOption};
use crate::{
    core::{
        error::AppError,
        response::{HandlerResult, ResponseBody, ResponseStatus},
        AppState,
    },
    extract::JsonPayload,
};

pub async fn get_options(
    Extension(state): Extension<Arc<AppState>>,
) -> HandlerResult<HashMap<String, Option<String>>> {
    let options = SiteOption::select_all(&mut state.rb.clone()).await?;
    let options = site_option::map(options);
    let options = Json(ResponseBody::ok(options));
    Ok(options)
}

pub async fn get_option(
    Extension(state): Extension<Arc<AppState>>,
    Path(name): Path<String>,
) -> HandlerResult<Option<String>> {
    let opt = SiteOption::select_by_name(&mut state.rb.clone(), "options".to_owned(), name).await?;
    match opt {
        Some(opt) => Ok(Json(ResponseBody::ok(opt.value))),
        None => Err(AppError::from_code(
            ResponseStatus::NotFound,
            Some("option does not exist.".to_owned()),
        )),
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdatePayload {
    pub name: String,
    pub value: String,
}

impl From<UpdatePayload> for SiteOption {
    fn from(UpdatePayload { name, value }: UpdatePayload) -> Self {
        Self {
            id: None,
            name: Some(name),
            value: Some(value),
        }
    }
}

// pub async fn update_option(
//     Extension(state): Extension<Arc<AppState>>,
//     Path(name): Path<String>,
//     JsonPayload(payload): JsonPayload<UpdatePayload>,
// ) -> HandlerResult<()> {
//     let option = SiteOption::from(payload);
//     SiteOption::update_by_name(&mut state.rb.clone(), &option, payload.name.clone());
//     // SiteOption::update_by_column(&mut state.rb.clone(), "options", column);
//     // SiteOption::update_by_name(&mut state.rb.clone(), name, value);
//     // SiteOption::update_by_column(rb, table, column)
//     Ok(Json(ResponseBody::ok(())))
// }

pub fn get_router() -> Router {
    Router::new().route("/", get(get_options))
    // .route("/:name", get(get_option).options(update_option))
}
