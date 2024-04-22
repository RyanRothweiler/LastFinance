#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: i64,
    pub display_name: String,
}

impl Account {
    pub fn new(name: &str) -> Account {
        Account {
            id: 0,
            display_name: name.to_string(),
        }
    }
}
