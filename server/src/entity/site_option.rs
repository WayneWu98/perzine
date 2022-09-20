use serde::Deserialize;
use serde_enum_str::Deserialize_enum_str;
use std::collections::HashMap;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "options")]
pub struct Model {
    #[serde(skip_deserializing)]
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique, indexed)]
    pub name: String,
    #[sea_orm(nullable)]
    pub value: String,
    pub level: OptionLevel,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Deserialize_enum_str)]
#[serde(rename_all = "camelCase")]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "OPTION_LEVEL")]
pub enum OptionLevel {
    #[sea_orm(string_value = "public")]
    Public,
    #[sea_orm(string_value = "protected")]
    Protected,
    #[sea_orm(string_value = "private")]
    Private,
}

impl Model {
    pub fn is_public(&self) -> bool {
        self.level == OptionLevel::Public
    }
    pub fn is_protected(&self) -> bool {
        self.level == OptionLevel::Protected
    }
    pub fn is_private(&self) -> bool {
        self.level == OptionLevel::Private
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub trait Utils<'a> {
    type Item;
    fn to_map(self) -> HashMap<String, String>;

    fn exclude_private(self) -> Vec<Self::Item>;
    fn filter_public(self) -> Vec<Self::Item>;
}

impl<'a> Utils<'a> for Vec<Model> {
    type Item = Model;
    fn to_map(self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for opt in self.into_iter() {
            map.insert(opt.name.clone(), opt.value.clone());
        }
        map
    }
    fn exclude_private(self) -> Vec<Self::Item> {
        self.into_iter().filter(|opt| !opt.is_private()).collect()
    }
    fn filter_public(self) -> Vec<Self::Item> {
        self.into_iter().filter(|opt| opt.is_public()).collect()
    }
}
