use rusqlite::{params, Connection, Result};

use data::account::Account;
use data::category::Category;
use data::category::CategoryList;
use data::transaction::Transaction;
use data::Table;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new(path: &str) -> Database {
        let connection = Connection::open(path).unwrap();

        let db = Database {
            connection: connection,
        };

        fn setup_table<T: data::Table>(db: &Database) {
            let table_name = &T::get_table_name();

            if !db.table_exists(table_name) {
                let query = format!("CREATE TABLE {} ( {} )", table_name, T::get_table_schema());
                db.connection.execute(&query, ()).unwrap();
                println!("Created table {}", table_name);
            }
        }

        setup_table::<Category>(&db);
        setup_table::<Transaction>(&db);
        setup_table::<Account>(&db);

        return db;
    }

    pub fn table_exists(&self, table: &str) -> bool {
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

    pub fn get_all_categories(&self) -> Vec<Category> {
        let query = format!(
            "SELECT display_name FROM {}",
            data::category::Category::get_table_name()
        );

        let mut stmt = self.connection.prepare(&query).unwrap();

        let iter = stmt
            .query_map([], |row| {
                Ok(Category {
                    display_name: row.get(0)?,
                })
            })
            .unwrap();

        let mut ret: Vec<Category> = vec![];
        for c in iter {
            ret.push(c.unwrap());
        }

        return ret;
    }

    pub fn insert<T: data::Table>(&self, data: T) {
        let mut query = String::new();
        query.push_str("INSERT INTO ");
        query.push_str(&T::get_table_name());

        query.push_str(" ( ");
        query.push_str(&T::get_insert_schema());
        query.push_str(" ) ");

        query.push_str(" VALUES ('");
        query.push_str(&data.to_insert_data());
        query.push_str("')");

        println!("{query}");
        self.connection.execute(&query, ()).unwrap();
    }

    pub fn get_account(&self, id: i64) -> Result<Account, String> {
        let query = format!(
            "SELECT balance  FROM {}",
            data::account::Account::get_table_name()
        );

        let mut stmt = self.connection.prepare(&query).unwrap();

        let iter = stmt
            .query_map([], |row| {
                Ok(Account {
                    balance: row.get(0)?,
                })
            })
            .unwrap();

        let mut accounts: Vec<Account> = vec![];
        for c in iter {
            accounts.push(c.unwrap());
        }

        if accounts.len() == 0 {
            return Err("No account with that ID".to_string());
        }
        if accounts.len() >= 2 {
            return Err("Multiple accounts with that ID".to_string());
        }

        return Ok(accounts[0].clone());
    }

    pub fn fund_account(&self, amount: i64, id: i64) -> Result<(), String> {
        let account: Account = self.get_account(id)?;
        println!("account {} funded {}", id, amount);
        Ok(())
    }
}

fn test_setup_db() -> Database {
    let db_dir = "C:/Digital Archive/testing_db.db3";
    let _ = std::fs::remove_file(db_dir);

    let db = Database::new(db_dir);
    return db;
}

#[test]
fn database_setup() {
    let db = test_setup_db();
}

#[test]
fn insert() {
    let db = test_setup_db();

    let cat = Category {
        display_name: "testing here".to_string(),
    };
    db.insert(cat);
}
