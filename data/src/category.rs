#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Category {
    pub display_name: String,
}

impl Category {
    pub fn to_json_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}

impl super::Table for Category {
    fn get_table_name() -> String {
        return "categories".to_string();
    }

    fn get_table_schema() -> String {
        return "display_name   INTEGER NOT NULL".to_string();
    }

    fn get_insert_schema() -> String {
        return "display_name".to_string();
    }

    fn to_insert_data(&self) -> String {
        return format!("{}", self.display_name);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryList {
    pub categories: Vec<Category>,
}

impl CategoryList {
    pub fn new() -> CategoryList {
        CategoryList { categories: vec![] }
    }

    pub fn to_json_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}
