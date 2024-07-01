use crate::configuration_parameters::ConfigurationParameters;
use crate::extract::rates_key::RatesKey;
use crate::extract::rates_value::RatesValue;
use crate::macros;
use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use get_avg_rates::*;
use itertools::Itertools;
use rbdate::*;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use sdb_util::expand;
use slog::Logger;
use std::{
    cmp::*,
    collections::{hash_map::Entry, HashMap},
    env::current_dir,
    io::prelude::*,
};

mod get_avg_rates;
mod rates_key;
mod rates_value;

pub fn extract_rates(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut date_wise_data: HashMap<i64, Vec<String>> = HashMap::new();
    let mut temp_curve_map: HashMap<String, Vec<String>> = HashMap::new();

    let mut curve_ids: Vec<i64> = Vec::new();
    let parse_from_str = NaiveDate::parse_from_str;
    let input_file = match new_buf_rdr(config_params.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not find file `{}` on location `{}` : {}.",
            config_params.input_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    let mut start_date = i64::MAX;
    let mut end_date = i64::MIN;
    let as_on_date = timestamp(*config_params.as_on_date());
    let curve_id_len: u32 = 4;
    let mut curve_id: i64;
    let mut date_curve: i64;
    for (line_num, lines) in input_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => {
                panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.input_file_path(),
                    line_num,
                    error
                );
            }
        };

        if line.matches('|').count() != 4 {
            log_error!(log, "Line {} not well formatted", line_num + 1);
            continue;
        }

        // Split the string at index
        let (date_curve_str, data) = line.split_at(12 + curve_id_len as usize);
        let date_curve_vec: Vec<&str> = date_curve_str.split('|').collect();
        let date = timestamp(
            parse_from_str(date_curve_vec[0], "%d-%m-%Y").expect("Error in parsing date field"),
        );

        // Finding start_date and end_date
        start_date = min(date, start_date);
        end_date = max(date, start_date);

        // Addition of date and curve id
        curve_id = date_curve_vec[1].parse().unwrap_or(0);
        date_curve = (date * 10_i64.pow(curve_id_len)) + curve_id;
        curve_ids.push(curve_id);

        // Generating output till as_on_date
        if start_date < as_on_date {
            end_date = as_on_date
        }

        date_wise_data
            .entry(date_curve)
            .and_modify(|d| d.push(data.to_string()))
            .or_insert_with(|| vec![data.to_string()]);
    }
    // Remove duplicates curve_ids
    curve_ids.sort_unstable();
    curve_ids.dedup();

    // Copy data from previous date, for each missing date between start_date and end_date
    let mut date = start_date;
    let one_daytime_diff = 24 * 60 * 60;
    let mut one_daytime_curve_diff;
    let mut date_curve_data: Vec<String>;
    let mut holiday_map: HashMap<String, Vec<String>> = HashMap::new();
    while date <= end_date {
        for curve_id in curve_ids.iter() {
            one_daytime_curve_diff =
                ((date - one_daytime_diff) * 10_i64.pow(curve_id_len)) + curve_id;
            date_curve = (date * 10_i64.pow(curve_id_len)) + curve_id;
            if !date_wise_data.contains_key(&date_curve) {
                if !config_params
                    .skip_bmid_vec()
                    .contains(&curve_id.to_string())
                {
                    holiday_map
                        .entry(curve_id.to_string())
                        .and_modify(|holiday| {
                            holiday.push(
                                rbdate::date_from_timestamp(date)
                                    .format("%d-%m-%Y")
                                    .to_string(),
                            )
                        })
                        .or_insert_with(|| {
                            vec![rbdate::date_from_timestamp(date)
                                .format("%d-%m-%Y")
                                .to_string()]
                        });
                }

                date_curve_data = match date_wise_data.get(&one_daytime_curve_diff) {
                    Some(val) => val.to_vec(),
                    None => Default::default(),
                };
                date_wise_data.insert(date_curve, date_curve_data);
                debug!(
                    log,
                    "Data for the date {} duplicated from the previous date",
                    date_from_timestamp(date)
                );
            }
        }
        date += one_daytime_diff;
    }
    // Store Data for each file
    let mut rates: HashMap<RatesKey, Vec<RatesValue>> = HashMap::new();
    let mut date: NaiveDate;
    for (date_curve, date_curve_data) in &date_wise_data {
        for line in date_curve_data.iter() {
            let fields: Vec<String> = expand(line.to_string(), '|');

            // Derive date and curve_id from date_curve variable
            curve_id = date_curve % 10_i64.pow(curve_id_len);
            date = date_from_timestamp(date_curve / 10_i64.pow(curve_id_len));
            let rate_key = RatesKey::new(curve_id, date);
            let rate_value = RatesValue::new(
                fields[0].parse().unwrap_or(0),
                fields[1].to_string(),
                fields[2].parse().unwrap_or(0.0),
            );

            match rates.entry(rate_key) {
                Entry::Vacant(e) => {
                    e.insert(vec![rate_value]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(rate_value);
                }
            }
        }
    }
    //Store all the holiday dates given in params for all years till as_on_date
    let mut holiday_vec: Vec<String> = Vec::new();
    let mut start_year = rbdate::date_from_timestamp(start_date).year();
    while start_year <= config_params.as_on_date().year() {
        for date in config_params.skip_date_vec() {
            let holiday = format!("{}-{}", start_year, date);
            holiday_vec.push(holiday);
        }
        start_year += 1;
    }
    //Store all data from holiday_vec and Weekends in date_map
    let mut date_map: HashMap<String, String> = HashMap::new();
    for key in rates.keys().sorted() {
        let value = rates.get(key).unwrap();
        for rec in value.iter() {
            //Storing data in temp HashMap for avg-rates calc
            let key_str = [
                key.curve_id.to_string(),
                rec.period.to_string(),
                rec.uom.to_string(),
            ]
            .join("|");
            let val_str = [
                key.as_on.format("%d-%m-%Y").to_string(),
                rec.rate.to_string(),
            ]
            .join("|");
            let time = NaiveTime::from_hms(17, 0, 0);
            let tz_offset = FixedOffset::east(3600);
            let datetime = NaiveDateTime::new(key.as_on, time);
            let dt_with_tz: DateTime<FixedOffset> =
                tz_offset.from_local_datetime(&datetime).unwrap();
            let dt_with_tz_utc: DateTime<Utc> = Utc.from_utc_datetime(&dt_with_tz.naive_utc());
            let current_time = dt_with_tz_utc;
            let weekday = current_time.date().weekday();
            if !config_params.skip_bmid_vec().is_empty()
                && (weekday.num_days_from_sunday() == 0
                    || weekday.num_days_from_sunday() == 6
                    || config_params
                        .skip_date_vec()
                        .contains(&key.as_on.to_string())
                    || holiday_vec.contains(&key.as_on.to_string()))
            {
                date_map.insert(
                    key.as_on.format("%d-%m-%Y").to_string(),
                    "Skipped".to_string(),
                );
            }
            temp_curve_map
                .entry(key_str)
                .and_modify(|d| d.push(val_str.to_string()))
                .or_insert_with(|| vec![val_str.to_string()]);
        }
    }
    //Get average rates for number of days passed
    let rates = get_avg_rate_for_ndays(
        &mut temp_curve_map,
        curve_id_len,
        &mut date_map,
        rates,
        &mut holiday_map,
        config_params,
    );
    //Sorting the rates for each bmid|as_on combination by converting period into D(days)
    let mut final_rates: HashMap<&RatesKey, Vec<RatesValue>> = HashMap::new();
    let mut period_uom_map: HashMap<String, i64> = HashMap::new();
    period_uom_map.insert("M".to_string(), 30);
    period_uom_map.insert("Y".to_string(), 365);
    for (k, v) in &rates {
        let mut sorted_vec = v.to_owned();
        sorted_vec.sort_by_key(|val| val.uom.to_string());
        sorted_vec.sort_by_key(|val| (val.period) * (period_uom_map.get(&val.uom)).unwrap_or(&1));
        final_rates.insert(k, sorted_vec);
    }
    //Write the files for each date and curve_id
    for (key, value) in &final_rates {
        let mut op_line = String::new();
        let filepath = format!(
            "{}{}_{}.txt",
            &config_params.output_file_path(),
            key.as_on.format("%d-%m-%Y"),
            key.curve_id
        );
        debug!(diag_log, "Writing Records to file: {}.", filepath);
        let mut out_writer = match buf_file_wrtr(&filepath, None) {
            Ok(file) => file,
            Err(error) => panic!(
                "Unable to create output file `{}` on location `{}` : {}",
                filepath,
                current_dir()
                    .expect("Unable to get current directory path.")
                    .display(),
                error
            ),
        };
        for rec in value {
            let line = format!("{}|{}|{}", rec.period, rec.uom, rec.rate);
            op_line.push_str(&line);
            op_line.push('\n');
        }
        match out_writer.write_all(op_line.as_bytes()) {
            Ok(val) => val,
            Err(error) => panic!(
                "Unable to write processed lines to output file `{}`: {}.",
                config_params.output_file_path(),
                error
            ),
        };
    }
}
