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

fn build_error(err: &str) -> String {
    let ret: Result<(), String> = Result::Err(err.to_string());
    return serde_json::to_string(&ret).unwrap();
}

fn build_ok() -> String {
    let res: Result<(), String> = Result::Ok(());
    return serde_json::to_string(&res).unwrap();
}

#[tauri::command]
fn create_category(name: &str, ts: tauri::State<GuardedState>) -> ResultWrapped<(), String> {
    let state = match ts.state.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking database".to_string()),
    };

    let cat = Category::new(name);
    match state.db.insert(cat) {
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
        _ => {}
    };

    ResultWrapped::ok(())
}

#[tauri::command]
fn get_category_id(name: &str, ts: tauri::State<GuardedState>) -> ResultWrapped<i64, String> {
    let state = match ts.state.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db.".to_string()),
    };

    match state.db.get_category_id(name) {
        Ok(v) => return ResultWrapped::ok(v),
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
    };
}

#[tauri::command]
// sb -> starting balance
fn create_account(
    name: &str,
    sb: i64,
    ts: tauri::State<GuardedState>,
) -> ResultWrapped<(), String> {
    let state = match ts.state.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    let mut account_id: i64;
    let account = Account::new(name);
    match state.db.insert(account) {
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
        Ok(v) => account_id = v,
    };

    let starting_trans = match Transaction::new(
        "Starting Balance".to_string(),
        sb,
        0,
        Local::now().timestamp(),
        account_id,
    ) {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
    };
    match state.db.insert(starting_trans) {
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
        _ => {}
    };

    return ResultWrapped::ok(());
}

#[tauri::command]
fn create_transaction(
    trans: Transaction,
    ts: tauri::State<GuardedState>,
) -> ResultWrapped<(), String> {
    let state = match ts.state.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    match state.db.insert(trans) {
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
        _ => {}
    };

    ResultWrapped::ok(())
}

#[tauri::command]
fn get_unassigned(ts: tauri::State<GuardedState>) -> ResultWrapped<f64, String> {
    return ResultWrapped::ok(100.5);
}

#[tauri::command]
fn get_all_categories(ts: tauri::State<GuardedState>) -> ResultWrapped<CategoryList, String> {
    let state = match ts.state.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    let mut list: CategoryList = CategoryList { categories: vec![] };
    list.categories = match state.db.get_all::<Category>(OrderBy::None) {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
    };

    ResultWrapped::ok(list)
}

#[tauri::command]
fn get_all_accounts(ts: tauri::State<GuardedState>) -> ResultWrapped<Vec<Account>, String> {
    let state = match ts.state.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    let list = match state.db.get_all::<Account>(OrderBy::None) {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
    };
    ResultWrapped::ok(list)
}

#[tauri::command]
fn get_all_transactions(ts: tauri::State<GuardedState>) -> ResultWrapped<TransactionList, String> {
    let state = match ts.state.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    let mut list: TransactionList = TransactionList::new();
    list.transactions = match state.db.get_all::<Transaction>(OrderBy::None) {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
    };
    ResultWrapped::ok(list)
}

#[tauri::command]
fn get_all_transactions_display(
    ts: tauri::State<GuardedState>,
) -> ResultWrapped<TransactionDisplayList, String> {
    let state = match ts.state.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    let ret = match state.db.get_transaction_list_display() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
    };

    ResultWrapped::ok(ret)
}

#[tauri::command]
fn get_category_display_list(
    start: i64,
    end: i64,
    ts: tauri::State<GuardedState>,
) -> ResultWrapped<Vec<CategoryDisplay>, String> {
    let state = match ts.state.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    let ret = match state.db.get_category_display_list(start, end) {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
    };

    ResultWrapped::ok(ret)
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
) -> ResultWrapped<Vec<AccountHistoryEntry>, String> {
    let state = match ts.state.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    let ret = match state.db.get_account_history(acid) {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
    };

    ResultWrapped::ok(ret)
}

#[tauri::command]
fn import(acc: i64, ts: tauri::State<GuardedState>) -> ResultWrapped<(), String> {
    let file_path_buf = match dialog::blocking::FileDialogBuilder::new()
        .add_filter("CSV", &["csv"])
        .pick_file()
    {
        Some(v) => v,
        None => {
            return ResultWrapped::error("Error creating file dialog.".to_string());
        }
    };

    let selected_file_path = match file_path_buf.as_path().to_str() {
        Some(v) => v,
        None => {
            return ResultWrapped::error("Invalid file path selected.".to_string());
        }
    };

    let state = match ts.state.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };
    match state.db.import(selected_file_path, acc) {
        Err(v) => return ResultWrapped::error(format!("{:?}", v)),
        _ => {}
    };

    ResultWrapped::ok(())
}

#[tauri::command]
fn file_dialog() -> OptionWrapped<String> {
    let file_path_buf = match dialog::blocking::FileDialogBuilder::new()
        .add_filter("CSV", &["csv"])
        .pick_file()
    {
        Some(v) => v,
        None => return OptionWrapped::none(),
    };

    match file_path_buf.as_path().to_str() {
        Some(v) => return OptionWrapped::some(v.to_string()),
        None => return OptionWrapped::none(),
    };
}

#[tauri::command]
fn get_db_info(ts: tauri::State<GuardedState>) -> ResultWrapped<DatabaseInfo, String> {
    let state = match ts.state.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    return ResultWrapped::ok(DatabaseInfo {
        file_name: state.db.file_name.clone(),
        file_path: state.db.folder_dir.clone(),
    });
}

#[tauri::command]
fn create_db(ts: tauri::State<GuardedState>) -> ResultWrapped<(), String> {
    let mut state = match ts.state.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    let mut file_path_buf = match dialog::blocking::FileDialogBuilder::new().save_file() {
        Some(v) => v,
        None => return ResultWrapped::error("Error picking file path.".to_string()),
    };

    file_path_buf.set_extension("db3");
    state.db = Database::new(file_path_buf, &mut state.persist_data);
    ResultWrapped::ok(())
}

#[tauri::command]
fn open_db(ts: tauri::State<GuardedState>) -> ResultWrapped<(), String> {
    let mut state = match ts.state.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    let mut file_path_buf: PathBuf = match dialog::blocking::FileDialogBuilder::new()
        .add_filter("DB3", &["db3"])
        .pick_file()
    {
        Some(v) => v,
        None => return ResultWrapped::error("Error picking file path.".to_string()),
    };

    state.db = Database::new(file_path_buf, &mut state.persist_data);

    ResultWrapped::ok(())
}

#[tauri::command]
fn export_to_csv(ts: tauri::State<GuardedState>) -> ResultWrapped<(), String> {
    let mut state = match ts.state.lock() {
        Ok(v) => v,
        Err(v) => return ResultWrapped::error("Error locking db".to_string()),
    };

    state
        .db
        .export_csv(PathBuf::from("C:/Digital Archive/export.csv"))
        .unwrap();

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
            get_all_categories,
            get_all_accounts,
            get_all_transactions,
            get_all_transactions_display,
            get_unassigned,
            get_category_id,
            get_category_display_list,
            get_account_display_list,
            get_account_history,
            file_dialog,
            import,
            get_db_info,
            create_db,
            open_db,
            export_to_csv,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
