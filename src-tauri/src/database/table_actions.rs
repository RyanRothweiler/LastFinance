use super::Database;

use rusqlite::{params, Connection, Result, Row, Rows};

use data::account::Account;
use data::category::Category;
use data::category::CategoryList;
use data::category_transfer::CategoryTransfer;

pub trait TableActions {
    fn get_table_name() -> String;
    fn get_table_schema() -> String;
    fn get_insert_schema() -> String;
    fn get_fetch_schema() -> String;
    fn to_insert_data(&self) -> String;
    fn row_to_data(row: &Row) -> Self;
}

impl TableActions for super::Category {
    fn row_to_data(row: &Row) -> Self {
        Category {
            display_name: row.get(0).unwrap(),
            balance: row.get(1).unwrap(),
        }
    }

    fn get_table_name() -> String {
        return "categories".to_string();
    }

    fn get_table_schema() -> String {
        return "display_name TEXT NOT NULL, balance INTEGER NOT NULL".to_string();
    }

    fn get_insert_schema() -> String {
        return "display_name, balance".to_string();
    }

    fn get_fetch_schema() -> String {
        return "display_name, balance".to_string();
    }

    fn to_insert_data(&self) -> String {
        return format!("'{}', '{}'", self.display_name, self.balance);
    }
}

impl TableActions for super::Account {
    fn row_to_data(row: &Row) -> Self {
        Account {
            id: row.get(0).unwrap(),
            balance: row.get(1).unwrap(),
            display_name: row.get(2).unwrap(),
        }
    }

    fn get_table_name() -> String {
        return "accounts".to_string();
    }

    fn get_table_schema() -> String {
        return "balance INTEGER NOT NULL, display_name TEXT NOT NULL".to_string();
    }

    fn get_insert_schema() -> String {
        return "balance, display_name".to_string();
    }

    fn get_fetch_schema() -> String {
        return "rowid, balance, display_name".to_string();
    }

    fn to_insert_data(&self) -> String {
        return format!("'{}', '{}'", self.balance, self.display_name);
    }
}

impl TableActions for super::CategoryTransfer {
    fn row_to_data(row: &Row) -> Self {
        CategoryTransfer {
            source: row.get(0).unwrap(),
            dest: row.get(1).unwrap(),
            amount: row.get(1).unwrap(),
        }
    }

    fn get_table_name() -> String {
        return "category_transfer".to_string();
    }

    fn get_table_schema() -> String {
        return "source INTEGER NOT NULL, dest INTEGER NOT NULL, amount INTEGER NOT NULL"
            .to_string();
    }

    fn get_insert_schema() -> String {
        return "source, dest, amount".to_string();
    }

    fn get_fetch_schema() -> String {
        return "source, dest, amount".to_string();
    }

    fn to_insert_data(&self) -> String {
        return format!("'{}', '{}', '{}'", self.source, self.dest, self.amount);
    }
}
