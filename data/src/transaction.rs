#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// a real life bank transaction
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub payee: String,
    pub notes: String,
    pub account_id: i64,
    pub category_id: i64,

    // unix timestamp
    pub date: i64,

    // negative is outflow, positive is inflow
    pub amount: i64,
}

impl Transaction {
    pub fn new(payee: String, amount: i64, date: i64, account_id: i64) -> Transaction {
        Transaction {
            payee: payee,
            amount,
            date,
            account_id,
            category_id: 0,
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

// Transaction data for displaying to user
// ids are resolved, etc
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionDisplay {
    pub trans_raw: Transaction,
    pub category_display: String,
}

impl TransactionDisplay {
    pub fn new(trans_raw: Transaction, category_display: String) -> TransactionDisplay {
        TransactionDisplay {
            trans_raw,
            category_display,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionDisplayList {
    pub transactions: Vec<TransactionDisplay>,
}

impl TransactionDisplayList {
    pub fn new() -> Self {
        Self {
            transactions: vec![],
        }
    }

    pub fn to_json_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}
