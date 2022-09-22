use std::{collections::HashMap, sync::Arc};

use axum::Extension;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use crate::entity::site_option::{self, OptionLevel, Utils};
use crate::extract::{Claims, JsonPayload, WeekClaims};
use crate::{
    core::{error::ErrorCode, response::HandlerResult, AppState},
    extract::Path,
};
use crate::{e_code_err, res_ok};

pub async fn get_options(
    wc: WeekClaims,
    Extension(state): Extension<Arc<AppState>>,
) -> HandlerResult<HashMap<String, String>> {
    let opts = site_option::Entity::find().all(&state.db).await?;
    let filtered = if wc.is_authed() {
        opts.exclude_private()
    } else {
        opts.filter_public()
    };
    res_ok!(filtered.to_map())
}

pub async fn get_option(
    week_claims: WeekClaims,
    Extension(state): Extension<Arc<AppState>>,
    Path(name): Path<String>,
) -> HandlerResult<String> {
    let opt = site_option::Entity::find()
        .filter(
            site_option::Column::Name
                .eq(name)
                .and(site_option::Column::Level.ne(OptionLevel::Private)),
        )
        .one(&state.db)
        .await?;
    match opt {
        Some(opt) => {
            if opt.is_protected() && !week_claims.is_authed() {
                return e_code_err!(ErrorCode::NotFound);
            }
            res_ok!(opt.value)
        }
        None => e_code_err!(ErrorCode::NotFound),
    }
}

pub async fn update_options(
    _claims: Claims,
    Extension(state): Extension<Arc<AppState>>,
    JsonPayload(opts): JsonPayload<HashMap<String, String>>,
) -> HandlerResult<HashMap<String, String>> {
    let mut updated = Vec::new();
    for (k, v) in opts.iter() {
        let old = site_option::Entity::find()
            .filter(site_option::Column::Name.eq(k.clone()))
            .one(&state.db)
            .await?;

        if let Some(opt) = old {
            let mut opt: site_option::ActiveModel = opt.into();
            opt.value = sea_orm::ActiveValue::Set(v.clone());
            updated.push(opt.update(&state.db).await?);
        }
    }
    res_ok!(updated.exclude_private().to_map())
}
