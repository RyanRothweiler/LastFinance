
pub mod category;
pub mod transaction;

pub trait Table {
    fn get_table_name() -> String;
    fn get_table_schema() -> String;
}

