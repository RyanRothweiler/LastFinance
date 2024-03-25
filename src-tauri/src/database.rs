#![allow(unused_macros)]

use rusqlite::{params, Connection, Result};

use data::account::Account;
use data::category::Category;
use data::category::CategoryList;
use data::transaction::Transaction;

mod table_actions;
use table_actions::TableActions;

macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        type_name_of(f)
            .rsplit("::")
            .find(|&part| part != "f" && part != "{{closure}}")
            .expect("Short function name")
    }};
}

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
        //setup_table::<Transaction>(&db);

        return db;
    }

    fn insert<T: TableActions>(&self, data: T) -> Result<(), rusqlite::Error> {
        let mut query = String::new();
        query.push_str("INSERT INTO ");
        query.push_str(&T::get_table_name());

        query.push_str(" ( ");
        query.push_str(&T::get_insert_schema());
        query.push_str(" ) ");

        query.push_str(" VALUES ('");
        query.push_str(&data.to_insert_data());
        query.push_str("')");

        self.connection.execute(&query, ())?;
        Ok(())
    }

    pub fn get<T: TableActions>(&self, id: i64) -> T {
        let query = format!(
            "SELECT {} FROM {} WHERE ROWID={}",
            T::get_insert_schema(),
            T::get_table_name(),
            id
        );

        self.connection
            .query_row(&query, [], |row| Ok(T::row_to_data(row)))
            .unwrap()
    }

    pub fn get_all<T: TableActions>(&self) -> Vec<T> {
        let query = format!(
            "SELECT {} FROM {}",
            T::get_insert_schema(),
            T::get_table_name(),
        );

        let mut stmt = self.connection.prepare(&query).unwrap();
        let iter = stmt.query_map([], |row| Ok(T::row_to_data(row))).unwrap();

        let mut ret: Vec<T> = vec![];
        for c in iter {
            ret.push(c.unwrap());
        }

        return ret;
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

    pub fn create_category(&self, name: &str) {
        self.insert(Category {
            display_name: name.to_string(),
        })
        .unwrap();
    }

    pub fn create_account(&self) -> Result<(), String> {
        let res = self.insert(Account {
            balance: 0,
            display_name: "account name here".to_string(),
        });
        match res {
            Ok(()) => Ok(()),
            Err(t) => Err(format!("{}", t)),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_setup_db(name: &str) -> Database {
        let db_dir = &format!("C:/Digital Archive/{}_db.db3", name);
        let _ = std::fs::remove_file(db_dir);

        let db = Database::new(db_dir);
        return db;
    }

    fn test_remove_db(name: &str, db: Database) {
        db.connection.close().unwrap();

        let db_dir = format!("C:/Digital Archive/{}_db.db3", name);
        std::fs::remove_file(db_dir).unwrap();
    }

    #[test]
    fn database_setup() {
        let db = test_setup_db(function!());
        test_remove_db(function!(), db);
    }

    #[test]
    fn insert_get() {
        let db = test_setup_db(function!());
        db.create_category("testing here");

        let cat_ret = db.get::<Category>(1);
        assert_eq!(
            cat_ret,
            Category {
                display_name: "testing here".to_string(),
            }
        );

        test_remove_db(function!(), db);
    }

    #[test]
    fn fund_get_ccount() {
        let db = test_setup_db(function!());
        db.create_account().unwrap();
        db.fund_account(data::dollars_to_cents(123.45), 1).unwrap();

        let ac = db.get::<Account>(1);
        assert_eq!(ac.balance, 12345);

        test_remove_db(function!(), db);
    }

    #[test]
    fn get_all_categories() {
        let db = test_setup_db(function!());

        db.create_category("first");
        db.create_category("second");

        let categories = db.get_all::<Category>();
        assert_eq!(categories.len(), 2);

        test_remove_db(function!(), db);
    }
}
