#![allow(unused_macros)]

use rusqlite::{Connection, Result};

use time::format_description::well_known::Iso8601;
use time::PrimitiveDateTime;

use std::fs::File;
use std::io::{prelude::*, BufReader};

use data::account::*;
use data::category::*;
use data::category_transfer::CategoryTransfer;
use data::transaction::*;

mod table_actions;
use table_actions::TableActions;

#[cfg(test)]
mod tests;

pub enum OrderBy {
    None,
    Date,
}

pub struct Database {
    connection: Connection,
    pub file_name: String,
    pub folder_dir: String,
}

impl Database {
    pub fn new(folder_dir: &str, file_name: &str) -> Database {
        let path = format!("{folder_dir}/{file_name}.db3");
        let connection = Connection::open(path).unwrap();

        let db = Database {
            connection: connection,
            file_name: file_name.to_string(),
            folder_dir: folder_dir.to_string(),
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

    // Returns the row id
    pub fn insert<T: TableActions>(&self, data: T) -> Result<i64, rusqlite::Error> {
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
        Ok(self.connection.last_insert_rowid())
    }

    pub fn get<T: TableActions>(&self, id: i64) -> T {
        let query = format!(
            "SELECT {} FROM {} WHERE ROWID={}",
            T::get_fetch_schema(),
            T::get_table_name(),
            id
        );

        // TODO don't unwrap here. Return error.
        self.connection
            .query_row(&query, [], |row| Ok(T::row_to_data(row)))
            .unwrap()
    }

    pub fn get_all<T: TableActions>(&self, order_by: OrderBy) -> Result<Vec<T>, rusqlite::Error> {
        let order_str: &str = match order_by {
            OrderBy::None => "",
            OrderBy::Date => "ORDER BY date",
        };

        let query = format!(
            "SELECT {} FROM {} {}",
            T::get_fetch_schema(),
            T::get_table_name(),
            order_str,
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
        let query = "
            SELECT 
            payee, 
        amount, 
        date, 
        account_id,
        ifnull(categories.display_name, '') as category_display_name,
        ifnull(accounts.display_name, '') as account_display_name
            from transactions 
            left join categories on transactions.category_id = categories.ROWID
            left join accounts on transactions.account_id = accounts.ROWID
            ";

        let mut stmt = self.connection.prepare(query)?;
        let mut iter = stmt.query_map([], |row| {
            Ok(TransactionDisplay {
                trans_raw: Transaction::new_raw(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?),
                category_display: row.get(4).unwrap(),
                account_display: row.get(5).unwrap(),
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

    pub fn category_exists(&self, name: &str) -> Result<bool, rusqlite::Error> {
        let query = format!(
            "select COUNT(*) as count
            from categories
            where display_name='{}'",
            name
        );
        let count: i64 = self
            .connection
            .query_row(&query, [], |row| Ok(row.get(0)?))?;
        Ok(count > 0)
    }

    // This will error if the category doesn't exist
    pub fn get_category_id(&self, name: &str) -> Result<i64, rusqlite::Error> {
        let query = format!("SELECT rowid FROM categories WHERE display_name='{name}'");
        let id: i64 = self
            .connection
            .query_row(&query, [], |row| Ok(row.get(0)?))?;
        Ok(id)
    }

    pub fn get_unassigned(&self) -> Result<i64, rusqlite::Error> {
        todo!();
        /*
        let accounts = self.get_all::<Account>(OrderBy::None)?;
        let mut accounts_total = 0;
        for a in accounts {
        accounts_total += a.balance;
        }

        let categories = self.get_all::<Category>(OrderBy::None)?;
        let mut categories_total = 0;
        for c in categories {
        categories_total += c.balance;
        }

        return Ok(accounts_total - categories_total);
        */
    }

    pub fn get_account_display_list(&self) -> Result<Vec<AccountDisplay>, rusqlite::Error> {
        let query = "
            SELECT 
            accounts.rowid,
            accounts.display_name,
            sum(amount) as balance
                from accounts
                left join transactions on transactions.account_id = accounts.rowid           
                group by account_id
                ";

        let mut stmt = self.connection.prepare(query)?;
        let mut iter = stmt.query_map([], |row| {
            Ok(AccountDisplay {
                account_id: row.get(0)?,
                display_name: row.get(1)?,
                balance: row.get(2)?,
            })
        })?;

        let mut ret: Vec<AccountDisplay> = vec![];
        for c in iter {
            ret.push(c.unwrap());
        }

        Ok(ret)
    }

    pub fn get_account_history(
        &self,
        account_id: i64,
    ) -> Result<Vec<AccountHistoryEntry>, rusqlite::Error> {
        let query = format!(
            "
            SELECT 
            accounts.rowid,
            accounts.display_name,
            sum(transactions.amount) over (order by date asc) as running_total,
            transactions.date
            from accounts
            left join transactions on transactions.account_id = accounts.rowid
            where accounts.rowid = {}
            order by date asc
            ",
            account_id
        );

        let mut stmt = self.connection.prepare(&query)?;
        let mut iter = stmt.query_map([], |row| {
            Ok(AccountHistoryEntry {
                account_id: row.get(0)?,
                display_name: row.get(1)?,
                running_balance: row.get(2)?,
                date: row.get(3)?,
            })
        })?;

        let mut ret: Vec<AccountHistoryEntry> = vec![];
        for c in iter {
            ret.push(c.unwrap());
        }

        Ok(ret)
    }

    pub fn get_category_display_list(
        &self,
        unix_start: i64,
        unix_end: i64,
    ) -> Result<Vec<CategoryDisplay>, rusqlite::Error> {
        let query = format!(
            "
            SELECT 
            id, 
            display_name,
            coalesce(avg(amount), 0) as transactions_average,
            coalesce(sum(amount), 0) as transactions_total
            from categories 
            left join transactions on transactions.category_id = categories.rowid
            where coalesce(transactions.date, 0) between {unix_start} and {unix_end}
            group by id
            order by id
            "
        );

        let mut stmt = self.connection.prepare(&query)?;
        let mut iter = stmt.query_map([], |row| {
            Ok(CategoryDisplay {
                category_id: row.get(0)?,
                display_name: row.get(1)?,
                transaction_average: row.get(2)?,
                transaction_total: row.get(3)?,
            })
        })?;

        let mut ret: Vec<CategoryDisplay> = vec![];
        for c in iter {
            ret.push(c.unwrap());
        }

        // get list of all categories and add any missing to the list
        let mut new_cats: Vec<CategoryDisplay> = vec![];

        let all_cats: Vec<Category> = self.get_all(OrderBy::None).unwrap();
        for c_all in &all_cats {
            let mut found = false;

            for c_ret in &ret {
                if c_ret.category_id == c_all.id {
                    found = true;
                    break;
                }
            }

            if !found {
                new_cats.push(CategoryDisplay {
                    category_id: c_all.id,
                    display_name: c_all.display_name.clone(),
                    transaction_average: 0.0,
                    transaction_total: 0,
                });
            }
        }
        ret.append(&mut new_cats);

        Ok(ret)
    }

    pub fn import(&self, file_path: &str, account_id: i64) -> Result<(), String> {
        let mut headers = true;
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            // to appease the borrow checker
            let line_str = line.unwrap();

            if headers {
                headers = false;
                continue;
            }

            // account, date, payee, outflow, inflow, categories
            let parts: Vec<&str> = line_str.split(',').collect();

            // 1 date
            let mut date_str: String = parts.get(1).unwrap().to_string();
            date_str.push_str("T00:00:00");
            let date = PrimitiveDateTime::parse(&date_str, &Iso8601::DEFAULT).unwrap();
            let unix_date = date.assume_utc().unix_timestamp();

            // 2 payee
            let payee: String = parts.get(2).unwrap().to_string();

            // 3 outflow
            let outflow = parts.get(3).unwrap();
            let outflow: i64 = match outflow.trim().parse::<f64>() {
                Ok(v) => data::dollars_to_cents(v),
                Err(v) => 0,
            };

            // 4 inflow
            let inflow = parts.get(4).unwrap();
            let inflow: i64 = match inflow.trim().parse::<f64>() {
                Ok(v) => data::dollars_to_cents(v),
                Err(v) => 0,
            };

            // 5 category
            let category_str: String = parts.get(5).unwrap().to_string();

            // build transaction
            let mut trans = Transaction::new(payee, inflow, outflow, unix_date, account_id)?;

            // Only add categories for outflow
            if outflow > 0 {
                // get category id, otherwise create the category
                if self.category_exists(&category_str).unwrap() {
                    trans.category_id = self.get_category_id(&category_str).unwrap();
                } else {
                    // create that category
                    self.insert(Category::new(&category_str)).unwrap();
                    trans.category_id = self.get_category_id(&category_str).unwrap();
                }
            }

            self.insert(trans).unwrap();
        }

        Ok(())
    }
}
