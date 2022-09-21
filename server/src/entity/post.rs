use chrono::{DateTime, Utc};
use sea_orm::{entity::prelude::*, IntoActiveValue};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

use super::taxonomy::{self};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[serde(skip_deserializing)]
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    #[sea_orm(nullable)]
    pub subtitle: Option<String>,
    #[serde(skip)]
    #[sea_orm(created_at)]
    pub created: Option<DateTime<Utc>>,
    #[sea_orm(updated_at)]
    pub modified: Option<DateTime<Utc>>,
    #[sea_orm(created_at)]
    pub published: Option<DateTime<Utc>>,
    #[serde(skip_deserializing)]
    #[sea_orm(ignore)]
    pub categories: Option<Vec<taxonomy::Model>>,
    #[serde(skip_deserializing)]
    #[sea_orm(ignore)]
    pub tags: Option<Vec<taxonomy::Model>>,
    #[serde(skip_deserializing)]
    #[sea_orm(ignore)]
    pub series: Option<taxonomy::Model>,
    #[sea_orm(nullable)]
    pub excerpts: Option<String>,
    #[sea_orm(nullable)]
    pub content: Option<serde_json::Value>,
    #[sea_orm(nullable, default_value = Option::None)]
    pub route: Option<String>,
    #[sea_orm(default_value = false)]
    pub is_page: Option<bool>,
    pub status: Option<PostStatus>,
    #[sea_orm(nullable)]
    pub extra: Option<serde_json::Value>,
}

#[derive(
    Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize_enum_str, Deserialize_enum_str,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "POST_STATUS")]
#[serde(rename_all = "camelCase")]
pub enum PostStatus {
    #[sea_orm(string_value = "draft")]
    Draft,
    #[sea_orm(string_value = "published")]
    Published,
    #[sea_orm(string_value = "hidden")]
    Hidden,
    #[sea_orm(string_value = "trashed")]
    Trashed,
}

impl IntoActiveValue<PostStatus> for PostStatus {
    fn into_active_value(self) -> sea_orm::ActiveValue<PostStatus> {
        sea_orm::ActiveValue::Set(self)
    }
}

impl Default for PostStatus {
    fn default() -> Self {
        Self::Draft
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::comment::Entity")]
    Comment,
}

impl Related<super::taxonomy::Entity> for Entity {
    fn to() -> RelationDef {
        super::post_taxonomy::Relation::Taxonomy.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::post_taxonomy::Relation::Post.def().rev())
    }
}

impl Related<super::comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, Serialize, DeriveModel, Deserialize)]
pub struct VisitorSimplePost {
    pub id: i64,
    pub title: String,
    pub subtitle: Option<String>,
    pub published: DateTime<Utc>,
    pub excerpts: Option<String>,
    pub extra: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, DeriveModel, Deserialize)]
pub struct ManagerSimplePost {
    pub id: i64,
    pub title: String,
    pub subtitle: Option<String>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub published: DateTime<Utc>,
    pub excerpts: Option<String>,
    pub route: Option<String>,
    pub is_page: bool,
    pub status: PostStatus,
    pub extra: Option<serde_json::Value>,
}
