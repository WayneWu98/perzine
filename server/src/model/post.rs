use serde::{Deserialize, Serialize};
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
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: Option<u32>,
    pub title: Option<String>,
    pub sub_title: Option<String>,
    pub content: Option<serde_json::Value>,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    pub published: Option<chrono::DateTime<chrono::Utc>>,
    pub status: Option<Status>,
    #[serde(skip)]
    pub categories: Option<Vec<u32>>,
    #[serde(rename(serialize = "categories"))]
    pub _categories: Option<serde_json::Value>,
    #[serde(skip)]
    pub tags: Option<Vec<u32>>,
    #[serde(rename(serialize = "tags"))]
    pub _tags: Option<serde_json::Value>,
    #[serde(skip)]
    pub series: Option<u32>,
    #[serde(rename(serialize = "series"))]
    pub _series: Option<serde_json::Value>,
}

rbatis::crud!(Post {}, "posts");
