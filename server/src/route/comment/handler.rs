use std::{cmp::Ordering, sync::Arc};

use axum::{
    extract::{Path, Query},
    Extension,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder,
};
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        error::ErrorCode,
        response::{HandlerResult, PaginationData},
        AppState,
    },
    dto, e_code_err,
    entity::{
        comment::{self, CommentStatus},
        site_option, UserRole,
    },
    extract::{Claims, JsonPayload, Pagination, WeekClaims},
    res_ok,
};

use super::utils::list_children;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisitorQuery {
    pub post_id: i64,
}

pub async fn get_comments(
    Extension(state): Extension<Arc<AppState>>,
    Pagination { page, per }: Pagination,
    Query(VisitorQuery { post_id }): Query<VisitorQuery>,
) -> HandlerResult<impl Serialize> {
    let cond = sea_orm::Condition::all()
        .add(comment::Column::PostId.eq(post_id))
        .add(comment::Column::Status.eq(comment::CommentStatus::Published));
    let total = comment::Entity::find()
        .filter(cond.clone())
        .count(&state.db)
        .await?;
    let paginator = comment::Entity::find()
        .filter(cond.add(comment::Column::Parent.is_null()))
        .order_by_desc(comment::Column::Created)
        .paginate(&state.db, per);

    let parents = paginator.fetch_page(page).await?;
    let mut items = Vec::new();

    for mut parent in parents.into_iter() {
        let children = list_children(parent.id, &state.db).await?;
        parent.children = Some(children);
        items.push(parent);
    }

    res_ok!(PaginationData { total, items })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuery {
    pub post_id: i64,
}

pub async fn create_comment(
    w_claims: WeekClaims,
    Extension(state): Extension<Arc<AppState>>,
    JsonPayload(jv): JsonPayload<serde_json::Value>,
    Query(CreateQuery { post_id }): Query<CreateQuery>,
) -> HandlerResult<impl Serialize> {
    let mut am = comment::ActiveModel::from_json(jv.clone())?;
    let email = am
        .email
        .clone()
        .take()
        .unwrap_or(None)
        .unwrap_or("".to_owned());
    am.post_id = ActiveValue::Set(Some(post_id));
    if !w_claims.is_authed() {
        let passed = comment::Entity::find()
            .filter(
                comment::Column::Email
                    .eq(email.clone())
                    .and(comment::Column::Status.eq(comment::CommentStatus::Published)),
            )
            .one(&state.db)
            .await?;
        let status = if passed.is_some() {
            comment::CommentStatus::Published
        } else {
            comment::CommentStatus::Pending
        };
        am.status = ActiveValue::Set(Some(status));
        am.role = ActiveValue::Set(UserRole::Visitor);
    } else {
        am.role = ActiveValue::Set(UserRole::Visitor);
        let opt = site_option::Entity::find()
            .filter(site_option::Column::Name.eq("email".to_owned()))
            .one(&state.db)
            .await?;
        if let Some(opt) = opt {
            match email.clone().cmp(&opt.value) {
                Ordering::Equal => am.role = ActiveValue::Set(UserRole::Manager),
                _ => am.role = ActiveValue::Set(UserRole::Visitor),
            }
        }
        am.status = ActiveValue::Set(Some(comment::CommentStatus::Published));
    }
    let m = am.insert(&state.db).await?;
    res_ok!(dto::comment::Comment::from(m))
}

pub async fn reply_comment(
    w_claims: WeekClaims,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
    JsonPayload(jv): JsonPayload<serde_json::Value>,
) -> HandlerResult<impl Serialize> {
    let parent = comment::Entity::find_by_id(id).one(&state.db).await?;
    if let Some(parent) = parent {
        let mut am = comment::ActiveModel::from_json(jv.clone())?;
        am.parent = ActiveValue::Set(Some(parent.id));
        am.post_id = ActiveValue::Set(parent.post_id);
        am.role = ActiveValue::Set(UserRole::Visitor);

        let email = am
            .email
            .clone()
            .take()
            .unwrap_or(None)
            .unwrap_or("".to_owned());
        if w_claims.is_authed() {
            if am.status.is_not_set() {
                am.status = ActiveValue::Set(Some(comment::CommentStatus::Published));
            }
            let opt = site_option::Entity::find()
                .filter(site_option::Column::Name.eq("email".to_owned()))
                .one(&state.db)
                .await?;
            if let Some(opt) = opt {
                match email.clone().cmp(&opt.value) {
                    Ordering::Equal => am.role = ActiveValue::Set(UserRole::Manager),
                    _ => am.role = ActiveValue::Set(UserRole::Visitor),
                }
            }
        } else {
            let passed = comment::Entity::find()
                .filter(
                    comment::Column::Email
                        .eq(email.clone())
                        .and(comment::Column::Status.eq(comment::CommentStatus::Published)),
                )
                .one(&state.db)
                .await?;
            let status = if let Some(_) = passed {
                CommentStatus::Published
            } else {
                CommentStatus::Pending
            };
            am.status = ActiveValue::Set(Some(status));
        }

        let item = am.insert(&state.db).await?;

        return res_ok!(dto::comment::Comment::from(item));
    }
    e_code_err!(
        ErrorCode::NotFound,
        Some("the reply target does not exist.".to_owned())
    )
}

pub async fn update_comment(
    _claims: Claims,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
    JsonPayload(jv): JsonPayload<serde_json::Value>,
) -> HandlerResult<impl Serialize> {
    let mut am = comment::ActiveModel::from_json(jv.clone())?;
    am.id = ActiveValue::Set(id);
    am.modified = ActiveValue::Set(Some(Utc::now()));
    let m = am.update(&state.db).await?;
    res_ok!(dto::comment::Comment::from(m))
}
