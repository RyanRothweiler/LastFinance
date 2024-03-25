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

use database::Database;

struct State {
    db: Mutex<Database>,
}

#[tauri::command]
fn create_category(name: &str, ts: tauri::State<State>) -> String {
    let conn_res = ts.db.lock();
    let conn = match conn_res {
        Ok(v) => v,
        Err(v) => {
            let ret: Result<(), String> = Result::Err("Error locking db.".to_string());
            return serde_json::to_string(&ret).unwrap();
        }
    };

    let res = conn.create_category(name);
    return serde_json::to_string(&res).unwrap();
}

#[tauri::command]
fn create_account(name: &str, ts: tauri::State<State>) -> String {
    let conn_res = ts.db.lock();
    let conn = match conn_res {
        Ok(v) => v,
        Err(v) => {
            let ret: Result<(), String> = Result::Err("Error locking db.".to_string());
            return serde_json::to_string(&ret).unwrap();
        }
    };

    let res = conn.create_account(name);
    return serde_json::to_string(&res).unwrap();
}

#[tauri::command]
fn get_all_categories(ts: tauri::State<State>) -> String {
    let mut list: CategoryList = CategoryList { categories: vec![] };
    list.categories = ts.db.lock().unwrap().get_all::<Category>();
    return list.to_json_string();
}

#[tauri::command]
fn get_all_accounts(ts: tauri::State<State>) -> String {
    let mut list: AccountList = AccountList::new();
    list.accounts = ts.db.lock().unwrap().get_all::<Account>();
    return list.to_json_string();
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
            get_all_categories,
            get_all_accounts,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
