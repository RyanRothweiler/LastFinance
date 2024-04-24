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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountDisplay {
    pub account_id: i64,
    pub display_name: String,
    pub balance: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountHistoryEntry {
    pub account_id: i64,
    pub display_name: String,
    pub running_balance: i64,
}
