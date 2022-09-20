use std::sync::Arc;

use axum::Extension;

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, TransactionTrait,
};
use serde::Deserialize;

use crate::{
    core::{
        error::{AppError, ErrorCode},
        response::{HandlerResult, PaginationData, ResponseBody},
        AppState,
    },
    entity::{
        post::{self, NewPost},
        post_taxonomy,
        taxonomy::{self, TaxonomyType},
    },
    extract::{Claims, JsonPayload, Pagination, Path, Query, WeekClaims},
};

use super::utils::{fill_post, gen_common_selec, Filter, WithExtra};

pub async fn get_posts(
    w_claims: WeekClaims,
    Extension(state): Extension<Arc<AppState>>,
    Pagination { page, per }: Pagination,
    Query(filter): Query<Filter>,
    Query(WithExtra { extra }): Query<WithExtra>,
) -> HandlerResult<PaginationData<Vec<post::Model>>> {
    let paginator = gen_common_selec(w_claims.is_authed(), extra.is_some())
        .filter(filter.condition(w_claims.is_authed()))
        .order_by(filter.order_by(w_claims.is_authed()), filter.order())
        .paginate(&state.db, per);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    let mut formatted = Vec::with_capacity(items.len());
    for item in items.into_iter() {
        formatted.push(item.with_taxonomy(&state.db).await?);
    }
    Ok(axum::Json(ResponseBody::with_pagination_data(
        formatted, total,
    )))
}

pub async fn get_post_by_id(
    w_claims: WeekClaims,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
    Query(WithExtra { extra }): Query<WithExtra>,
) -> HandlerResult<post::Model> {
    let model = super::utils::get_post_by_filter(
        &state.db,
        post::Column::Id.eq(id),
        w_claims.is_authed(),
        extra.is_some(),
    )
    .await?;
    match model {
        None => return Err(AppError::from_code(ErrorCode::NotFound, None)),
        Some(v) => Ok(axum::Json(ResponseBody::ok(
            v.with_taxonomy(&state.db).await?,
        ))),
    }
}

pub async fn get_post_by_route(
    w_claims: WeekClaims,
    Extension(state): Extension<Arc<AppState>>,
    Path(route): Path<String>,
    Query(WithExtra { extra }): Query<WithExtra>,
) -> HandlerResult<post::Model> {
    let model = super::utils::get_post_by_filter(
        &state.db,
        post::Column::Route.eq(route),
        w_claims.is_authed(),
        extra.is_some(),
    )
    .await?;
    match model {
        None => return Err(AppError::from_code(ErrorCode::NotFound, None)),
        Some(v) => Ok(axum::Json(ResponseBody::ok(
            v.with_taxonomy(&state.db).await?,
        ))),
    }
}

#[derive(Deserialize)]
pub struct UpdateExtraPayload {
    pub categories: Option<Vec<i32>>,
    pub tags: Option<Vec<i32>>,
    pub series: Option<i32>,
}
#[derive(Deserialize)]
pub struct CreateExtraPayload {
    pub categories: Vec<i32>,
    pub tags: Option<Vec<i32>>,
    pub series: Option<i32>,
}

pub async fn create_post(
    _claims: Claims,
    Extension(state): Extension<Arc<AppState>>,
    JsonPayload(jv): JsonPayload<serde_json::Value>,
) -> HandlerResult<post::Model> {
    let CreateExtraPayload {
        categories,
        tags,
        series,
    } = serde_json::from_value(jv.clone())?;
    let txn = state.db.begin().await?;
    let new_post: NewPost = serde_json::from_value(jv.clone())?;

    let model = new_post.into_active_model()?;
    let model = model.insert(&state.db).await?;
    post_taxonomy::update(&txn, model.id, categories, TaxonomyType::Category).await?;
    if let Some(tids) = tags {
        post_taxonomy::update(&txn, model.id, tids, TaxonomyType::Tag).await?;
    }
    if let Some(tid) = series {
        post_taxonomy::update(&txn, model.id, vec![tid], TaxonomyType::Series).await?;
    }
    txn.commit().await?;

    Ok(axum::Json(ResponseBody::ok(
        fill_post(&state.db, model).await?,
    )))
}

pub async fn update_post(
    _claims: Claims,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
    JsonPayload(jv): JsonPayload<serde_json::Value>,
) -> HandlerResult<post::Model> {
    let UpdateExtraPayload {
        categories,
        tags,
        series,
    } = serde_json::from_value(jv.clone())?;

    let mut am = post::ActiveModel::from_json(jv)?;
    am.id = ActiveValue::Set(id);
    let txn = (&state.db).begin().await?;
    let model = am.update(&txn).await?;
    if let Some(tids) = categories {
        post_taxonomy::update(&txn, id, tids, taxonomy::TaxonomyType::Category).await?;
    }
    if let Some(tids) = tags {
        post_taxonomy::update(&txn, id, tids, taxonomy::TaxonomyType::Tag).await?;
    }
    if let Some(tid) = series {
        post_taxonomy::update(&txn, id, vec![tid], taxonomy::TaxonomyType::Series).await?;
    }
    txn.commit().await?;
    Ok(axum::Json(ResponseBody::ok(
        model.with_taxonomy(&state.db).await?,
    )))
}

pub async fn delete_post(
    _claims: Claims,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> HandlerResult<()> {
    let txn = (state.db).begin().await?;
    post::Entity::delete_by_id(id).exec(&txn).await?;
    post_taxonomy::update(&txn, id, vec![], TaxonomyType::Category).await?;
    post_taxonomy::update(&txn, id, vec![], TaxonomyType::Tag).await?;
    post_taxonomy::update(&txn, id, vec![], TaxonomyType::Series).await?;
    txn.commit().await?;
    Ok(axum::Json(ResponseBody::ok(())))
}
