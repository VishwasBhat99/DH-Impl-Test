use crate::configuration_parameters::ConfigurationParameters;
use crate::process::input_account::*;
use crate::process::output_account::*;
use chrono::NaiveDate;
use health_report::HealthReport;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::BufRead;
use std::io::Write;
mod input_account;
mod output_account;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_proc = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;

    let mut op_writer = get_writer(config_params.output_file());

    //Reading ACH File
    let mut ach_map: HashMap<String, ACHData> = HashMap::new();
    let ach_file = match new_buf_rdr(config_params.ach_input_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_params.ach_input_file(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in ach_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.ach_input_file(),
                line_num + 1,
                error
            ),
        };
        let ach_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let ach_data: ACHData = ACHData::new(
            config_params,
            config_params.ach_input_file(),
            &ach_vec,
            line_num + 1,
        );
        if ach_data.user_classification_date <= *config_params.as_on_date() {
            ach_map.insert(ach_data.b2k_id.to_string(), ach_data);
        }
    }

    let mut npa_map: HashMap<String, NPAData> = HashMap::new();
    let npa_file = match new_buf_rdr(config_params.npa_input_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_params.npa_input_file(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in npa_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.npa_input_file(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        npa_map.insert(
            fields[1].to_string(),
            NPAData {
                npa_classification: fields[8].to_string(),
                cust_hlth_code: fields[12].to_string(),
                cust_npa_class: fields[17].to_string(),
                final_npa_class: fields[18].to_string(),
                npa_amount: fields[10].to_string(),
            },
        );
    }

    //Reading Loans Overdue File
    let mut overdue_map: HashMap<String, f64> = HashMap::new();
    let overdue_file = match new_buf_rdr(config_params.overdue_input_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_params.overdue_input_file(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in overdue_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.overdue_input_file(),
                line_num + 1,
                error
            ),
        };
        let overdue_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        overdue_map.insert(
            get_str(
                config_params.overdue_input_file(),
                &overdue_vec,
                0,
                line_num + 1,
            ),
            get_str(
                config_params.overdue_input_file(),
                &overdue_vec,
                1,
                line_num + 1,
            )
            .parse::<f64>()
            .unwrap_or(0.0),
        );
    }
    //Reading EIT File
    let mut eit_map: HashMap<String, EITData> = HashMap::new();

    let eit_file = match new_buf_rdr(config_params.eit_input_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_params.eit_input_file(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in eit_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.eit_input_file(),
                line_num + 1,
                error
            ),
        };
        let eit_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let eit_data: EITData = EITData::new(
            config_params,
            config_params.eit_input_file(),
            &eit_vec,
            line_num + 1,
        );
        eit_map.insert(eit_data.entity_id.to_string(), eit_data);
    }
    //Reading Rate Code Master File
    let mut rate_code_master: HashMap<String, RateCodeMaster> = HashMap::new();
    let rate_code_file = match new_buf_rdr(config_params.rate_code_mapping_master()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_params.rate_code_mapping_master(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in rate_code_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.rate_code_mapping_master(),
                line_num + 1,
                error
            ),
        };
        let rate_code_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let rate_code_data: RateCodeMaster = RateCodeMaster::new(
            config_params.rate_code_mapping_master(),
            &rate_code_vec,
            line_num + 1,
        );
        rate_code_master.insert(rate_code_data.int_rate_code.to_string(), rate_code_data);
    }
    //Reading LAM File
    let mut lam_map: HashMap<String, LAMData> = HashMap::new();

    let lam_file = match new_buf_rdr(config_params.lam_input_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_params.lam_input_file(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in lam_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.lam_input_file(),
                line_num + 1,
                error
            ),
        };
        let lam_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let lam_data: LAMData = LAMData::new(
            config_params,
            config_params.lam_input_file(),
            &lam_vec,
            line_num + 1,
        );
        lam_map.insert(lam_data.acid.to_string(), lam_data);
    }
    //Reading Loans IntRate File
    let mut intrate_map: HashMap<String, f64> = HashMap::new();
    let intrate_reader = match new_buf_rdr(config_params.intrate_input_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_params.intrate_input_file(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_no, lines) in intrate_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.lam_input_file(),
                line_no + 1,
                error
            ),
        };
        let intrate_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        intrate_map.insert(
            get_str(
                config_params.intrate_input_file(),
                &intrate_vec,
                0,
                line_no + 1,
            ),
            get_str(
                config_params.intrate_input_file(),
                &intrate_vec,
                1,
                line_no + 1,
            )
            .parse::<f64>()
            .unwrap_or(1.0),
        );
    }
    //Reading ITC File
    let mut itc_map: HashMap<String, ITCData> = HashMap::new();
    let itc_reader = match new_buf_rdr(config_params.itc_input_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_params.itc_input_file(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_no, lines) in itc_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.itc_input_file(),
                line_no + 1,
                error
            ),
        };
        let itc_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let itc_int_tbl_code_srl_num = itc_vec[17].parse::<i64>().unwrap_or(0);
        itc_map
            .entry(itc_vec[0].to_string())
            .and_modify(|val| {
                if itc_int_tbl_code_srl_num > val.int_tbl_code_srl_num {
                    *val = ITCData {
                        entity_id: itc_vec[0].to_string(),
                        int_tbl_code: itc_vec[2].to_string(),
                        int_tbl_code_srl_num: itc_int_tbl_code_srl_num,
                        peg_review_date: NaiveDate::parse_from_str(itc_vec[36], "%d-%m-%Y")
                            .unwrap_or(
                                NaiveDate::from_ymd_opt(1970, 1, 1)
                                    .expect("Could not write default date"),
                            ),
                    }
                }
            })
            .or_insert(ITCData {
                entity_id: itc_vec[0].to_string(),
                int_tbl_code: itc_vec[2].to_string(),
                int_tbl_code_srl_num: itc_int_tbl_code_srl_num,
                peg_review_date: NaiveDate::parse_from_str(itc_vec[36], "%d-%m-%Y").unwrap_or(
                    NaiveDate::from_ymd_opt(1970, 1, 1).expect("Could not write default date"),
                ),
            });
    }
    //Reading Loan_File_additional
    let mut loan_file_add_map: HashMap<String, LOANFILEData> = HashMap::new();
    let loan_file_add = match new_buf_rdr(config_params.loan_add_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_params.loan_add_file(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in loan_file_add.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.loan_add_file(),
                line_num + 1,
                error
            ),
        };
        let loan_add_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let loan_acc_data: LOANFILEData = LOANFILEData::new(
            config_params,
            config_params.loan_add_file(),
            &loan_add_vec,
            line_num + 1,
        );
        loan_file_add_map.insert(loan_acc_data.foracid.to_string(), loan_acc_data);
    }
    //Reading Loans File
    let loans_reader = match new_buf_rdr(config_params.gam_input_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_params.gam_input_file(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_no, lines) in loans_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.lam_input_file(),
                line_no + 1,
                error
            ),
        };
        let loans_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let loans_data = GAMData::new(
            config_params,
            config_params.gam_input_file(),
            &loans_vec,
            line_no + 1,
        );
        acc_enc += 1;
        ip_amt += loans_data.clr_bal_amt;
        let output_data = OutputAccount::new(
            loans_data,
            config_params,
            &mut ach_map,
            &mut eit_map,
            &mut overdue_map,
            &mut intrate_map,
            &mut lam_map,
            &mut itc_map,
            &mut rate_code_master,
            &mut loan_file_add_map,
            &npa_map,
            logger,
        );
        op_amt += output_data.clr_bal_amt;
        writeln!(op_writer, "{}", format_output(output_data)).expect("Error in Writing Output");
        acc_proc += 1;
    }
    let health_report = HealthReport::new(acc_enc, acc_proc, acc_enc - acc_proc, ip_amt, op_amt, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file());
}
