use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "comments")]
pub struct Model {
    #[serde(skip_deserializing)]
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(created_at)]
    pub created: DateTime<Utc>,
    #[sea_orm(updated_at)]
    pub modified: DateTime<Utc>,
    pub hidden: bool,
    pub nickname: String,
    #[sea_orm(indexed)]
    pub email: String,
    #[sea_orm(nullable)]
    pub site: Option<String>,
    pub parent: i64,
    pub post_id: i64,
    #[serde(skip_serializing_if = "Option::is_none", skip_deserializing)]
    #[sea_orm(ignore)]
    pub children: Option<Vec<Model>>,
    #[sea_orm(nullable)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::post::Entity",
        from = "Column::PostId",
        to = "super::post::Column::Id"
    )]
    Post,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
