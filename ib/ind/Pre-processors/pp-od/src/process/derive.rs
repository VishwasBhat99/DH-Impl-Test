use super::account::RepDateData;
use crate::configuration_parameters::ConfigurationParameters;
use chrono::{Datelike, NaiveDate};
use rbdate::{datevalue_to_naive_date, incr_dt_by_mon_presrv_eom_checked, DateParser};
use std::collections::HashMap;

pub fn get_next_rep_date(
    config_params: &ConfigurationParameters,
    next_rep_date: String,
    benchmark: String,
    instl_months: String,
    rep_date_map: &mut HashMap<String, Vec<RepDateData>>,
    rep_day_month_map: &mut HashMap<String, u32>,
    limit_expiry_date: String,
) -> String {
    let date_parser: DateParser = DateParser::new("%d-%m-%Y".to_string(), false);
    let mut rep_date = if (0..=99999).contains(&next_rep_date.parse::<i64>().unwrap_or(0)) {
        datevalue_to_naive_date(&next_rep_date).unwrap_or(*config_params.as_on_date())
    } else {
        *config_params.as_on_date()
    }
    .format("%d-%m-%Y")
    .to_string();
    let rep_vec = rep_date_map
        .get(&benchmark)
        .unwrap_or(&RepDateData::def())
        .to_owned();
    if rep_date_map.contains_key(&benchmark)
        && rep_vec[0].v_or_f_flag.to_uppercase().starts_with('F')
    {
        rep_date = limit_expiry_date;
    } else if !rep_date_map.contains_key(&benchmark)
        || (rep_vec.len() == 1 && rep_vec[0].override_cbs_reset_date != "Y")
    {
        rep_date = incr_dt_by_mon_presrv_eom_checked(
            NaiveDate::parse_from_str(&rep_date, "%d-%m-%Y").unwrap_or(*config_params.as_on_date()),
            instl_months.parse::<usize>().unwrap_or(0),
        )
        .unwrap_or(*config_params.as_on_date())
        .format("%d-%m-%Y")
        .to_string();
    } else if rep_vec.len() == 1
        && rep_vec[0].override_cbs_reset_date == "Y"
        && rep_vec[0].reset_month == 0
    {
        rep_date = get_rep_date(
            rep_vec[0]
                .rep_freq
                .trim()
                .trim_matches(' ')
                .trim_matches('-')
                .to_uppercase(),
            rep_vec[0].reset_day_of_month,
            *config_params.as_on_date(),
        );
        rep_date = check_date(rep_date, *config_params.as_on_date(), date_parser);
    } else if rep_vec.len() == 1 && rep_vec[0].reset_month != 0 {
        rep_date = NaiveDate::from_ymd_opt(
            config_params.as_on_date().year(),
            rep_vec[0].reset_month,
            rep_vec[0].reset_day_of_month,
        )
        .unwrap_or(*config_params.as_on_date())
        .format("%d-%m-%Y")
        .to_string();
        rep_date = check_date(rep_date, *config_params.as_on_date(), date_parser);
    } else if rep_vec[0].reset_month != 0 {
        for val in rep_vec.iter() {
            let dd_parser: DateParser = DateParser::new("%d-%m-%Y".to_string(), false);
            if val.reset_month == config_params.as_on_date.month()
                || val.reset_month == config_params.as_on_date().month() + 1
            {
                if config_params.as_on_date().day() <= val.reset_day_of_month
                    || val.reset_month > config_params.as_on_date.month()
                {
                    rep_date = NaiveDate::from_ymd_opt(
                        config_params.as_on_date().year(),
                        val.reset_month,
                        val.reset_day_of_month,
                    )
                    .unwrap_or(*config_params.as_on_date())
                    .format("%d-%m-%Y")
                    .to_string();
                } else if config_params.as_on_date().month() == 12 {
                    rep_date = NaiveDate::from_ymd_opt(
                        config_params.as_on_date().year(),
                        2,
                        *rep_day_month_map
                            .get(&(format!("{}{}", val.bm, 2)))
                            .unwrap_or(&config_params.as_on_date.day()),
                    )
                    .unwrap_or(*config_params.as_on_date())
                    .format("%d-%m-%Y")
                    .to_string();
                } else {
                    rep_date = NaiveDate::from_ymd_opt(
                        config_params.as_on_date().year(),
                        val.reset_month + 2,
                        *rep_day_month_map
                            .get(&(format!("{}{}", val.bm, val.reset_month + 2)))
                            .unwrap_or(&config_params.as_on_date.day()),
                    )
                    .unwrap_or(*config_params.as_on_date())
                    .format("%d-%m-%Y")
                    .to_string();
                }
            }
            rep_date = check_date(rep_date, *config_params.as_on_date(), dd_parser);
        }
    }
    rep_date
}

