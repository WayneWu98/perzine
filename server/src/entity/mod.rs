use sea_orm::{DeriveActiveEnum, EnumIter};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

pub mod comment;
pub mod post;
pub mod post_taxonomy;
pub mod site_option;
pub mod taxonomy;

#[derive(
    Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize_enum_str, Deserialize_enum_str,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "USER_ROLE")]
#[serde(rename_all = "camelCase")]
pub enum UserRole {
    #[sea_orm(string_value = "visitor")]
    Visitor,
    #[sea_orm(string_value = "manager")]
    Manager,
}

impl Default for UserRole {
    fn default() -> Self {
        Self::Visitor
    }
}
