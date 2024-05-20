#![allow(unused_variables, dead_code, unused_mut, unused_imports)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod persistent_data;

use std::path::PathBuf;
use std::sync::Mutex;

use chrono::prelude::*;
use rusqlite::Result;

use data::account::*;
use data::category::*;
use data::transaction::*;
use data::RytError;
use data::{DatabaseInfo, OptionWrapped, ResultWrapped};

use database::{Database, OrderBy};
use persistent_data::PersistentData;

use tauri::api::dialog;

struct State {
    db: Database,
    persist_data: PersistentData,
}

struct GuardedState {
    state: Mutex<State>,
}

fn rusqlite_to_ryt(_rusq_error: rusqlite::Error) -> RytError {
    // TODO more info about error here
    RytError::Rusqlite
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
fn create_category(name: &str, ts: tauri::State<GuardedState>) -> Result<i64, RytError> {
    let state = ts.state.lock()?;

    let cat = Category::new(name);
    return state.db.insert(cat).map_err(rusqlite_to_ryt);
}

#[tauri::command]
fn get_category_id(name: &str, ts: tauri::State<GuardedState>) -> Result<i64, RytError> {
    let state = ts.state.lock()?;
    return state.db.get_category_id(name).map_err(rusqlite_to_ryt);
}

#[tauri::command]
// sb -> starting balance
fn create_account(name: &str, sb: i64, ts: tauri::State<GuardedState>) -> Result<i64, RytError> {
    let state = ts.state.lock()?;

    let mut account_id: i64;
    let account = Account::new(name);
    match state.db.insert(account) {
        Err(v) => return Err(rusqlite_to_ryt(v)),
        Ok(v) => account_id = v,
    };

    let starting_trans = Transaction::new(
        "Starting Balance".to_string(),
        sb,
        0,
        Local::now().timestamp(),
        account_id,
    )?;

    return state.db.insert(starting_trans).map_err(rusqlite_to_ryt);
}

#[tauri::command]
fn create_transaction(trans: Transaction, ts: tauri::State<GuardedState>) -> Result<i64, RytError> {
    let state = ts.state.lock()?;
    return state.db.insert(trans).map_err(rusqlite_to_ryt);
}

#[tauri::command]
fn get_all_transactions_display(
    ts: tauri::State<GuardedState>,
) -> Result<TransactionDisplayList, RytError> {
    let state = ts.state.lock()?;
    return state
        .db
        .get_transaction_list_display()
        .map_err(rusqlite_to_ryt);
}

#[tauri::command]
fn get_category_display_list(
    start: i64,
    end: i64,
    ts: tauri::State<GuardedState>,
) -> Result<Vec<CategoryDisplay>, RytError> {
    let state = ts.state.lock()?;
    return state
        .db
        .get_category_display_list(start, end)
        .map_err(rusqlite_to_ryt);
}

#[tauri::command]
fn get_account_display_list(
    ts: tauri::State<GuardedState>,
) -> ResultWrapped<Vec<AccountDisplay>, String> {
    let state = match ts.state.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    let ret = match state.db.get_account_display_list() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
    };

    ResultWrapped::ok(ret)
}

#[tauri::command]
fn get_account_history(
    acid: i64,
    ts: tauri::State<GuardedState>,
) -> Result<Vec<AccountHistoryEntry>, RytError> {
    let state = ts.state.lock()?;
    return state.db.get_account_history(acid).map_err(rusqlite_to_ryt);
}

#[tauri::command]
fn import(acc: i64, ts: tauri::State<GuardedState>) -> Result<(), RytError> {
    // Show dialog
    let file_path_buf: PathBuf = dialog::blocking::FileDialogBuilder::new()
        .add_filter("CSV", &["csv"])
        .pick_file()
        .ok_or(RytError::PickFileNone)?;

    // path buf to string
    let selected_file_path = file_path_buf
        .as_path()
        .to_str()
        .ok_or(RytError::PathBufToStringFail)?;

    let state = ts.state.lock()?;
    return state.db.import(selected_file_path, acc);
}

#[tauri::command]
fn get_db_info(ts: tauri::State<GuardedState>) -> Result<DatabaseInfo, RytError> {
    let state = ts.state.lock()?;

    return Ok(DatabaseInfo {
        file_name: state.db.file_name.clone(),
        file_path: state.db.folder_dir.clone(),
    });
}

#[tauri::command]
fn create_db(ts: tauri::State<GuardedState>) -> Result<(), RytError> {
    let mut state = ts.state.lock()?;

    let mut file_path_buf = dialog::blocking::FileDialogBuilder::new()
        .save_file()
        .ok_or(RytError::PickFileNone)?;

    file_path_buf.set_extension("db3");
    state.db = Database::new(file_path_buf, &mut state.persist_data);

    Ok(())
}

#[tauri::command]
fn open_db(ts: tauri::State<GuardedState>) -> Result<(), RytError> {
    let mut state = ts.state.lock()?;

    let file_path_buf: PathBuf = dialog::blocking::FileDialogBuilder::new()
        .add_filter("DB3", &["db3"])
        .pick_file()
        .ok_or(RytError::PickFileNone)?;

    state.db = Database::new(file_path_buf, &mut state.persist_data);

    Ok(())
}

#[tauri::command]
fn export_to_csv(ts: tauri::State<GuardedState>) -> ResultWrapped<(), String> {
    let mut state = match ts.state.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    let mut file_path_buf: PathBuf = match dialog::blocking::FileDialogBuilder::new().save_file() {
        Some(v) => v,
        None => return ResultWrapped::error("Error picking file path.".to_string()),
    };

    file_path_buf.set_extension("csv");
    state.db.export_csv(file_path_buf).unwrap();

    ResultWrapped::ok(())
}

fn main() {
    // TODO handle error
    let mut persist_data = PersistentData::new_from_file().unwrap();
    if persist_data.last_db_path.len() == 0 {
        println!("No previous known db. Creating default location.");
        persist_data.last_db_path = "C:/Digital Archive/db.db3".to_string();
    }

    let db = Database::new(PathBuf::from(&persist_data.last_db_path), &mut persist_data);

    let guarded_state = GuardedState {
        state: Mutex::new(State {
            persist_data: persist_data,
            db: db,
        }),
    };

    tauri::Builder::default()
        .manage(guarded_state)
        .invoke_handler(tauri::generate_handler![
            create_category,
            create_account,
            create_transaction,
            get_all_transactions_display,
            get_category_id,
            get_category_display_list,
            get_account_display_list,
            get_account_history,
            import,
            get_db_info,
            create_db,
            open_db,
            export_to_csv,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
