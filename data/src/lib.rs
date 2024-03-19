pub mod account;
pub mod category;
pub mod transaction;

pub fn dollars_to_cents(dollars: f64) -> i64 {
    return (dollars * 100.0) as i64;
}
