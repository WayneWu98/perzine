// pub fn get_contents(_claims: WeekClaims, Extension(state): Extension<Arc<AppState>>, pagination: Pagina)

// macro_rules! get_contents_handler {
//     ($name: ident) => {
//         pub fn $name() ->
//     };
// }

use std::sync::Arc;

use axum::Extension;
use chrono::Utc;
use sea_orm::{ColumnTrait, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder};
use serde::Deserialize;
use serde_enum_str::Deserialize_enum_str;

use crate::{
    core::{
        response::{HandlerResult, PaginationData, ResponseBody},
        AppState,
    },
    entity::post,
    extract::{Pagination, Query, WeekClaims},
};

#[derive(Deserialize_enum_str)]
#[serde(rename_all = "camelCase")]
enum OrderKey {
    Published,
    Modified,
}

impl OrderKey {
    fn column(&self) -> post::Column {
        match self {
            OrderKey::Published => post::Column::Published,
            OrderKey::Modified => post::Column::Modified,
        }
    }
}

fn is_valid_order(col: &post::Column) -> bool {
    match col {
        post::Column::Modified => true,
        post::Column::Published => true,
        _ => false,
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    all: Option<String>,
    created_from: Option<chrono::DateTime<Utc>>,
    created_to: Option<chrono::DateTime<Utc>>,
    modified_from: Option<chrono::DateTime<Utc>>,
    modified_to: Option<chrono::DateTime<Utc>>,
    published_from: Option<chrono::DateTime<Utc>>,
    published_to: Option<chrono::DateTime<Utc>>,
    status: Option<post::PostStatus>,
    keyword: Option<String>,
    order_by: Option<post::Column>,
    order: Option<sea_orm::Order>,
}

pub async fn get_contents(
    w_claims: WeekClaims,
    Extension(state): Extension<Arc<AppState>>,
    Pagination { page, per }: Pagination,
    Query(filter): Query<Filter>,
) -> HandlerResult<PaginationData<Vec<post::Model>>> {
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
            filter.order_by.map_or(post::Column::Published, |col| {
                if is_valid_order(&col) {
                    col
                } else {
                    post::Column::Published
                }
            }),
            filter.order.map_or(sea_orm::Order::Desc, |v| v),
        )
        .paginate(&state.db, per);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok(axum::Json(ResponseBody::with_pagination_data(items, total)))
}
