use sea_orm::entity::prelude::*;
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

impl Model {
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
}

pub struct ClassifiedTaxonomy {
    pub categories: Vec<Model>,
    pub tags: Vec<Model>,
    pub series: Vec<Model>,
}

impl ClassifiedTaxonomy {
    pub fn new() -> Self {
        Self {
            categories: Vec::new(),
            tags: Vec::new(),
            series: Vec::new(),
        }
    }
}

pub(crate) trait ClassifyTaxonomy {
    fn classify(self) -> ClassifiedTaxonomy;
}

impl ClassifyTaxonomy for Vec<Model> {
    fn classify(self) -> ClassifiedTaxonomy {
        let mut classified = ClassifiedTaxonomy::new();
        for model in self {
            match model.t_type.clone() {
                TaxonomyType::Category => classified.categories.push(model),
                TaxonomyType::Tag => classified.tags.push(model),
                TaxonomyType::Series => classified.series.push(model),
            }
        }
        classified
    }
}
