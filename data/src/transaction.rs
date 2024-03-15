#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    source: i64,
    dest: i64,

    // amount is in minmum size of the currency (cents, pence)
    amount: i64,
}

impl super::Table for Transaction {
    fn get_table_name() -> String {
        return "transactions".to_string();
    }

    fn get_table_schema() -> String {
        return "source   INTEGER NOT NULL,
            dest      INTEGER NOT NULL,
            amount  INTEGER NOT NULL"
            .to_string();
    }

    fn get_insert_schema() -> String {
        return "source, dest, amount".to_string();
    }

    fn to_insert_data(&self) -> String {
        todo!()
    }
}

impl Transaction {
    pub fn new(source: i64, dest: i64, amount: i64) -> Transaction {
        Transaction {
            source,
            dest,
            amount,
        }
    }

    pub fn to_json_schema(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}
