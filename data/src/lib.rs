use serde::{Deserialize, Serialize};

use num_format::{Locale, ToFormattedString};

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

// TODO handle cents here? num_format doesn't seem to handle reals.
pub fn amount_to_display(cents: i64) -> String {
    let dollars = cents_to_dollars(cents) as i64;
    return dollars.to_formatted_string(&Locale::en);
}

// Necessary because wasm_bindgen requires serde to serialize between javascript and rust
// That is my guess atleast wasm_bindget from_value doesn't work on regular results
// so we need to wrap the result.
#[derive(Serialize, Deserialize)]
pub struct ResultWrapped<T, V> {
    pub res: std::result::Result<T, V>,
}

impl<T, V> ResultWrapped<T, V> {
    pub fn error(error: V) -> Self {
        ResultWrapped {
            res: std::result::Result::Err(error),
        }
    }

    pub fn ok(inn: T) -> Self {
        ResultWrapped {
            res: std::result::Result::Ok(inn),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OptionWrapped<T> {
    pub res: std::option::Option<T>,
}

impl<T> OptionWrapped<T> {
    pub fn some(data: T) -> Self {
        OptionWrapped {
            res: std::option::Option::Some(data),
        }
    }

    pub fn none() -> Self {
        OptionWrapped {
            res: std::option::Option::None,
        }
    }
}
