use super::Database;

use rusqlite::{params, Connection, Result, Row, Rows};

use data::account::Account;
use data::category::Category;
use data::category::CategoryList;

pub trait TableActions {
    fn get_table_name() -> String;
    fn get_table_schema() -> String;
    fn get_insert_schema() -> String;
    fn to_insert_data(&self) -> String;
    fn row_to_data(row: &Row) -> Self;
}

impl TableActions for super::Category {
    fn row_to_data(row: &Row) -> Self {
        Category {
            display_name: row.get(0).unwrap(),
        }
    }

    fn get_table_name() -> String {
        return "categories".to_string();
    }

    fn get_table_schema() -> String {
        return "display_name   INTEGER NOT NULL".to_string();
    }

    fn get_insert_schema() -> String {
        return "display_name".to_string();
    }

    fn to_insert_data(&self) -> String {
        return format!("'{}'", self.display_name);
    }
}

impl TableActions for super::Account {
    fn row_to_data(row: &Row) -> Self {
        Account {
            balance: row.get(0).unwrap(),
            display_name: row.get(1).unwrap(),
        }
    }

    fn get_table_name() -> String {
        return "accounts".to_string();
    }

    fn get_table_schema() -> String {
        return "balance   INTEGER NOT NULL, display_name TEXT NOT NULL".to_string();
    }

    fn get_insert_schema() -> String {
        return "balance, display_name".to_string();
    }

    fn to_insert_data(&self) -> String {
        return format!("'{}', '{}'", self.balance, self.display_name);
    }
}
