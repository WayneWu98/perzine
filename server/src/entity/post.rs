use chrono::{DateTime, Utc};
use sea_orm::{entity::prelude::*, ConnectionTrait, IntoActiveValue};
use serde::Deserialize;
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

use super::{comment, taxonomy};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize)]
#[sea_orm(table_name = "posts")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[serde(skip_deserializing)]
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    #[sea_orm(nullable)]
    pub subtitle: Option<String>,
    #[serde(skip)]
    pub created: Option<DateTime<Utc>>,
    pub modified: Option<DateTime<Utc>>,
    pub published: Option<DateTime<Utc>>,
    #[sea_orm(nullable)]
    pub excerpts: Option<String>,
    #[sea_orm(nullable)]
    pub content: Option<serde_json::Value>,
    #[sea_orm(nullable, unique, indexed, default_value = Option::None)]
    pub route: Option<String>,
    #[sea_orm(default_value = false)]
    pub is_page: Option<bool>,
    pub status: Option<PostStatus>,
    #[sea_orm(nullable)]
    pub extra: Option<serde_json::Value>,
    #[sea_orm(ignore)]
    #[serde(skip)]
    pub comment_count: usize,
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

impl Model {
    pub async fn txs(&self, db: &impl ConnectionTrait) -> Result<Vec<taxonomy::Model>, DbErr> {
        Ok(self.find_related(taxonomy::Entity).all(db).await?)
    }
    pub async fn comment_count(&self, db: &impl ConnectionTrait) -> Result<usize, DbErr> {
        Ok(self.find_related(comment::Entity).count(db).await?)
    }
}
