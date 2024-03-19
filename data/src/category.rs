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
