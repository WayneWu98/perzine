use crate::{
    core::{
        error::{AppError, ErrorCode},
        response::{HandlerResult, PaginationData, ResponseBody},
        AppState,
    },
    extract::{Claims, JsonPayload, Pagination, Path},
};
use axum::Extension;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder,
};
use std::sync::Arc;

use crate::entity::taxonomy::{self, Model as Taxonomy, TaxonomyType};

pub async fn get_taxonomies(
    Extension(state): Extension<Arc<AppState>>,
    Pagination { page, per }: Pagination,
    t_type: TaxonomyType,
) -> HandlerResult<PaginationData<Vec<Taxonomy>>> {
    let paginator = taxonomy::Entity::find()
        .filter(taxonomy::Column::TType.eq(t_type.clone()))
        .order_by_desc(taxonomy::Column::Id)
        .paginate(&state.db, per);

    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page).await?;
    Ok(axum::Json(ResponseBody::with_pagination_data(items, total)))
}

pub async fn get_taxonomy(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
    t_type: TaxonomyType,
) -> HandlerResult<Taxonomy> {
    let res = taxonomy::Entity::find_by_id(id)
        .filter(taxonomy::Column::TType.eq(t_type))
        .one(&state.db)
        .await?;
    match res {
        Some(item) => Ok(axum::Json(ResponseBody::ok(item))),
        None => Err(AppError::from_code(ErrorCode::NotFound, None)),
    }
}

pub async fn create_taxonomy(
    _claims: Claims,
    Extension(state): Extension<Arc<AppState>>,
    JsonPayload(jv): JsonPayload<serde_json::Value>,
    t_type: TaxonomyType,
) -> HandlerResult<Taxonomy> {
    let mut am = taxonomy::ActiveModel::from_json(jv)?;
    let name: String = am.name.clone().take().unwrap_or("".to_owned());
    if taxonomy::is_exist_in_name(name.clone(), t_type.clone(), &state.db).await? {
        return Err(AppError::from_code(
            ErrorCode::InvalidRequest,
            Some(format!("the taxonomy named \"{}\" has been exist.", name)),
        ));
    }

    am.t_type = sea_orm::ActiveValue::Set(t_type.clone());

    Ok(axum::Json(ResponseBody::ok(am.insert(&state.db).await?)))
}

pub async fn update_taxonomy(
    _claims: Claims,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
    JsonPayload(jv): JsonPayload<serde_json::Value>,
    t_type: TaxonomyType,
) -> HandlerResult<Taxonomy> {
    let mut am = taxonomy::ActiveModel::from_json(jv)?;
    am.id = ActiveValue::Set(id);
    if !taxonomy::is_exist_in_id(id, t_type, &state.db).await? {
        return Err(AppError::from_code(ErrorCode::NotFound, None));
    }
    if let Some(name) = am.name.clone().take() {
        let repeated = taxonomy::Entity::find()
            .filter(
                taxonomy::Column::Name
                    .eq(name.clone())
                    .and(taxonomy::Column::Id.ne(id)),
            )
            .one(&state.db)
            .await?;
        if repeated.is_some() {
            return Err(AppError::from_code(
                ErrorCode::InvalidRequest,
                Some(format!(
                    "the taxonomy named \"{}\" has been exist.",
                    name.clone()
                )),
            ));
        }
    }
    Ok(axum::Json(ResponseBody::ok(am.update(&state.db).await?)))
}

pub async fn delete_taxonomy(
    _claims: Claims,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
    t_type: TaxonomyType,
) -> HandlerResult<()> {
    taxonomy::Entity::delete_by_id(id)
        .filter(taxonomy::Column::TType.eq(t_type))
        .exec(&state.db)
        .await?;
    Ok(axum::Json(ResponseBody::ok(())))
}
