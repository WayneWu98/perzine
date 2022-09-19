use std::sync::Arc;

use axum::Extension;

use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder};

use crate::{
    core::{
        response::{HandlerResult, PaginationData, ResponseBody},
        AppState,
    },
    entity::{
        post,
        taxonomy::{self, ClassifiedTaxonomy, ClassifyTaxonomy},
    },
    extract::{Pagination, Query, WeekClaims},
};

use super::utils::{Filter, PostRes};

pub async fn get_contents(
    w_claims: WeekClaims,
    Extension(state): Extension<Arc<AppState>>,
    Pagination { page, per }: Pagination,
    Query(filter): Query<Filter>,
) -> HandlerResult<PaginationData<Vec<PostRes>>> {
    // #region filter condition
    let mut cond = sea_orm::Condition::all();
    if let Some(v) = filter.keyword {
        cond = cond.add(post::Column::Title.like(&v));
    }
    if let Some(v) = filter.modified_from {
        cond = cond.add(post::Column::Modified.gte(v));
    }
    if let Some(v) = filter.modified_to {
        cond = cond.add(post::Column::Modified.lte(v));
    }
    if w_claims.is_authed() {
        if let Some(v) = filter.created_from {
            cond = cond.add(post::Column::Created.gte(v));
        }
        if let Some(v) = filter.created_to {
            cond = cond.add(post::Column::Created.lte(v));
        }
        if let Some(v) = filter.published_from {
            cond = cond.add(post::Column::Published.gte(v));
        }
        if let Some(v) = filter.published_to {
            cond = cond.add(post::Column::Published.lte(v));
        }
        if filter.all.is_none() {
            cond = cond.add(post::Column::IsPage.eq(false));
        }
        if let Some(v) = filter.status {
            cond = cond.add(post::Column::Status.eq(v));
        }
    } else {
        cond = cond
            .add(post::Column::Published.lte(chrono::Utc::now()))
            .add(post::Column::Status.eq(post::PostStatus::Published))
            .add(post::Column::IsPage.eq(false));
    };
    // #endregion
    let paginator = post::Entity::find()
        .filter(cond)
        .order_by(
            filter
                .order_by
                .unwrap_or_default()
                .column(w_claims.is_authed()),
            filter.order.map_or(sea_orm::Order::Desc, |v| v.order()),
        )
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
