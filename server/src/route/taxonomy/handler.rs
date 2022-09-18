use axum::Extension;
use std::sync::Arc;

use crate::{
    core::{
        response::{HandlerResult, PaginationData},
        AppState,
    },
    entity::taxonomy::{Model as Taxonomy, TaxonomyType},
    extract::{Claims, JsonPayload, Pagination, Path},
};

use super::curl::{
    create_taxonomy, delete_taxonomy, get_taxonomies, get_taxonomy, update_taxonomy,
};

macro_rules! get_list_handler {
    ($name: ident, $t_type: expr) => {
        pub async fn $name(
            state: Extension<Arc<AppState>>,
            pagination: Pagination,
        ) -> HandlerResult<PaginationData<Vec<Taxonomy>>> {
            Ok(get_taxonomies(state, pagination, $t_type).await?)
        }
    };
}

macro_rules! get_item_handler {
    ($name: ident, $t_type: expr) => {
        pub async fn $name(
            state: Extension<Arc<AppState>>,
            path: Path<i32>,
        ) -> HandlerResult<Taxonomy> {
            Ok(get_taxonomy(state, path, $t_type).await?)
        }
    };
}

macro_rules! get_create_handler {
    ($name: ident, $t_type: expr) => {
        pub async fn $name(
            claims: Claims,
            state: Extension<Arc<AppState>>,
            payload: JsonPayload<serde_json::Value>,
        ) -> HandlerResult<Taxonomy> {
            Ok(create_taxonomy(claims, state, payload, $t_type).await?)
        }
    };
}

macro_rules! get_update_handler {
    ($name: ident, $t_type: expr) => {
        pub async fn $name(
            claims: Claims,
            state: Extension<Arc<AppState>>,
            path: Path<i32>,
            payload: JsonPayload<serde_json::Value>,
        ) -> HandlerResult<Taxonomy> {
            Ok(update_taxonomy(claims, state, path, payload, $t_type).await?)
        }
    };
}

macro_rules! get_delete_handler {
    ($name: ident, $t_type: expr) => {
        pub async fn $name(
            claims: Claims,
            state: Extension<Arc<AppState>>,
            path: Path<i32>,
        ) -> HandlerResult<()> {
            Ok(delete_taxonomy(claims, state, path, $t_type).await?)
        }
    };
}

get_list_handler!(get_categories, TaxonomyType::Category);
get_list_handler!(get_tags, TaxonomyType::Tag);
get_list_handler!(get_series, TaxonomyType::Series);

get_item_handler!(get_the_category, TaxonomyType::Category);
get_item_handler!(get_the_tag, TaxonomyType::Tag);
get_item_handler!(get_the_series, TaxonomyType::Series);

get_create_handler!(create_category, TaxonomyType::Category);
get_create_handler!(create_tag, TaxonomyType::Tag);
get_create_handler!(create_series, TaxonomyType::Series);

get_update_handler!(update_category, TaxonomyType::Category);
get_update_handler!(update_tag, TaxonomyType::Tag);
get_update_handler!(update_series, TaxonomyType::Series);

get_delete_handler!(delete_category, TaxonomyType::Category);
get_delete_handler!(delete_tag, TaxonomyType::Tag);
get_delete_handler!(delete_series, TaxonomyType::Series);
