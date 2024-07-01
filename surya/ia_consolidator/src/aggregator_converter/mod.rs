use self::file_map::{get_map, get_smry_map, BucketValue, SummaryValue};
use self::writer::{write_to_file, write_to_file_smry};
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

mod exchange_rate;
mod file_map;
mod writer;

#[derive(Debug)]
pub struct ConsolCurrency {
    consol_ccy: String,
    display_consol_ccy: String,
}

pub fn converter(config_params: &ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let config_rdr = match new_buf_rdr(config_params.consol_config_file_path()) {
        Ok(rdr) => rdr,
        Err(err) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            config_params.consol_config_file_path, err
        )),
    };
    let mut consol_config_map: HashMap<String, Vec<ConsolCurrency>> = HashMap::new();
    for line in config_rdr.lines() {
        let config_line = line.unwrap();
        let fields: Vec<&str> = config_line.split('|').collect();
        let consol_ccy = ConsolCurrency {
            consol_ccy: fields[1].to_string(),
            display_consol_ccy: fields[2].to_string(),
        };
        if consol_config_map.contains_key(fields[0]) {
            let prev_val = consol_config_map
                .remove(fields[0])
                .expect("Cannot read value for key in consol config map.");
            let mut consols = prev_val;
            consols.push(consol_ccy);
            consol_config_map.insert(fields[0].to_string(), consols);
        } else {
            let mut consols: Vec<ConsolCurrency> = Vec::new();
            consols.push(consol_ccy);
            consol_config_map.insert(fields[0].to_string(), consols);
        }
    }
    let exchange_rate_map =
        exchange_rate::read_exchange_rate(config_params.currency_conversion_file_path());
    let input_file_principal;
    let output_file_principal;
    let input_file_rate;
    let output_file_rate;
    let input_file_smry;
    let output_file_smry;
    input_file_smry = format!("{}_smry.txt", config_params.input_file_path());
    output_file_smry = format!("{}_consol_smry.txt", config_params.output_file_path());
    let mut original_map: HashMap<Vec<String>, BucketValue> = HashMap::new();
    let mut final_map: HashMap<Vec<String>, BucketValue> = HashMap::new();
    let mut original_map_smry: HashMap<Vec<String>, SummaryValue> = HashMap::new();
    let mut final_map_smry: HashMap<Vec<String>, SummaryValue> = HashMap::new();
    output_file_principal = format!(
        "{}_consol_principal_amt.txt",
        config_params.output_file_path()
    );
    output_file_rate = format!("{}_consol_rate.txt", config_params.output_file_path());
    if config_params.is_maturity() {
        input_file_principal = format!("{}_principal_amt.txt", config_params.input_file_path());
        input_file_rate = format!("{}_rate.txt", config_params.input_file_path());
        let principal_file_open =
            File::open(&input_file_principal).expect("Cannot open input pricipal file.");
        let rate_file_open = File::open(&input_file_rate).expect("Cannot open input rate file.");
        get_map(&mut original_map, principal_file_open, rate_file_open);
        for (key, value) in original_map {
            let len = value.principal_vec.len();
            let ccy = &key[2];
            let target_consols = match consol_config_map.get(ccy) {
                Some(val) => val,
                None => {
                    log_error!(
                        log,
                        "Consol Currency {} Not Found in Consol Config File.",
                        ccy
                    );
                    continue;
                }
            };
            for consol in target_consols {
                let mut new_key = key.clone();
                new_key[2] = consol.display_consol_ccy.to_string();
                let ccy_lookup =
                    key[2].to_string() + &"|".to_string() + &consol.consol_ccy.to_string();
                let ex_rt = match exchange_rate_map.get(&ccy_lookup) {
                    Some(exchange_rate) => exchange_rate.to_string().parse::<f64>().unwrap(),
                    _ => "0.0".to_string().parse::<f64>().unwrap(),
                };
                let mut new_value = value.clone();
                if final_map.contains_key(&new_key) {
                    let prev_value = final_map.get(&new_key).unwrap();
                    for i in 0..len {
                        new_value.principal_vec[i] =
                            (prev_value.principal_vec[i].parse::<f64>().unwrap()
                                + value.principal_vec[i].parse::<f64>().unwrap() * ex_rt)
                                .to_string();
                        new_value.rate_vec[i] = (prev_value.rate_vec[i].parse::<f64>().unwrap()
                            + value.rate_vec[i].parse::<f64>().unwrap()
                                * new_value.principal_vec[i].parse::<f64>().unwrap())
                        .to_string();
                    }
                    final_map.insert(new_key.clone(), new_value.clone());
                } else {
                    for i in 0..len {
                        new_value.principal_vec[i] =
                            (value.principal_vec[i].parse::<f64>().unwrap() * ex_rt).to_string();
                        new_value.rate_vec[i] = (value.rate_vec[i].parse::<f64>().unwrap()
                            * new_value.principal_vec[i].parse::<f64>().unwrap())
                        .to_string();
                    }
                    final_map.insert(new_key.clone(), new_value.clone());
                }
            }
        }
    }
    let smry_file_open = File::open(&input_file_smry).expect("Cannot open input pricipal file.");
    get_smry_map(&mut original_map_smry, smry_file_open);
    for (key, value) in original_map_smry {
        let ccy = &key[2];
        let target_consols = match consol_config_map.get(ccy) {
            Some(val) => val,
            None => {
                log_error!(
                    log,
                    "Consol Currency {} Not Found in Consol Config File.",
                    ccy
                );
                continue;
            }
        };
        for consol in target_consols {
            let mut new_key = key.clone();
            new_key[2] = consol.display_consol_ccy.to_string();
            let ccy_lookup = key[2].to_string() + &"|".to_string() + &consol.consol_ccy.to_string();
            let ex_rt = match exchange_rate_map.get(&ccy_lookup) {
                Some(exchange_rate) => exchange_rate.to_string().parse::<f64>().unwrap(),
                _ => "0.0".to_string().parse::<f64>().unwrap(),
            };
            let mut new_value = value.clone();
            if final_map_smry.contains_key(&new_key) {
                let prev_value = final_map_smry.get(&new_key).unwrap();
                new_value.principal_smry =
                    prev_value.principal_smry + (value.principal_smry * ex_rt);
                new_value.rate_smry =
                    prev_value.rate_smry + (value.rate_smry * new_value.principal_smry);
                final_map_smry.insert(new_key.clone(), new_value.clone());
            } else {
                new_value.principal_smry = value.principal_smry * ex_rt;
                new_value.rate_smry = value.rate_smry * new_value.principal_smry;
                final_map_smry.insert(new_key.clone(), new_value.clone());
            }
        }
    }
    if config_params.is_maturity() {
        write_to_file(
            &output_file_principal,
            &output_file_rate,
            &final_map,
            &config_params.output_file_path(),
        )
    }

    write_to_file_smry(
        &output_file_smry,
        &final_map_smry,
        &config_params.output_file_path(),
    )
}
