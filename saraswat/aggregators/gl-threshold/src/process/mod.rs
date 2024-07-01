use self::structs::GrpData;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::new_buf_rdr;
use slog::Logger;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};
use std::collections::HashMap;
use std::io::BufRead;

mod bucket_gen;
mod structs;
pub(crate) mod summary_gen;

pub fn process(config_params: ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut prod_rpt_map: HashMap<i64, GrpData> = HashMap::new();
    let mut llg_mapping: HashMap<i64, i64> = HashMap::new();
    let mut grp_amt_map: HashMap<String, f64> = HashMap::new();
    let exchange_rates = get_exchnage_rates(
        config_params.currency_conversion_file_path(),
        config_params.base_ccy(),
    );
    get_product_rpt_map(
        config_params.product_rpt_file_path(),
        logger,
        &mut prod_rpt_map,
        &mut llg_mapping,
    );
    get_llg_amt(
        &config_params,
        logger,
        diag_logger,
        &mut grp_amt_map,
        &exchange_rates,
        &llg_mapping,
    );
    let mut is_active_map: HashMap<String, String> = HashMap::new();
    let op_map = summary_gen::gen_summary(
        &config_params,
        logger,
        diag_logger,
        &prod_rpt_map,
        &llg_mapping,
        &mut is_active_map,
        &grp_amt_map,
        &exchange_rates,
    );
    bucket_gen::bucket_gen(
        &config_params,
        logger,
        &llg_mapping,
        &op_map,
        &prod_rpt_map,
        &mut is_active_map,
        &grp_amt_map,
    );
}

//this function reads the product_report excel and gets llg -> limit_amount
pub fn get_product_rpt_map(
    product_rpt_file: &str,
    logger: &Logger,
    mut pdt_rpt_map: &mut HashMap<i64, GrpData>,
    mut llg_mapping: &mut HashMap<i64, i64>,
) {
    //get the extension of file
    let file_type: Vec<&str> = product_rpt_file.split(".").collect();
    if file_type[1] == "xlsx" {
        let mut pdt_rpt_excel: Xlsx<_> =
            open_workbook(product_rpt_file).expect("Error while opening `product report file`.");
        let sheet_name = pdt_rpt_excel
            .sheet_names()
            .first()
            .unwrap_or(&"Sheet2".to_string())
            .to_owned();
        if let Some(Ok(reader)) = pdt_rpt_excel.worksheet_range(sheet_name.as_str()) {
            for row in reader.rows().skip(1) {
                let mut input_values = String::new();
                for i in row {
                    input_values.push_str((i.to_string()).as_str());
                    input_values.push_str("|");
                }
                input_values.pop();
                let line_contents = input_values.split("|").collect();
                get_llg_pdt_rtp_map(line_contents, &mut pdt_rpt_map, &mut llg_mapping);
            }
        }
    }
    if file_type[1] == "txt" {
        let rdr = match new_buf_rdr(product_rpt_file) {
            Ok(r) => r,
            Err(e) => panic!(format!(
                "Cannot read file at path: '{}', Error: '{}'",
                product_rpt_file, e
            )),
        };
        for line in rdr.lines() {
            if let Ok(each_line) = line {
                let line_contents: Vec<&str> = each_line.split("|").collect();
                get_llg_pdt_rtp_map(line_contents, &mut pdt_rpt_map, &mut llg_mapping);
            }
        }
    }
}

//this function gets exchnage rates for INR -> Anything
pub fn get_exchnage_rates(exchange_rate_file: &str, base_ccy: &str) -> HashMap<String, f64> {
    let mut exchanges_rates: HashMap<String, f64> = HashMap::new();
    let rdr = match new_buf_rdr(exchange_rate_file) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            exchange_rate_file, e
        )),
    };
    for line in rdr.lines() {
        if let Ok(each_line) = line {
            let line_contents: Vec<&str> = each_line.split("|").collect();
            if line_contents.len() < 3 {
                continue;
            }
            if line_contents[1].eq(base_ccy) {
                exchanges_rates.insert(line_contents[0].to_string(), str_to_flt(line_contents[2]));
            }
        }
    }
    exchanges_rates
}

pub fn str_to_flt(num: &str) -> f64 {
    num.parse().unwrap_or(DEFAULT_FLOAT)
}
pub fn str_to_int(num: &str) -> i64 {
    num.parse().unwrap_or(DEFAULT_INT)
}
pub fn flt_to_str(num: f64) -> String {
    num.to_string()
}
pub fn get_llg_amt(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    llg_amt_map: &mut HashMap<String, f64>,
    exchange_rates: &HashMap<String, f64>,
    llg_mapping: &HashMap<i64, i64>,
) {
    let input_file_name = format!("{}-summary.txt", config_params.input_file_path());
    let input_rdr = match new_buf_rdr(&input_file_name) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            config_params.input_file_path(),
            e
        )),
    };
    for line in input_rdr.lines() {
        match line {
            //if the line has no errors
            Ok(each_line) => {
                let line_vec: Vec<&str> = each_line.split("|").collect();
                if !llg_mapping.contains_key(&str_to_int(line_vec[0])) {
                    continue;
                }
                let grp_id = llg_mapping
                    .get(&str_to_int(line_vec[0]))
                    .expect("cannot fetch grp id for llg");

                let llg_amt_key = grp_id.to_string() + &"|".to_string() + &line_vec[2].to_string();
                let exrt = exchange_rates
                    .get(&line_vec[2].to_string())
                    .expect("Cant fetch exchange rate");

                if line_vec[3] != "SLR" && line_vec[3] != "ALL" {
                    continue;
                }
                let mut llg_amt = str_to_flt(line_vec[6]) * exrt;
                if llg_amt_map.contains_key(&llg_amt_key) {
                    let existing_amt = llg_amt_map
                        .get(&llg_amt_key)
                        .expect("Cant fetch amount for llg ccy combination");
                    llg_amt += existing_amt;
                }
                llg_amt_map.insert(llg_amt_key, llg_amt);
            }
            Err(..) => {}
        }
    }
}

pub fn get_llg_pdt_rtp_map(
    row: Vec<&str>,
    pdt_rpt_map: &mut HashMap<i64, GrpData>,
    llg_mapping: &mut HashMap<i64, i64>,
) {
    let limit: f64;
    if row[1].is_empty() {
        return;
    }
    if row[6].is_empty() {
        limit = 0.0;
    } else {
        limit = str_to_flt(row[6].to_string().as_str())
    }
    let mut grp_data: GrpData;
    let mut llg_vec: Vec<String> = Vec::new();
    if pdt_rpt_map.contains_key(&str_to_int(row[3].to_string().as_str())) {
        grp_data = pdt_rpt_map
            .get(&str_to_int(row[3].to_string().as_str()))
            .expect("Cannot fetch value for id")
            .to_owned();
        llg_vec = grp_data.llg_vec;
    }
    llg_vec.push(row[2].to_string());
    grp_data = GrpData {
        llg_vec: llg_vec,
        limit: limit,
        limit_llg: str_to_int(row[7].to_string().as_str()),
    };
    pdt_rpt_map.insert(str_to_int(row[1].to_string().as_str()), grp_data);
    llg_mapping.insert(
        str_to_int(row[2].to_string().as_str()),
        str_to_int(row[1].to_string().as_str()),
    );
}
