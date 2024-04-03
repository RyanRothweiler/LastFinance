use serde::{Deserialize, Serialize};

pub mod account;
pub mod category;
pub mod category_transfer;
pub mod transaction;

pub fn dollars_to_cents(dollars: f64) -> i64 {
    return (dollars * 100.0) as i64;
}

pub fn cents_to_dollars(cents: i64) -> f64 {
    return (cents as f64) / 100.0;
}

// Necessary because wasm_bindgen requires serde to serialize between javascript and rust
// so we need to wrap the result.
#[derive(Serialize, Deserialize)]
pub struct ResultWrapped<T, V> {
    pub res: std::result::Result<T, V>,
}

impl<T, V> ResultWrapped<T, V> {
    pub fn error(error: V) -> ResultWrapped<T, V> {
        ResultWrapped {
            res: std::result::Result::Err(error),
        }
    }

    pub fn ok(inn: T) -> ResultWrapped<T, V> {
        ResultWrapped {
            res: std::result::Result::Ok(inn),
        }
    }
}
