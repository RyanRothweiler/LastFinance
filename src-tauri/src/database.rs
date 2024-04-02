#![allow(unused_macros)]

use rusqlite::{params, Connection, Result};

use data::account::Account;
use data::category::Category;
use data::category::CategoryList;
use data::category_transfer::CategoryTransfer;
use data::transaction::*;

mod table_actions;
use table_actions::TableActions;

#[cfg(test)]
mod tests;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new(path: &str) -> Database {
        let connection = Connection::open(path).unwrap();

        let db = Database {
            connection: connection,
        };

        fn setup_table<T: TableActions>(db: &Database) {
            let table_name = &T::get_table_name();

            if !db.table_exists(table_name) {
                let query = format!("CREATE TABLE {} ( {} )", table_name, T::get_table_schema());
                db.connection.execute(&query, ()).unwrap();
                println!("Created table {}", table_name);
            }
        }

        setup_table::<Category>(&db);
        setup_table::<Account>(&db);
        setup_table::<Transaction>(&db);
        setup_table::<CategoryTransfer>(&db);

        return db;
    }

    pub fn insert<T: TableActions>(&self, data: T) -> Result<(), rusqlite::Error> {
        let mut query = String::new();
        query.push_str("INSERT INTO ");
        query.push_str(&T::get_table_name());

        query.push_str(" ( ");
        query.push_str(&T::get_insert_schema());
        query.push_str(" ) ");

        query.push_str(" VALUES (");
        query.push_str(&data.to_insert_data());
        query.push_str(")");

        self.connection.execute(&query, ())?;
        Ok(())
    }

    pub fn get<T: TableActions>(&self, id: i64) -> T {
        let query = format!(
            "SELECT {} FROM {} WHERE ROWID={}",
            T::get_fetch_schema(),
            T::get_table_name(),
            id
        );

        self.connection
            .query_row(&query, [], |row| Ok(T::row_to_data(row)))
            .unwrap()
    }

    pub fn get_all<T: TableActions>(&self) -> Result<Vec<T>, rusqlite::Error> {
        let query = format!(
            "SELECT {} FROM {}",
            T::get_fetch_schema(),
            T::get_table_name(),
        );

        let mut stmt = self.connection.prepare(&query)?;
        let mut iter = stmt.query_map([], |row| Ok(T::row_to_data(row)))?;

        let mut ret: Vec<T> = vec![];
        for c in iter {
            ret.push(c.unwrap());
        }

        return Ok(ret);
    }

    fn table_exists(&self, table: &str) -> bool {
        #[derive(Debug)]
        struct Entry {
            name: String,
        }

        let query: String =
            format!("SELECT name FROM sqlite_master WHERE type='table' AND name='{table}';");
        let mut statement = self.connection.prepare(&query).unwrap();
        let rows_iter = statement
            .query_map([], |row| {
                Ok(Entry {
                    name: row.get(0).unwrap(),
                })
            })
            .unwrap();

        for p in rows_iter {
            return true;
        }
        return false;
    }

    pub fn get_transaction_list_display(&self) -> Result<TransactionDisplayList, rusqlite::Error> {
        let query = "SELECT 
                            payee, 
                            amount, 
                            date, 
                            account_id,
                            ifnull(categories.display_name, '') as category_display_name
                    from transactions 
                    left join categories on transactions.category_id = categories.ROWID";

        let mut stmt = self.connection.prepare(query)?;
        let mut iter = stmt.query_map([], |row| {
            Ok(TransactionDisplay {
                trans_raw: Transaction::new(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?),
                category_display: row.get(4).unwrap(),
            })
        })?;

        let mut ret = TransactionDisplayList {
            transactions: vec![],
        };
        for c in iter {
            ret.transactions.push(c.unwrap());
        }

        Ok(ret)
    }

    // adds to the amount in the account
    pub fn fund_account(&self, amount: i64, id: i64) -> Result<(), String> {
        let account: Account = self.get::<Account>(id);
        let new_bal = account.balance + amount;

        let update_query = format!(
            "UPDATE {} SET balance = {} WHERE ROWID = {}",
            data::account::Account::get_table_name(),
            new_bal,
            id,
        );
        self.connection.execute(&update_query, ()).unwrap();

        println!("account {} funded {}", id, amount);
        Ok(())
    }

    pub fn get_category_id(&self, name: &str) -> Result<i64, rusqlite::Error> {
        let query = format!("SELECT rowid FROM categories WHERE display_name='{name}'");
        let id: i64 = self
            .connection
            .query_row(&query, [], |row| Ok(row.get(0)?))?;
        Ok(id)
    }

    pub fn get_unassigned(&self) -> Result<i64, rusqlite::Error> {
        let accounts = self.get_all::<Account>()?;
        let mut accounts_total = 0;
        for a in accounts {
            accounts_total += a.balance;
        }

        let categories = self.get_all::<Category>()?;
        let mut categories_total = 0;
        for c in categories {
            categories_total += c.balance;
        }

        return Ok(accounts_total - categories_total);
    }
}
