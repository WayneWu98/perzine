#[derive(Clone, Copy)]
pub enum Order {
    DESC,
    ASC,
}

pub struct Pagination {
    pub per: u32,
    pub page: u32,
    pub order_by: String,
    pub order: Order,
}

impl Pagination {
    pub fn from_page(page: u32) -> Self {
        Self {
            per: 10,
            page,
            order_by: "created".to_string(),
            order: Order::DESC,
        }
    }
    pub fn order(&self) -> String {
        match self.order {
            Order::ASC => "ASC".to_string(),
            Order::DESC => "DESC".to_string(),
        }
    }
    pub fn offset(&self) -> u32 {
        self.per * (self.page - 1)
    }
}

pub struct SQLFilter {
    pub where_str: String,
}

impl SQLFilter {
    pub fn new() -> Self {
        Self {
            where_str: String::new(),
        }
    }
    pub fn and(&mut self, con: &str) -> &mut Self {
        self.where_str.push_str(&format!(" AND {}", con));
        self
    }
    pub fn or(&mut self, con: &str) -> &mut Self {
        self.where_str.push_str(&format!("OR {}", con));
        self
    }
}
