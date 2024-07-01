pub mod amb_account;
pub mod core_master;
pub mod input_account;

pub fn calc_per_val(amt: f64, per: f64) -> f64 {
    amt * per / 100.0
}
