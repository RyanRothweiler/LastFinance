#[derive(Debug)]
pub struct Category {
    pub display_name: String,
}

pub const TABLE_ID: &str = "category";

impl Category {
    pub fn sql_schema() -> String {
        return "display_name    TEXT NOT NULL".to_string();
    }
}
