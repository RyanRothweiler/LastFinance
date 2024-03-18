use super::Database;

use rusqlite::{params, Connection, Result, Row, Rows};

use data::account::Account;
use data::category::Category;
use data::category::CategoryList;
use data::Table;

pub trait TableActions {
    fn row_to_data(row: &Row) -> Self;
}

impl TableActions for super::Category {
    fn row_to_data(row: &Row) -> Self {
        Category {
            display_name: row.get(0).unwrap(),
        }
    }
}

impl TableActions for super::Account {
    fn row_to_data(row: &Row) -> Self {
        Account {
            balance: row.get(0).unwrap(),
        }
    }
}
