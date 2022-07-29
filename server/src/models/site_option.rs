use deadpool_postgres::Pool;
use std::{collections::HashMap, error::Error};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(PostgresMapper, Debug, Clone)]
#[pg_mapper(table = "options")]
pub struct SiteOption {
    id: u32,
    pub name: String,
    pub value: String,
}

impl SiteOption {
    pub fn id(&self) -> u32 {
        self.id
    }
    pub async fn from_name(pool: &Pool, name: &str) -> Result<Self, Box<dyn Error>> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("SELECT * FROM \"options\" WHERE \"name\"=$1")
            .await?;
        let row = client.query_one(&stmt, &[&name]).await?;
        Ok(Self::from_row(row)?)
    }
    pub async fn all(pool: &Pool) -> Result<HashMap<String, Self>, Box<dyn Error>> {
        let client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM \"options\"").await?;
        let mut options = HashMap::new();
        let rows = client
            .query(&stmt, &[])
            .await?
            .into_iter()
            .map(|row| Self::from_row(row))
            .filter(|res| res.is_ok())
            .map(|res| res.unwrap())
            .collect::<Vec<Self>>();
        for row in rows {
            options.insert(row.name.clone(), row);
        }
        Ok(options)
    }
    pub async fn save(&self, pool: &Pool) -> Result<bool, Box<dyn Error>> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(&format!(
                "UPDATE \"options\" SET \"value\"=$1 WHERE \"id\"={}",
                self.id
            ))
            .await?;
        let effected = client.execute(&stmt, &[&self.value]).await?;
        Ok(if effected >= 1 { true } else { false })
    }
}
