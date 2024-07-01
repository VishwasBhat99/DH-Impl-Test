use self::output_lines::get_output_line;
use calamine::open_workbook_auto;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::time::SystemTime;

mod output_lines;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();

    let input_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => {
            log_error!(
                log,
                "Could not found input file: `{}`",
                config_param.input_file_path()
            );
            panic!(
                "Could not found input file: `{}` on location `{}` : {}.",
                config_param.input_file_path(),
                current_dir()
                    .expect("Error while getting current directory path.")
                    .display(),
                error
            )
        }
    };

    let mut asset_class_excel = open_workbook_auto(config_param.asset_class_path())
        .expect("Error opening Asset classs file.");
    println!(
        "Sheets present in Asset class file: `{:?}`",
        asset_class_excel.sheet_names()
    );
    if !asset_class_excel
        .sheet_names()
        .contains(&config_param.asset_class_sheet_name().to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in Asset Class file: `{}`",
            config_param.asset_class_sheet_name().to_string(),
            config_param.asset_class_path()
        );
    }
    println!(
        "Reading Sheet: `{}` from Asset class file",
        config_param.asset_class_sheet_name().to_string(),
    );
    let mut asset_class_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) =
        asset_class_excel.worksheet_range(config_param.asset_class_sheet_name())
    {
        for row in reader.rows() {
            asset_class_map.insert(row[0].to_string().to_ascii_lowercase(), row[1].to_string());
        }
    }
    let mut lcr_cat_excel =
        open_workbook_auto(config_param.lcr_cat_path()).expect("Error opening LCR Category File.");
    println!(
        "Sheets present in LCR Category file: `{:?}`",
        lcr_cat_excel.sheet_names()
    );
    // println!("{:?}",config_param.lcr_cat_sheet_name());
    if !lcr_cat_excel
        .sheet_names()
        .contains(&config_param.lcr_cat_sheet_name().trim().to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in LCR Category file: `{}`",
            config_param.lcr_cat_sheet_name(),
            config_param.lcr_cat_path()
        );
    }
    println!(
        "Reading Sheet: `{}` from LCR Category file",
        config_param.lcr_cat_sheet_name().to_string(),
    );
    let mut lcr_cat_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) =
        lcr_cat_excel.worksheet_range(config_param.lcr_cat_sheet_name().trim())
    {
        for row in reader.rows() {
            lcr_cat_map.insert(row[0].to_string().to_ascii_lowercase(), row[1].to_string());
        }
    }
    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration read timer.");
    debug!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );
    let start_derive_timer = SystemTime::now();

    let output_file = match File::create(config_param.output_file_path()) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };

    let reader = BufReader::new(input_file);
    let mut writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };
    let missing_file_path = config_param
        .output_file_path()
        .replace(".txt", "SmryMissingProd.txt");
    let mut missing_lines_writer = match buf_file_wrtr(&missing_file_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            missing_file_path,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };
    let mut output_line = String::new();
    let mut ttl_amt = 0.0;
    let mut tot_acc_encntrd: i64 = 0;
    let mut skp_acc: i64 = 0;
    let mut missing_line = String::new();
    let v_src_system_ids: Vec<&str> = config_param
        .v_src_system_ids()
        .split(',')
        .map(|s| s.trim())
        .collect();
    for (line_no, line) in reader.lines().enumerate() {
        if line_no == 0 {
            continue;
        }
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
        let fields: Vec<&str> = acc_info.split(config_param.delimiter()).collect();
        tot_acc_encntrd += 1;

        if fields.len() != 47 {
            skp_acc += 1;
            log_error!(log, "Cannot Process line no: {}", line_no + 1);
            continue;
        }

        if fields[0] == "" {
            log_info!(
                log,
                "`v_src_sys_id` not present for the account, Skipping Process line no: {}",
                line_no + 1
            );
            continue;
        }

        // Derivation of LCR Category
        let lcr_category =
            match lcr_cat_map.get(&fields[17].trim().to_string().to_ascii_lowercase()) {
                Some(val) => val.trim(),
                None => "Others",
            };
        if lcr_category == "" {
            missing_line.push_str(&acc_info);
            missing_line.push_str("\n");
        }
        // Derivation of Asset Class Desc
        let asset_class_desc = if lcr_category == "NFC & PSE"
            || lcr_category == "Retail & SME"
            || lcr_category == "Financial Entity"
            || lcr_category == "Banks"
        {
            "0"
        } else {
            fields[16].trim()
        };

        // Derivation of Final Mapping for LCR
        let final_mapping_lcr = if lcr_category == "Others" {
            match asset_class_map.get(&asset_class_desc.to_string().to_ascii_lowercase()) {
                Some(val) => val.trim(),
                None => "Others",
            }
        } else {
            lcr_category
        };

        // Derivation of SLS Amount
        let exposure_amount: f64 = fields[29].trim().parse().unwrap_or(0.0);
        let undrawn_amount: f64 = fields[31].trim().parse().unwrap_or(0.0);
        let mut sls_amount = 0.0;
        let mut lcr_amount = 0.0;
        if v_src_system_ids.contains(&fields[0]) {
            sls_amount = exposure_amount + undrawn_amount;
            // Derivation of LCR Amount
            if fields[6].trim().parse().unwrap_or(0.0) > 0.0 {
                lcr_amount = undrawn_amount
            }
        }

        let op_line = get_output_line(
            fields,
            lcr_category,
            asset_class_desc,
            final_mapping_lcr,
            sls_amount,
            lcr_amount,
        );
        output_line.push_str(&op_line.to_string());
        ttl_amt += undrawn_amount;
        log_debug!(diag_log, "Processed line no: {}", line_no + 1);
    }
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);
    let start_write_timer = SystemTime::now();
    log_debug!(diag_log, "Total cf amount: {}", ttl_amt);
    match writer.write_all(output_line.as_bytes()) {
        Ok(_val) => println!("Successfully processed all accounts"),
        Err(error) => {
            panic!("Cannot pre process the input file: {:?}", error);
        }
    }
    match missing_lines_writer.write_all(missing_line.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`: {}.",
            missing_file_path, error,
        ),
    }

    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing pre-processed output and reconcilation files.");
    debug!(
        diag_log,
        "Writing Records and Reconcilation File, Total Duration: {:?}.", duration
    );

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skp_acc,
        skp_acc,
        ttl_amt,
        ttl_amt,
        0,
    );
    health_report.gen_health_rpt(&config_param.output_file_path());
}
