use self::derive::get_comp_result;
use configuration_parameters::ConfigurationParameters;
use itertools::Itertools;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

mod derive;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration read timer.");
    debug!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );

    let start_derive_timer = SystemTime::now();
    let mut re_strct_out: String = String::new();
    let mut res_out: String = String::new();
    let mut rw_out: String = String::new();
    let mut tot_acc_encntrd: i64 = 0;
    let mut re_str_accs: i64 = 0;
    let mut resid_accs: i64 = 0;
    let mut rw_accs: i64 = 0;
    let mut skp_accs: i64 = 0;
    let mut is_header = true;
    for (line_num, lines) in file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num + 1,
                error
            ),
        };
        tot_acc_encntrd += 1;
        let skip_rec = line.to_string();
        let mut fields: Vec<&str> = line.split(config_param.inp_del()).collect();
        let resi_field_string = config_param.resid_desc().to_uppercase();
        let resi_fields: Vec<&str> = resi_field_string.split(",").collect();
        let in_exp_field_desc = config_param
            .exp_def_flag_desc()
            .trim()
            .to_string()
            .to_ascii_uppercase();
        let exp_field_pos = config_param.exp_def_flag_pos();
        let in_cap_mkt_exp_desc = config_param
            .cap_mkt_exp_desc()
            .trim()
            .to_string()
            .to_ascii_uppercase();
        let cap_mkt_exp_pos = config_param.cap_mkt_exp_pos();

        if fields.len() > 1 {
            if is_header {
                re_strct_out.push_str(&format!(
                    "{}|{}\n",
                    fields[config_param.acc_no_pos()],
                    fields[config_param.re_struct_field_pos()]
                ));
                res_out.push_str(&format!(
                    "{}|{}\n",
                    fields[config_param.acc_no_pos()],
                    fields[config_param.resid_field_pos()],
                ));
                rw_out.push_str(&format!(
                    "{}|{}\n",
                    fields[config_param.acc_no_pos()],
                    fields[config_param.rw_field_pos()]
                ));
                is_header = false;
                continue;
            }
            let mut new_acc_no = fields[config_param.acc_no_pos()].to_string();
            // Iterate through each source code in the 'src_codes' collection
            for src_code in config_param.src_codes() {
                // Check if the account number contains the current source code
                if fields[config_param.acc_no_pos()].contains(src_code) {
                    // Replace the source code with an empty string to remove it from the account number
                    new_acc_no =
                        fields[config_param.acc_no_pos()].replace(src_code, &String::default());
                    break; // Exit the loop since the replacement has been made
                }
            }
            if new_acc_no.len() < config_param.remove_last_char() {
                log_error!(
                    log,
                    "Length of account number is less than remove last character.
                    Length of account number is {:?} and remove last character is {:?}",
                    new_acc_no.len(),
                    config_param.remove_last_char()
                );
            } else {
                let acc_no_length = new_acc_no.len() - config_param.remove_last_char();
                new_acc_no = String::from(&new_acc_no[..acc_no_length]);
            }

            fields[config_param.acc_no_pos()] = &new_acc_no;
            if config_param.check_and_write_for_all_cases {
                if fields[config_param.re_struct_field_pos()]
                    .to_uppercase()
                    .replace(" ", "")
                    == config_param
                        .re_struct_desc()
                        .to_uppercase()
                        .replace(" ", "")
                {
                    re_str_accs += 1;
                    re_strct_out.push_str(&format!(
                        "{}|{}\n",
                        fields[config_param.acc_no_pos()],
                        fields[config_param.re_struct_field_pos()]
                    ));
                    log_debug!(
                        log,
                        "Account: {} written to Re-structred Output File",
                        fields[config_param.acc_no_pos()],
                    );
                }
                let exp_flag_desc = fields[exp_field_pos]
                    .trim()
                    .to_string()
                    .to_ascii_uppercase();
                let cap_mkt_desc = fields[cap_mkt_exp_pos]
                    .trim()
                    .to_string()
                    .to_ascii_uppercase();
                let resid_desc = fields[config_param.resid_field_pos()]
                    .to_string()
                    .trim()
                    .to_uppercase();
                let flag = resi_fields.contains(&resid_desc.as_str()).clone();
                if flag && cap_mkt_desc == in_cap_mkt_exp_desc && exp_flag_desc == in_exp_field_desc
                {
                    resid_accs += 1;
                    res_out.push_str(&format!("{}|{}\n", fields[config_param.acc_no_pos()], "Y"));
                    log_debug!(
                        log,
                        "Account: {} written to Residual-Mortgage Output File",
                        fields[config_param.acc_no_pos()],
                    );
                }
                if get_comp_result(
                    fields[config_param.rw_field_pos()]
                        .parse()
                        .expect("Incorrect input risk weight."),
                    config_param.comparator(),
                    config_param.rw_desc(),
                ) {
                    rw_accs += 1;
                    rw_out.push_str(&format!(
                        "{}|{}\n",
                        fields[config_param.acc_no_pos()],
                        fields[config_param.rw_field_pos()]
                    ));
                    log_debug!(
                        log,
                        "Account: {} written to Risk-Weight Output File",
                        fields[config_param.acc_no_pos()],
                    );
                }
            } else {
                let exp_flag_desc = fields[exp_field_pos]
                    .trim()
                    .to_string()
                    .to_ascii_uppercase();
                let cap_mkt_desc = fields[cap_mkt_exp_pos]
                    .trim()
                    .to_string()
                    .to_ascii_uppercase();
                let resid_desc = fields[config_param.resid_field_pos()]
                    .to_string()
                    .trim()
                    .to_uppercase();
                let flag = resi_fields.contains(&resid_desc.as_str()).clone();
                if fields[config_param.re_struct_field_pos()]
                    .to_uppercase()
                    .replace(" ", "")
                    == config_param
                        .re_struct_desc()
                        .to_uppercase()
                        .replace(" ", "")
                {
                    re_str_accs += 1;
                    re_strct_out.push_str(&format!(
                        "{}|{}\n",
                        fields[config_param.acc_no_pos()],
                        fields[config_param.re_struct_field_pos()]
                    ));
                    log_debug!(
                        log,
                        "Account: {} written to Re-structred Output File",
                        fields[config_param.acc_no_pos()],
                    );
                } else if flag
                    && cap_mkt_desc == in_cap_mkt_exp_desc
                    && exp_flag_desc == in_exp_field_desc
                {
                    resid_accs += 1;
                    res_out.push_str(&format!("{}|{}\n", fields[config_param.acc_no_pos()], "Y"));
                    log_debug!(
                        log,
                        "Account: {} written to Residual-Mortgage Output File",
                        fields[config_param.acc_no_pos()],
                    );
                } else if get_comp_result(
                    fields[config_param.rw_field_pos()]
                        .parse()
                        .expect("Incorrect input risk weight."),
                    config_param.comparator(),
                    config_param.rw_desc(),
                ) {
                    rw_accs += 1;
                    rw_out.push_str(&format!(
                        "{}|{}\n",
                        fields[config_param.acc_no_pos()],
                        fields[config_param.rw_field_pos()]
                    ));
                    log_debug!(
                        log,
                        "Account: {} written to Risk-Weight Output File",
                        fields[config_param.acc_no_pos()],
                    );
                }
            }
        } else {
            skp_accs += 1;
            log_debug!(log, "Skipped record: `{}`.", skip_rec);
            continue;
        }
    }
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    let start_write_timer = SystemTime::now();
    let mut re_strct_writer = match buf_file_wrtr(config_param.re_struct_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_param.re_struct_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };

    match re_strct_writer.write_all(re_strct_out.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`: {}.",
            config_param.re_struct_file_path(),
            error,
        ),
    }

    let mut res_writer = match buf_file_wrtr(config_param.resid_mort_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create reconcilation file `{}` on location `{}` : {}",
            config_param.resid_mort_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match res_writer.write_all(res_out.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write reconcilation lines on file `{}`: {}.",
            config_param.resid_mort_file_path(),
            error,
        ),
    };

    let mut rw_writer = match buf_file_wrtr(config_param.rw_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create concat file: `{}` on location `{}` : {}",
            config_param.rw_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match rw_writer.write_all(rw_out.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write concat lines to the file `{}`: {}.",
            config_param.rw_file_path(),
            error,
        ),
    }
    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing pre-processed output and reconcilation files.");
    debug!(
        diag_log,
        "Writing Output Files, Total Duration: {:?}.", duration
    );

    let report_string = format!(
        "Accounts encountered: {}\n\
         Restructured Accounts: {}\n\
         Residential Mortgage Accounts: {}\n\
         Risk Weight Accounts: {}\n\
         Uncategorised Accounts: {}\n\
         Accounts failed to process: {}",
        tot_acc_encntrd,
        re_str_accs,
        resid_accs,
        rw_accs,
        tot_acc_encntrd - re_str_accs - resid_accs - rw_accs - skp_accs,
        skp_accs,
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
}
