use axum::Json;
use deadpool_postgres::Pool;
use once_cell::sync::Lazy;
use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Serialize,
};
use std::{error::Error, fmt::Write};
use tokio_postgres::types::{to_sql_checked, FromSql, ToSql};

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Debug, Clone, Copy, Deserialize_enum_str, Serialize_enum_str)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    PUBLISHED,
    UNPUBLISHED,
    HIDDEN,
}

impl Default for Status {
    fn default() -> Self {
        Status::PUBLISHED
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
        Ok(s.parse::<Self>().unwrap_or_default())
    }
}

impl ToSql for Status {
    fn accepts(_: &tokio_postgres::types::Type) -> bool
    where
        Self: Sized,
    {
        true
    }

    fn to_sql(
        &self,
        _: &tokio_postgres::types::Type,
        out: &mut tokio_postgres::types::private::BytesMut,
    ) -> Result<tokio_postgres::types::IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        out.extend_from_slice(self.to_string().as_bytes());
        Ok(tokio_postgres::types::IsNull::No)
    }

    to_sql_checked!();
}

#[derive(Debug, Clone)]
pub struct Post {
    id: u32,
    pub created: String,
    pub modified: String,
    pub content: String,
    pub categories: Vec<u32>,
    pub _categories: serde_json::Value,
    pub series: u32,
    pub _series: serde_json::Value,
    pub tags: Vec<u32>,
    pub _tags: serde_json::Value,
    pub status: Status,
    pub title: String,
    pub sub_title: String,
}

impl From<tokio_postgres::Row> for Post {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            created: row.get("created"),
            modified: row.get("modified"),
            content: row.get("content"),
            categories: row.get("categories"),
            _categories: row.get("_categories"),
            series: row.get("series"),
            _series: row.get("_series"),
            tags: row.get("tags"),
            _tags: row.get("_tags"),
            status: row.get("status"),
            title: row.get("title"),
            sub_title: row.get("sub_title"),
        }
    }
}

// impl Post {
//     pub fn id(&self) -> u32 {
//         self.id
//     }
//     pub async fn from_id(pool: &Pool, id: i32) -> Result<Self, Box<dyn Error>> {
//         let client = pool.get().await?;
//         let stmt = client
//             .prepare(&"SELECT * FROM \"posts\" WHERE id=$1".to_string())
//             .await?;
//         let row = client.query_one(&stmt, &[&id]).await?;
//         Ok(Self::from_row(row)?)
//     }

//     pub async fn list(
//         pool: &Pool,
//         p: Pagination,
//         sql_filter: SQLFilter,
//     ) -> Result<Vec<Self>, Box<dyn Error>> {
//         let client = pool.get().await?;
//         let stmt = client
//             .prepare(
//                 &"SELECT * FROM \"posts\" WHERE {$1} ORDER BY $2 $3 LIMIT $4 OFFSET $5".to_string(),
//             )
//             .await?;
//         let rows = client
//             .query(
//                 &stmt,
//                 &[
//                     &sql_filter.where_str,
//                     &p.order_by,
//                     &p.order(),
//                     &p.per,
//                     &p.offset(),
//                 ],
//             )
//             .await?
//             .into_iter()
//             .map(|row| Self::from_row(row))
//             .filter(|res| res.is_ok())
//             .map(|res| res.unwrap())
//             .collect();
//         Ok(rows)
//     }
// }
