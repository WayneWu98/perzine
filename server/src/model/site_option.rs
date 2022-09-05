use rbatis::{impl_select, impl_update};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub static PRIVATE_OPTIONS: [&'static str; 1] = ["password"];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiteOption {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub value: Option<String>,
}

impl SiteOption {
    pub fn is_private(&self) -> bool {
        match self.name {
            Some(ref name) => PRIVATE_OPTIONS.contains(&&name[..]),
            None => true,
        }
    }
}

rbatis::crud!(SiteOption {}, "options");
impl_select!(SiteOption { select_by_name(table_name: &str, name: String) -> Option => "`where name = #{name} limit 1`" });
impl_update!(SiteOption { update_by_name(name: String) => "`where name = #{name}`" });

pub fn map(options: Vec<SiteOption>) -> HashMap<String, Option<String>> {
    let mut opts = HashMap::new();
    options
        .iter()
        .filter(|opt| opt.name.is_some())
        .for_each(|opt| {
            let name = match opt.name {
                Some(ref name) => name.to_owned(),
                _ => "unkown".to_owned(),
            };
            opts.insert(name, opt.value.clone());
        });
    opts
}

pub fn filter_publics(options: Vec<SiteOption>) -> Vec<SiteOption> {
    options
        .into_iter()
        .filter(|opt| !opt.is_private())
        .collect::<Vec<SiteOption>>()
}
