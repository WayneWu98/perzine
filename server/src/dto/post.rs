use crate::{
    dto::taxonomy::{ClassifiedTaxonomy, ClassifyTaxonomy},
    entity::post::{self, PostStatus},
};
use chrono::{DateTime, Utc};
use serde::{ser::SerializeStruct, Serialize};

use crate::entity::taxonomy::Model as TaxonomyModel;

// #region SimplePost
pub struct SimplePost {
    pub id: i64,
    pub title: String,
    pub subtitle: Option<String>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub published: DateTime<Utc>,
    pub excerpts: Option<String>,
    pub route: Option<String>,
    pub is_page: Option<bool>,
    pub status: Option<PostStatus>,
    pub extra: Option<serde_json::Value>,
    pub is_authed: bool,
}

impl From<post::Model> for SimplePost {
    fn from(model: post::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            subtitle: model.subtitle,
            created: model.created.unwrap_or(Utc::now()),
            modified: model.modified.unwrap_or(Utc::now()),
            published: model.published.unwrap_or(Utc::now()),
            excerpts: model.excerpts,
            route: model.route,
            is_page: model.is_page,
            status: model.status,
            extra: model.extra,
            is_authed: false,
        }
    }
}

impl Serialize for SimplePost {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("SimplePost", 12)?;
        state.serialize_field("id", &self.id);
        state.serialize_field("title", &self.title);
        state.serialize_field("subtitle", &self.subtitle);
        state.serialize_field("published", &self.published);
        state.serialize_field("excerpts", &self.excerpts);
        state.serialize_field("extra", &self.extra);
        if self.is_authed {
            state.serialize_field("created", &self.created)?;
            state.serialize_field("modified", &self.modified)?;
            state.serialize_field("route", &self.route)?;
            state.serialize_field("isPage", &self.is_page)?;
            state.serialize_field("status", &self.status)?;
        }
        state.end()
    }
}
// #endregion

// #region FulledPost
pub struct FulledPost {
    pub id: i64,
    pub title: String,
    pub subtitle: Option<String>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub published: DateTime<Utc>,
    pub excerpts: Option<String>,
    pub content: Option<serde_json::Value>,
    pub route: Option<String>,
    pub is_page: Option<bool>,
    pub status: Option<PostStatus>,
    pub extra: Option<serde_json::Value>,
    pub is_authed: bool,
}

impl From<post::Model> for FulledPost {
    fn from(model: post::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            subtitle: model.subtitle,
            created: model.created.unwrap_or(Utc::now()),
            modified: model.modified.unwrap_or(Utc::now()),
            published: model.published.unwrap_or(Utc::now()),
            excerpts: model.excerpts,
            content: model.content,
            route: model.route,
            is_page: model.is_page,
            status: model.status,
            extra: model.extra,
            is_authed: false,
        }
    }
}

impl Serialize for FulledPost {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("SimplePost", 12)?;
        state.serialize_field("id", &self.id);
        state.serialize_field("title", &self.title);
        state.serialize_field("subtitle", &self.subtitle);
        state.serialize_field("published", &self.published);
        state.serialize_field("excerpts", &self.excerpts);
        state.serialize_field("extra", &self.extra);
        state.serialize_field("content", &self.content);
        if self.is_authed {
            state.serialize_field("created", &self.created)?;
            state.serialize_field("modified", &self.modified)?;
            state.serialize_field("route", &self.route)?;
            state.serialize_field("isPage", &self.is_page)?;
            state.serialize_field("status", &self.status)?;
        }
        state.end()
    }
}
// #endregion

#[derive(Serialize)]
pub struct PostWithTaxonomy<T: Serialize> {
    #[serde(flatten)]
    pub post: T,
    pub categories: Vec<TaxonomyModel>,
    pub tags: Vec<TaxonomyModel>,
    pub series: Option<TaxonomyModel>,
}

impl<T: Serialize> PostWithTaxonomy<T> {
    pub fn from_classified(
        post: T,
        ClassifiedTaxonomy {
            categories,
            tags,
            mut series,
        }: ClassifiedTaxonomy,
    ) -> Self {
        Self {
            post,
            categories,
            tags,
            series: series.pop(),
        }
    }

    pub fn from_unclassified(post: T, taxonomies: Vec<TaxonomyModel>) -> Self {
        Self::from_classified(post, taxonomies.classify())
    }
}
