use crate::entity::taxonomy::{Model as TaxonomyModel, TaxonomyType};
pub struct ClassifiedTaxonomy {
    pub categories: Vec<TaxonomyModel>,
    pub tags: Vec<TaxonomyModel>,
    pub series: Vec<TaxonomyModel>,
}

impl ClassifiedTaxonomy {
    pub fn new() -> Self {
        Self {
            categories: Vec::new(),
            tags: Vec::new(),
            series: Vec::new(),
        }
    }
}

pub trait ClassifyTaxonomy {
    fn classify(self) -> ClassifiedTaxonomy;
}

impl ClassifyTaxonomy for Vec<TaxonomyModel> {
    fn classify(self) -> ClassifiedTaxonomy {
        let mut classified = ClassifiedTaxonomy::new();
        for model in self {
            match model.t_type.clone() {
                TaxonomyType::Category => classified.categories.push(model),
                TaxonomyType::Tag => classified.tags.push(model),
                TaxonomyType::Series => classified.series.push(model),
            }
        }
        classified
    }
}
