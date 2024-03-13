#![allow(unused_imports, unused_variables, dead_code, unused_mut)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use rusqlite::{params, Connection, Result};

mod database;
use database::Database;

struct State {
    db: Mutex<Database>,
}

#[tauri::command]
fn create_category(name: &str, ts: tauri::State<State>) -> String {
    let conn = ts.db.lock().unwrap();
    conn.insert_category(name);
    println!("Inserted new category {}", name);

    let cat = database::Category {
        display_name: "wtf".to_string(),
    };

    return cat.to_json_string();
}

#[tauri::command]
fn get_all_categories(ts: tauri::State<State>) -> String {
    let mut list: database::CategoryList = database::CategoryList { categories: vec![] };
    list.categories = ts.db.lock().unwrap().get_all_categories();
    return list.to_json_string();
}

fn main() {
    let state = State {
        db: Mutex::new(Database::new("db.db3")),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            create_category,
            get_all_categories
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
