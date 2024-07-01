use rbdate::NaiveDate;
use rbdate::{date_from_timestamp, num_days_start_to_end};

pub fn get_derived_fields(
    ost_prin_amt: &f64,
    int_amt_aip: &f64,
    int_rate: &f64,
    mat_dt: &i64,
    next_reset_dt: &i64,
    penalty_amt: &f64,
    mat_tenor: &mut i64,
    int_tenor: &mut i64,
    prin_amt: &mut f64,
    int_amt: &mut f64,
    int_amt_30_days: &mut f64,
    tot_int_amt: &mut f64,
    to_bucket: &mut i64,
    from_bucket: &mut i64,
    int_basis: &mut i64,
    as_on_date: &NaiveDate,
) {
    *mat_tenor = num_days_start_to_end(*as_on_date, date_from_timestamp(*mat_dt));
    if *next_reset_dt == 0 {
        *int_tenor = if as_on_date > &date_from_timestamp(*mat_dt) {
            -num_days_start_to_end(date_from_timestamp(*mat_dt), *as_on_date)
        } else {
            num_days_start_to_end(*as_on_date, date_from_timestamp(*mat_dt))
        };
    } else {
        *int_tenor = if as_on_date > &date_from_timestamp(*next_reset_dt) {
            -num_days_start_to_end(date_from_timestamp(*next_reset_dt), *as_on_date)
        } else {
            num_days_start_to_end(*as_on_date, date_from_timestamp(*next_reset_dt))
        }
    }

    if mat_tenor >= from_bucket && mat_tenor <= to_bucket {
        *prin_amt = *ost_prin_amt;
    } else {
        *prin_amt = 0.0;
    }

    if int_tenor >= from_bucket && int_tenor <= to_bucket {
        *int_amt = *int_amt_aip + *penalty_amt;
        *int_amt_30_days =
            (*ost_prin_amt * (*int_rate / 100.00) * (*int_tenor - 1) as f64) / *int_basis as f64;
    } else {
        *int_amt_30_days = 0.0;
        *int_amt = 0.0;
    }

    *tot_int_amt = *int_amt + *int_amt_30_days;
}
