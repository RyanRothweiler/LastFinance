#![allow(dead_code, unused_imports)]

use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub display_name: String,
}

pub const TABLE_ID: &str = "category";

impl Category {
    pub fn sql_schema() -> String {
        return "display_name    TEXT NOT NULL".to_string();
    }

    pub fn to_json_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}

#[derive(Debug, Serialize)]
pub struct CategoryList {
    pub categories: Vec<Category>,
}

impl CategoryList {
    pub fn to_json_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}
