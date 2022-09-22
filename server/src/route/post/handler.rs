use std::sync::Arc;

use axum::Extension;

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait,
    QueryFilter, QueryOrder, TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        error::ErrorCode,
        response::{HandlerResult, PaginationData},
        AppState,
    },
    e_code_err,
    entity::{
        post::{self},
        post_taxonomy,
        taxonomy::{self, TaxonomyType},
    },
    extract::{Claims, JsonPayload, Pagination, Path, Query, WeekClaims},
    res_ok,
};

use crate::dto::post::PostWithTaxonomy;

use super::utils::Filter;

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
        item.is_authed = w_claims.is_authed();
        item.fulled = false;
        let taxonomies = item.find_related(taxonomy::Entity).all(&state.db).await?;
        formatted.push(PostWithTaxonomy::from_unclassified(item, taxonomies));
    }

    res_ok!(PaginationData::new(formatted, total))
}

pub async fn get_post_by_id(
    w_claims: WeekClaims,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> HandlerResult<impl Serialize> {
    let model = post::Entity::find()
        .filter(post::Column::Id.eq(id))
        .one(&state.db)
        .await?;
    if let Some(mut model) = model {
        model.is_authed = w_claims.is_authed();
        let taxonomies = model.find_related(taxonomy::Entity).all(&state.db).await?;
        return res_ok!(PostWithTaxonomy::from_unclassified(model, taxonomies));
    }
    e_code_err!(ErrorCode::NotFound)
}

pub async fn get_post_by_route(
    w_claims: WeekClaims,
    Extension(state): Extension<Arc<AppState>>,
    Path(route): Path<String>,
) -> HandlerResult<impl Serialize> {
    let model = post::Entity::find()
        .filter(post::Column::Route.eq(route))
        .one(&state.db)
        .await?;
    match model {
        None => return e_code_err!(ErrorCode::NotFound),
        Some(mut model) => {
            model.is_authed = w_claims.is_authed();
            let taxonomies = model.find_related(taxonomy::Entity).all(&state.db).await?;
            res_ok!(PostWithTaxonomy::from_unclassified(model, taxonomies))
        }
    }
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
    println!("{:#?}", am.is_page);
    let txn = state.db.begin().await?;
    let model = am.insert(&state.db).await?;
    if categories.is_none() {
        return e_code_err!(
            ErrorCode::InvalidRequest,
            Some("categories is required.".to_owned())
        );
    }
    if let Some(tids) = categories {
        post_taxonomy::update(&txn, model.id, tids, TaxonomyType::Category).await?
    }
    if let Some(tids) = tags {
        post_taxonomy::update(&txn, model.id, tids, TaxonomyType::Tag).await?;
    }
    if let Some(tid) = series {
        post_taxonomy::update(&txn, model.id, vec![tid], TaxonomyType::Series).await?;
    }
    txn.commit().await?;

    let txs = model.find_related(taxonomy::Entity).all(&state.db).await?;

    res_ok!(PostWithTaxonomy::from_unclassified(model, txs))
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
    let mut am = post::ActiveModel {
        ..Default::default()
    };
    am.set_from_json(jv)?;
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
    let txs = model.find_related(taxonomy::Entity).all(&state.db).await?;
    res_ok!(PostWithTaxonomy::from_unclassified(model, txs))
}

pub async fn delete_post(
    _claims: Claims,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> HandlerResult<()> {
    let txn = (state.db).begin().await?;
    post_taxonomy::Entity::delete_many()
        .filter(post_taxonomy::Column::PostId.eq(id))
        .exec(&txn)
        .await?;
    post::Entity::delete_by_id(id).exec(&txn).await?;
    txn.commit().await?;
    res_ok!(())
}
