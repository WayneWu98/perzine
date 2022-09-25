use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "comments")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[serde(skip_deserializing)]
    #[sea_orm(primary_key)]
    pub id: i64,
    #[serde(skip_deserializing)]
    pub created: Option<DateTime<Utc>>,
    pub modified: Option<DateTime<Utc>>,
    pub status: Option<CommentStatus>,
    pub nickname: Option<String>,
    pub email: Option<String>,
    #[sea_orm(nullable)]
    pub site: Option<String>,
    pub content: Option<String>,
    pub parent: Option<i64>,
    pub post_id: Option<i64>,
    #[serde(skip_deserializing)]
    pub role: super::UserRole,
    #[sea_orm(ignore)]
    #[serde(skip_deserializing)]
    pub children: Option<Vec<Model>>,
}

#[derive(
    Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize_enum_str, Deserialize_enum_str,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "COMMENT_STATUS")]
#[serde(rename_all = "camelCase")]
pub enum CommentStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "published")]
    Published,
    #[sea_orm(string_value = "hidden")]
    Hidden,
    #[sea_orm(string_value = "trashed")]
    Trashed,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::post::Entity",
        from = "Column::PostId",
        to = "super::post::Column::Id"
    )]
    Post,
    #[sea_orm(belongs_to = "Entity", from = "Column::Parent", to = "Column::Id")]
    Comment,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
