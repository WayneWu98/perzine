use std::{any::TypeId, sync::Arc};

use axum::Extension;

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        error::{AppError, ErrorCode},
        response::{HandlerResult, PaginationData, ResponseBody},
        AppState,
    },
    entity::{
        post::{self},
        post_taxonomy,
        taxonomy::{self, ClassifiedTaxonomy, ClassifyTaxonomy},
    },
    extract::{Claims, JsonPayload, Pagination, Path, Query, WeekClaims},
};

use super::utils::{fill_post, gen_common_selec, Filter, PostRes};

pub async fn get_posts(
    w_claims: WeekClaims,
    Extension(state): Extension<Arc<AppState>>,
    Pagination { page, per }: Pagination,
    Query(filter): Query<Filter>,
) -> HandlerResult<PaginationData<Vec<PostRes>>> {
    let paginator = gen_common_selec(w_claims.is_authed())
        .filter(filter.condition(w_claims.is_authed()))
        .order_by(filter.order_by(w_claims.is_authed()), filter.order())
        .paginate(&state.db, per);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    let mut formatted = Vec::with_capacity(items.len());
    for item in items.into_iter() {
        let ClassifiedTaxonomy {
            categories,
            tags,
            mut series,
        } = item
            .find_related(taxonomy::Entity)
            .all(&state.db)
            .await?
            .classify();
        formatted.push(
            PostRes::new(item)
                .with_categories(categories)
                .with_tags(tags)
                .with_series(series.pop()),
        );
    }
    Ok(axum::Json(ResponseBody::with_pagination_data(
        formatted, total,
    )))
}

pub async fn get_post_by_id(
    w_claims: WeekClaims,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> HandlerResult<PostRes> {
    let content =
        super::utils::get_post_by_filter(&state.db, post::Column::Id.eq(id), w_claims.is_authed())
            .await?;
    let content = match content {
        None => return Err(AppError::from_code(ErrorCode::NotFound, None)),
        Some(v) => v,
    };
    Ok(axum::Json(ResponseBody::ok(content)))
}

pub async fn get_post_by_route(
    w_claims: WeekClaims,
    Extension(state): Extension<Arc<AppState>>,
    Path(route): Path<String>,
) -> HandlerResult<PostRes> {
    let content = super::utils::get_post_by_filter(
        &state.db,
        post::Column::Route.eq(route),
        w_claims.is_authed(),
    )
    .await?;
    let content = match content {
        None => return Err(AppError::from_code(ErrorCode::NotFound, None)),
        Some(v) => v,
    };
    Ok(axum::Json(ResponseBody::ok(content)))
}

#[derive(Deserialize)]
pub struct UpdateExtraPayload {
    pub categories: Option<Vec<i32>>,
    pub tags: Option<Vec<i32>>,
    pub series: Option<i32>,
}

pub async fn create_post(
    _claims: Claims,
    Extension(state): Extension<Arc<AppState>>,
    JsonPayload(jv): JsonPayload<serde_json::Value>,
    JsonPayload(UpdateExtraPayload {
        categories,
        tags,
        series,
    }): JsonPayload<UpdateExtraPayload>,
) -> HandlerResult<PostRes> {
    let txn = state.db.begin().await?;
    let model = post::ActiveModel::from_json(jv)?.insert(&state.db).await?;
    if let Some(tids) = categories {
        post_taxonomy::update(&state.db, model.id, tids, taxonomy::TaxonomyType::Category).await?;
    }
    if let Some(tids) = tags {
        post_taxonomy::update(&state.db, model.id, tids, taxonomy::TaxonomyType::Tag).await?;
    }
    if let Some(tid) = series {
        post_taxonomy::update(
            &state.db,
            model.id,
            vec![tid],
            taxonomy::TaxonomyType::Series,
        )
        .await?;
    }
    txn.commit().await?;
    let data = fill_post(&state.db, model).await?;

    Ok(axum::Json(ResponseBody::ok(data)))
}

pub async fn update_post(
    _claims: Claims,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
    JsonPayload(jv): JsonPayload<serde_json::Value>,
    JsonPayload(UpdateExtraPayload {
        categories,
        tags,
        series,
    }): JsonPayload<UpdateExtraPayload>,
) -> HandlerResult<()> {
    let mut am = post::ActiveModel::from_json(jv)?;
    am.id = ActiveValue::Set(id);
    let txn = (&state.db).begin().await?;
    am.update(&txn).await?;
    if let Some(tids) = categories {
        post_taxonomy::update(&state.db, id, tids, taxonomy::TaxonomyType::Category).await?;
    }
    if let Some(tids) = tags {
        post_taxonomy::update(&state.db, id, tids, taxonomy::TaxonomyType::Tag).await?;
    }
    if let Some(tid) = series {
        post_taxonomy::update(&state.db, id, vec![tid], taxonomy::TaxonomyType::Series).await?;
    }
    txn.commit().await?;
    Ok(axum::Json(ResponseBody::ok(())))
}
