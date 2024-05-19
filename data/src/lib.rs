#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use num_format::{Locale, ToFormattedString};
use regex::Regex;

pub mod account;
pub mod category;
pub mod category_transfer;
pub mod transaction;

pub fn dollars_to_cents(dollars: f64) -> i64 {
    return (dollars * 100.0).round() as i64;
}

pub fn cents_to_dollars(cents: i64) -> f64 {
    return (cents as f64) / 100.0;
}

// TODO handle cents here? num_format doesn't seem to handle reals.
pub fn amount_to_display(cents: i64) -> String {
    let dollars = cents_to_dollars(cents) as i64;
    return format!("${}", dollars.to_formatted_string(&Locale::en)).to_string();
}

// NOTE the handle_invoke in the frontent can't handle params in the enum,
// so don't add any.
#[derive(Serialize, Deserialize, Debug)]
pub enum RytError {
    LockingDB,
    Rusqlite,

    // handle_invoke erorrs
    FromBindingRegexError,
    BindingDeserializationError,
    TauriSysError(String),
}

impl<T> From<std::sync::PoisonError<T>> for RytError {
    fn from(_error: std::sync::PoisonError<T>) -> Self {
        RytError::LockingDB
    }
}

impl RytError {
    pub fn from_binding(value: String) -> Self {
        let re = Regex::new(r#"JsValue\((?P<error>"\w+")\)"#).unwrap();
        match re.captures(&value) {
            Some(caps) => match caps["error"].parse::<String>() {
                Ok(str) => match serde_json::from_str::<RytError>(str.as_str()) {
                    Ok(error) => error,
                    _ => RytError::BindingDeserializationError,
                },
                _ => unreachable!(),
            },
            _ => RytError::FromBindingRegexError,
        }
    }
}

// For some reason
// Necessary because wasm_bindgen requires serde to serialize between javascript and rust
// That is my guess atleast, wasm_bindgen from_value doesn't work on regular results
// so we need to wrap the result.
// T is ok
// V is error
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

#[derive(Serialize, Deserialize, Clone)]
pub struct DatabaseInfo {
    pub file_name: String,
    pub file_path: String,
}
