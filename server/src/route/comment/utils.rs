use async_recursion::async_recursion;
use chrono::Utc;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

use crate::entity::comment;

#[async_recursion]
pub async fn list_children(
    parent: i64,
    db: &impl ConnectionTrait,
) -> Result<Vec<comment::Model>, Box<dyn std::error::Error>> {
    let wraps = comment::Entity::find()
        .filter(
            comment::Column::Parent
                .eq(parent)
                .and(comment::Column::Created.lte(Utc::now())),
        )
        .all(db)
        .await?;
    let mut formatted = Vec::new();

    for child in wraps.into_iter() {
        let children = list_children(child.id, db).await?;
        formatted.push(child);
        formatted.extend(children.into_iter());
    }

    Ok(formatted)
}
