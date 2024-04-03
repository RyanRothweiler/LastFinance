#![allow(unused_imports, unused_variables, dead_code, unused_mut)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

use std::sync::Mutex;

use rusqlite::{params, Connection, Result};

use data::account::Account;
use data::account::AccountList;
use data::category::Category;
use data::category::CategoryList;
use data::transaction::*;
use data::ResultWrapped;

use database::Database;

struct State {
    db: Mutex<Database>,
}

fn build_error(err: &str) -> String {
    let ret: Result<(), String> = Result::Err(err.to_string());
    return serde_json::to_string(&ret).unwrap();
}

fn build_ok() -> String {
    let res: Result<(), String> = Result::Ok(());
    return serde_json::to_string(&res).unwrap();
}

#[tauri::command]
fn create_category(name: &str, ts: tauri::State<State>) -> ResultWrapped<(), String> {
    let conn = match ts.db.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking database".to_string()),
    };

    let cat = Category::new(name);
    match conn.insert(cat) {
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
        _ => {}
    };

    ResultWrapped::ok(())
}

#[tauri::command]
fn get_category_id(name: &str, ts: tauri::State<State>) -> ResultWrapped<i64, String> {
    let conn = match ts.db.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db.".to_string()),
    };

    match conn.get_category_id(name) {
        Ok(v) => return ResultWrapped::ok(v),
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
    };
}

#[tauri::command]
fn create_account(name: &str, ts: tauri::State<State>) -> ResultWrapped<(), String> {
    let conn = match ts.db.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    let account = Account::new(name);
    match conn.insert(account) {
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
        _ => {}
    };

    return ResultWrapped::ok(());
}

#[tauri::command]
fn create_transaction(trans: Transaction, ts: tauri::State<State>) -> ResultWrapped<(), String> {
    let conn = match ts.db.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    match conn.insert(trans) {
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
        _ => {}
    };

    ResultWrapped::ok(())
}

#[tauri::command]
fn fund_account(id: i64, cents: i64, ts: tauri::State<State>) -> ResultWrapped<(), String> {
    let conn = match ts.db.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    match conn.fund_account(cents, id) {
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
        _ => {}
    };

    ResultWrapped::ok(())
}

#[tauri::command]
fn get_unassigned(ts: tauri::State<State>) -> ResultWrapped<f64, String> {
    return ResultWrapped::ok(100.5);
}

#[tauri::command]
fn get_all_categories(ts: tauri::State<State>) -> ResultWrapped<CategoryList, String> {
    let conn = match ts.db.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    let mut list: CategoryList = CategoryList { categories: vec![] };
    list.categories = match conn.get_all::<Category>() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
    };

    ResultWrapped::ok(list)
}

#[tauri::command]
fn get_all_accounts(ts: tauri::State<State>) -> ResultWrapped<AccountList, String> {
    let conn = match ts.db.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    let mut list: AccountList = AccountList::new();
    list.accounts = match conn.get_all::<Account>() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
    };
    ResultWrapped::ok(list)
}

#[tauri::command]
fn get_all_transactions(ts: tauri::State<State>) -> ResultWrapped<TransactionList, String> {
    let conn = match ts.db.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    let mut list: TransactionList = TransactionList::new();
    list.transactions = match conn.get_all::<Transaction>() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
    };
    ResultWrapped::ok(list)
}

#[tauri::command]
fn get_all_transactions_display(
    ts: tauri::State<State>,
) -> ResultWrapped<TransactionDisplayList, String> {
    let conn = match ts.db.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    let ret = match conn.get_transaction_list_display() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
    };

    ResultWrapped::ok(ret)
}
fn main() {
    let state = State {
        db: Mutex::new(Database::new("C:/Digital Archive/db.db3")),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            create_category,
            create_account,
            create_transaction,
            get_all_categories,
            get_all_accounts,
            get_all_transactions,
            get_all_transactions_display,
            fund_account,
            get_unassigned,
            get_category_id,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
