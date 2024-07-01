use calamine::{open_workbook, Reader, Xlsx};
use calamine::{open_workbook_auto, DataType};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::default;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::SystemTime;

use crate::pre_processor::config::LnmData;
use crate::pre_processor::config::SheetName;
use crate::pre_processor::config::UbsData;
mod config;
mod derive_fields;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let mut lcr_master_sheets: SheetName = Default::default();
    let mut lcr_master_basel_sheets: String = "".to_string();
    let mut odfd_sheets: String = "".to_owned();
    let mut template_undrawn_sheets: SheetName = Default::default();
    let mut derived_flag = config_param.derived_flag();
    let files_config = config::get_files(config_param.config_file_path());
    for config_fields in files_config.files {
        lcr_master_sheets = config_fields.lcr_master_sheet_names;
        lcr_master_basel_sheets = config_fields.lcr_master_basel_sheet_name;
        odfd_sheets = config_fields.odfd_sheet_name;
        template_undrawn_sheets = config_fields.template_undrawn_sheet_name;
    }
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

    let mut lcr_master_excel = open_workbook_auto(config_param.lcr_master_file_path())
        .expect("Error opening LCR Master File.");
    println!(
        "Sheets present in LCR-Master-File: `{:?}`",
        lcr_master_excel.sheet_names()
    );
    if !lcr_master_excel
        .sheet_names()
        .contains(&lcr_master_sheets.ubs_sheet_name.to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in LCR-Master-File: `{}`",
            lcr_master_sheets.ubs_sheet_name, config_param.lcr_master_file_path
        );
    }
    println!(
        "Reading Sheet: `{}` from LCR-Master-File",
        lcr_master_sheets.ubs_sheet_name,
    );

    let mut ubs_map_lcr: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = lcr_master_excel.worksheet_range(&lcr_master_sheets.ubs_sheet_name) {
        for row in reader.rows() {
            let v_party_type_desc = get_str_from_xlsx(row, 0);
            let lcr_cat1 = get_str_from_xlsx(row, 1);
            ubs_map_lcr.insert(v_party_type_desc.to_ascii_lowercase(), lcr_cat1);
        }
    }
    if !lcr_master_excel
        .sheet_names()
        .contains(&lcr_master_sheets.lnm_sheet_name.to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in LCR-Master-File: `{}`",
            lcr_master_sheets.lnm_sheet_name, config_param.lcr_master_file_path
        );
    }
    println!(
        "Reading Sheet: `{}` from LCR-Master-File",
        lcr_master_sheets.lnm_sheet_name
    );
    let mut lnm_map_lcr: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = lcr_master_excel.worksheet_range(&lcr_master_sheets.lnm_sheet_name) {
        for row in reader.rows() {
            let v_exposure_id = get_str_from_xlsx(row, 0);
            let mapping = get_str_from_xlsx(row, 1);
            lnm_map_lcr.insert(v_exposure_id.to_lowercase(), mapping);
        }
    }

    //Reading LCR Classification Master_Basel.xlsx
    let mut lcr_basel_master_excel = open_workbook_auto(config_param.lcr_master_basel_path())
        .expect("Error opening LCR BASEL MASTER excel");
    println!(
        "Sheets present in LCR-Master-Basel-File: `{:?}`",
        lcr_basel_master_excel.sheet_names()
    );
    if !lcr_basel_master_excel
        .sheet_names()
        .contains(&lcr_master_basel_sheets.to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in LCR-Master-Basel-File: `{}`",
            lcr_master_basel_sheets, config_param.lcr_master_basel_path
        );
    }
    println!(
        "Reading Sheet: `{}` from LCR-Master-Basel-File",
        lcr_master_basel_sheets,
    );
    let mut lcr_basel_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = lcr_basel_master_excel.worksheet_range(&lcr_master_basel_sheets) {
        for row in reader.rows() {
            let v_basel_asset_class_desc = get_str_from_xlsx(row, 0);
            let classification = get_str_from_xlsx(row, 1);
            lcr_basel_map.insert(
                v_basel_asset_class_desc.to_ascii_lowercase(),
                classification,
            );
        }
    }
    let mut odfd_excel =
        open_workbook_auto(config_param.odfd_path()).expect("Error opening Funded File.");
    println!(
        "Sheets present in ODFD-Excel : `{:?}`",
        odfd_excel.sheet_names()
    );
    if !odfd_excel.sheet_names().contains(&odfd_sheets.to_string()) {
        panic!(
            "Sheet passed: `{}` not present in LCR-Master-Basel-File: `{}`",
            odfd_sheets,
            config_param.odfd_path()
        );
    }
    println!(
        "Reading Sheet: `{}` from LCR-Master-Basel-File",
        odfd_sheets,
    );
    let mut odfd_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = odfd_excel.worksheet_range(&odfd_sheets) {
        for row in reader.rows() {
            odfd_map.insert(
                row[0].to_string().trim().trim_matches('"').to_string(),
                row[1].to_string(),
            );
        }
    }

    //Reading Line Template Undrawn File
    let mut template_undrawn_excel = open_workbook_auto(config_param.line_template_undrawn_path())
        .expect("Error opening line template undrawn file.");
    println!(
        "Sheets present in Line Template Undrawn: `{:?}`",
        template_undrawn_excel.sheet_names()
    );
    if !template_undrawn_excel
        .sheet_names()
        .contains(&template_undrawn_sheets.ubs_sheet_name.to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in LCR-Master-Basel-File: `{}`",
            template_undrawn_sheets.ubs_sheet_name, config_param.line_template_undrawn_path
        );
    }
    println!(
        "Reading Sheet: `{}` from LCR-Master-Basel-File",
        template_undrawn_sheets.ubs_sheet_name,
    );
    let mut ubs_map_line: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) =
        template_undrawn_excel.worksheet_range(&template_undrawn_sheets.ubs_sheet_name)
    {
        for row in reader.rows() {
            let line_code = get_str_from_xlsx(row, 0);
            let description = get_str_from_xlsx(row, 1);
            let f_nf = get_str_from_xlsx(row, 2);
            ubs_map_line.insert(line_code.to_ascii_lowercase(), f_nf);
        }
    }
    if !template_undrawn_excel
        .sheet_names()
        .contains(&template_undrawn_sheets.lnm_sheet_name.to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in Template undrawn file: `{}`",
            template_undrawn_sheets.lnm_sheet_name, config_param.line_template_undrawn_path
        );
    }
    println!(
        "Reading Sheet: `{}` from LCR Template undrawn file",
        template_undrawn_sheets.lnm_sheet_name,
    );
    let mut lnm_map_line: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) =
        template_undrawn_excel.worksheet_range(&template_undrawn_sheets.lnm_sheet_name)
    {
        for row in reader.rows() {
            let v_exposure_id = get_str_from_xlsx(row, 0);
            let nine_digit = get_str_from_xlsx(row, 1);
            let f_nf = get_str_from_xlsx(row, 2);
            lnm_map_line.insert(nine_digit.to_ascii_lowercase(), f_nf);
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
        .replace(".txt", "IndMissingProd.txt");
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

    let reader = BufReader::new(input_file);
    let mut output_line = String::new();
    let mut ttl_amt = 0.0;
    let mut tot_acc_encntrd: i64 = 0;
    let mut skp_acc: i64 = 0;
    let mut missing_line = String::new();

    for (line_no, line) in reader.lines().enumerate().skip(1) {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(_error) => {
                panic!("Cannot read line from input file: {:?}", line_no);
            }
        };
        let fields: Vec<&str> = acc_info.split(config_param.delimiter()).collect();
        tot_acc_encntrd += 1;
        // if line_no == 0 {
        //     continue;
        // }
        if fields.len() < 59 {
            skp_acc += 1;
            log_error!(diag_log, "Cannot Process line no: {}", line_no + 1);
            continue;
        }
        // Derivation of FB/NFB
        let default_fb_nfb = "".to_string();
        let mut fb_nfp = &default_fb_nfb.clone();
        if derived_flag.to_string().trim().to_ascii_uppercase() == "UBS" {
            let key = fields[4].trim().to_string();
            if ubs_map_line.contains_key(&key.to_ascii_lowercase()) {
                let data = ubs_map_line
                    .get(&key.to_ascii_lowercase())
                    .unwrap_or(&default_fb_nfb);
                fb_nfp = data;
            }
        }

        if derived_flag.to_string().trim().to_ascii_uppercase() == "LNM" {
            let key = extract_last_9_digits(fields[1].trim()).unwrap_or("");
            if lnm_map_line.contains_key(&key.to_ascii_lowercase()) {
                let data = lnm_map_line
                    .get(&key.to_ascii_lowercase())
                    .unwrap_or(&default_fb_nfb);
                fb_nfp = data;
            }
        }
        // Derivation of CCOD Flag
        let ccod_flag = match odfd_map.get(fields[50].trim().trim_matches('"')) {
            Some(val) => val.trim(),
            None => {
                log_error!(
                    log,
                    "Missing ccod_flag for facility_desc: `{}`.",
                    fields[50]
                );
                ""
            }
        };

        // Derivation of LCR Category
        let default_lcr_cat = "Others".to_string();
        let mut lcr_category = &default_lcr_cat;
        if derived_flag.to_string().trim().to_ascii_uppercase() == "UBS" {
            let key = fields[31].trim();
            if ubs_map_lcr.contains_key(&key.to_ascii_lowercase()) {
                let data = ubs_map_lcr
                    .get(&key.to_ascii_lowercase())
                    .unwrap_or(&default_lcr_cat);
                lcr_category = data;
            }
        } else if derived_flag.to_string().trim().to_ascii_uppercase() == "LNM" {
            let key = fields[1].trim();
            if lnm_map_lcr.contains_key(&key.to_ascii_lowercase()) {
                let data = lnm_map_lcr
                    .get(&key.to_ascii_lowercase())
                    .unwrap_or(&default_lcr_cat);
                lcr_category = data;
            }
        }

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
            fields[30].trim()
        };

        // Derivation of Final Mapping for LCR
        let final_mapping_lcr = if lcr_category == "Others" {
            match lcr_basel_map.get(&asset_class_desc.to_ascii_lowercase()) {
                Some(val) => val.trim(),
                None => "Others",
            }
        } else {
            lcr_category
        };

        // Derivation of SLS Amount
        let exposure_amount: f64 = fields[24].trim().parse().unwrap_or(0.0);
        let undrawn_amount: f64 = fields[26].trim().parse().unwrap_or(0.0);
        let sls_amount = if ccod_flag == "Yes" {
            exposure_amount + undrawn_amount
        } else {
            0.0
        };

        // Derivation of LCR Amount
        let ccf_percent: f64 = fields[11].trim().parse().unwrap_or(0.0);
        let lcr_amount = if fb_nfp == "Funded" && ccf_percent > 0.0 {
            undrawn_amount
        } else {
            0.0
        };

        let cf_amt: f64 = fields[26].parse().unwrap_or(0.0);
        ttl_amt += cf_amt;

        let derived_output = derive_fields::get_output_line(
            fields,
            fb_nfp,
            ccod_flag,
            lcr_category,
            asset_class_desc,
            final_mapping_lcr,
            sls_amount,
            lcr_amount,
        );

        output_line.push_str(derived_output.as_str());

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
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`: {}.",
            config_param.output_file_path(),
            error,
        ),
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
pub fn get_str_from_xlsx(data: &[DataType], index: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` for row: `{:?}`",
                index + 1,
                data
            )
        })
        .to_string()
        .replace("\n", " ")
        .trim()
        .to_string()
}
fn extract_last_9_digits(input: &str) -> Option<&str> {
    if input.len() >= 9 {
        let last_9_chars = &input[input.len() - 9..];
        return Some(last_9_chars);
    }
    None
}
