use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "post_taxonomy")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub post_id: i64,
    pub taxonomy_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::post::Entity",
        from = "Column::PostId",
        to = "super::post::Column::Id"
    )]
    Post,
    #[sea_orm(
        belongs_to = "super::taxonomy::Entity",
        from = "Column::TaxonomyId",
        to = "super::taxonomy::Column::Id"
    )]
    Taxonomy,
}

impl ActiveModelBehavior for ActiveModel {}
