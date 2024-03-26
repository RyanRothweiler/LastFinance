#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: i64,
    pub balance: i64,
    pub display_name: String,
}

impl Account {
    pub fn new(name: &str) -> Account {
        Account {
            id: 0,
            balance: 0,
            display_name: name.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountList {
    pub accounts: Vec<Account>,
}

impl AccountList {
    pub fn new() -> AccountList {
        AccountList { accounts: vec![] }
    }

    pub fn to_json_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}
