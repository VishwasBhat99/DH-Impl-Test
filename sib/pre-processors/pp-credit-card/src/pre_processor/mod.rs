extern crate serde;
use self::format::get_op_line;
use self::format::*;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;
mod format;

pub fn process(config_param: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let op_emi = format!("{}-emi.txt", config_param.output_file_path());
    let op_non_emi = format!("{}-non-emi.txt", config_param.output_file_path());
    let output_file_emi = match buf_file_wrtr(&op_emi, None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let output_file_non_emi = match buf_file_wrtr(&op_non_emi, None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let mut tot_acc = 0;
    let mut tot_succ = 0;
    let mut ttl_amt = 0.0;
    let mut tot_cfs = 0;

    let mut writer_emi = BufWriter::new(output_file_emi);
    let mut writer_non_emi = BufWriter::new(output_file_non_emi);
    let cashflow_file = match new_buf_rdr(config_param.cashflow_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_param.cashflow_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let start_timer = SystemTime::now();
    let mut cashflow_map: HashMap<String, Vec<CashflowData>> = HashMap::new();
    let as_on_date = config_param.as_on_date();
    for (line_num, lines) in cashflow_file.lines().enumerate() {
        let cashflow_line = match lines {
            Ok(cashflow_line) => cashflow_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.cashflow_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = cashflow_line.split('|').collect();
        let key = format!("{}_{}_{}", fields[0], fields[1], fields[9]);
        let cashflow = get_cashflow_data(fields);
        cashflow_map
            .entry(key)
            .and_modify(|cf| cf.push(cashflow))
            .or_insert(vec![cashflow]);
    }
    let end_timer = SystemTime::now();
    let duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate CashflowData file processing duration.");
    log_debug!(log, "CashflowData file derive duration:{:?}", duration);

    //Sort the cashflows based on cf-date.
    for (_, cashflow_list) in cashflow_map.iter_mut() {
        cashflow_list.sort_by(|a, b| a.cf_date.cmp(&b.cf_date));
    }

    let input_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input data file: `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let npa_file = match new_buf_rdr(config_param.npa_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_param.npa_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut npa_map: HashMap<String, NPAFields> = HashMap::new();
    for (line_num, lines) in npa_file.lines().enumerate() {
        let npa_line = match lines {
            Ok(npa_line) => npa_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.npa_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = npa_line.split('|').collect();
        npa_map.insert(fields[0].to_string(), NPAFields::get_npa_fields(fields));
    }
    let deafult_npa = NPAFields::new();
    for (line_no, lines) in input_file.lines().enumerate() {
        let line = match lines {
            Ok(data) => data,
            Err(error) => panic!(
                "Unable to read file `{}` at line no:{}: {}",
                config_param.input_file_path(),
                line_no + 1,
                error
            ),
        };
        tot_acc += 1;
        let fields: Vec<&str> = line.split('|').collect();
        //Match the output format to that of cf_ubs_loans ltd.
        let acc_id_concat = format!("{}_{}_{}", fields[0], fields[1], fields[3]);
        let emi_lien_bal = fields[14].parse::<f64>().unwrap_or(0.0);
        let emi_end_date = NaiveDate::parse_from_str(fields[25], "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd(1900, 1, 1));

        let default_cf_value = vec![CashflowData {
            tenure: 0,
            instl_id: 0,
            cf_date: emi_end_date,
            payment: 0.0,
            principal_payment: emi_lien_bal,
            interest_payment: 0.0,
            int_rate: 0.0,
            card_number: 0,
        }];
        let cf_vec = match cashflow_map.get(&acc_id_concat) {
            Some(cashflow_vec) => cashflow_vec,
            None => &default_cf_value,
        };
        let last_inst_date = NaiveDate::parse_from_str(fields[26], "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd(1900, 1, 1));
        let npa_fields: &NPAFields = npa_map.get(&fields[0].to_string()).unwrap_or(&deafult_npa);
        let mut no_cf_flag = false;
        for cf in cf_vec.iter() {
            //Condition to write cashflows.
            if cf.cf_date > last_inst_date {
                no_cf_flag = true;
                get_output(
                    cf,
                    acc_id_concat.to_owned(),
                    &mut ttl_amt,
                    &mut tot_cfs,
                    fields.to_owned(),
                    *as_on_date,
                    &mut writer_emi,
                    &mut writer_non_emi,
                    &npa_fields,
                );
                tot_cfs += 1;
            }
        }
        if no_cf_flag == false {
            get_output(
                &default_cf_value[0],
                acc_id_concat.to_owned(),
                &mut ttl_amt,
                &mut tot_cfs,
                fields.to_owned(),
                *as_on_date,
                &mut writer_emi,
                &mut writer_non_emi,
                &npa_fields,
            )
        }
        tot_succ += 1;
    }
    let health_report = HealthReport::new(
        tot_acc,
        tot_succ,
        tot_acc - tot_succ,
        ttl_amt,
        ttl_amt,
        tot_cfs,
    );
    health_report.gen_health_rpt(config_param.output_file_path());
}

fn get_output(
    cf: &CashflowData,
    acc_id_concat: String,
    ttl_amt: &mut f64,
    tot_cfs: &mut i64,
    fields: Vec<&str>,
    as_on_date: NaiveDate,
    writer_emi: &mut BufWriter<BufWriter<std::fs::File>>,
    writer_non_emi: &mut BufWriter<BufWriter<std::fs::File>>,
    npa_fields: &NPAFields,
) {
    //Principal cashflow.
    let component = "PRINCIPAL";
    let cf_amt = cf.principal_payment;
    *ttl_amt += cf.principal_payment;
    *tot_cfs += 1;
    let op_line_prin = get_op_line(
        &acc_id_concat,
        &fields,
        cf,
        component,
        as_on_date,
        cf_amt,
        npa_fields,
    );
    //Interest cashflow.
    let component = "MAIN_INT";
    let cf_amt = cf.interest_payment;
    let op_line_int = get_op_line(
        &acc_id_concat,
        &fields,
        cf,
        component,
        as_on_date,
        cf_amt,
        npa_fields,
    );
    //Segregate to EMI and NON EMI cashflows.
    if fields[2].to_string().to_uppercase() == "EI" {
        match writer_emi.write_all(op_line_prin.as_bytes()) {
            Ok(val) => val,
            Err(error) => {
                panic!("Error writing processed data: {:?}", error);
            }
        }
        match writer_emi.write_all(op_line_int.as_bytes()) {
            Ok(val) => val,
            Err(error) => {
                panic!("Error writing processed data: {:?}", error);
            }
        }
    } else {
        match writer_non_emi.write_all(op_line_prin.as_bytes()) {
            Ok(val) => val,
            Err(error) => {
                panic!("Error writing processed data: {:?}", error);
            }
        }
        match writer_non_emi.write_all(op_line_int.as_bytes()) {
            Ok(val) => val,
            Err(error) => {
                panic!("Error writing processed data: {:?}", error);
            }
        }
    }
}
