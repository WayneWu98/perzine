pub enum TaxonomyType {
    Category,
    Tag,
    Serise,
}

pub struct Taxonomy {
    id: u32,
    name: String,
    taxonomy_type: TaxonomyType,
    cover: String,
    desc: String,
    ord: i32,
}
