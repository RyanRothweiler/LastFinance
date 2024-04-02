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
fn create_category(name: &str, ts: tauri::State<State>) -> String {
    let conn_res = ts.db.lock();
    let conn = match conn_res {
        Ok(v) => v,
        Err(v) => return build_error("Error locking db."),
    };

    match conn.create_category(name) {
        Err(v) => return build_error(&format!("{:?}", v)),
        _ => {}
    }

    return build_ok();
}

#[tauri::command]
fn create_account(name: &str, ts: tauri::State<State>) -> String {
    let conn_res = ts.db.lock();
    let conn = match conn_res {
        Ok(v) => v,
        Err(v) => return build_error("Error locking db."),
    };

    match conn.create_account(name) {
        Err(v) => return build_error(&format!("{:?}", v)),
        _ => {}
    };

    return build_ok();
}

#[tauri::command]
fn create_transaction(trans: Transaction, ts: tauri::State<State>) -> String {
    let conn_res = ts.db.lock();
    let conn = match conn_res {
        Ok(v) => v,
        Err(v) => return build_error("Error locking db."),
    };

    match conn.insert(trans) {
        Err(v) => return build_error(&format!("{:?}", v)),
        _ => {}
    };

    return build_ok();
}
#[tauri::command]
fn fund_account(id: i64, cents: i64, ts: tauri::State<State>) -> String {
    let conn_res = ts.db.lock();
    let conn = match conn_res {
        Ok(v) => v,
        Err(v) => return build_error("Error locking db."),
    };

    match conn.fund_account(cents, id) {
        Err(v) => return build_error(&format!("{:?}", v)),
        _ => {}
    };

    return build_ok();
}

#[tauri::command]
fn get_unassigned(ts: tauri::State<State>) -> String {
    let ret: Result<f64, String> = Result::Ok(100.5);
    return serde_json::to_string(&ret).unwrap();
}

#[tauri::command]
fn get_all_categories(ts: tauri::State<State>) -> String {
    let mut list: CategoryList = CategoryList { categories: vec![] };
    list.categories = ts.db.lock().unwrap().get_all::<Category>().unwrap();
    return list.to_json_string();
}

#[tauri::command]
fn get_all_accounts(ts: tauri::State<State>) -> String {
    let mut list: AccountList = AccountList::new();
    list.accounts = ts.db.lock().unwrap().get_all::<Account>().unwrap();
    return list.to_json_string();
}

#[tauri::command]
fn get_all_transactions(ts: tauri::State<State>) -> String {
    let mut list: TransactionList = TransactionList::new();
    list.transactions = ts.db.lock().unwrap().get_all::<Transaction>().unwrap();
    return list.to_json_string();
}

#[tauri::command]
fn get_all_transactions_display(ts: tauri::State<State>) -> String {
    let ret = ts.db.lock().unwrap().get_transaction_list_display().unwrap();
    return ret.to_json_string();
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
