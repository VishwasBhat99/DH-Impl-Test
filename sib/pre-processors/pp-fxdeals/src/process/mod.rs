use crate::configuration_parameters::ConfigurationParameters;
use crate::process::input_account::*;
use crate::process::output_account::{format_output, get_writer, OutputField};
use health_report::HealthReport;
use rbdate::num_days_start_to_end;
use slog::Logger;
use std::collections::HashMap;
use std::{fs, io::Write};
mod input_account;
mod output_account;

pub fn process(config_params: &ConfigurationParameters, _logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_proc = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;

    //Reading Mapping master File
    let mut currency_prrate_map: HashMap<String, Vec<(i64, f64)>> = HashMap::new();
    let currency_master_reader = fs::read_to_string(config_params.currency_prrate_file_path())
        .expect("Could Not Read Currency pr rate file");
    for (line_no, line) in currency_master_reader.lines().enumerate() {
        let currency_master_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let ccy = get_str(
            config_params.currency_prrate_file_path(),
            &currency_master_vec,
            1,
            line_no,
        );
        let days = get_str(
            config_params.currency_prrate_file_path(),
            &currency_master_vec,
            3,
            line_no,
        )
        .parse::<i64>()
        .unwrap_or(0);
        let rate = get_str(
            config_params.currency_prrate_file_path(),
            &currency_master_vec,
            7,
            line_no,
        )
        .parse::<f64>()
        .unwrap_or(0.0);
        currency_prrate_map
            .entry(ccy.clone())
            .and_modify(|prev_data| prev_data.push((days.clone(), rate)))
            .or_insert(vec![(days, rate)]);
    }
    let mut sorted_currency_prrate_map: HashMap<String, Vec<(i64, f64)>> = HashMap::new();
    for (ccy, mut ele) in currency_prrate_map {
        ele.sort_by(|a, b| a.0.cmp(&b.0));
        sorted_currency_prrate_map.insert(ccy, ele);
    }
    let mut op_writer = get_writer(config_params.output_file());

    let adf_fxdeals_file = fs::read_to_string(config_params.adf_fxdeals_file_path())
        .expect("Could Not Read adf fxdeals file");

    for (line_no, line) in adf_fxdeals_file.lines().enumerate() {
        acc_enc += 1;
        acc_proc += 1;
        let adf_fxdeals_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let adf_fxdeals_data: AdfFxdeals = AdfFxdeals::new(
            config_params,
            config_params.adf_fxdeals_file_path(),
            &adf_fxdeals_vec,
            0,
        );
        let as_on_date = adf_fxdeals_data.as_on_date;
        let maturity_date = adf_fxdeals_data.maturity_date;
        let resudial_tenor = num_days_start_to_end(as_on_date, maturity_date);
        let ccy = adf_fxdeals_data.crncy1;
        let def_vec: Vec<(i64, f64)> = Vec::new();
        let ccy_prrate_vec = sorted_currency_prrate_map
            .get(&ccy.to_uppercase())
            .unwrap_or(&def_vec);
        let mut reval_loss: f64 = 0.0;
        let ccy_vec_len = ccy_prrate_vec.len();
        let mut length_counter = 1;
        if ccy_vec_len != 0 && ccy_prrate_vec[0].0 == resudial_tenor {
            reval_loss = ccy_prrate_vec[0].1;
        }
        let mut flag = false;
        while length_counter < ccy_vec_len {
            if ccy_prrate_vec[length_counter].0 >= resudial_tenor {
                if ccy_prrate_vec[length_counter].0 == resudial_tenor {
                    reval_loss = ccy_prrate_vec[length_counter].1;
                    break;
                }
                flag = true;
                let lower_bound = ccy_prrate_vec[length_counter - 1].0;
                let upper_bound = ccy_prrate_vec[length_counter].0;
                let lower_rates = ccy_prrate_vec[length_counter - 1].1;
                let upper_rates = ccy_prrate_vec[length_counter].1;
                let rates = upper_rates - lower_rates;
                let days = upper_bound - lower_bound;
                let lower_days = resudial_tenor - lower_bound;
                reval_loss = lower_rates + ((rates / days as f64) * lower_days as f64);
            }
            if flag == true {
                break;
            }
            length_counter += 1;
        }
        let output_data: OutputField = OutputField {
            as_on_date: as_on_date.format("%d-%m-%Y").to_string(),
            deal_number: adf_fxdeals_data.deal_number,
            deal_date: adf_fxdeals_data.deal_date,
            product_type: adf_fxdeals_data.product_type,
            deal_ref: adf_fxdeals_data.deal_ref,
            transaction_type: adf_fxdeals_data.transaction_type,
            portfolio: adf_fxdeals_data.portfolio,
            counter_party: adf_fxdeals_data.counter_party,
            counterparty_category: adf_fxdeals_data.counterparty_category,
            internal_external_deal_type: adf_fxdeals_data.internal_external_deal_type,
            maturity_date: maturity_date.format("%d-%m-%Y").to_string(),
            crncy1: ccy.clone(),
            crncy2: adf_fxdeals_data.crncy2,
            deal_rate: adf_fxdeals_data.deal_rate,
            crncy1_amt: adf_fxdeals_data.crncy1_amt,
            crncy2_amt: adf_fxdeals_data.crncy2_amt,
            reval_rate: adf_fxdeals_data.reval_rate,
            reval_profit: adf_fxdeals_data.reval_profit,
            reval_loss: reval_loss.to_string(),
            profit_and_loss_amount: adf_fxdeals_data.profit_and_loss_amount,
            m_duration: adf_fxdeals_data.m_duration,
            treasury_gl_code: adf_fxdeals_data.treasury_gl_code,
        };
        writeln!(op_writer, "{}", format_output(output_data)).expect("Error in Writing Output");
    }

    let health_report = HealthReport::new(acc_enc, acc_proc, acc_enc - acc_proc, ip_amt, op_amt, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file());
}
