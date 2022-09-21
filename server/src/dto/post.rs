use crate::dto::taxonomy::{ClassifiedTaxonomy, ClassifyTaxonomy};
use serde::Serialize;

use crate::entity::taxonomy::Model as TaxonomyModel;

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
