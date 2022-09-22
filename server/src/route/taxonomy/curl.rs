use crate::{
    core::{
        error::ErrorCode,
        response::{HandlerResult, PaginationData},
        AppState,
    },
    e_code_err,
    extract::{Claims, JsonPayload, Pagination, Path},
    res_ok,
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
    res_ok!(PaginationData::new(items, total))
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
        Some(item) => res_ok!(item),
        None => e_code_err!(ErrorCode::NotFound),
    }
}

pub async fn create_taxonomy(
    _claims: Claims,
    Extension(state): Extension<Arc<AppState>>,
    JsonPayload(jv): JsonPayload<serde_json::Value>,
    t_type: TaxonomyType,
) -> HandlerResult<Taxonomy> {
    println!("{:?}", jv);
    let mut am = taxonomy::ActiveModel::from_json(jv)?;
    let name: String = am.name.clone().take().unwrap_or("".to_owned());
    if taxonomy::is_exist_in_name(name.clone(), t_type.clone(), &state.db).await? {
        return e_code_err!(
            ErrorCode::InvalidRequest,
            Some(format!("the taxonomy named \"{}\" has been exist.", name))
        );
    }

    am.t_type = sea_orm::ActiveValue::Set(t_type.clone());

    res_ok!(am.insert(&state.db).await?)
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
        return e_code_err!(ErrorCode::NotFound);
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
            return e_code_err!(
                ErrorCode::InvalidRequest,
                Some(format!(
                    "the taxonomy named \"{}\" has been exist.",
                    name.clone()
                ),)
            );
        }
    }
    res_ok!(am.update(&state.db).await?)
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
    res_ok!(())
}
