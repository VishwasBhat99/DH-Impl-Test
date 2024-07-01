use chrono::Datelike;
use rbdate::NaiveDate;

pub fn get_months(start_date_opt: Option<NaiveDate>, end_date_opt: Option<NaiveDate>) -> i64 {
    let mut days_in_month = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let months: i64;

    if let Some(start_date) = start_date_opt {
        if let Some(end_date) = end_date_opt {
            if start_date > end_date {
                return -1;
            }
            let start_date_year = start_date.year();
            if is_leap_year(start_date_year) {
                days_in_month[1] = 29;
            }
            let end_date_year = end_date.year();
            let start_date_month = start_date.month();
            let end_date_month = end_date.month();
            let start_date_day = start_date.day();
            let end_date_day = end_date.day();
            let mut no_years = end_date_year - start_date_year;
            let mut no_months = 0;
            if end_date_month < start_date_month {
                no_years -= 1;
                no_months += (12 - start_date_month) + end_date_month;
            } else {
                no_months += end_date_month - start_date_month;
            }
            if end_date_day > start_date_day
                && start_date_day != days_in_month[(start_date_month - 1) as usize]
            {
                no_months += 1;
            }
            months = (no_years * 12) as i64 + no_months as i64;
        } else {
            months = 0;
        }
    } else {
        months = 0;
    }
    months
}

fn is_leap_year(year: i32) -> bool {
    return (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0);
}
