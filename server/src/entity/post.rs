use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::{self, NotSet};
use sea_orm::IntoActiveModel;
use sea_orm::{entity::prelude::*, ConnectionTrait, IntoActiveValue};
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

use super::taxonomy::{self, ClassifiedTaxonomy, ClassifyTaxonomy};

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
    pub created: DateTime<Utc>,
    #[sea_orm(updated_at)]
    pub modified: DateTime<Utc>,
    #[sea_orm(created_at)]
    pub published: DateTime<Utc>,
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
    pub excerpts: String,
    #[sea_orm(nullable)]
    pub content: serde_json::Value,
    #[sea_orm(nullable)]
    pub route: Option<String>,
    #[sea_orm]
    pub is_page: bool,
    pub status: PostStatus,
    #[sea_orm(nullable)]
    pub extra: Option<serde_json::Value>,
}

impl Model {
    pub async fn with_taxonomy(
        mut self,
        db: &impl ConnectionTrait,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let ClassifiedTaxonomy {
            categories,
            tags,
            mut series,
        } = self
            .find_related(taxonomy::Entity)
            .all(db)
            .await?
            .classify();
        self.categories = Some(categories);
        self.tags = Some(tags);
        self.series = series.pop();
        Ok(self)
    }
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

#[derive(Deserialize, Serialize)]
pub struct NewPost {
    pub title: String,
    // pub subtitle: Option<String>,
    pub modified: Option<DateTime<Utc>>,
    // pub published: Option<DateTime<Utc>>,
    // pub categories: Option<Vec<i32>>,
    // pub tags: Option<i32>,
    // pub series: Option<i32>,
    // pub excerpts: Option<String>,
    // pub content: Option<serde_json::Value>,
    // pub is_page: Option<bool>,
    // pub status: Option<PostStatus>,
    // pub extra: Option<serde_json::Value>,
}

impl NewPost {
    pub fn into_active_model(self) -> Result<ActiveModel, Box<dyn std::error::Error>> {
        Ok(ActiveModel::from_json(serde_json::to_value(self)?)?)
    }
}
