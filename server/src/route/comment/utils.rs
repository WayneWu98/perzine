use async_recursion::async_recursion;
use chrono::Utc;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder};

use crate::entity::comment;

#[async_recursion]
pub async fn list_children(
    parent: &comment::Model,
    db: &impl ConnectionTrait,
    order: sea_orm::Order,
) -> Result<Vec<comment::Model>, Box<dyn std::error::Error>> {
    let wraps = comment::Entity::find()
        .filter(
            comment::Column::Parent
                .eq(parent.id)
                .and(comment::Column::Created.lte(Utc::now())),
        )
        .order_by(comment::Column::Created, order.clone())
        .all(db)
        .await?;
    let mut formatted = Vec::new();

    for mut child in wraps.into_iter() {
        let children = list_children(&child, db, order.clone()).await?;
        child.formatted_parent = Some(Box::new(parent.clone()));
        formatted.push(child);
        formatted.extend(children.into_iter());
    }

    Ok(formatted)
}
