use deadpool_postgres::Pool;
use once_cell::sync::Lazy;
use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Serialize,
};
use std::{collections::HashMap, error::Error};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::types::FromSql;

use super::{Pagination, SQLFilter};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Status {
    PUBLISHED,
    UNPUBLISHED,
    HIDDEN,
}

static STATUS_MAP: [(Status, &str); 3] = [
    (Status::PUBLISHED, "published"),
    (Status::UNPUBLISHED, "unpublished"),
    (Status::HIDDEN, "hidden"),
];

static STATUS_2_STR_MAP: Lazy<HashMap<Status, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for (k, v) in STATUS_MAP.into_iter() {
        map.insert(k, v);
    }
    map
});

static STR_2_STATUS_MAP: Lazy<HashMap<&str, Status>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for (v, k) in STATUS_MAP.into_iter() {
        map.insert(k, v);
    }
    map
});

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match STATUS_2_STR_MAP.get(self) {
            Some(v) => v.serialize(serializer),
            None => Err(serde::ser::Error::custom("serialize fail")),
        }
    }
}

struct StatusVistor;

impl<'de> Visitor<'de> for StatusVistor {
    type Value = Status;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut v: Vec<&str> = Vec::with_capacity(STATUS_MAP.len());
        let s: Vec<&str> = STATUS_MAP.into_iter().map(|(_, v)| v).collect();
        formatter.write_str(&format!("a string in one of: {}", s.join(", ")))
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match STR_2_STATUS_MAP.get(v) {
            Some(v) => Ok(*v),
            _ => Err(E::invalid_value(Unexpected::Str(v), &self)),
        }
    }
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(StatusVistor)
    }
}

impl<'a> FromSql<'a> for Status {
    fn accepts(ty: &tokio_postgres::types::Type) -> bool {
        match ty {
            &tokio_postgres::types::Type::TEXT => true,
            _ => false,
        }
    }

    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let s = String::from_utf8(raw.to_vec())?;
        match STR_2_STATUS_MAP.get(&s[..]) {
            Some(status) => Ok(*status),
            None => Err(Box::new(crate::utils::error::AppError {
                msg: Some("translate SQLValue to RustValue fail.".to_string()),
                source: None,
            })),
        }
    }
}

#[derive(PostgresMapper, Debug, Clone)]
#[pg_mapper(table = "posts")]
pub struct Post {
    id: u32,
    pub created: String,
    pub modified: String,
    pub content: String,
    pub category: Vec<u32>,
    pub series: u32,
    pub tags: Vec<u32>,
    pub status: Status,
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
