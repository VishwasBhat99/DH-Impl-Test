use crate::structs::*;
use rbdate::timestamp;
use rbdate::NaiveDate;
use sdb_day_convention::conventions::Days;

pub fn get_last_pay_date(acc_st_dt: NaiveDate, as_on_date: &NaiveDate, pay_freq: i64) -> NaiveDate {
    let mut next_date;
    let mut st_dt = acc_st_dt;
    let mut last_pay_dt = acc_st_dt;
    loop {
        next_date = rbdate::incr_dt_by_mon_presrv_eom(st_dt, pay_freq as usize).unwrap();
        if next_date > *as_on_date {
            break;
        }
        st_dt = next_date;
        last_pay_dt = next_date;
    }

    last_pay_dt
}

pub fn interest_amount(o_a: f64, i_r: f32, days: Days) -> f64 {
    let num_days = days.days_btw_dts;
    let days_in_yr = days.day_in_yr as f64;
    (o_a * i_r as f64 * num_days as f64) / (days_in_yr * 100.0)
}

pub fn new_cashflow(i_a: f64, p_a: f64, d: &NaiveDate) -> Cashflow {
    let cf = Cashflow {
        int_amt: i_a,
        prin_amt: p_a,
        cf_date: timestamp(*d),
    };
    cf
}

pub fn get_pay_freq(pay_freq: &str) -> i64 {
    return match pay_freq {
        "M" => 1,
        "Q" => 3,
        "H" => 6,
        "Y" => 12,
        _ => 1,
    };
}

pub fn get_curr_month_idx(cf_date: NaiveDate, as_on_date: NaiveDate, freq: i64) -> usize {
    let mut next_date;
    let mut st_dt = as_on_date;
    let mut month_idx: usize = 1;
    loop {
        next_date = rbdate::incr_dt_by_mon_presrv_eom(st_dt, freq as usize).unwrap();
        if next_date > cf_date {
            break;
        }
        month_idx += 1;
        st_dt = next_date;
    }
    month_idx
}
