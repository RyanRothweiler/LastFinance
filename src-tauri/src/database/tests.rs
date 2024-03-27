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
    db.create_category("testing here").unwrap();

    let cat_ret = db.get::<Category>(1);
    assert_eq!(cat_ret, Category::new("testing here"));

    test_remove_db(function!(), db);
}

#[test]
fn fund_get_ccount() {
    let db = test_setup_db(function!());
    db.create_account("Ryans Account").unwrap();
    db.fund_account(data::dollars_to_cents(123.45), 1).unwrap();

    let ac = db.get::<Account>(1);
    assert_eq!(ac.balance, 12345);

    test_remove_db(function!(), db);
}

#[test]
fn get_all_categories() {
    let db = test_setup_db(function!());

    db.create_category("first").unwrap();
    db.create_category("second").unwrap();

    let categories = db.get_all::<Category>().unwrap();
    assert_eq!(categories.len(), 2);

    test_remove_db(function!(), db);
}

#[test]
fn get_unassigned() {
    let db = test_setup_db(function!());

    db.create_account("first account").unwrap();
    db.fund_account(data::dollars_to_cents(100.0), 1).unwrap();

    db.create_category("first").unwrap();
    db.create_category("second").unwrap();

    let unassigned = db.get_unassigned().unwrap();
    assert_eq!(unassigned, data::dollars_to_cents(100.0));

    test_remove_db(function!(), db);
}
