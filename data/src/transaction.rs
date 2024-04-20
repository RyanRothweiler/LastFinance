#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// a real life bank transaction
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
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
    pub fn new(
        payee: String,
        inflow: i64,
        outflow: i64,
        date: i64,
        account_id: i64,
    ) -> Result<Transaction, String> {
        if outflow != 0 && inflow != 0 {
            return Err("Cannot create transaction with both inflow and outflow.".to_string());
        }
        if outflow == 0 && inflow == 0 {
            return Err("Cannot create transaction with no outflow and no inflow.".to_string());
        }

        let mount: i64;
        if outflow > 0 {
            mount = -outflow;
        } else {
            mount = inflow;
        }

        Ok(Transaction {
            payee: payee,
            amount: mount,
            date,
            account_id,
            category_id: 0,
            notes: "".to_string(),
        })
    }

    // no validation on input
    pub fn new_raw(payee: String, amount: i64, date: i64, account_id: i64) -> Transaction {
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
    pub account_display: String,
}

impl TransactionDisplay {
    pub fn new(
        trans_raw: Transaction,
        category_display: String,
        account_display: String,
    ) -> TransactionDisplay {
        TransactionDisplay {
            trans_raw,
            category_display,
            account_display,
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

#[test]
fn transaction_new_outflow_and_inflow() {
    assert_eq!(
        Transaction::new("payee".to_string(), 10, 10, 0, 0),
        Err("Cannot create transaction with both inflow and outflow.".to_string())
    );
}

#[test]
fn transaction_new() {
    let trans = Transaction::new("payee".to_string(), 0, 10, 0, 0).unwrap();
    assert_eq!(trans.amount, -10);
}
