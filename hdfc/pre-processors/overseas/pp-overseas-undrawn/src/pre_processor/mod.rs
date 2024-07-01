use calamine::open_workbook_auto;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::time::SystemTime;
mod derive_fields;

pub fn process(config_param: ConfigurationParameters, _log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();

    let input_file = match File::open(config_param.input_file_path()) {
        Ok(input_file) => input_file,
        Err(error) => panic!("{}", error),
    };

    let mut lcr_cat_excel =
        open_workbook_auto(config_param.lcr_cat_path()).expect("Error opening LCR Category File.");
    println!(
        "Sheets present in LCR-Category-File: `{:?}`",
        lcr_cat_excel.sheet_names()
    );
    if !lcr_cat_excel
        .sheet_names()
        .contains(&config_param.lcr_cat_sheet_name().to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in LCR-Master-File: `{}`",
            config_param.lcr_cat_sheet_name().to_string(),
            config_param.lcr_cat_path()
        );
    }
    println!(
        "Reading Sheet: `{}` from LCR-Master-File",
        config_param.lcr_cat_sheet_name().to_string(),
    );

    let mut lcr_cat_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = lcr_cat_excel.worksheet_range(&config_param.lcr_cat_sheet_name) {
        for row in reader.rows() {
            lcr_cat_map.insert(row[0].to_string().to_ascii_lowercase(), row[1].to_string());
        }
    }

    let mut cd_od_excel =
        open_workbook_auto(config_param.cd_od_path()).expect("Error opening CD OD WCDL Line File.");
    println!(
        "Sheets present in CD-OD File: `{:?}`",
        cd_od_excel.sheet_names()
    );
    if !cd_od_excel
        .sheet_names()
        .contains(&config_param.cd_od_sheet_name.to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in CD-OD File: `{}`",
            config_param.cd_od_sheet_name().to_string(),
            config_param.cd_od_path()
        );
    }
    println!(
        "Reading Sheet: `{}` from LCR-Master-File",
        config_param.cd_od_sheet_name().to_string(),
    );

    let mut cd_od_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = cd_od_excel.worksheet_range(config_param.cd_od_sheet_name()) {
        for row in reader.rows() {
            cd_od_map.insert(row[0].to_string(), row[1].to_string());
        }
    }

    let mut funded_excel =
        open_workbook_auto(config_param.funded_path()).expect("Error opening Funded File.");
    println!(
        "Sheets present in Funded File: `{:?}`",
        funded_excel.sheet_names()
    );
    if !funded_excel
        .sheet_names()
        .contains(&config_param.funded_sheet_name().to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in Fundede File: `{}`",
            config_param.funded_sheet_name().to_string(),
            config_param.funded_path()
        );
    }
    println!(
        "Reading Sheet: `{}` from LCR-Master-File",
        config_param.funded_sheet_name().to_string(),
    );
    let mut funded_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = funded_excel.worksheet_range(config_param.funded_sheet_name()) {
        for row in reader.rows() {
            funded_map.insert(row[0].to_string().to_ascii_lowercase(), row[1].to_string());
        }
    }

    let mut lcr_master_excel = open_workbook_auto(config_param.lcr_master_basel_path())
        .expect("Error opening Mater basel path.");
    println!(
        "Sheets present in Lcr master basel file: `{:?}`",
        lcr_master_excel.sheet_names()
    );
    if !lcr_master_excel
        .sheet_names()
        .contains(&config_param.lcr_master_sheet_name().to_string())
    {
        panic!(
            "Sheet passed: `{}` not LCR master basel file: `{}`",
            config_param.lcr_master_sheet_name().to_string(),
            config_param.lcr_master_basel_path()
        );
    }
    println!(
        "Reading Sheet: `{}` from LCR-Master-File",
        config_param.lcr_master_sheet_name().to_string(),
    );
    let mut lcr_master_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = lcr_master_excel.worksheet_range(config_param.lcr_master_sheet_name())
    {
        for row in reader.rows() {
            lcr_master_map.insert(row[0].to_string().to_ascii_lowercase(), row[1].to_string());
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
    let mut writer = BufWriter::new(output_file);
    let mut output_line = String::new();
    //adding header to output
    let header = "v_d_cust_ref_code|v_line_code|n_ccf_prcnt|n_undrawn_amt|v_basel_asset_class_desc|v_party_type_desc|gl_code|v_ccy_code|branch_code|country_code|lcr_category|asset_class_desc|final_map_lcr|f_uncond_cancelled_exp_ind|ccod_flag|fb_nfb\n";
    output_line.push_str(&header);
    let mut ttl_amt = 0.0;
    let mut tot_acc_encntrd: i64 = 0;
    let mut skp_acc: i64 = 0;
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

        if fields.len() != 59 {
            skp_acc += 1;
            log_error!(diag_log, "Cannot Process line no: {}", line_no + 1);
            continue;
        }
        if &fields[2][..3] != config_param.cust_ref_code {
            skp_acc += 1;
            continue;
        }
        // Derivation of Branch Code
        let branch_code = &fields[2][..3];

        // Derivation of Country Code
        let country_code = if branch_code == "300" {
            "GC"
        } else if branch_code == "000" {
            "BH"
        } else if branch_code == "100" {
            "HK"
        } else {
            ""
        };

        // Derivation of LCR Category
        let lcr_category = match lcr_cat_map.get(&fields[31].trim().to_ascii_lowercase()) {
            Some(val) => val.trim(),
            None => "Others",
        };
        let v_basel_asset_class_desc = fields[30].trim();
        let default = "Others".to_string();

        // Derivation of Asset Class Desc
        let mut asset_class_desc = fields[30].trim();   
        if lcr_category.trim().to_ascii_lowercase() == "NFC & PSE".trim().to_ascii_lowercase()
            || lcr_category.trim().to_ascii_lowercase() == "Retail & SME".trim().to_ascii_lowercase()
            || lcr_category.trim().to_ascii_lowercase() == "Financial Entity".trim().to_ascii_lowercase()
            || lcr_category.trim().to_ascii_lowercase() == "Banks".trim().to_ascii_lowercase()
        {
            asset_class_desc = "0";
        } else {
            asset_class_desc = &lcr_master_map
                .get(&v_basel_asset_class_desc.clone().to_ascii_lowercase())
                .unwrap_or(&default);
        };

        // Derivation of Final Mapping for LCR
        let final_mapping_lcr = if lcr_category == "Others" {
            match lcr_master_map.get(&asset_class_desc.to_string().to_ascii_lowercase()) {
                Some(val) => val.trim(),
                None => "Others",
            }
        } else {
            lcr_category
        };
        // Derivation of CCOD Flag
        let ccod_flag = match cd_od_map.get(fields[4].trim()) {
            Some(val) => val,
            None => "",
        };

        // Derivation of FB/NFB
        let fb_nfp = match funded_map.get(&fields[4].trim().to_ascii_lowercase()) {
            Some(val) => val,
            None => "",
        };

        let cf_amt: f64 = fields[26].parse().unwrap_or(0.0);
        ttl_amt += cf_amt;

        output_line.push_str(
            derive_fields::get_output_line(
                fields,
                branch_code,
                country_code,
                lcr_category,
                asset_class_desc,
                ccod_flag,
                fb_nfp,
                final_mapping_lcr,
            )
            .as_str(),
        );

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
