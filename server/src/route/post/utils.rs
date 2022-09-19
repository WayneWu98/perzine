use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_enum_str::Deserialize_enum_str;

use crate::{
    entity::{post, taxonomy},
    utils::SqlOrder,
};

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

#[derive(Debug, Deserialize_enum_str)]
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

#[derive(Serialize)]
pub struct PostRes {
    #[serde(flatten)]
    pub post: post::Model,
    pub categories: Vec<taxonomy::Model>,
    pub tags: Vec<taxonomy::Model>,
    pub series: Option<taxonomy::Model>,
}

impl PostRes {
    pub fn new(post: post::Model) -> Self {
        Self {
            post,
            categories: Vec::with_capacity(0),
            tags: Vec::with_capacity(0),
            series: None,
        }
    }
    pub fn with_categories(mut self, categories: Vec<taxonomy::Model>) -> Self {
        self.categories = categories;
        self
    }
    pub fn with_tags(mut self, tags: Vec<taxonomy::Model>) -> Self {
        self.tags = tags;
        self
    }
    pub fn with_series(mut self, series: Option<taxonomy::Model>) -> Self {
        self.series = series;
        self
    }
}
