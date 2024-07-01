use self::structs::{Account, GrpData, OPKey, OPVal};
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use process::structs;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufWriter, Write};
use std::time::{Duration, SystemTime};

pub fn gen_summary(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    prod_rpt_map: &HashMap<i64, GrpData>,
    llg_mapping: &HashMap<i64, i64>,
    is_active_map: &mut HashMap<String, String>,
    grp_amt_map: &HashMap<String, f64>,
    exrt_map: &HashMap<String, f64>,
) -> HashMap<OPKey, OPVal> {
    let start_time = SystemTime::now();
    let input_file_name = format!("{}-summary.txt", config_params.input_file_path());
    let output_file_name = format!("{}-converted-summary.txt", config_params.output_file_path());
    let input_rdr = match new_buf_rdr(&input_file_name) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            config_params.input_file_path(),
            e
        )),
    };
    let mut output_wtr = match buf_file_wrtr(&output_file_name, None) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot write to file at path: '{}', Error: '{}'",
            config_params.output_file_path(),
            e
        )),
    };

    let mut total_account = 0;
    let mut total_failed_account = 0;
    let mut total_success_account = 0;
    let mut total_input_amount = 0.0;
    let total_output_amount = 0.0;
    let mut op_map: HashMap<OPKey, OPVal> = HashMap::new();
    for line in input_rdr.lines() {
        match line {
            //if the line has no errors
            Ok(each_line) => {
                match Account::new_from_line(each_line.to_owned()) {
                    //if theres no error when line was parsed as account struct
                    Ok(account) => {
                        total_account += 1;
                        total_success_account += 1;
                        let mut field_1 = account.field_1.to_string();
                        if field_1 == "IRS" || field_1 == "INT" {
                            continue;
                        }
                        if field_1 == "SLR" {
                            field_1 = "ALL".to_string();
                        }
                        if !llg_mapping.contains_key(&account.llg_code) {
                            let op_key = OPKey {
                                llg: account.llg_code.to_string(),
                                as_on: account.date.to_owned(),
                                ccy: account.currency.to_owned(),
                                field4: field_1.to_owned(),
                                field5: account.field_2.to_owned(),
                                flow_type: account.field_3.to_owned(),
                            };
                            let op_val = OPVal {
                                amt: account.amount,
                                int: account.field_5,
                            };
                            op_map.insert(op_key, op_val);
                            continue;
                        }
                        let grp_id = llg_mapping
                            .get(&account.llg_code)
                            .expect("Cannot fetch infor for llg");
                        let grp_ccy =
                            grp_id.to_string() + &"|".to_string() + &account.currency.to_string();
                        if !prod_rpt_map.contains_key(&grp_id) {
                            continue;
                        }
                        let grp_info = prod_rpt_map
                            .get(grp_id)
                            .expect("cannot fetch info for group id");
                        let mut limit = grp_info.limit;
                        let exrt = exrt_map
                            .get(&account.currency)
                            .expect("cannot fetch exchange rate");
                        let llg_amt = grp_amt_map
                            .get(&grp_ccy)
                            .expect("Cannot find grp currency combination in map");
                        let mut ratio = 1.0;
                        let base_curr_amt = grp_amt_map
                            .get(
                                &(grp_id.to_string() + &"|".to_string() + config_params.base_ccy()),
                            )
                            .expect("Cannot find grp currency combination in map");
                        limit = (llg_amt / base_curr_amt) * limit;
                        if llg_amt > &limit {
                            ratio = 1.0 - (limit / llg_amt);
                        } else {
                            ratio = 0.0;
                        }
                        total_input_amount += account.amount;
                        //if the limit_amount was found in the prod_map hashmap
                        //if the conversation from INT->INR no need to lookup in hashmap
                        let mut amount = account.amount * ratio;
                        let op_key = OPKey {
                            llg: account.llg_code.to_string(),
                            as_on: account.date.to_owned(),
                            ccy: account.currency.to_owned(),
                            field4: field_1.to_owned(),
                            field5: account.field_2.to_owned(),
                            flow_type: account.field_3.to_owned(),
                        };
                        let op_val = OPVal {
                            amt: amount,
                            int: account.field_5,
                        };
                        let mut is_active_key = account.llg_code.to_string()
                            + &"|".to_string()
                            + &account.currency.to_string();
                        is_active_map.insert(is_active_key, "N".to_string());
                        op_map.insert(op_key, op_val);
                        //if the account amount greater than limit
                        let account_total_amount = account.amount;

                        let llg_alternative = grp_info.limit_llg;
                        let mut alternate_op_val = OPVal::default();

                        let alternate_op_key = OPKey {
                            llg: llg_alternative.to_string(),
                            as_on: account.date.to_string(),
                            ccy: account.currency.to_string(),
                            field4: field_1.to_string(),
                            field5: account.field_2.to_string(),
                            flow_type: account.field_3.to_string(),
                        };
                        if op_map.contains_key(&alternate_op_key) {
                            alternate_op_val = op_map.get(&alternate_op_key).unwrap().to_owned();
                        }
                        is_active_key = llg_alternative.to_string()
                            + &"|".to_string()
                            + &account.currency.to_string();
                        if llg_amt > &limit {
                            alternate_op_val.amt = limit.to_owned() * (1.0 / exrt);
                        } else {
                            alternate_op_val.amt = llg_amt.to_owned() * (1.0 / exrt);
                        }
                        is_active_map.insert(is_active_key, "N".to_string());
                        op_map.insert(alternate_op_key, alternate_op_val);

                        //if the limit amount was not found this llgcode, write line as it is
                    }
                    Err(e) => {
                        total_account += 1;
                        total_failed_account += 1;
                        log_error!(logger, "Couldn't parse InputAccount: {}", e);
                    }
                }
            }
            Err(..) => {}
        }
    }
    let health_report = HealthReport::new(
        total_account,
        total_success_account,
        total_failed_account,
        total_input_amount,
        total_output_amount,
        0,
    );
    log_info!(logger, "{}", health_report.display());
    log_info!(
        logger,
        "total time for Summary Generation : {:#?}",
        start_time.elapsed()
    );

    println!("{}", health_report.display());
    println!(
        "total time for Summary Generation :{:?}",
        start_time.elapsed().unwrap_or(Duration::new(0, 0))
    );
    health_report.gen_health_rpt(config_params.output_file_path());
    write_output(&op_map, &mut output_wtr, logger);
    op_map
}

//function to write each account to output
fn write_output(op_map: &HashMap<OPKey, OPVal>, writer: &mut BufWriter<File>, logger: &Logger) {
    for (key, value) in op_map {
        write!(
            writer,
            "{}|{}|{}|{}|{}|{}|{}|{}\n",
            key.llg,
            key.as_on,
            key.ccy,
            key.field4,
            key.field5,
            key.flow_type,
            value.amt,
            value.int
        )
        .expect("summary file writing error");
    }
}
