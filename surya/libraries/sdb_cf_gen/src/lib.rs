#![cfg_attr(feature = "strict-build", deny(warnings))]
#![cfg_attr(feature = "cargo-clippy", deny(warnings))]
#![cfg_attr(feature = "cargo-clippy", deny(clippy::expect_fun_call))]
#![cfg_attr(
    feature = "cargo-clippy",
    deny(
        clippy::result_unwrap_used,
        clippy::panicking_unwrap,
        clippy::option_unwrap_used
    )
)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy::float_cmp))]
extern crate math;
extern crate rbdate;
extern crate sdb_day_convention;

mod structs;
mod tests;

use math::round::{ceil, half_away_from_zero};
use rbdate::*;
use sdb_day_convention::*;
pub use structs::*;

pub fn calc_daywise_cf(
    open_prin: f64,
    rate: f64,
    emi_amt: f64,
    emi_start_date: NaiveDate,
    precision: i8,
) -> Option<Cashflow> {
    let month_end_date = get_month_end_date(emi_start_date);
    if month_end_date == emi_start_date {
        return None;
    }

    // This int_basis calculation can never be crashed
    let int_basis = days_with_convn(emi_start_date, month_end_date, &Conventions::ACTbyACT)
        .expect("Unexpected error occured while computing int_basis.");
    let int_amt = calc_int_amt_day_wise(open_prin, rate, int_basis);
    let prin_amt = calc_prin_amt(open_prin, emi_amt, int_amt);
    Some(gen_cf(int_amt, prin_amt, month_end_date, precision))
}

pub fn calc_monthly_cf(
    open_prin: f64,
    rate: f64,
    emi_amt: f64,
    freq: f64,
    cf_date: NaiveDate,
    precision: i8,
) -> Cashflow {
    let int_amt = calc_int_amt_monthly(open_prin, rate, freq);
    let prin_amt = calc_prin_amt(open_prin, emi_amt, int_amt);
    gen_cf(int_amt, prin_amt, cf_date, precision)
}

pub fn calc_int_amt_day_wise(prin_amt: f64, rate: f64, int_basis: Days) -> f64 {
    prin_amt * rate * int_basis.days_btw_dts as f64 / (int_basis.days_btw_dts * 100) as f64
}

pub fn calc_int_amt_monthly(prin_amt: f64, rate: f64, freq: f64) -> f64 {
    prin_amt * rate * freq as f64 / (12 * 100) as f64
}

pub fn calc_emi_amt(prin_amt: f64, rate: f64, freq: f64, precision: i8) -> f64 {
    ceil(
        prin_amt * (rate / (12.0 * 100.0))
            / (1.0 - (1.0 + (rate / (12.0 * 100.0))).powf(-1.0 * freq)),
        precision,
    )
}

pub fn calc_prin_amt(open_prin: f64, emi_amt: f64, int_amt: f64) -> f64 {
    if open_prin < emi_amt {
        open_prin
    } else {
        emi_amt - int_amt
    }
}

pub fn get_freq(freq: &str) -> usize {
    match freq
        .replace(" ", "")
        .replace("-", "")
        .to_lowercase()
        .as_str()
    {
        "monthly" => 1,
        "bimonthly" => 2,
        "quarterly" => 3,
        "halfyearly" => 6,
        "annually" => 12,
        _ => 0,
    }
}

pub fn calc_bult_cf(ost_amt: f64, mat_date: NaiveDate, precision: i8) -> Cashflow {
    gen_cf(0.0, ost_amt, mat_date, precision)
}

pub fn calc_resid_int(int_amt: f64, prin_amt: f64, emi_amt: f64) -> f64 {
    if prin_amt > emi_amt {
        0.0
    } else {
        emi_amt - (prin_amt + int_amt)
    }
}

pub fn gen_cf(int_amt: f64, prin_amt: f64, cf_date: NaiveDate, precision: i8) -> Cashflow {
    Cashflow {
        int_amt: half_away_from_zero(int_amt, precision),
        prin_amt: half_away_from_zero(prin_amt, precision),
        date: timestamp(cf_date),
    }
}
