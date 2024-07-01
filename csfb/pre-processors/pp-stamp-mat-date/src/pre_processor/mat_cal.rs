use chrono::{Datelike, NaiveDate};
use rbdate::{
    datevalue_to_naive_date, get_month_end_date, incr_dt_by_days, incr_dt_by_mon_presrv_eom,
    incr_dt_by_yrs, increment_date_by_months,
};
pub fn mat_dt_cal(
    month_long: &Vec<String>,
    month_short: &Vec<String>,
    as_on_date: &NaiveDate,
    bucket: &str,
    reporting_date_days: i64,
) -> NaiveDate {
    let as_on_year = as_on_date.year();
    let bucket_upper_case = bucket.trim().to_uppercase();
    //When bucket is like as 10 oct (yearly)
    if bucket_upper_case.contains("(YEARLY)") {
        let month_date_vec: Vec<String> = bucket_upper_case
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        let yearly_day = month_date_vec[0].parse().unwrap_or(0);
        let yearly_month = match month_long.iter().position(|x| x == &month_date_vec[1]) {
            Some(x) => x + 1,
            None => match month_short.iter().position(|x| x == &month_date_vec[1]) {
                Some(x) => x + 1,
                None => panic!(
                    "for bucket {},the given month is not in correct formate",
                    bucket
                ),
            },
        };
        let current_date =
            match NaiveDate::from_ymd_opt(as_on_year, yearly_month as u32, yearly_day) {
                Some(date) => date,
                None => get_month_end_date(
                    NaiveDate::from_ymd_opt(as_on_year, yearly_month as u32, 1)
                        .unwrap_or(*as_on_date),
                ),
            };
        let mat_date = if *as_on_date <= current_date {
            current_date
        } else {
            match NaiveDate::from_ymd_opt(as_on_year + 1, yearly_month as u32, yearly_day) {
                Some(date) => date,
                //date get out of range in case of february
                None => get_month_end_date(
                    NaiveDate::from_ymd_opt(as_on_year + 1, yearly_month as u32, 1)
                        .unwrap_or(*as_on_date),
                ),
            }
        };
        return mat_date;
    }
    let mat_date = match NaiveDate::parse_from_str(bucket, "%d-%m-%y") {
        //Given bucket value as a date in dd-mm-yy format
        Ok(x) => x,
        Err(_err) => match NaiveDate::parse_from_str(bucket, "%d-%m-%Y") {
            //Given  bucket value as a date on dd-mm-yyyy format
            Ok(x) => x,
            //Given  bucket value as a date on dd-mom-yyyy format
            Err(_err) => match NaiveDate::parse_from_str(bucket, "%d-%b-%Y") {
                Ok(x) => x,
                Err(_err) => {
                    if bucket.parse::<u64>().unwrap_or(0) >= 1
                        && bucket.parse::<u64>().unwrap_or(0) <= 31
                    {
                        effective_date_cal_from_day(as_on_date, bucket.parse().unwrap_or(0))
                    } else {
                        //if date value is given in the bucket
                        match datevalue_to_naive_date(bucket) {
                            Ok(x) => x,
                            Err(_err) => match bucket_upper_case.as_str() {
                                "JANUARY END" | "JAN END" | "JA END" => {
                                    effective_date_cal_from_month(as_on_date, 1)
                                }
                                "FEBRUARY END" | "FEB END" | "FE END" => {
                                    effective_date_cal_from_month(as_on_date, 2)
                                }
                                "MARCH END" | "MAR END" | "MR END" => {
                                    effective_date_cal_from_month(as_on_date, 3)
                                }
                                "APRIL END" | "APR END" | "AP END" => {
                                    effective_date_cal_from_month(as_on_date, 4)
                                }
                                "MAY END" | "MY END" => {
                                    effective_date_cal_from_month(as_on_date, 5)
                                }
                                "JUNE END" | "JUN END" | "JN END" => {
                                    effective_date_cal_from_month(as_on_date, 6)
                                }
                                "JULY END" | "JUL END" | "JL END" => {
                                    effective_date_cal_from_month(as_on_date, 7)
                                }
                                "AUGUST END" | "AUG END" | "AU END" => {
                                    effective_date_cal_from_month(as_on_date, 8)
                                }
                                "SEPTEMBER END" | "SEP END" | "SE END" => {
                                    effective_date_cal_from_month(as_on_date, 9)
                                }
                                "OCTOBER END" | "OCT END" | "OC END" => {
                                    effective_date_cal_from_month(as_on_date, 10)
                                }
                                "NOVEMBER END" | "NOV END" | "NV END" => {
                                    effective_date_cal_from_month(as_on_date, 11)
                                }
                                "DECEMBER END" | "DEC END" | "DE END" => {
                                    effective_date_cal_from_month(as_on_date, 12)
                                }
                                "QUARTERLY" => {
                                    if *as_on_date
                                        >= NaiveDate::from_ymd_opt(as_on_year, 1, 1)
                                            .unwrap_or(*as_on_date)
                                        && *as_on_date
                                            <= NaiveDate::from_ymd_opt(as_on_year, 3, 31)
                                                .unwrap_or(*as_on_date)
                                    {
                                        NaiveDate::from_ymd_opt(as_on_year, 4, 1)
                                            .unwrap_or(*as_on_date)
                                    } else if *as_on_date
                                        >= NaiveDate::from_ymd_opt(as_on_year, 4, 1)
                                            .unwrap_or(*as_on_date)
                                        && *as_on_date
                                            <= NaiveDate::from_ymd_opt(as_on_year, 6, 1)
                                                .unwrap_or(*as_on_date)
                                    {
                                        NaiveDate::from_ymd_opt(as_on_year, 7, 1)
                                            .unwrap_or(*as_on_date)
                                    } else if *as_on_date
                                        >= NaiveDate::from_ymd_opt(as_on_year, 7, 1)
                                            .unwrap_or(*as_on_date)
                                        && *as_on_date
                                            <= NaiveDate::from_ymd_opt(as_on_year, 9, 30)
                                                .unwrap_or(*as_on_date)
                                    {
                                        NaiveDate::from_ymd_opt(as_on_year, 10, 1)
                                            .unwrap_or(*as_on_date)
                                    } else {
                                        NaiveDate::from_ymd_opt(as_on_year+1, 1, 1)
                                            .unwrap_or(*as_on_date)
                                    }
                                }

                                "HALF YEARLY" | "HALF-YEARLY" => {
                                    if reporting_date_days == 0 {
                                        if *as_on_date
                                            >= NaiveDate::from_ymd_opt(as_on_year, 4, 1)
                                                .unwrap_or(*as_on_date)
                                            && *as_on_date
                                                <= NaiveDate::from_ymd_opt(as_on_year, 9, 30)
                                                    .unwrap_or(*as_on_date)
                                        {
                                            println!("ason{}", as_on_date);
                                            NaiveDate::from_ymd_opt(as_on_year, 10, 1)
                                                .unwrap_or(*as_on_date)
                                        } else if *as_on_date
                                            >= NaiveDate::from_ymd_opt(as_on_year, 10, 1)
                                                .unwrap_or(*as_on_date)
                                            && *as_on_date
                                                <= NaiveDate::from_ymd_opt(as_on_year + 1, 3, 31)
                                                    .unwrap_or(*as_on_date)
                                        {
                                            NaiveDate::from_ymd_opt(as_on_year + 1, 4, 1)
                                                .unwrap_or(*as_on_date)
                                        } else {
                                            NaiveDate::from_ymd_opt(as_on_year, 4, 1)
                                                .unwrap_or(*as_on_date)
                                        }
                                    } else {
                                        if *as_on_date
                                            >= NaiveDate::from_ymd_opt(as_on_year, 4, 1)
                                                .unwrap_or(*as_on_date)
                                            && *as_on_date
                                                <= NaiveDate::from_ymd_opt(as_on_year, 9, 30)
                                                    .unwrap_or(*as_on_date)
                                        {
                                            NaiveDate::from_ymd_opt(
                                                as_on_year,
                                                10,
                                                reporting_date_days as u32,
                                            )
                                            .unwrap_or(*as_on_date)
                                        } else if *as_on_date
                                            >= NaiveDate::from_ymd_opt(as_on_year, 10, 1)
                                                .unwrap_or(*as_on_date)
                                            && *as_on_date
                                                <= NaiveDate::from_ymd_opt(as_on_year + 1, 3, 31)
                                                    .unwrap_or(*as_on_date)
                                        {
                                            NaiveDate::from_ymd_opt(
                                                as_on_year + 1,
                                                4,
                                                reporting_date_days as u32,
                                            )
                                            .unwrap_or(*as_on_date)
                                        } else {
                                            NaiveDate::from_ymd_opt(
                                                as_on_year,
                                                4,
                                                reporting_date_days as u32,
                                            )
                                            .unwrap_or(*as_on_date)
                                        }
                                    }
                                }

                                "YEAR END" | "YEAR-END" => {
                                    NaiveDate::from_ymd_opt(as_on_year, 12, 31)
                                        .unwrap_or(*as_on_date)
                                }
                                "DAY 1" => incr_dt_by_days(*as_on_date, 1),
                                "2D - 7D" | "2D-7D" => incr_dt_by_days(*as_on_date, 6),
                                "8D - 14D" | "8D-14D" => incr_dt_by_days(*as_on_date, 13),
                                "15D - 30D" | "15D-30D" => incr_dt_by_days(*as_on_date, 29),
                                "31D - 2M" | "31D-2M" => incr_dt_by_days(*as_on_date, 45),
                                "2M - 3M" | "2M-3M" => {
                                    incr_dt_by_mon_presrv_eom(*as_on_date, 2).unwrap_or(*as_on_date)
                                }
                                "3M - 6M" | "3M-6M" => {
                                    incr_dt_by_mon_presrv_eom(*as_on_date, 5).unwrap_or(*as_on_date)
                                }
                                "6M - 1Y" | "6M-1Y" => incr_dt_by_yrs(*as_on_date, 1),
                                "1Y - 3Y" | "1Y-3Y" => incr_dt_by_yrs(*as_on_date, 2),
                                "3Y - 5Y" | "3Y-5Y" => incr_dt_by_yrs(*as_on_date, 4),
                                "5Y - 7Y" | "5Y-7Y" => incr_dt_by_yrs(*as_on_date, 6),
                                "7Y - 10Y" | "7Y-10Y" => incr_dt_by_yrs(*as_on_date, 9),
                                "10Y - 15Y" | "10Y-15Y" => incr_dt_by_yrs(*as_on_date, 14),
                                "Over 15Y" | "Over15Y" => {
                                    NaiveDate::parse_from_str("31-12-2099", "%d-%m-%Y")
                                        .unwrap_or(*as_on_date)
                                }
                                "REPORTING DATE" => {
                                    incr_dt_by_days(*as_on_date, reporting_date_days)
                                }
                                _ => *as_on_date,
                            },
                        }
                    }
                }
            },
        },
    };

    mat_date
}
pub fn effective_date_cal_from_day(as_on_date: &NaiveDate, day: u32) -> NaiveDate {
    let cal_date = match NaiveDate::from_ymd_opt(as_on_date.year(), as_on_date.month(), day) {
        Some(date) => date,
        //if the date goes out of index in case of february
        None => {
            //increment the date by one month
            match NaiveDate::from_ymd_opt(as_on_date.year(), as_on_date.month() + 1, day) {
                Some(date) => date,
                None => get_month_end_date(*as_on_date),
            }
        }
    };
    if *as_on_date < cal_date {
        cal_date
    } else {
        //next month date
        if as_on_date.month() == 12 {
            increment_date_by_months(cal_date, 1)
        } else {
            match NaiveDate::from_ymd_opt(cal_date.year(), cal_date.month() + 1, day) {
                Some(date) => date,
                //if the date goes out of index in case of february
                None => get_month_end_date(increment_date_by_months(cal_date, 1)),
            }
        }
    }
}
pub fn effective_date_cal_from_month(as_on_date: &NaiveDate, month: u32) -> NaiveDate {
    let first_day_cal_date =
        NaiveDate::from_ymd_opt(as_on_date.year(), month, 1).unwrap_or(*as_on_date);
    let month_end_cal_date = get_month_end_date(first_day_cal_date);
    if *as_on_date <= month_end_cal_date {
        month_end_cal_date
    } else {
        //next year month end date
        get_month_end_date(
            NaiveDate::from_ymd_opt(as_on_date.year() + 1, month, 1).unwrap_or(*as_on_date),
        )
    }
}
