pub mod account;
pub mod category;
pub mod transaction;

pub fn dollars_to_cents(dollars: f64) -> i64 {
    return (dollars * 100.0) as i64;
}

pub fn cents_to_dollars(cents: i64) -> f64 {
    return (cents as f64) / 100.0;
}
