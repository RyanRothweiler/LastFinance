pub mod account;
pub mod category;
pub mod transaction;

pub fn dollars_to_cents(dollars: f64) -> i64 {
    return (dollars * 100.0) as i64;
}

pub trait Table {
    fn get_table_name() -> String;
    fn get_table_schema() -> String;
    fn get_insert_schema() -> String;
    fn to_insert_data(&self) -> String;
}
