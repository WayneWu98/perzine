use axum::{routing::get, Router};

mod curl;
mod handler;

use handler::{
    create_category, create_series, create_tag, delete_category, delete_series, delete_tag,
    get_categories, get_series, get_tags, get_the_category, get_the_series, get_the_tag,
    update_category, update_series, update_tag,
};

pub fn get_router() -> Router {
    Router::new()
        .route("/categories", get(get_categories).post(create_category))
        .route(
            "/categories/:id",
            get(get_the_category)
                .put(update_category)
                .delete(delete_category),
        )
        .route("/tags", get(get_tags).post(create_tag))
        .route(
            "/tags/:id",
            get(get_the_tag).put(update_tag).delete(delete_tag),
        )
        .route("/series", get(get_series).post(create_series))
        .route(
            "/series/:id",
            get(get_the_series).put(update_series).delete(delete_series),
        )
}
