use serde_enum_str::Deserialize_enum_str;

#[derive(Deserialize_enum_str)]
#[serde(rename_all = "camelCase")]
pub enum SqlOrder {
    Asc,
    Desc,
}

impl SqlOrder {
    pub fn order(&self) -> sea_orm::Order {
        match self {
            Self::Asc => sea_orm::Order::Asc,
            _ => sea_orm::Order::Desc,
        }
    }
}
