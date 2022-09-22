use chrono::Utc;
use sea_orm::ColumnTrait;
use serde::Deserialize;
use serde_enum_str::Deserialize_enum_str;

use crate::{entity::post, utils::SqlOrder};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub all: Option<String>,
    pub created_from: Option<chrono::DateTime<Utc>>,
    pub created_to: Option<chrono::DateTime<Utc>>,
    pub modified_from: Option<chrono::DateTime<Utc>>,
    pub modified_to: Option<chrono::DateTime<Utc>>,
    pub published_from: Option<chrono::DateTime<Utc>>,
    pub published_to: Option<chrono::DateTime<Utc>>,
    pub status: Option<post::PostStatus>,
    pub keyword: Option<String>,
    pub order_by: Option<OrderKey>,
    pub order: Option<SqlOrder>,
}

impl Filter {
    pub fn condition(&self, is_authed: bool) -> sea_orm::Condition {
        let mut cond = sea_orm::Condition::all();
        if let Some(v) = self.keyword.clone() {
            cond = cond.add(post::Column::Title.like(&v));
        }
        if let Some(v) = self.modified_from {
            cond = cond.add(post::Column::Modified.gte(v));
        }
        if let Some(v) = self.modified_to {
            cond = cond.add(post::Column::Modified.lte(v));
        }
        if is_authed {
            if let Some(v) = self.created_from {
                cond = cond.add(post::Column::Created.gte(v));
            }
            if let Some(v) = self.created_to {
                cond = cond.add(post::Column::Created.lte(v));
            }
            if let Some(v) = self.published_from {
                cond = cond.add(post::Column::Published.gte(v));
            }
            if let Some(v) = self.published_to {
                cond = cond.add(post::Column::Published.lte(v));
            }
            if self.all.is_none() {
                cond = cond.add(post::Column::IsPage.eq(false));
            }
            if let Some(v) = self.status.clone() {
                cond = cond.add(post::Column::Status.eq(v));
            }
        } else {
            cond = cond
                .add(post::Column::Published.lte(chrono::Utc::now()))
                .add(post::Column::Status.eq(post::PostStatus::Published))
                .add(post::Column::IsPage.eq(false));
        };
        cond
    }

    pub fn order_by(&self, is_authed: bool) -> post::Column {
        self.order_by.clone().unwrap_or_default().column(is_authed)
    }

    pub fn order(&self) -> sea_orm::Order {
        self.order
            .clone()
            .map_or(sea_orm::Order::Desc, |v| v.order())
    }
}

#[derive(Debug, Clone, Deserialize_enum_str)]
#[serde(rename_all = "camelCase")]
pub enum OrderKey {
    Created,
    Modified,
    Published,
}

impl OrderKey {
    pub fn column(&self, authed: bool) -> post::Column {
        match self {
            OrderKey::Published => post::Column::Published,
            OrderKey::Modified => post::Column::Modified,
            OrderKey::Created => {
                if authed {
                    post::Column::Created
                } else {
                    post::Column::Published
                }
            }
        }
    }
}

impl Default for OrderKey {
    fn default() -> Self {
        Self::Published
    }
}
