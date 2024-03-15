
pub mod category;
pub mod transaction;
pub mod account;

pub trait Table {
    fn get_table_name() -> String;
    fn get_table_schema() -> String;
    fn get_insert_schema() -> String;
    fn to_insert_data(&self) -> String;
}

