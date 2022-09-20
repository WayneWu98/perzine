use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[serde(skip_deserializing)]
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    #[sea_orm(nullable)]
    pub subtitle: String,
    #[serde(skip)]
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub published: DateTime<Utc>,
    #[serde(skip_serializing)]
    #[sea_orm(ignore)]
    pub categories: Vec<i32>,
    #[serde(skip_serializing)]
    #[sea_orm(ignore)]
    pub tags: Vec<i32>,
    #[serde(skip_serializing)]
    #[sea_orm(ignore)]
    pub series: Option<i32>,
    #[sea_orm(nullable)]
    pub excerpts: String,
    pub content: serde_json::Value,
    #[sea_orm(nullable)]
    pub route: String,
    #[serde(skip)]
    pub is_page: bool,
    #[serde(skip)]
    pub status: PostStatus,
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

impl Default for PostStatus {
    fn default() -> Self {
        Self::Draft
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::taxonomy::Entity> for Entity {
    fn to() -> RelationDef {
        super::post_taxonomy::Relation::Taxonomy.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::post_taxonomy::Relation::Post.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
