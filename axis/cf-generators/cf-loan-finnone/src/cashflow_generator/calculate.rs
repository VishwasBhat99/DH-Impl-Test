use rbdate::{increment_date_by_months, num_days_start_to_end, NaiveDate};

pub fn get_month_equivalent(freq: String, int_calc_basis: String, cur_inst_date: NaiveDate) -> i64 {
    let month_equivalent = match freq.as_str() {
        "D" | "17" => get_month_equivalent_const(1.0, int_calc_basis, cur_inst_date),
        "W" | "18" => get_month_equivalent_const(7.0, int_calc_basis, cur_inst_date),
        "F" | "19" => get_month_equivalent_const(14.0, int_calc_basis, cur_inst_date),
        "M" | "1" => 1.0,
        "Q" | "2" => 3.0,
        "H" | "3" => 6.0,
        "Y" | "4" => 12.0,
        "B" => 1.0,
        _ => 1.0,
    };
    month_equivalent as i64
}

pub fn get_month_equivalent_const(
    num: f64,
    int_calc_basis: String,
    cur_inst_date: NaiveDate,
) -> f64 {
    let month_equivalent = match int_calc_basis.as_str() {
        "ACTUAL/ACTUAL" => {
            let next_date = increment_date_by_months(cur_inst_date, 12);
            num / (num_days_start_to_end(cur_inst_date, next_date)) as f64 / 12.0
        }
        "ACTUAL/365" => (365 / 12) as f64,
        "ACTUAL/360" => (360 / 12) as f64,
        "30/" => (360 / 12) as f64,
        _ => (365 / 12) as f64,
    };
    month_equivalent
}

pub fn calculate_si_by_months(osbal: f64, int_rate: f64, num_months: i64) -> f64 {
    (osbal * int_rate * num_months as f64) / 1200.0
}
pub fn calculate_interest(
    principal_balance: f64,
    interest_rate: f64,
    mut num_of_days: i64,
    int_calc_basis: String,
    cur_inst_date: NaiveDate,
    freq: String,
    is_first_cf: bool,
) -> f64 {
    let actual_days_in_year = match int_calc_basis.as_str() {
        "ACTUAL/ACTUAL" => {
            let next_date = increment_date_by_months(cur_inst_date, 12);
            num_days_start_to_end(next_date, cur_inst_date)
        }
        "ACTUAL/360" => 360,
        "30/" | "30/360" => {
            if !is_first_cf {
                num_of_days = 30 * get_month_equivalent(freq, int_calc_basis, cur_inst_date);
            }
            360
        }
        _ => 365,
    };
    (principal_balance * num_of_days as f64 * interest_rate) / (actual_days_in_year * 100) as f64
}
