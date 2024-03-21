#![allow(unused_imports, unused_variables, dead_code, unused_mut)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

use std::sync::Mutex;

use rusqlite::{params, Connection, Result};

use data::category::Category;
use data::category::CategoryList;
use data::account::Account;
use data::account::AccountList;

use database::Database;

struct State {
    db: Mutex<Database>,
}

#[tauri::command]
fn create_category(name: &str, ts: tauri::State<State>) {
    let conn = ts.db.lock().unwrap();
    conn.create_category(name);
}

#[tauri::command]
fn create_account(ts: tauri::State<State>) {
    let conn = ts.db.lock().unwrap();
    conn.create_account();
    println!("creating account");
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
