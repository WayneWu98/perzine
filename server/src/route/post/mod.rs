use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Json, Router,
};

use crate::{
    core::{
        response::{HandlerResult, ResponseBody},
        AppState,
    },
    model::post::Post,
};

async fn get_posts(Extension(state): Extension<Arc<AppState>>) -> HandlerResult<Vec<Post>> {
    let posts = Post::select_all(&mut state.rb.clone()).await?;
    Ok(Json(ResponseBody::ok(posts)))
}

pub fn get_router() -> Router {
    Router::new().route("/", get(get_posts))
}
