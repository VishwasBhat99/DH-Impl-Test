use chrono::Datelike;
use configuration_parameters::ConfigurationParameters;
use rbdate::{
    get_month_end_date, incr_dt_by_days, incr_dt_by_yrs, increment_date_by_months, DateParser,
    NaiveDate,
};
use std::collections::HashMap;

pub fn get_maturity_date(
    fields: &mut Vec<&str>,
    holiday_map: &mut HashMap<NaiveDate, String>,
    config_param: &ConfigurationParameters,
    date_format: String,
) -> NaiveDate {
    let dt_parser = DateParser::new(date_format, false);
    let no_of_flows = fields[2].parse::<f64>().unwrap_or(0.0) as i64;
    let no_of_demands = fields[3].parse::<f64>().unwrap_or(0.0) as i64;
    let flow_start_date = dt_parser.parse(fields[4]);
    let mut day = fields[9].parse::<f64>().unwrap_or(0.0) as u32;
    if day == 0 {
        day = flow_start_date.day();
    }
    let freq = fields[6].to_string();

    if no_of_flows == 0 || (no_of_flows < no_of_demands && no_of_demands != 0) {
        let cf_dates: Vec<NaiveDate> = get_cf_dates(day, no_of_demands, freq, flow_start_date);
        if holiday_map
            .get(&cf_dates[cf_dates.len() - 1])
            .unwrap_or(&"W".to_string())
            == "W"
        {
            return cf_dates[cf_dates.len() - 1];
        } else {
            let mut pick_date = cf_dates[cf_dates.len() - 1];
            loop {
                let next_date = incr_dt_by_days(pick_date, 1);
                if holiday_map.get(&next_date).unwrap() == "W" {
                    return next_date;
                }
                pick_date = next_date;
            }
        }
    } else {
        if no_of_flows > 0 && no_of_demands == 0 && flow_start_date > *config_param.as_on_date() {
            return flow_start_date;
        }
        let pickup_date_no = no_of_flows - (no_of_flows - no_of_demands);
        let cf_dates: Vec<NaiveDate> = get_cf_dates(day, no_of_flows, freq, flow_start_date);
        for date in cf_dates {
            for _ in 0..pickup_date_no {
                continue;
            }
            if date < *config_param.as_on_date() {
                continue;
            }
            if holiday_map.get(&date).unwrap() == "W" {
                return date;
            } else {
                let mut pick_date = date;
                loop {
                    let next_date = incr_dt_by_days(pick_date, 1);
                    if holiday_map.get(&next_date).unwrap() == "W" {
                        return next_date;
                    }
                    pick_date = next_date;
                }
            }
        }
    }
    flow_start_date
}

pub fn get_cf_dates(
    day: u32,
    no_of_dates: i64,
    rep_freq: String,
    flow_st_dt: NaiveDate,
) -> Vec<NaiveDate> {
    let mut start_date = flow_st_dt;
    let mut cf_dates: Vec<NaiveDate> = Vec::new();
    for _ in 0..no_of_dates {
        let mut next_date =
            match NaiveDate::from_ymd_opt(start_date.year(), start_date.month(), day) {
                Some(val) => val,
                None => get_month_end_date(start_date),
            };
        if rep_freq.contains("Y") {
            //Yearly
            next_date = incr_dt_by_yrs(next_date, 1);
            cf_dates.push(next_date);
        } else if rep_freq.contains("H") {
            //Half yearly
            next_date = increment_date_by_months(next_date, 6);
            cf_dates.push(next_date);
        } else if rep_freq.contains("Q") {
            //Quarterly
            next_date = increment_date_by_months(next_date, 3);
            cf_dates.push(next_date);
        } else if rep_freq.contains("M") {
            //Monthly
            next_date = increment_date_by_months(next_date, 1);
            cf_dates.push(next_date);
        } else {
            //Weekly
            next_date = incr_dt_by_days(next_date, 7);
            cf_dates.push(next_date);
        }
        start_date = match NaiveDate::from_ymd_opt(next_date.year(), next_date.month(), day) {
            Some(val) => val,
            None => get_month_end_date(next_date),
        };
    }
    cf_dates
}
