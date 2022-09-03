use deadpool_postgres::Pool;
use std::{collections::HashMap, error::Error};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

pub static PrivateOptions: [&'static str; 1] = ["password"];

#[derive(PostgresMapper, Debug, Clone)]
#[pg_mapper(table = "options")]
pub struct SiteOption {
    id: i32,
    pub name: String,
    pub value: String,
}

impl SiteOption {
    pub fn id(&self) -> i32 {
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
    pub async fn all(pool: &Pool) -> Result<Vec<Self>, Box<dyn Error>> {
        let client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM \"options\"").await?;
        let rows = client
            .query(&stmt, &[])
            .await?
            .into_iter()
            .map(|row| Self::from_row(row))
            .filter(|res| res.is_ok())
            .map(|res| res.unwrap())
            .collect::<Vec<Self>>();
        Ok(rows)
    }
    pub fn is_private(&self) -> bool {
        PrivateOptions.contains(&&self.name[..])
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

pub fn map(options: Vec<SiteOption>) -> HashMap<String, String> {
    let mut opts = HashMap::new();
    options.iter().for_each(|opt| {
        opts.insert(opt.name.clone(), opt.value.clone());
    });
    opts
}

pub fn filter_publics(options: Vec<SiteOption>) -> Vec<SiteOption> {
    options
        .into_iter()
        .filter(|opt| !opt.is_private())
        .collect::<Vec<SiteOption>>()
}
