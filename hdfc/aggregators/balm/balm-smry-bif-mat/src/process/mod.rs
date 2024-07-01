use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::new_buf_rdr;
use slog::Logger;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};
use std::collections::HashMap;
use std::io::BufRead;

mod bucket_gen;
pub(crate) mod summary_gen;

pub fn process(config_params: ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let prod_rpt_map = get_product_rpt_map(config_params.product_rpt_file_path(), logger);
    let llg_mapping = get_llg_mapping(config_params.llg_mapping_file_path());
    let exchange_rates = get_exchnage_rates(config_params.currency_conversion_file_path());
    let llg_conversion_ration = summary_gen::gen_summary(
        &config_params,
        logger,
        diag_logger,
        &prod_rpt_map,
        &llg_mapping,
        &exchange_rates,
    );
    bucket_gen::bucket_gen(&config_params, logger, &llg_mapping, &llg_conversion_ration);
}

//this function reads the product_report excel and gets llg -> limit_amount
pub fn get_product_rpt_map(product_rpt_file: &str, logger: &Logger) -> HashMap<i64, f64> {
    let mut pdt_rpt_map: HashMap<i64, f64> = HashMap::new();
    let mut pdt_rpt_excel: Xlsx<_> =
        open_workbook(product_rpt_file).expect("Error while opening `product report file`.");
    let sheet_name = pdt_rpt_excel
        .sheet_names()
        .first()
        .unwrap_or(&"Sheet1".to_string())
        .to_owned();
    if let Some(Ok(reader)) = pdt_rpt_excel.worksheet_range(sheet_name.as_str()) {
        for row in reader.rows().skip(1) {
            let limit: f64;
            if row[3].is_empty() {
                continue;
            }
            if row[11].is_empty() {
                limit = 0.0;
            } else {
                limit = str_to_flt(row[11].to_string().as_str())
            }
            pdt_rpt_map.insert(str_to_int(row[3].to_string().as_str()), limit);
        }
    }
    pdt_rpt_map
}

//this function gets exchnage rates for INR -> Anything
pub fn get_exchnage_rates(exchange_rate_file: &str) -> HashMap<String, f64> {
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
            if line_contents[0].eq("INR") {
                exchanges_rates.insert(line_contents[1].to_string(), str_to_flt(line_contents[2]));
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
//this function reads the llgmaping file and return a hashmap<from_llg, to_llog>
fn get_llg_mapping(llg_mapping_file: &str) -> HashMap<i64, i64> {
    let mut llg_map: HashMap<i64, i64> = HashMap::new();
    let rdr = match new_buf_rdr(llg_mapping_file) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            llg_mapping_file, e
        )),
    };
    for line in rdr.lines() {
        if let Ok(each_line) = line {
            let line_contents: Vec<&str> = each_line.split("|").collect();
            if line_contents.len() < 2 {
                continue;
            }
            llg_map.insert(str_to_int(line_contents[0]), str_to_int(line_contents[1]));
        }
    }
    llg_map
}
