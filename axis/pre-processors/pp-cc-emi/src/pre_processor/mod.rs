extern crate serde;
use self::format::*;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::DateParser;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;

mod format;
pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let npa_output_file = match buf_file_wrtr(
        &config_param.output_file_path().replace(".txt", "_npa.txt"),
        None,
    ) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };

    let performing_output_file = match buf_file_wrtr(
        &config_param
            .output_file_path()
            .replace(".txt", "_performing.txt"),
        None,
    ) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };

    let mut tot_input_acc_encntrd: i64 = 0;
    let mut tot_data_acc_encntrd: i64 = 0;
    let mut tot_live_acc_encntrd: i64 = 0;
    let mut tot_config_acc_encntrd: i64 = 0;
    let mut npa_writer = BufWriter::new(npa_output_file);
    let mut performing_writer = BufWriter::new(performing_output_file);
    let date_parser = DateParser::new("%d-%b-%y".to_string(), true);

    let default_date_parser = DateParser::new("%d-%m-%Y".to_string(), true);

    let npa_data_file = match new_buf_rdr(config_param.npa_data_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found npa data file: `{}` on location `{}` : {}.",
            config_param.npa_data_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let start_datafile_derive_timer = SystemTime::now();
    let mut npa_data_map: HashMap<String, (String, f64)> = HashMap::new();
    for (line_num, lines) in npa_data_file.lines().enumerate() {
        let npa_data_line = match lines {
            Ok(npa_data_line) => npa_data_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.npa_data_file_path(),
                line_num + 1,
                error
            ),
        };

        tot_data_acc_encntrd += 1;
        let npa_data_fields = npa_data_line.split('|').collect::<Vec<&str>>();
        if npa_data_fields.len() >= 5 {
            //Store the LoanAccNo and NPA_Classification as key-value pairs.
            npa_data_map.insert(
                npa_data_fields[0].to_string(),
                (
                    npa_data_fields[2].to_string(),
                    npa_data_fields[4]
                        .to_string()
                        .trim()
                        .parse::<f64>()
                        .unwrap_or(0.0),
                ),
            );
        } else {
            log_error!(
                log,
                "Found line :{} at line number {} defective in NPA data file.",
                line_num,
                npa_data_line
            );
        }
    }
    let end_datafile_derive_timer = SystemTime::now();
    let duration = end_datafile_derive_timer
        .duration_since(start_datafile_derive_timer)
        .expect("Could not calculate NPA Data File derive process duration.");
    debug!(
        diag_log,
        "NPA Data File Derive Process Total Duration: {:?}.", duration
    );

    let npa_live_file = match new_buf_rdr(config_param.npa_live_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found npa live file: `{}` on location `{}` : {}.",
            config_param.npa_live_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let start_livefile_derive_timer = SystemTime::now();
    let mut npa_live_hashmap: HashMap<String, String> = HashMap::new();

    for (line_num, lines) in npa_live_file.lines().enumerate() {
        let npa_live_line = match lines {
            Ok(npa_live_line) => npa_live_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.npa_live_file_path(),
                line_num + 1,
                error
            ),
        };

        tot_live_acc_encntrd += 1;

        let npa_live_fields = npa_live_line.split('|').collect::<Vec<&str>>();
        if npa_live_fields.len() >= 7 {
            //Store the Finacle_CustID and cust_hlth_code as key-value pairs.
            npa_live_hashmap.insert(
                npa_live_fields[0].to_string(),
                npa_live_fields[2].to_string(),
            );
        } else {
            log_error!(
                log,
                "Found line :{} at line number {} defective in NPA Live file.",
                line_num,
                npa_live_line
            );
        }
    }
    let end_livefile_derive_timer = SystemTime::now();
    let duration = end_livefile_derive_timer
        .duration_since(start_livefile_derive_timer)
        .expect("Could not calculate NPA Live File derive process duration.");
    debug!(
        diag_log,
        "NPA Live File Derive Process Total Duration: {:?}.", duration
    );

    let npa_config_file = match new_buf_rdr(config_param.npa_config_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found npa config file: `{}` on location `{}` : {}.",
            config_param.npa_config_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let start_configfile_derive_timer = SystemTime::now();

    let mut npa_config_hashmap: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in npa_config_file.lines().enumerate() {
        let npa_config_line = match lines {
            Ok(npa_config_line) => npa_config_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.npa_config_file_path(),
                line_num + 1,
                error
            ),
        };

        tot_config_acc_encntrd += 1;

        let npa_config_fields = npa_config_line.split('|').collect::<Vec<&str>>();
        npa_config_hashmap.insert(
            npa_config_fields[0].to_string(),
            npa_config_fields[1].to_string(),
        );
    }
    let end_configfile_derive_timer = SystemTime::now();
    let duration = end_configfile_derive_timer
        .duration_since(start_configfile_derive_timer)
        .expect("Could not calculate NPA Config File derive process duration.");
    debug!(
        diag_log,
        "NPA Config File Derive Process Total Duration: {:?}.", duration
    );

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
    let mut enc_acids: HashSet<String> = HashSet::new();
    for (line_no, lines) in input_file.lines().enumerate() {
        let fields = match lines {
            Ok(data) => data,
            Err(error) => panic!(
                "Unable to read file `{}` at line no:{}: {}",
                config_param.input_file_path(),
                line_no + 1,
                error
            ),
        };
        tot_input_acc_encntrd += 1;
        let input_account: Vec<&str> = fields.split('|').collect();
        let acid = input_account[0].trim().to_string();
        let cust_id = input_account[1].trim().to_string();
        let npa_type = match npa_data_map.get(&acid) {
            Some(typ) => typ.to_owned().0,
            None => "0".to_string(),
        };
        let npa_amt = match npa_data_map.get(&acid) {
            Some(npa_amt) => npa_amt.1,
            None => 0.0,
        };
        let cust_health_code = npa_live_hashmap
            .get(&cust_id)
            .unwrap_or(&"0".to_string())
            .to_owned();
        let cust_npa_class = npa_config_hashmap
            .get(&cust_health_code.to_owned())
            .unwrap_or(&"0".to_string())
            .to_owned();
        let final_npa_class = if npa_type != "0" {
            npa_type.to_owned()
        } else if cust_npa_class != "0" {
            cust_npa_class.to_string()
        } else {
            "0".to_string()
        };
        let user_def_stats = match npa_live_hashmap.get(&cust_id) {
            Some(stats) => match npa_config_hashmap.get(stats) {
                Some(code) => code.to_owned(),
                None => "0".to_string(),
            },
            None => "0".to_string(),
        };
        let mut component = match input_account[10].trim().to_uppercase().as_str() {
            "IINT" => "MAIN_INT",
            _ => "PRINCIPAL",
        };

        let overdue_amt = if final_npa_class == "0" {
            input_account[3].parse::<f64>().unwrap_or(0.0)
        } else {
            npa_amt
        };
        let mut cf_amt = if final_npa_class == "0" {
            input_account[12].parse::<f64>().unwrap_or(0.0)
        } else {
            npa_amt
        };
        let mut due_date = date_parser
            .parse_opt(input_account[11])
            .unwrap_or(default_date_parser.parse("01-01-1900"))
            .format("%d-%m-%Y")
            .to_string();
        //Output field format: cust_no|reference|cust_name|branch_cd|norm_int_rt|acurl_freq|book_dt|val_dt|mat_dt|due_dt|user_def_stats|prod_cd|gl|curr|prin_ost_bal|component|amt_due|amt_setld|cf_amt|spread|compmis1|compmis2|compmis3|old_rt_cd|old_rt_typ|old_benchmark|nxt_reset_dt|last_reset_dt|rt_flag_new|rt_cd_new|division|concat|alm_line|ia_llg|balm_llg|repricing_freq|nxt_repricing_dt|lst_repricing_dt|as_on_dt|int_basis|int_calc_typ|cust_typ|npa_typ|bmid|cntr_party|lcy_amount|raw_benchmark|der_int_rate|bnchmrk_rate|spread_val|fully_floating_flg|call_option_date|put_option_date|is_acc_weaker|ews_weaker_value
        let op_line = get_op_line(
            input_account.to_owned(),
            cust_id.to_owned(),
            acid.to_owned(),
            user_def_stats.to_owned(),
            component.to_string(),
            npa_type.to_owned(),
            config_param.currency().to_string(),
            config_param.as_on_date(),
            overdue_amt,
            cf_amt,
            due_date.to_owned(),
            &cust_health_code,
            &cust_npa_class,
            &final_npa_class,
        );
        if (final_npa_class == "S" || final_npa_class == "D" || final_npa_class == "L")
            && component == "PRINCIPAL"
        {
            match npa_writer.write_all(op_line.as_bytes()) {
                Ok(val) => val,
                Err(error) => {
                    panic!("Error writing processed data: {:?}", error);
                }
            }
        } 
        if final_npa_class != "S" && final_npa_class != "D" && final_npa_class != "L"{
            match performing_writer.write_all(op_line.as_bytes()) {
                Ok(val) => val,
                Err(error) => {
                    panic!("Error writing processed data: {:?}", error);
                }
            }
        }

        due_date = date_parser
            .parse_opt(input_account[4])
            .unwrap_or(default_date_parser.parse("01-01-1900"))
            .format("%d-%m-%Y")
            .to_string();

        if overdue_amt != 0.0
            && !enc_acids.contains(&acid.to_owned())
            && due_date
                != default_date_parser
                    .parse("01-01-1900")
                    .format("%d-%m-%Y")
                    .to_string()
            && date_parser
                .parse_opt(input_account[4])
                .unwrap_or(default_date_parser.parse("01-01-1900"))
                <= *config_param.as_on_date()
        {
            enc_acids.insert(acid.to_owned());
            component = "PRINCIPAL";
            cf_amt = overdue_amt;

            let op2 = get_op_line(
                input_account.to_owned(),
                cust_id,
                acid,
                user_def_stats,
                component.to_string(),
                npa_type,
                config_param.currency().to_string(),
                config_param.as_on_date(),
                overdue_amt,
                cf_amt,
                due_date,
                &cust_health_code,
                &cust_npa_class,
                &final_npa_class,
            );
            if (final_npa_class == "S" || final_npa_class == "D" || final_npa_class == "L")
            {
                match npa_writer.write_all(op2.as_bytes()) {
                    Ok(val) => val,
                    Err(error) => {
                        panic!("Error writing processed data: {:?}", error);
                    }
                }
            } else {
                match performing_writer.write_all(op2.as_bytes()) {
                    Ok(val) => val,
                    Err(error) => {
                        panic!("Error writing processed data: {:?}", error);
                    }
                }
            }
        }
    }

    println!(
        "Total Accounts present in input File: {}\n\
        Total Accounts present in NPA Data File: {}\n\
        Total Accounts present in NPA Live File: {}\n\
        Total Accounts present in NPA Config File: {}",
        tot_input_acc_encntrd, tot_data_acc_encntrd, tot_live_acc_encntrd, tot_config_acc_encntrd
    );
}
