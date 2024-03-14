use rusqlite::{params, Connection, Result};

use data::category::Category;
use data::category::CategoryList;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new(path: &str) -> Database {
        let connection = Connection::open(path).unwrap();

        let db = Database {
            connection: connection,
        };

        // setup initial tables
        if !db.table_exists(data::category::TABLE_ID) {
            let query = format!(
                "CREATE TABLE {} ( {} )",
                data::category::TABLE_ID,
                Category::sql_schema()
            );
            db.connection.execute(&query, ()).unwrap();
            println!("Created category table");
        }

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

    pub fn get_category(&self, category: &str) -> Option<Category> {
        /*
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
        */

        return None;
    }

    pub fn get_all_categories(&self) -> Vec<Category> {
        let query = format!("SELECT display_name FROM {}", data::category::TABLE_ID);
        println!("{query}");

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

    pub fn insert_category(&self, display_name: &str) {
        let query: String = format!(
            "INSERT INTO {:?} ( display_name ) VALUES ('{}')",
            data::category::TABLE_ID,
            display_name,
        );
        self.connection.execute(&query, ()).unwrap();
    }
}
