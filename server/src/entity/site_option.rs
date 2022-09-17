use std::collections::HashMap;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "options")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique, indexed)]
    pub name: String,
    #[sea_orm(nullable)]
    pub value: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub trait Utils<'a> {
    fn map(&'a self) -> HashMap<&'a String, &'a String>;
}

impl<'a> Utils<'a> for Vec<Model> {
    fn map(&'a self) -> HashMap<&'a String, &'a String> {
        let mut map = HashMap::new();
        for opt in self.iter() {
            map.insert(&opt.name, &opt.value);
        }
        map
    }
}
