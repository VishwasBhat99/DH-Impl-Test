use super::npa_structs::RepDateData;
use crate::configuration_parameters::ConfigurationParameters;
use chrono::{Datelike, NaiveDate};
use rbdate::{datevalue_to_naive_date, incr_dt_by_mon_presrv_eom_checked, DateParser};
use std::collections::HashMap;

pub fn get_next_rep_date(
    config_params: &ConfigurationParameters,
    next_rep_date: String,
    benchmark: String,
    rep_date_map: &mut HashMap<String, Vec<RepDateData>>,
    rep_day_month_map: &mut HashMap<String, u32>,
    maturity_date: String,
) -> String {
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
    let mut rep_date = (date_parser
        .parse_opt(&next_rep_date)
        .unwrap_or(*config_params.as_on_date())
        .succ_opt())
    .unwrap_or_else(|| *config_params.as_on_date())
    .format("%d-%m-%Y")
    .to_string();
    let rep_vec = rep_date_map
        .get(&benchmark)
        .unwrap_or(&RepDateData::def())
        .to_owned();
    if rep_date_map.contains_key(&benchmark)
        && rep_vec[0].v_or_f_flag.to_uppercase().starts_with('F')
    {
        rep_date = maturity_date;
    } else if !rep_date_map.contains_key(&benchmark)
        || (rep_vec.len() == 1 && rep_vec[0].override_cbs_reset_date != "Y")
    {
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let date: chrono::format::DelayedFormat<chrono::format::StrftimeItems<'_>> = (date_parser
            .parse_opt(&next_rep_date)
            .unwrap_or(*config_params.as_on_date())
            .succ_opt())
        .unwrap_or_else(|| *config_params.as_on_date())
        .format("%d-%m-%Y");
        rep_date = date.to_string();
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
        }
        rep_date = check_date(rep_date, *config_params.as_on_date(), date_parser);
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
        NaiveDate::from_ymd_opt(rep_date.year(), 4, rep_date.day()).unwrap_or(
            NaiveDate::from_ymd_opt(
                rep_date.year(),
                4,
                rbdate::get_month_end_date(
                    NaiveDate::from_ymd_opt(rep_date.year(), 4, 1)
                        .expect("Error getting Month-End-Date"),
                )
                .day(),
            )
            .expect("Incorrect Date Found"),
        )
    } else if [5, 6, 7].contains(&rep_date.month()) {
        NaiveDate::from_ymd_opt(rep_date.year(), 7, rep_date.day()).unwrap_or(
            NaiveDate::from_ymd_opt(
                rep_date.year(),
                7,
                rbdate::get_month_end_date(
                    NaiveDate::from_ymd_opt(rep_date.year(), 7, 1)
                        .expect("Error getting Month-End-Date"),
                )
                .day(),
            )
            .expect("Incorrect Date Found"),
        )
    } else {
        NaiveDate::from_ymd_opt(rep_date.year(), 10, rep_date.day()).unwrap_or(
            NaiveDate::from_ymd_opt(
                rep_date.year(),
                10,
                rbdate::get_month_end_date(
                    NaiveDate::from_ymd_opt(rep_date.year(), 10, 1)
                        .expect("Error getting Month-End-Date"),
                )
                .day(),
            )
            .expect("Incorrect Date Found"),
        )
    }
}

pub fn get_halfyearly_date(rep_date: NaiveDate) -> NaiveDate {
    if [1, 2, 3, 4, 5, 6].contains(&rep_date.month()) {
        NaiveDate::from_ymd_opt(rep_date.year(), 6, rep_date.day()).unwrap_or(
            NaiveDate::from_ymd_opt(
                rep_date.year(),
                6,
                rbdate::get_month_end_date(
                    NaiveDate::from_ymd_opt(rep_date.year(), 6, 1)
                        .expect("Error getting Month-End-Date"),
                )
                .day(),
            )
            .expect("Incorrect Date Found"),
        )
    } else {
        NaiveDate::from_ymd_opt(rep_date.year(), 12, rep_date.day()).expect("Incorrect Date Found")
    }
}

pub fn get_mat_date(input_data: Vec<&str>, config_params: &ConfigurationParameters) -> String {
    let apprv_date_value = input_data[14].parse::<i64>().unwrap_or(0) + 1;
    let apprv_date = datevalue_to_naive_date(&apprv_date_value.to_string())
        .unwrap_or(*config_params.as_on_date());
    let loan_trm = input_data[20]
        .to_string()
        .parse::<usize>()
        .expect("unable to parse LOAN_TRM");
    let mut mat_date = incr_dt_by_mon_presrv_eom_checked(apprv_date, loan_trm)
        .unwrap_or(*config_params.as_on_date());

    //if loan term is in days values
    if input_data[61] == "D" {
        mat_date = datevalue_to_naive_date(&(apprv_date_value + loan_trm as i64).to_string())
            .unwrap_or(*config_params.as_on_date());
    }
    mat_date.format("%d-%m-%Y").to_string()
}

pub fn check_date(rep_date: String, as_on_date: NaiveDate, date_parser: DateParser) -> String {
    let mut final_rep_date = date_parser.parse_opt(&rep_date).unwrap_or(as_on_date);

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
