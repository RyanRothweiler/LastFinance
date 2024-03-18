#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub balance: i64,
}

impl super::Table for Account {
    fn get_table_name() -> String {
        return "accounts".to_string();
    }

    fn get_table_schema() -> String {
        return "balance   INTEGER NOT NULL".to_string();
    }

    fn get_insert_schema() -> String {
        return "balance".to_string();
    }

    fn to_insert_data(&self) -> String {
        return format!("{}", self.balance);
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
