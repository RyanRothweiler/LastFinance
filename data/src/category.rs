#![allow(dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

pub const TABLE_ID: &str = "category";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub display_name: String,
}

impl Category {
    pub fn sql_schema() -> String {
        return "display_name    TEXT NOT NULL".to_string();
    }

    pub fn to_json_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryList {
    pub categories: Vec<Category>,
}

impl CategoryList {
    pub fn new () -> CategoryList {
        CategoryList {
            categories: vec![],
        }
    }

    pub fn to_json_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}
