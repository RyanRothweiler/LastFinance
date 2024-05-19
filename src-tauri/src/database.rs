#![allow(unused_macros)]

mod table_actions;
use table_actions::TableActions;

use rusqlite::{Connection, Result};

use time::format_description::well_known::Iso8601;
use time::PrimitiveDateTime;

use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};

use data::account::*;
use data::category::*;
use data::category_transfer::CategoryTransfer;
use data::transaction::*;
use data::RytError;

use super::persistent_data::PersistentData;

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
    // TODO change this to path_buf?
    pub fn new(file_path: PathBuf, persist_data: &mut PersistentData) -> Database {
        let path_str: &str = file_path.to_str().unwrap();
        println!("Opening DB {0}", path_str);

        persist_data.set_last_db(path_str);

        // TODO handle error
        let connection = Connection::open(path_str).unwrap();

        let db = Database {
            connection: connection,
            file_name: file_path.file_name().unwrap().to_str().unwrap().to_string(),
            folder_dir: file_path.parent().unwrap().to_str().unwrap().to_string(),
        };

        // Create schema if needed
        {
            fn setup_table<T: TableActions>(db: &Database) {
                let table_name = &T::get_table_name();

                if !db.table_exists(table_name) {
                    let query =
                        format!("CREATE TABLE {} ( {} )", table_name, T::get_table_schema());
                    db.connection.execute(&query, ()).unwrap();
                    println!("Created table {}", table_name);
                }
            }

            setup_table::<Category>(&db);
            setup_table::<Account>(&db);
            setup_table::<Transaction>(&db);
            setup_table::<CategoryTransfer>(&db);
        }

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

    // TODO handle error this whole method here
    pub fn export_csv(&self, mut path: PathBuf) -> Result<(), String> {
        println!("Exporting to {} ", path.to_str().unwrap());

        // Remove past file
        let _ = std::fs::remove_file(&path);

        // Create append handle
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .unwrap();

        // Write column headers
        file.write("payee, amount, date, account, category".as_bytes())
            .unwrap();
        file.write("\n".as_bytes()).unwrap();

        struct Row {
            payee: String,
            amount: i64,
            date: i64,
            account: String,
            category: String,
        }

        let query = "
            select 
                payee, 
                amount, 
                date, 
                ifnull(accounts.display_name, '') as account_display_name,
                ifnull(categories.display_name, '') as category_display_name
            from transactions 
                left join categories on transactions.category_id = categories.rowid
                left join accounts on transactions.account_id = accounts.rowid
            ORDER BY date
            ";

        let mut stmt = self.connection.prepare(&query).unwrap();
        let mut iter: Vec<_> = stmt
            .query_map([], |row| {
                let r = Row {
                    payee: row.get(0).unwrap(),
                    amount: row.get(1).unwrap(),
                    date: row.get(2).unwrap(),
                    account: row.get(3).unwrap(),
                    category: row.get(4).unwrap(),
                };

                // payee
                file.write(r.payee.as_bytes()).unwrap();
                file.write(",".as_bytes()).unwrap();

                // amount
                file.write(format!("{}", data::cents_to_dollars(r.amount)).as_bytes())
                    .unwrap();
                file.write(",".as_bytes()).unwrap();

                // date
                let date_offset = time::OffsetDateTime::from_unix_timestamp(r.date).unwrap();
                let format_desc = time::format_description::parse("[year]-[month]-[day]").unwrap();
                let date_disp: String = date_offset.format(&format_desc).unwrap();
                file.write(date_disp.as_bytes()).unwrap();
                file.write(",".as_bytes()).unwrap();

                // account
                file.write(r.account.as_bytes()).unwrap();
                file.write(",".as_bytes()).unwrap();

                // category
                file.write(r.category.as_bytes()).unwrap();

                file.write("\n".as_bytes()).unwrap();
                Ok(())
            })
            .unwrap()
            .collect();

        println!("Successfully exported");
        Ok(())
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
            select 
            payee, 
        amount, 
        date, 
        account_id,
        ifnull(categories.display_name, '') as category_display_name,
        ifnull(accounts.display_name, '') as account_display_name
            from transactions 
            left join categories on transactions.category_id = categories.rowid
            left join accounts on transactions.account_id = accounts.rowid
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

    pub fn import(&self, file_path: &str, account_id: i64) -> Result<(), RytError> {
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
