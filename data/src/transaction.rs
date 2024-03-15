#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    from: i64,
    to: i64,

    // amount is in minmum size of the currency (cents, pence)
    amount: i64,
}

impl super::Table for Transaction {
    fn get_table_name() -> String {
        return "transactions".to_string();
    }

    fn get_table_schema() -> String {
        return "from   INTEGER NOT NULL
            to      INTEGER NOT NULL
            amount  INTEGER NOT NULL"
            .to_string();
    }
}

impl Transaction {
    pub fn new(from: i64, to: i64, amount: i64) -> Transaction {
        Transaction { from, to, amount }
    }

    pub fn to_json_schema(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}
