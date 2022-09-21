use sea_orm::{entity::prelude::*, ConnectionTrait};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "taxonomy")]
pub struct Model {
    #[serde(skip_deserializing)]
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(nullable)]
    pub description: Option<String>,
    #[sea_orm(nullable)]
    pub cover: Option<String>,
    #[serde(skip)]
    #[sea_orm(column_name = "type", indexed)]
    pub t_type: TaxonomyType,
    #[sea_orm(nullable)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "TAXONOMY_TYPE")]
pub enum TaxonomyType {
    #[sea_orm(string_value = "category")]
    Category,
    #[sea_orm(string_value = "tag")]
    Tag,
    #[sea_orm(string_value = "series")]
    Series,
}

impl sea_orm::IntoActiveValue<TaxonomyType> for TaxonomyType {
    fn into_active_value(self) -> sea_orm::ActiveValue<TaxonomyType> {
        sea_orm::ActiveValue::Set(self)
    }
}

impl Default for TaxonomyType {
    fn default() -> Self {
        Self::Category
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

pub async fn is_exist_in_name(
    name: String,
    t_type: TaxonomyType,
    db: &DatabaseConnection,
) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(Entity::find()
        .filter(Column::Name.eq(name))
        .filter(Column::TType.eq(t_type))
        .one(db)
        .await?
        .is_some())
}
pub async fn is_exist_in_id(
    id: i32,
    t_type: TaxonomyType,
    db: &DatabaseConnection,
) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(Entity::find()
        .filter(Column::Id.eq(id))
        .filter(Column::TType.eq(t_type))
        .one(db)
        .await?
        .is_some())
}
pub async fn is_valid_taxonomy(
    db: &impl ConnectionTrait,
    ids: Vec<i32>,
    t_type: TaxonomyType,
) -> Result<bool, Box<dyn std::error::Error>> {
    let len = ids.len();
    let finded = Entity::find().filter(Column::Id.is_in(ids)).all(db).await?;
    if finded.len() < len {
        return Ok(false);
    }
    Ok(finded.into_iter().all(|item| item.t_type.eq(&t_type)))
}
