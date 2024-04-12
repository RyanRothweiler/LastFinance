#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Category {
    pub display_name: String,
    pub balance: i64,
    pub id: i64,
}

impl Category {
    pub fn new(name: &str) -> Category {
        Category {
            display_name: name.to_string(),
            balance: 0,
            id: 0,
        }
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
    pub fn new() -> CategoryList {
        CategoryList { categories: vec![] }
    }

    pub fn to_json_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryDisplay {
    pub category_id: i64,
    pub display_name: String,
    pub transaction_total: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryDetails {
    pub category_id: i64,
    pub max_monthly: i64,
    pub min_monthly: i64,
    pub avr_monthly: i64,
}
