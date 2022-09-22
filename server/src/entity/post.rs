use chrono::{DateTime, Utc};
use sea_orm::{entity::prelude::*, IntoActiveValue};
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize)]
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
    pub modified: Option<DateTime<Utc>>,
    #[sea_orm(created_at)]
    pub published: Option<DateTime<Utc>>,
    #[sea_orm(nullable)]
    pub excerpts: Option<String>,
    #[sea_orm(nullable)]
    pub content: Option<serde_json::Value>,
    #[sea_orm(nullable, unique, indexed, default_value = Option::None)]
    pub route: Option<String>,
    #[serde(rename(deserialize = "isPage"), alias = "isPage")]
    #[sea_orm(default_value = false)]
    pub is_page: Option<bool>,
    pub status: Option<PostStatus>,
    #[sea_orm(nullable)]
    pub extra: Option<serde_json::Value>,
    #[serde(skip)]
    #[sea_orm(ignore)]
    pub is_authed: bool,
    #[serde(skip_deserializing)]
    #[sea_orm(ignore)]
    pub fulled: bool,
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

impl Serialize for Model {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Model", 12)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("subtitle", &self.subtitle)?;
        state.serialize_field("published", &self.published)?;
        state.serialize_field("excerpts", &self.excerpts)?;
        state.serialize_field("extra", &self.extra)?;
        if self.fulled {
            state.serialize_field("content", &self.content)?;
        }
        if self.is_authed {
            state.serialize_field("created", &self.created)?;
            state.serialize_field("modified", &self.modified)?;
            state.serialize_field("route", &self.route)?;
            state.serialize_field("isPage", &self.is_page)?;
            state.serialize_field("status", &self.status)?;
        }
        state.end()
    }
}
