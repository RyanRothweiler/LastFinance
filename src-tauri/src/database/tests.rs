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
fn fund_get_ccount() {
    let db = test_setup_db(function!());
    db.insert(Account::new("Ryans Account")).unwrap();
    db.fund_account(data::dollars_to_cents(123.45), 1).unwrap();

    let ac = db.get::<Account>(1);
    assert_eq!(ac.balance, 12345);

    test_remove_db(function!(), db);
}

#[test]
fn get_all_categories() {
    let db = test_setup_db(function!());

    db.insert(Category::new("first")).unwrap();
    db.insert(Category::new("second")).unwrap();

    let categories = db.get_all::<Category>().unwrap();
    assert_eq!(categories.len(), 2);

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
fn get_unassigned() {
    let db = test_setup_db(function!());

    db.insert(Account::new("first account")).unwrap();
    db.fund_account(data::dollars_to_cents(100.0), 1).unwrap();

    db.insert(Category::new("first")).unwrap();
    db.insert(Category::new("second")).unwrap();

    let unassigned = db.get_unassigned().unwrap();
    assert_eq!(unassigned, data::dollars_to_cents(100.0));

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
fn get_category_display_list() {
    let db = test_setup_db(function!());

    db.insert(Category::new("first")).unwrap();
    db.insert(Category::new("second")).unwrap();
    db.insert(Category::new("third")).unwrap();

    let mut trans = Transaction::new_raw("ryans transaction".to_string(), 5, 0, 0);
    db.insert(trans).unwrap();

    let mut trans = Transaction::new_raw("ryans transaction".to_string(), 100, 0, 0);
    trans.category_id = 1;
    db.insert(trans).unwrap();

    let mut trans = Transaction::new_raw("ryans transaction".to_string(), 1000, 0, 0);
    trans.category_id = 1;
    db.insert(trans).unwrap();

    let mut trans = Transaction::new_raw("ryans second transaction".to_string(), 1, 0, 0);
    trans.category_id = 2;
    db.insert(trans).unwrap();

    let mut trans = Transaction::new_raw("ryans second transaction".to_string(), -10, 0, 0);
    trans.category_id = 2;
    db.insert(trans).unwrap();

    let category_displays = db.get_category_display_list().unwrap();

    assert_eq!(category_displays.len(), 3);

    assert_eq!(category_displays[0].display_name, "first");
    assert_eq!(category_displays[0].transaction_total, 1100);

    assert_eq!(category_displays[1].display_name, "second");
    assert_eq!(category_displays[1].transaction_total, -9);

    assert_eq!(category_displays[2].display_name, "third");
    assert_eq!(category_displays[2].transaction_total, 0);

    test_remove_db(function!(), db);
}
