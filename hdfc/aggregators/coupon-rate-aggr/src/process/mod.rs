use crate::configuration_parameters::ConfigurationParameters;
use crate::process::input_account::*;
use crate::process::output_account::{format_output, get_writer, OutputField};
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use slog::Logger;
use std::collections::{HashMap, HashSet};
use std::{fs, io::Write};
mod input_account;
mod output_account;

pub fn process(config_params: &ConfigurationParameters, _logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_proc = 0;
    let consolidated_ccy = config_params.consolidated_ccy().trim().to_uppercase();
    let local_ccy = config_params.local_ccy().trim().to_uppercase();
    //Reading Exchange Rate File
    let mut exchange_rate_map: HashMap<(String,String), f64> = HashMap::new();
    let exchange_rate_reader = fs::read_to_string(config_params.exchange_rate_file())
        .expect("Could Not Read Exchange rate file");
    for (line_no, line) in exchange_rate_reader.lines().enumerate() {
        let exchange_rate_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let exchange_rate_data: ExchangeRate = ExchangeRate::new(
            config_params.exchange_rate_file(),
            &exchange_rate_vec,
            line_no + 1,
        );
        exchange_rate_map.insert((exchange_rate_data.from_ccy,exchange_rate_data.to_ccy), exchange_rate_data.val);
    }
    //Reading Coupon master file
    let mut coupon_file_path = open_workbook_auto(config_params.coupon_master_file_path())
        .expect("Unable to open the coupon master xlsx file.");
    if !coupon_file_path
        .sheet_names()
        .contains(&config_params.coupon_sheet_name().to_string())
    {
        println!(
            "Sheets present in Coupon Master-File: `{:?}`",
            coupon_file_path.sheet_names()
        );
        panic!(
            "Sheet passed: `{}` not present in Coupon Master-File: `{}`",
            config_params.coupon_sheet_name(),
            config_params.coupon_master_file_path()
        );
    }
    let mut coupon_master_map: HashMap<(String, String), f64> = HashMap::new();
    let mut coupon_set: HashSet<String> = HashSet::new();
    if let Some(Ok(coupon_file_reader)) =
        coupon_file_path.worksheet_range(config_params.coupon_sheet_name())
    {
        for (_row_no, row) in coupon_file_reader.rows().enumerate().skip(1) {
            let coupon_data = CouponData::new_from_excel(row);
            let llg_id = coupon_data.llg_id.clone();
            let ccy = coupon_data.ccy.clone();
            coupon_master_map.insert((llg_id.clone(), ccy), coupon_data.coupon_rate);
            coupon_set.insert(llg_id);
        }
    }
    //Writing exchange rate file
    let mut op_writer = get_writer(config_params.exchange_rate_output_file());
    let mut output_line: String = String::new();
    for (key, val) in coupon_master_map.clone() {
        output_line.push_str(&key.0);
        output_line.push('|');
        output_line.push_str(&key.1);
        writeln!(op_writer, "{}", output_line).expect("Unable to generate exchange rate file.");
        output_line.clear();
    }
    //Reading input file

    let mut fccy_map: HashMap<String, AmountData> = HashMap::new();
    let mut inr_map: HashMap<String, AmountData> = HashMap::new();
    let input_file_reader =
        fs::read_to_string(config_params.input_file_path()).expect("Could Not Read Input file");
    for (line_no, line) in input_file_reader.lines().enumerate() {
        acc_enc += 1;
        let input_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let input_data: InputData = InputData::new(
            config_params,
            config_params.input_file_path(),
            &input_vec,
            line_no + 1,
        );
        let ccy = input_data.ccy;
        let llg = input_data.llg_id;
        let consol_ccy=config_params.consolidated_ccy();
        if coupon_set.contains(&llg) {
            let val = input_data.amt;
            let concat = (llg.clone(), ccy.clone());
            if (!coupon_master_map.contains_key(&concat)
                && ccy.to_uppercase() != consolidated_ccy
                && ccy.to_uppercase() != "FCY"
                && ccy.to_uppercase() != local_ccy)
                && coupon_set.contains(&llg.to_string())
            {
                info!(
                    _logger,
                    "The currecy `{}` is not present in Master-Yield file for llg : `{}` ",
                    ccy,
                    llg
                );
            }
            let rate_code = coupon_master_map.get(&concat).unwrap_or(&0.0);
            if ccy.to_ascii_uppercase() != consolidated_ccy
                && ccy.to_ascii_uppercase() != "FCY"
                && ccy.to_ascii_uppercase() != local_ccy
            {
                let exchange_rate_val = exchange_rate_map.get(&(ccy,consol_ccy.to_string())).unwrap_or(&1.0);
                let new_amt = val * exchange_rate_val;
                let new_rate_amt = new_amt * rate_code;

                fccy_map
                    .entry(llg.clone())
                    .and_modify(|prev_amt| {
                        prev_amt.amt += new_amt;
                        prev_amt.rate_amt += new_rate_amt;
                    })
                    .or_insert(AmountData {
                        amt: new_amt,
                        rate_amt: new_rate_amt,
                    });
            } else if ccy.to_ascii_uppercase() == local_ccy {
                let new_amt = val;
                let new_rate_amt = new_amt * rate_code;
                inr_map
                    .entry(llg.clone())
                    .and_modify(|prev_amt| {
                        prev_amt.amt += val;
                        prev_amt.rate_amt += new_rate_amt;
                    })
                    .or_insert(AmountData {
                        amt: new_amt,
                        rate_amt: new_rate_amt,
                    });
            }
        }
    }
    let mut op_writer = get_writer(config_params.output_file());
    for (line_no, line) in input_file_reader.lines().enumerate() {
        acc_proc += 1;
        let input_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let input_data: InputData = InputData::new(
            config_params,
            config_params.input_file_path(),
            &input_vec,
            line_no + 1,
        );
        let llg_id = input_data.llg_id;
        let as_on_date = input_data.as_on_date;
        let ccy = input_data.ccy;
        let sls_irs = input_data.sls_irs;
        let source = input_data.source;
        let flow_type = input_data.flow_type;
        let amt = input_data.amt;
        let concat = (llg_id.clone(), ccy.clone());
        let mut llg_flag = true;
        let mut coupon_rate = coupon_master_map
            .get(&concat)
            .unwrap_or(&input_data.coupon_rate)
            .clone();

        let def = AmountData {
            amt: 0.0,
            rate_amt: 0.0,
        };
        let new_data = fccy_map.get(&llg_id.clone()).unwrap_or(&def);
        if coupon_set.contains(&llg_id) {
            if ccy.to_ascii_uppercase() == "FCY" {
                if new_data.rate_amt != 0.0 && new_data.amt != 0.0 {
                    coupon_rate = new_data.rate_amt / new_data.amt;
                }
            } else if ccy.to_ascii_uppercase() == consolidated_ccy {
                let inr_amt = inr_map.get(&llg_id.clone()).unwrap_or(&def);
                let tot_amt = new_data.rate_amt + inr_amt.rate_amt;
                let tot_sum = new_data.amt + inr_amt.amt;
                if tot_sum != 0.0 && tot_amt != 0.0 {
                    coupon_rate = tot_amt / tot_sum;
                }
            }
        }
        let yield_rate = coupon_rate.to_owned();
        let output_data: OutputField = OutputField {
            llg_id,
            as_on_date,
            ccy,
            sls_irs,
            source,
            flow_type,
            amt,
            coupon_rate,
        };
        writeln!(op_writer, "{}", format_output(output_data)).expect("Error in Writing Output");
    }

    let health_report = HealthReport::new(acc_enc, acc_proc, acc_enc - acc_proc, 0.0, 0.0, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file());
}
