use super::*;

macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        type_name_of(f)
            .rsplit("::")
            .find(|&part| part != "f" && part != "{{closure}}")
            .expect("Short function name")
    }};
}

fn test_setup_db(name: &str) -> Database {
    let db_dir = &format!("C:/Digital Archive/{}_db.db3", name);
    let _ = std::fs::remove_file(db_dir);

    let db = Database::new(db_dir);
    return db;
}

fn test_remove_db(name: &str, db: Database) {
    db.connection.close().unwrap();

    let db_dir = format!("C:/Digital Archive/{}_db.db3", name);
    std::fs::remove_file(db_dir).unwrap();
}

#[test]
fn database_setup() {
    let db = test_setup_db(function!());
    test_remove_db(function!(), db);
}

#[test]
fn insert_get() {
    let db = test_setup_db(function!());
    db.insert(Category::new("testing here")).unwrap();

    let cat_ret = db.get::<Category>(1);

    let mut cat_real = Category::new("testing here");
    cat_real.id = 1;

    assert_eq!(cat_ret, cat_real);

    test_remove_db(function!(), db);
}

#[test]
fn get_all_categories() {
    let db = test_setup_db(function!());

    db.insert(Category::new("first")).unwrap();
    db.insert(Category::new("second")).unwrap();

    let categories = db.get_all::<Category>(OrderBy::None).unwrap();
    assert_eq!(categories.len(), 2);

    test_remove_db(function!(), db);
}

#[test]
fn category_exists() {
    let db = test_setup_db(function!());

    db.insert(Category::new("first")).unwrap();
    db.insert(Category::new("second")).unwrap();

    assert_eq!(db.category_exists("first"), Ok(true));
    assert_eq!(db.category_exists("second"), Ok(true));
    assert_eq!(db.category_exists("third"), Ok(false));

    test_remove_db(function!(), db);
}

#[test]
fn get_category_id() {
    let db = test_setup_db(function!());

    db.insert(Category::new("first")).unwrap();
    db.insert(Category::new("second")).unwrap();

    assert_eq!(db.get_category_id("first"), Ok(1));
    assert_eq!(db.get_category_id("second"), Ok(2));
    assert_eq!(
        db.get_category_id("what??"),
        Err(rusqlite::Error::QueryReturnedNoRows)
    );

    test_remove_db(function!(), db);
}

#[test]
fn get_transaction_list_display() {
    let db = test_setup_db(function!());

    db.insert(Category::new("first")).unwrap();
    db.insert(Category::new("second")).unwrap();

    let mut trans = Transaction::new_raw("ryans transaction".to_string(), 100, 0, 0);
    trans.category_id = 1;
    db.insert(trans).unwrap();

    let mut trans = Transaction::new_raw("ryans second transaction".to_string(), 1, 0, 0);
    db.insert(trans).unwrap();

    let transaction_displays = db.get_transaction_list_display().unwrap();
    assert_eq!(transaction_displays.transactions.len(), 2);
    assert_eq!(
        transaction_displays.transactions[0].category_display,
        "first"
    );
    assert_eq!(transaction_displays.transactions[0].trans_raw.amount, 100);
    assert_eq!(transaction_displays.transactions[1].category_display, "");
    assert_eq!(transaction_displays.transactions[1].trans_raw.amount, 1);

    test_remove_db(function!(), db);
}

#[test]
fn get_account_display_list() {
    let db = test_setup_db(function!());

    db.insert(Account::new("first")).unwrap();
    db.import("test_input/month_daily_transactions.csv", 1)
        .unwrap();

    let account_list = db.get_account_display_list().unwrap();

    assert_eq!(account_list.len(), 1);
    assert_eq!(account_list[0].balance, 1101758);

    test_remove_db(function!(), db);
}

#[test]
fn get_account_history() {
    let db = test_setup_db(function!());

    db.insert(Account::new("first")).unwrap();
    db.import("test_input/month_daily_transactions.csv", 1)
        .unwrap();

    let entries = db.get_account_history(1).unwrap();

    assert_eq!(entries.len(), 31);
    assert_eq!(entries[0].running_balance, -11734);
    assert_eq!(entries[1].running_balance, 53126);
    assert_eq!(entries[2].running_balance, 579788);

    test_remove_db(function!(), db);
}

// TODO add start and end time testing here.
// Add testing to handle these cases
// - Categories for which there are no transacions
// - Categories for which there are transactions but their dates aren't within range
#[test]
fn get_category_display_list() {
    let db = test_setup_db(function!());

    db.insert(Category::new("first")).unwrap();
    db.insert(Category::new("second")).unwrap();
    db.insert(Category::new("third")).unwrap();

    let mut trans = Transaction::new_raw("ryans transaction".to_string(), 5, 10, 0);
    db.insert(trans).unwrap();

    let mut trans = Transaction::new_raw("ryans transaction".to_string(), 100, 10, 0);
    trans.category_id = 1;
    db.insert(trans).unwrap();

    let mut trans = Transaction::new_raw("ryans transaction".to_string(), 1000, 10, 0);
    trans.category_id = 1;
    db.insert(trans).unwrap();

    let mut trans = Transaction::new_raw("ryans second transaction".to_string(), 1, 10, 0);
    trans.category_id = 2;
    db.insert(trans).unwrap();

    let mut trans = Transaction::new_raw("ryans second transaction".to_string(), -10, 10, 0);
    trans.category_id = 2;
    db.insert(trans).unwrap();

    let category_displays = db.get_category_display_list(1, 10_000).unwrap();

    assert_eq!(category_displays.len(), 3);

    assert_eq!(category_displays[0].display_name, "first");
    assert_eq!(category_displays[0].transaction_total, 1100);

    assert_eq!(category_displays[1].display_name, "second");
    assert_eq!(category_displays[1].transaction_total, -9);

    assert_eq!(category_displays[2].display_name, "third");
    assert_eq!(category_displays[2].transaction_total, 0);

    test_remove_db(function!(), db);
}

#[test]
fn import() {
    let db = test_setup_db(function!());

    db.import("test_input/month_daily_transactions.csv", 0)
        .unwrap();

    let all_trans: Vec<Transaction> = db.get_all(OrderBy::Date).unwrap();

    assert_eq!(all_trans.len(), 31);

    assert_eq!(all_trans[0].payee, "Arbys");
    assert_eq!(all_trans[0].amount, data::dollars_to_cents(-117.34));

    let date = PrimitiveDateTime::parse("2024-01-01T00:00:00", &Iso8601::DEFAULT).unwrap();
    let unix_date = date.assume_utc().unix_timestamp();
    assert_eq!(all_trans[0].date, unix_date);

    assert_eq!(all_trans[1].payee, "The City");
    assert_eq!(all_trans[1].amount, data::dollars_to_cents(648.60));

    let date = PrimitiveDateTime::parse("2024-01-02T00:00:00", &Iso8601::DEFAULT).unwrap();
    let unix_date = date.assume_utc().unix_timestamp();
    assert_eq!(all_trans[1].date, unix_date);

    test_remove_db(function!(), db);
}
