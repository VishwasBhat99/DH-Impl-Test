use crate::configuration_parameters::ConfigurationParameters;
use crate::extract::rates_key::RatesKey;
use crate::extract::rates_value::RatesValue;
use rbdate::{timestamp, NaiveDate};
use std::collections::{hash_map::Entry, HashMap};

pub fn get_avg_rate_for_ndays(
    temp_curve_map: &mut HashMap<String, Vec<String>>,
    curve_id_len: u32,
    date_map: &mut HashMap<String, String>,
    prev_rates: HashMap<RatesKey, Vec<RatesValue>>,
    holiday_map: &mut HashMap<String, Vec<String>>,
    config_params: &ConfigurationParameters,
) -> HashMap<RatesKey, Vec<RatesValue>> {
    let mut rates: HashMap<RatesKey, Vec<RatesValue>> = HashMap::new();
    if *config_params.no_avg_days() == 1_usize {
        return prev_rates;
    }
    for (key, val) in temp_curve_map.iter() {
        for i in 0..val.len() {
            //For starting dates which are lesser than no_avg_days
            if i < (config_params.no_avg_days() - 1) {
                let (date, rate) = val[i].split_at(11);
                let date = (timestamp(
                    NaiveDate::parse_from_str(&date[..10], "%d-%m-%Y")
                        .expect("Error in parsing date field"),
                ) * 10_i64.pow(curve_id_len))
                    + key[..4].parse::<i64>().unwrap_or(0);
                let rate = format!("{:.6}", rate);
                let key_vec: Vec<&str> = key.split('|').collect();
                let rates_key = RatesKey::new(
                    key_vec[0].to_string().parse::<i64>().unwrap_or(0),
                    rbdate::date_from_timestamp(date / 10000),
                );
                let rates_val = RatesValue::new(
                    key_vec[1].to_string().parse::<i64>().unwrap_or(0),
                    key_vec[2].to_string(),
                    rate.to_string().parse::<f64>().unwrap_or(0.0),
                );
                match rates.entry(rates_key) {
                    Entry::Vacant(e) => {
                        e.insert(vec![rates_val]);
                    }
                    Entry::Occupied(mut e) => {
                        e.get_mut().push(rates_val);
                    }
                }
            } else {
                let mut date_val = "";
                let mut rate = 0.0;
                let mut j = i;
                let mut cntr = *config_params.no_avg_days();
                let key_vec: Vec<&str> = key.split('|').collect();
                while cntr != 0_usize {
                    let (date_j, rate_j) = val[j].split_at(11);
                    if j == i {
                        date_val = date_j
                    }
                    let mut holiday_flag = false;
                    if holiday_map.contains_key(&key_vec[0].to_string()) {
                        let holiday_vec = holiday_map.get(&key_vec[0].to_string()).unwrap();
                        if holiday_vec.contains(&date_j.trim_matches('|').to_string()) {
                            holiday_flag = true;
                        }
                    }
                    if !config_params
                        .skip_bmid_vec()
                        .contains(&key_vec[0].to_string())
                        && j != cntr - 1
                        && holiday_flag
                    {
                        j -= 1;
                        continue;
                    }
                    if date_map.contains_key(&date_j.trim_matches('|').to_string())
                        && j != cntr - 1
                        && config_params
                            .skip_bmid_vec()
                            .contains(&key_vec[0].to_string())
                    {
                        j -= 1;
                        continue;
                    }
                    rate += rate_j.to_string().parse::<f64>().unwrap_or(0.0);
                    j -= 1;
                    cntr -= 1;
                }
                //Take upto 6 digits after .
                let rate = format!("{0:.6}", (rate / *config_params.no_avg_days() as f64));
                let date = (timestamp(
                    NaiveDate::parse_from_str(&date_val[..10], "%d-%m-%Y")
                        .expect("Error in parsing date field"),
                ) * 10_i64.pow(curve_id_len))
                    + key[..4].parse::<i64>().unwrap_or(0);
                let rates_key = RatesKey::new(
                    key_vec[0].to_string().parse::<i64>().unwrap_or(0),
                    rbdate::date_from_timestamp(date / 10000),
                );
                let rates_val = RatesValue::new(
                    key_vec[1].to_string().parse::<i64>().unwrap_or(0),
                    key_vec[2].to_string(),
                    rate.to_string().parse::<f64>().unwrap_or(0.0),
                );
                match rates.entry(rates_key) {
                    Entry::Vacant(e) => {
                        e.insert(vec![rates_val]);
                    }
                    Entry::Occupied(mut e) => {
                        e.get_mut().push(rates_val);
                    }
                }
            }
        }
    }
    rates
}
