use super::taxonomy;
use sea_orm::prelude::*;
use sea_orm::ColumnTrait;
use sea_orm::ConnectionTrait;
use sea_orm::DbErr;
use sea_orm::IntoActiveModel;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "post_taxonomy")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub post_id: i64,
    pub taxonomy_id: i32,
    pub taxonomy_type: taxonomy::TaxonomyType,
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
#[derive(DeriveIntoActiveModel)]
pub struct UpdatePayload {
    pub post_id: i64,
    pub taxonomy_id: i32,
    pub taxonomy_type: taxonomy::TaxonomyType,
}

impl UpdatePayload {
    pub fn new(post_id: i64, taxonomy_id: i32, taxonomy_type: taxonomy::TaxonomyType) -> Self {
        Self {
            post_id,
            taxonomy_id,
            taxonomy_type,
        }
    }
}

pub async fn update(
    db: &impl ConnectionTrait,
    pid: i64,
    tids: Vec<i32>,
    t_type: taxonomy::TaxonomyType,
) -> Result<(), DbErr> {
    if tids.len() < 1 {
        return Ok(());
    }
    let is_valid = taxonomy::is_valid_taxonomy(db, tids.clone(), t_type.clone())
        .await
        .map_err(|err| DbErr::Query(err.to_string()))?;
    if !is_valid {
        return Err(DbErr::Custom("invalid taxonomy".to_owned()));
    }
    Entity::delete_many()
        .filter(
            Column::TaxonomyType
                .eq(t_type.clone())
                .and(Column::PostId.eq(pid)),
        )
        .exec(db)
        .await?;
    let items: Vec<ActiveModel> = tids
        .clone()
        .into_iter()
        .map(|tid| UpdatePayload::new(pid, tid, t_type.clone()).into_active_model())
        .collect();
    Entity::insert_many(items).exec(db).await?;
    Ok(())
}

impl ActiveModelBehavior for ActiveModel {}
