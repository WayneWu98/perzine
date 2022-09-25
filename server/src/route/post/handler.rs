use std::sync::Arc;

use axum::Extension;

use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, TransactionTrait,
};
use serde::{Deserialize, Serialize};

use super::utils::Filter;
use crate::core::{
    error::ErrorCode,
    response::{HandlerResult, PaginationData},
    AppState,
};
use crate::dto::post::{FulledPost, PostWithTaxonomy, SimplePost};
use crate::entity::{
    post, post_taxonomy,
    taxonomy::{self, TaxonomyType},
};
use crate::extract::{Claims, JsonPayload, Pagination, Path, Query, WeekClaims};
use crate::{e_code_err, res_ok};

pub async fn get_posts(
    w_claims: WeekClaims,
    Extension(state): Extension<Arc<AppState>>,
    Pagination { page, per }: Pagination,
    Query(filter): Query<Filter>,
) -> HandlerResult<impl Serialize> {
    let paginator = post::Entity::find()
        .filter(filter.condition(w_claims.is_authed()))
        .order_by(filter.order_by(w_claims.is_authed()), filter.order())
        .paginate(&state.db, per);

    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    let mut formatted = Vec::with_capacity(items.len());
    for mut item in items.into_iter() {
        let txs = item.txs(&state.db).await?;
        item.comment_count = item.comment_count(&state.db).await?;
        let mut item = SimplePost::from(item);
        item.is_authed = w_claims.is_authed();
        formatted.push(PostWithTaxonomy::from_unclassified(item, txs));
    }

    res_ok!(PaginationData::new(formatted, total))
}

pub async fn get_post_by_id(
    w_claims: WeekClaims,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> HandlerResult<impl Serialize> {
    let item = post::Entity::find()
        .filter(post::Column::Id.eq(id))
        .one(&state.db)
        .await?;
    if let Some(mut item) = item {
        item.comment_count = item.comment_count(&state.db).await?;
        let txs = item.txs(&state.db).await?;
        let mut item = FulledPost::from(item);
        item.is_authed = w_claims.is_authed();
        return res_ok!(PostWithTaxonomy::from_unclassified(item, txs));
    }
    e_code_err!(ErrorCode::NotFound)
}

pub async fn get_post_by_route(
    w_claims: WeekClaims,
    Extension(state): Extension<Arc<AppState>>,
    Path(route): Path<String>,
) -> HandlerResult<impl Serialize> {
    let item = post::Entity::find()
        .filter(post::Column::Route.eq(route))
        .one(&state.db)
        .await?;
    if let Some(mut item) = item {
        item.comment_count = item.comment_count(&state.db).await?;
        let txs = item.txs(&state.db).await?;
        let mut item = FulledPost::from(item);
        item.is_authed = w_claims.is_authed();
        return res_ok!(PostWithTaxonomy::from_unclassified(item, txs));
    }
    e_code_err!(ErrorCode::NotFound)
}

#[derive(Deserialize)]
pub struct ExtraPayload {
    pub categories: Option<Vec<i32>>,
    pub tags: Option<Vec<i32>>,
    pub series: Option<i32>,
}

pub async fn create_post(
    _claims: Claims,
    Extension(state): Extension<Arc<AppState>>,
    JsonPayload(jv): JsonPayload<serde_json::Value>,
) -> HandlerResult<impl Serialize> {
    let ExtraPayload {
        categories,
        tags,
        series,
    } = serde_json::from_value(jv.clone())?;
    let am = post::ActiveModel::from_json(jv.clone())?;
    let txn = state.db.begin().await?;
    let mut item = am.insert(&state.db).await?;
    if categories.is_none() {
        return e_code_err!(
            ErrorCode::InvalidRequest,
            Some("categories is required.".to_owned())
        );
    }
    if let Some(tids) = categories {
        post_taxonomy::update(&txn, item.id, tids, TaxonomyType::Category).await?
    }
    if let Some(tids) = tags {
        post_taxonomy::update(&txn, item.id, tids, TaxonomyType::Tag).await?;
    }
    if let Some(tid) = series {
        post_taxonomy::update(&txn, item.id, vec![tid], TaxonomyType::Series).await?;
    }
    txn.commit().await?;

    let txs = item.txs(&state.db).await?;
    item.comment_count = item.comment_count(&state.db).await?;
    let mut item = FulledPost::from(item);
    item.is_authed = true;

    res_ok!(PostWithTaxonomy::from_unclassified(item, txs))
}

pub async fn update_post(
    _claims: Claims,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
    JsonPayload(jv): JsonPayload<serde_json::Value>,
) -> HandlerResult<impl Serialize> {
    let ExtraPayload {
        categories,
        tags,
        series,
    } = serde_json::from_value(jv.clone())?;
    let mut am = post::ActiveModel::from_json(jv.clone())?;
    am.modified = ActiveValue::Set(Some(Utc::now()));
    am.id = ActiveValue::Set(id);
    let txn = (&state.db).begin().await?;
    let mut item = am.update(&txn).await?;
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
    let txs = item.txs(&state.db).await?;
    item.comment_count = item.comment_count(&state.db).await?;
    let mut item = FulledPost::from(item);
    item.is_authed = true;
    res_ok!(PostWithTaxonomy::from_unclassified(item, txs))
}

pub async fn delete_post(
    _claims: Claims,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> HandlerResult<()> {
    let target = post::Entity::find_by_id(id).one(&state.db).await?;
    match target {
        Some(v) => {
            if !v.status.eq(&Some(post::PostStatus::Trashed)) {
                return e_code_err!(
                    ErrorCode::InvalidRequest,
                    Some("only the post trashed can be deleted.".to_owned())
                );
            }
        }
        None => return e_code_err!(ErrorCode::NotFound),
    }
    let txn = (state.db).begin().await?;
    post_taxonomy::Entity::delete_many()
        .filter(post_taxonomy::Column::PostId.eq(id))
        .exec(&txn)
        .await?;
    post::Entity::delete_by_id(id).exec(&txn).await?;
    txn.commit().await?;
    res_ok!(())
}
