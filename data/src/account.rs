#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub balance: i64,
    pub display_name: String,
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