pub fn get_rep_date(freq: String, reset_day: u32, as_on_date: NaiveDate) -> String {
    let mut rep_date = NaiveDate::from_ymd_opt(as_on_date.year(), as_on_date.month(), reset_day)
        .unwrap_or(as_on_date);
    rep_date = match freq.as_str() {
        "DAILY" => as_on_date.succ_opt().unwrap_or(as_on_date),
        "MONTHLY" => rep_date,
        "BIMONTHLY" => get_bimonthly_date(rep_date),
        "QUARTERLY" => get_quarterly_date(rep_date),
        "HALFYEARLY" => get_halfyearly_date(rep_date),
        "YEARLY" => NaiveDate::from_ymd_opt(as_on_date.year(), 12, reset_day).unwrap_or(as_on_date),
        _ => as_on_date,
    };
    if rep_date < as_on_date {
        match freq.as_str() {
            "DAILY" => rep_date,
            "MONTHLY" => incr_dt_by_mon_presrv_eom_checked(rep_date, 1).unwrap_or(as_on_date),
            "BIMONTHLY" => incr_dt_by_mon_presrv_eom_checked(rep_date, 2).unwrap_or(as_on_date),
            "QUARTERLY" => incr_dt_by_mon_presrv_eom_checked(rep_date, 3).unwrap_or(as_on_date),
            "HALFYEARLY" => incr_dt_by_mon_presrv_eom_checked(rep_date, 6).unwrap_or(as_on_date),
            "YEARLY" => {
                NaiveDate::from_ymd_opt(as_on_date.year() + 1, 12, reset_day).unwrap_or(as_on_date)
            }
            _ => as_on_date,
        }
    } else {
        rep_date
    }
    .format("%d-%m-%Y")
    .to_string()
}

pub fn get_bimonthly_date(rep_date: NaiveDate) -> NaiveDate {
    if rep_date.month() % 2 == 0 {
        rep_date
    } else {
        incr_dt_by_mon_presrv_eom_checked(rep_date, 1).unwrap_or(rep_date)
    }
}

pub fn get_quarterly_date(rep_date: NaiveDate) -> NaiveDate {
    if [11, 12, 1].contains(&rep_date.month()) {
        if rep_date.month() == 1 {
            NaiveDate::from_ymd_opt(rep_date.year(), 1, rep_date.day())
                .expect("Incorrect Date Found")
        } else {
            NaiveDate::from_ymd_opt(rep_date.year() + 1, 1, rep_date.day())
                .expect("Incorrect Date Found")
        }
    } else if [2, 3, 4].contains(&rep_date.month()) {
        NaiveDate::from_ymd_opt(rep_date.year(), 4, rep_date.day()).expect("Incorrect Date Found")
    } else if [5, 6, 7].contains(&rep_date.month()) {
        NaiveDate::from_ymd_opt(rep_date.year(), 7, rep_date.day()).expect("Incorrect Date Found")
    } else {
        NaiveDate::from_ymd_opt(rep_date.year(), 10, rep_date.day()).expect("Incorrect Date Found")
    }
}

pub fn get_halfyearly_date(rep_date: NaiveDate) -> NaiveDate {
    if [1, 2, 3, 4, 5, 6].contains(&rep_date.month()) {
        NaiveDate::from_ymd_opt(rep_date.year(), 6, rep_date.day()).expect("Incorrect Date Found")
    } else {
        NaiveDate::from_ymd_opt(rep_date.year(), 12, rep_date.day()).expect("Incorrect Date Found")
    }
}

pub fn get_limit_expiry_date(
    input_data: Vec<&str>,
    config_params: &ConfigurationParameters,
) -> String {
    let limit_expiry_datevalue = input_data[37].parse::<i64>().unwrap_or(0) + 1;
    let limit_expiry_date = if limit_expiry_datevalue <= 0 || limit_expiry_datevalue >= 99999 {
        config_params
            .as_on_date()
            .succ_opt()
            .expect("Error stamping (AsOnDate + 1) Date")
    } else {
        datevalue_to_naive_date(&limit_expiry_datevalue.to_string())
            .unwrap_or(*config_params.as_on_date())
    };
    limit_expiry_date.format("%d-%m-%Y").to_string()
}

pub fn check_date(rep_date: String, as_on_date: NaiveDate, date_parser: DateParser) -> String {
    let mut final_rep_date = date_parser
        .parse_opt(&rep_date)
        .unwrap_or(as_on_date);

    if final_rep_date < as_on_date {
        final_rep_date = NaiveDate::from_ymd_opt(
            as_on_date.year() + 1,
            final_rep_date.month(),
            final_rep_date.day(),
        )
        .unwrap_or_else(|| {
            NaiveDate::from_ymd_opt(as_on_date.year() + 1, 2, 28)
                .expect("Unable to derive final-rep-date")
        });
    }
    final_rep_date.format("%d-%m-%Y").to_string()
}
