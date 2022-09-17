use sea_orm::{entity::prelude::*, ConnectionTrait};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "taxonomy")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(nullable)]
    pub description: Option<String>,
    #[sea_orm(nullable)]
    pub cover: Option<String>,
    #[serde(skip)]
    #[sea_orm(column_name = "type", default_value = TaxonomyType::CATEGORY, indexed)]
    pub t_type: TaxonomyType,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "TAXONOMY_TYPE")]
pub enum TaxonomyType {
    #[sea_orm(string_value = "category")]
    CATEGORY,
    #[sea_orm(string_value = "tag")]
    TAG,
    #[sea_orm(string_value = "series")]
    SERIES,
}

impl Default for TaxonomyType {
    fn default() -> Self {
        Self::CATEGORY
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        super::post_taxonomy::Relation::Post.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::post_taxonomy::Relation::Taxonomy.def().rev())
    }
}

impl Model {
    pub async fn is_exist<C: ConnectionTrait>(
        id: i32,
        t_type: TaxonomyType,
        db: &C,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(Entity::find_by_id(id)
            .filter(Column::TType.eq(t_type))
            .one(db)
            .await?
            .is_some())
    }
}
