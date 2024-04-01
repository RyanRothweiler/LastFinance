#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// a real life bank transaction
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub payee: String,

    // unix timestamp
    pub date: i64,

    // negative is outflow, positive is inflow
    pub amount: i64,

    pub notes: String,

    pub account_id: i64,
}

impl Transaction {
    pub fn new(payee: &str, amount: i64, date: i64, account_id: i64) -> Transaction {
        Transaction {
            payee: payee.to_string(),
            amount,
            date,
            account_id,
            notes: "".to_string(),
        }
    }

    pub fn to_json_schema(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionList {
    pub transactions: Vec<Transaction>,
}

impl TransactionList {
    pub fn new() -> TransactionList {
        TransactionList {
            transactions: vec![],
        }
    }

    pub fn to_json_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}
