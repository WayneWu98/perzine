use deadpool_postgres::Pool;
use std::error::Error;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

use super::{Pagination, SQLFilter};

#[derive(Debug, Clone, Copy)]
enum Status {
    PUBLISHED,
    UNPUBLISHED,
    HIDDEN,
}

#[derive(PostgresMapper, Debug, Clone)]
#[pg_mapper(table = "posts")]
pub struct Post {
    id: u32,
    pub created: String,
    pub modified: String,
    pub content: String,
    pub category: i32,
    pub series: Vec<i32>,
    pub tags: Vec<i32>,
    // pub status: Status,
    pub title: String,
    pub sub_title: String,
}

impl Post {
    pub fn id(&self) -> u32 {
        self.id
    }
    pub async fn from_id(pool: &Pool, id: i32) -> Result<Self, Box<dyn Error>> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(&"SELECT * FROM \"posts\" WHERE id=$1".to_string())
            .await?;
        let row = client.query_one(&stmt, &[&id]).await?;
        Ok(Self::from_row(row)?)
    }

    pub async fn list(
        pool: &Pool,
        p: Pagination,
        sql_filter: SQLFilter,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                &"SELECT * FROM \"posts\" WHERE {$1} ORDER BY $2 $3 LIMIT $4 OFFSET $5".to_string(),
            )
            .await?;
        let rows = client
            .query(
                &stmt,
                &[
                    &sql_filter.where_str,
                    &p.order_by,
                    &p.order(),
                    &p.per,
                    &p.offset(),
                ],
            )
            .await?
            .into_iter()
            .map(|row| Self::from_row(row))
            .filter(|res| res.is_ok())
            .map(|res| res.unwrap())
            .collect();
        Ok(rows)
    }
}
