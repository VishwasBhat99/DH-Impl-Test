use self::io::*;
use configuration_parameters::ConfigurationParameters;
use macros;
use process::get_fields::*;
use rbdate::NaiveDate;
use sdb_io::new_buf_rdr;
use slog::Logger;
mod get_fields;
mod io;
mod structs;
use self::structs::*;
use health_report::HealthReport;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

pub fn process(config_params: &ConfigurationParameters, log: &Logger, _diag_logger: &Logger) {
    let mut op_writer = get_writer(config_params.output_file_path());
    let mut tot_records_read = 0;
    let mut tot_records_success = 0;

    let loan_repay_structure_reader = match new_buf_rdr(config_params.loan_repay_structure()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` : {}.",
            config_params.loan_repay_structure(),
            error
        ),
    };
    let mclr_data_file = match new_buf_rdr(config_params.mclr_data_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` : {}.",
            config_params.mclr_data_file(),
            error
        ),
    };
    let npa_data_file = match new_buf_rdr(config_params.npa_data_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` : {}.",
            config_params.npa_data_file(),
            error
        ),
    };
    let plr_data_file = match new_buf_rdr(config_params.plr_loan_acc_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` : {}.",
            config_params.plr_loan_acc_file(),
            error
        ),
    };

    // QUARTER END DATES FILE

    let quarter_end_dates_file = match new_buf_rdr(config_params.quarter_end_dates_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` : {}.",
            config_params.mclr_data_file(),
            error
        ),
    };

    let mut quarter_end_dates: Vec<NaiveDate> = Vec::new();

    for (line_num, lines) in quarter_end_dates_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.quarter_end_dates_file(),
                line_num + 1,
                error
            ),
        };
        quarter_end_dates.push(
            NaiveDate::parse_from_str(&line, "%d-%m-%Y").expect(&format!(
                "Cannot parse {} to date, expect format DD-MM-YYYY",
                &line
            )),
        );
    }

    let all_account_required = if config_params.all_acc_required() {
        if quarter_end_dates.contains(config_params.as_on_date()) {
            true
        } else {
            false
        }
    } else {
        false
    };

    let loan_acc_detail_home = File::open(&config_params.loan_acc_detail_home())
        .expect("Could Not Read LoanAcct_Detail_Home.");
    let loan_acc_detail_auto = File::open(&config_params.loan_acc_detail_auto())
        .expect("Could Not Read LoanAcct_Detail_auto.");
    let loan_acc_detail_personal_f1 = File::open(&config_params.loan_acc_detail_personal_f1())
        .expect("Could Not Read File LoanAcct_Detail_Personal_F1.");
    let loan_acc_detail_personal_f2 = File::open(&config_params.loan_acc_detail_personal_f2())
        .expect("Could Not Read File LoanAcct_Detail_Personal_F2.");
    let input_files_vec: Vec<File> = vec![
        loan_acc_detail_home,
        loan_acc_detail_auto,
        loan_acc_detail_personal_f1,
        loan_acc_detail_personal_f2,
    ];

    //NPA PP output
    let mut npa_map: HashMap<String, NpaData> = HashMap::new();
    for (line_num, lines) in npa_data_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.npa_data_file(),
                line_num + 1,
                error
            ),
        };
        let input_fields: Vec<&str> = line.split('|').collect();
        npa_map.insert(
            input_fields[0].to_string(),
            NpaData {
                npa_classification: input_fields[8].to_string(),
                final_npa_class: input_fields[18].to_string(),
                cif_no: input_fields[2].to_string(),
                npa_amount: input_fields[10].to_string(),
            },
        );
    }
    //MCLR
    //AGREEMENT_NO|AGREEMENT_ID|DISB_STATUS|NEXT_RESET_DATE
    let mut mclr_map: HashMap<String, NaiveDate> = HashMap::new();
    for (line_num, lines) in mclr_data_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.mclr_data_file(),
                line_num + 1,
                error
            ),
        };
        let input_fields: Vec<&str> = line.split('|').collect();
        let next_reset_date = match NaiveDate::parse_from_str(input_fields[3], "%d-%b-%y") {
            Ok(date) => Some(date),
            Err(_) => match NaiveDate::parse_from_str(input_fields[3], "%Y-%m-%d") {
                Ok(date) => Some(date),
                Err(_) => NaiveDate::parse_from_str(input_fields[3], "%d-%m-%Y").ok(),
            },
        }
        .unwrap_or_else(|| {
            NaiveDate::parse_from_str("01-01-2099", "%d-%m-%Y")
                .expect("Cannot convert NEXT_RESET_DATE to naive date")
        });

        mclr_map.insert(input_fields[0].to_string(), next_reset_date);
    }

    //PLR
    //ACCOUNT_NUMBER
    let mut plr_data_map: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in plr_data_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.plr_loan_acc_file(),
                line_num + 1,
                error
            ),
        };
        plr_data_map.insert(line.trim().to_string(), "".to_string());
    }

    //LOAN REPAYMENT STRUCTURE
    let mut loan_repayment_str_map: HashMap<String, LoanRepStr> = HashMap::new();
    //LOANACCNO|CUSTID|NPACLASSIFICATION|SEGMENT_CODE|AMOUNT
    for (line_num, lines) in loan_repay_structure_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.npa_data_file(),
                line_num + 1,
                error
            ),
        };
        let input_fields: Vec<&str> = line.split('|').collect();
        get_loan_rep_str_fields(
            &input_fields,
            &mut loan_repayment_str_map,
            config_params.as_on_date().to_owned(),
        );
    }
    for input_file in input_files_vec {
        let loan_account_detail_reader = BufReader::new(input_file);
        //LOAN ACCOUNT INPUT FILE
        for (_index, line) in loan_account_detail_reader.lines().enumerate() {
            let line = line.expect("Could Not Read Line in one of the loan_acc_detail files.");
            let input_fields: Vec<&str> = line.split('|').collect();
            tot_records_read += 1;
            let mut new_acc = LoanAccount::new();
            let default_npa = NpaData {
                npa_classification: "NA".to_string(),
                final_npa_class: "NA".to_string(),
                cif_no: "NA".to_string(),
                npa_amount: "0.0".to_string(),
            };
            let mut cif_no = String::new();
            if input_fields.len() >= 11 {
                match npa_map.get(input_fields[0]) {
                    //Implement inner join.
                    Some(val) => {
                        get_loan_acct_fields(
                            &mut new_acc,
                            input_fields.to_owned(),
                            &mclr_map,
                            &plr_data_map,
                            &loan_repayment_str_map,
                            config_params.as_on_date().to_owned(),
                            val.to_owned(),
                        );
                        cif_no = val.cif_no.to_string();
                        tot_records_success += 1;
                    }
                    None => {
                        get_loan_acct_fields(
                            &mut new_acc,
                            input_fields.to_owned(),
                            &mclr_map,
                            &plr_data_map,
                            &loan_repayment_str_map,
                            config_params.as_on_date().to_owned(),
                            default_npa,
                        );
                        cif_no = "NA".to_string();
                    }
                }
            } else {
                log_error!(log, "Found incomplete record:{}", line);
            }
            if all_account_required {
                writeln!(
                    op_writer,
                    "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|||{}||{}|{}|{}|{}|{}|{}|||{}|{}|{}|{}|ACTUAL/365|{}|{}|||{}|{}|001|{}|{}|{}|{}",
                    new_acc.accid,
                    new_acc.cbs_gl_code,
                    new_acc.pout_bal,
                    new_acc.ccy,
                    new_acc.pout_bal,
                    new_acc.pout_bal,
                    new_acc.int_rate,
                    new_acc.int_type,
                    new_acc.c_date.format("%d-%m-%Y"),
                    new_acc.p_frequency_code,
                    new_acc.branch_id,
                    new_acc.sdate.format("%d-%m-%Y"),
                    new_acc.ei_or_nonei,
                    new_acc.cfp_amt,
                    new_acc.p_frequency_code,
                    new_acc.i_frequency_code,
                    new_acc.repricing_date.format("%d-%m-%Y"),
                    new_acc.pri_inst_start_date.format("%d-%m-%Y"),
                    new_acc.int_inst_start_date.format("%d-%m-%Y"),
                    new_acc.num_inst,
                    new_acc.num_inst,
                    new_acc.exrate,
                    new_acc.scheme_code,
                    cif_no,
                    new_acc.segment_code,
                    new_acc.floating_type,
                    new_acc.npa_classification,
                    new_acc.frequency_type,
                    new_acc.final_npa_class,
                )
                .expect("Could not write to output file.");
            } else if cif_no != "NA".to_string() && cif_no != "".to_string() {
                writeln!(
                    op_writer,
                    "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|||{}||{}|{}|{}|{}|{}|{}|||{}|{}|{}|{}|ACTUAL/365|{}|{}|||{}|{}|001|{}|{}|{}|{}",
                    new_acc.accid,
                    new_acc.cbs_gl_code,
                    new_acc.pout_bal,
                    new_acc.ccy,
                    new_acc.pout_bal,
                    new_acc.pout_bal,
                    new_acc.int_rate,
                    new_acc.int_type,
                    new_acc.c_date.format("%d-%m-%Y"),
                    new_acc.p_frequency_code,
                    new_acc.branch_id,
                    new_acc.sdate.format("%d-%m-%Y"),
                    new_acc.ei_or_nonei,
                    new_acc.cfp_amt,
                    new_acc.p_frequency_code,
                    new_acc.i_frequency_code,
                    new_acc.repricing_date.format("%d-%m-%Y"),
                    new_acc.pri_inst_start_date.format("%d-%m-%Y"),
                    new_acc.int_inst_start_date.format("%d-%m-%Y"),
                    new_acc.num_inst,
                    new_acc.num_inst,
                    new_acc.exrate,
                    new_acc.scheme_code,
                    cif_no,
                    new_acc.segment_code,
                    new_acc.floating_type,
                    new_acc.npa_classification,
                    new_acc.frequency_type,
                    new_acc.final_npa_class,
                )
                .expect("Could not write to output file.");
            } else {
                log_debug!(
                    log,
                    "Found CIF_NO'NA'for account number {}",
                    new_acc.accid.to_string()
                );
                continue;
            }
        }
    }

    let health_report = HealthReport::new(
        tot_records_read,
        tot_records_success,
        tot_records_read - tot_records_success,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(config_params.output_file_path());
}
