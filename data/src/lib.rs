#![allow(dead_code)]

use std::fmt;

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
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum RytError {
    LockingDB,
    Rusqlite,

    CreateTransactionInflowAndOutlow,
    CreateTransactionNoInflowOrOutflow,

    PickFileNone,
    PathBufToStringFail,

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

impl fmt::Display for RytError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO actually display the error here
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DatabaseInfo {
    pub file_name: String,
    pub file_path: String,
}
